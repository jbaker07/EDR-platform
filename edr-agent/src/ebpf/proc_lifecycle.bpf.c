#ifndef __maybe_unused
#define __maybe_unused __attribute__((unused))
#endif
// include/edr_events.h  (BPF-safe, no /usr/include/linux/*)

#ifndef EDR_EVENTS_H
#define EDR_EVENTS_H

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_endian.h>

/* Fallbacks only; vmlinux.h already has the structs */
#ifndef AF_INET
#define AF_INET 2
#endif
#ifndef AF_INET6
#define AF_INET6 10
#endif

/* Event types (override via -D if already defined elsewhere) */
#ifndef EVT_PROC_EXEC_TP
#define EVT_PROC_EXEC_TP  1001
#endif
#ifndef EVT_PROC_FORK
#define EVT_PROC_FORK     1002
#endif
#ifndef EVT_PROC_EXIT
#define EVT_PROC_EXIT     1003
#endif

/* Bytes of argv copied at exec. The rest is cut, argv_total says how long it was. */
#define PROC_LC_ARGV_CAP 256

/* The frame this file writes to the shared ring, 576 bytes.
 * The first 312 bytes are the older layout, unchanged, so the decoder still reads
 * frames from an object built before argv was added. New fields go at the end. */
struct proc_lc_event {
    __u64 ts;            // monotonic ns
    __u32 type;          // EVT_*
    __s32 syscall_id;    // always 0, part of the magic the decoder checks
    __u32 tgid;          // process id
    __u32 ppid;          // real_parent->tgid
    __u32 uid;           // uid at emit time
    __s32 fd;            // always -1, part of the magic
    __s32 ret;           // always 0
    __u32 aux_u32;       // exit: task->exit_code
    char  comm[16];      // task comm
    char  path[256];     // exec: filename the tracepoint reports, else empty
    __u32 argv_len;      // exec: bytes copied into argv
    __u32 argv_total;    // exec: arg_end - arg_start, the full argv length
    char  argv[PROC_LC_ARGV_CAP]; // exec: NUL-separated arguments
};

_Static_assert(sizeof(struct proc_lc_event) == 576, "struct proc_lc_event must be 576 bytes");

/* Same map name as raw_syscalls_catcher, so edr_attach_any points it at the shared ring */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 24); // 16MB
} edr_events_rb SEC(".maps");

/* Tiny helpers */
static __always_inline __maybe_unused __u32 edr_tgid(void) {
    return (__u32)(bpf_get_current_pid_tgid() >> 32);
}
static __always_inline __maybe_unused __u32 edr_uid(void) {
    return (__u32)(bpf_get_current_uid_gid() & 0xffffffff);
}

#endif /* EDR_EVENTS_H */

/* ===================== proc_lifecycle programs ===================== */

/* The frame is too big for the 512-byte BPF stack, so it is built in ring memory.
 * Ring memory is not zeroed and the string copies stop at the NUL, so clear it first. */
static __always_inline struct proc_lc_event *lc_reserve(__u32 type)
{
    struct proc_lc_event *ev = bpf_ringbuf_reserve(&edr_events_rb, sizeof(*ev), 0);
    if (!ev)
        return 0; // ring full, the event is dropped
    __builtin_memset(ev, 0, sizeof(*ev));

    ev->ts         = bpf_ktime_get_ns();
    ev->type       = type;
    ev->syscall_id = 0;
    ev->tgid       = edr_tgid();
    ev->uid        = edr_uid();
    ev->fd         = -1;
    ev->ret        = 0;
    ev->aux_u32    = 0;

    struct task_struct *t = (void *)bpf_get_current_task();
    ev->ppid = BPF_CORE_READ(t, real_parent, tgid);
    bpf_get_current_comm(&ev->comm, sizeof(ev->comm));
    return ev;
}

/* sched_process_fork */
SEC("tracepoint/sched/sched_process_fork")
int tp_sched_process_fork(void *ctx)
{
    struct proc_lc_event *ev = lc_reserve(EVT_PROC_FORK);
    if (!ev)
        return 0;
    bpf_ringbuf_submit(ev, 0);
    return 0;
}

/* sched_process_exec
 * A short-lived process can exit before user space reads /proc/<pid>, so the filename
 * and the start of argv are copied here, while the process still exists. */
SEC("tracepoint/sched/sched_process_exec")
int tp_sched_process_exec(struct trace_event_raw_sched_process_exec *ctx)
{
    struct proc_lc_event *ev = lc_reserve(EVT_PROC_EXEC_TP);
    if (!ev)
        return 0;

    /* filename is a __data_loc string: the low 16 bits are its offset in the record */
    unsigned int off = ctx->__data_loc_filename & 0xFFFF;
    bpf_probe_read_kernel_str(ev->path, sizeof(ev->path), (void *)ctx + off);

    /* argv of the new image. exec copied it to the new stack just before this
     * tracepoint fires, so the pages are present. */
    struct task_struct *t = (void *)bpf_get_current_task();
    unsigned long start = BPF_CORE_READ(t, mm, arg_start);
    unsigned long end   = BPF_CORE_READ(t, mm, arg_end);
    if (start && end > start) {
        /* n stays 64-bit and barrier_var follows the clamp, the same fix as in
         * dns_tunnel_monitor.bpf.c. Otherwise clang can pass an unclamped copy of n
         * to the helper and the verifier rejects the size. */
        __u64 n = end - start;
        ev->argv_total = (__u32)n;
        if (n > PROC_LC_ARGV_CAP)
            n = PROC_LC_ARGV_CAP;
        barrier_var(n);
        if (bpf_probe_read_user(ev->argv, n, (void *)start) == 0)
            ev->argv_len = (__u32)n;
    }

    bpf_ringbuf_submit(ev, 0);
    return 0;
}

/* sched_process_exit */
SEC("tracepoint/sched/sched_process_exit")
int tp_sched_process_exit(void *ctx)
{
    struct proc_lc_event *ev = lc_reserve(EVT_PROC_EXIT);
    if (!ev)
        return 0;

    struct task_struct *t = (void *)bpf_get_current_task();
    int exit_code = BPF_CORE_READ(t, exit_code);
    ev->aux_u32 = (__u32)exit_code;

    bpf_ringbuf_submit(ev, 0);
    return 0;
}

char LICENSE[] SEC("license") = "Dual BSD/GPL";

// edr-agent/src/ebpf/cred_dump_monitor.c

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_tracing.h>

#ifndef __user
#define __user
#endif

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct cred_dump_event_t {
    __u32 pid;
    __u32 uid;
    char  comm[64];
    char  details[128];
};

/* match loader’s shared ringbuf (already pinned in /sys/fs/bpf/edr/events) */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20);
} events SEC(".maps");

/* tiny prefix check for "/etc/shadow" (11 chars) */
static __always_inline bool is_etc_shadow(const char *s)
{
    return s[0]=='/' && s[1]=='e' && s[2]=='t' && s[3]=='c' && s[4]=='/' &&
           s[5]=='s' && s[6]=='h' && s[7]=='a' && s[8]=='d' && s[9]=='o' && s[10]=='w';
}

SEC("tracepoint/syscalls/sys_enter_ptrace")
int trace_cred_dump_ptrace(struct trace_event_raw_sys_enter *ctx)
{
    struct cred_dump_event_t ev = {};
    ev.pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    ev.uid = (__u32)(bpf_get_current_uid_gid() & 0xffffffff);
    bpf_get_current_comm(&ev.comm, sizeof(ev.comm));
    __builtin_memcpy(ev.details,
                     "syscall ptrace triggered - potential credential dump",
                     56);
    /* ringbuf copies from stack */
    bpf_ringbuf_output(&events, &ev, sizeof(ev), 0);
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_openat")
int trace_open_shadow_file(struct trace_event_raw_sys_enter *ctx)
{
    char fname[256] = {};
    unsigned long filename_ptr = 0;

    BPF_CORE_READ_INTO(&filename_ptr, ctx, args[1]);
    const char __user *filename = (const char __user *)filename_ptr;
    bpf_probe_read_user_str(fname, sizeof(fname), filename);

    if (is_etc_shadow(fname)) {
        struct cred_dump_event_t ev = {};
        ev.pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
        ev.uid = (__u32)(bpf_get_current_uid_gid() & 0xffffffff);
        bpf_get_current_comm(&ev.comm, sizeof(ev.comm));
        __builtin_memcpy(ev.details, "attempt to open /etc/shadow detected", 38);
        bpf_ringbuf_output(&events, &ev, sizeof(ev), 0);
    }
    return 0;
}

// mem_scan.bpf.c  (ringbuf version)

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_tracing.h>

// Userspace-style mmap PROT_* flags
#define PROT_READ   0x1
#define PROT_WRITE  0x2
#define PROT_EXEC   0x4

struct mem_event {
    __u32 pid;
    __u32 uid;
    __u32 ppid;
    char  comm[64];
    char  details[128];
};

/* Ring buffer named "events" (loader will try to reuse shared edr_events_rb) */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20); // 1 MiB if not reused
} events SEC(".maps");

/* Intercept mmap to detect RWX or otherwise executable mappings */
SEC("tracepoint/syscalls/sys_enter_mmap")
int trace_mmap(struct trace_event_raw_sys_enter *ctx)
{
    struct task_struct *task = (struct task_struct *)bpf_get_current_task();
    __u32 pid  = (__u32)(bpf_get_current_pid_tgid() >> 32);
    __u32 uid  = (__u32)(bpf_get_current_uid_gid());
    __u32 ppid = BPF_CORE_READ(task, real_parent, tgid);

    char comm[64] = {};
    bpf_get_current_comm(&comm, sizeof(comm));

    unsigned long prot = ctx->args[2]; // PROT flags

    if ((prot & PROT_EXEC) && (prot & PROT_WRITE) && (prot & PROT_READ)) {
        struct mem_event *evt = bpf_ringbuf_reserve(&events, sizeof(*evt), 0);
        if (!evt)
            return 0;

        evt->pid  = pid;
        evt->uid  = uid;
        evt->ppid = ppid;
        __builtin_memcpy(evt->comm, comm, sizeof(evt->comm));
        __builtin_memcpy(evt->details, "RWX mmap region detected", 27);

        bpf_ringbuf_submit(evt, 0);
    }
    return 0;
}

/* Intercept execve as a signal that code may be getting launched */
SEC("tracepoint/syscalls/sys_enter_execve")
int trace_execve_mem(struct trace_event_raw_sys_enter *ctx)
{
    struct task_struct *task = (struct task_struct *)bpf_get_current_task();
    __u32 pid  = (__u32)(bpf_get_current_pid_tgid() >> 32);
    __u32 uid  = (__u32)(bpf_get_current_uid_gid());
    __u32 ppid = BPF_CORE_READ(task, real_parent, tgid);

    char comm[64] = {};
    bpf_get_current_comm(&comm, sizeof(comm));

    struct mem_event *evt = bpf_ringbuf_reserve(&events, sizeof(*evt), 0);
    if (!evt)
        return 0;

    evt->pid  = pid;
    evt->uid  = uid;
    evt->ppid = ppid;
    __builtin_memcpy(evt->comm, comm, sizeof(evt->comm));
    __builtin_memcpy(evt->details, "execve called (possible shell)", 33);

    bpf_ringbuf_submit(evt, 0);
    return 0;
}

char LICENSE[] SEC("license") = "Dual BSD/GPL";

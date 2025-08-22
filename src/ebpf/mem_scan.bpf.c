// mem_scan.bpf.c

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_tracing.h>

// These are userspace protection flags (man mmap), defined here manually
#define PROT_READ   0x1
#define PROT_WRITE  0x2
#define PROT_EXEC   0x4

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct mem_event {
    u32 pid;
    u32 uid;
    u32 ppid;
    char comm[64];
    char details[128];
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} events SEC(".maps");

// Intercept mmap syscall to detect RWX or executable mappings
SEC("tracepoint/syscalls/sys_enter_mmap")
int trace_mmap(struct trace_event_raw_sys_enter *ctx) {
    struct mem_event evt = {};
    struct task_struct *task = (struct task_struct *)bpf_get_current_task();

    evt.pid = bpf_get_current_pid_tgid() >> 32;
    evt.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;
    evt.ppid = BPF_CORE_READ(task, real_parent, tgid);
    bpf_get_current_comm(&evt.comm, sizeof(evt.comm));

    unsigned long prot = ctx->args[2];

    if ((prot & PROT_EXEC) && (prot & PROT_WRITE) && (prot & PROT_READ)) {
        __builtin_memcpy(evt.details, "RWX mmap region detected", 27);
        bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &evt, sizeof(evt));
    }

    return 0;
}

// Intercept execve to catch suspicious payload loads
SEC("tracepoint/syscalls/sys_enter_execve")
int trace_execve_mem(struct trace_event_raw_sys_enter *ctx) {
    struct mem_event evt = {};
    struct task_struct *task = (struct task_struct *)bpf_get_current_task();

    evt.pid = bpf_get_current_pid_tgid() >> 32;
    evt.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;
    evt.ppid = BPF_CORE_READ(task, real_parent, tgid);
    bpf_get_current_comm(&evt.comm, sizeof(evt.comm));

    __builtin_memcpy(evt.details, "execve called (possible shell)", 33);
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &evt, sizeof(evt));

    return 0;
}

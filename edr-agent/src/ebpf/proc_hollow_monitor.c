#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct hollow_event_t {
    u32 pid;
    u32 ppid;
    u32 uid;
    char comm[64];
    char details[128];
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} events SEC(".maps");

// Tracepoint for execve — monitor for process hollowing patterns
SEC("tracepoint/syscalls/sys_enter_execve")
int trace_execve_hollowing(struct trace_event_raw_sys_enter *ctx) {
    struct hollow_event_t event = {};
    struct task_struct *task;

    // Basic context
    event.pid = bpf_get_current_pid_tgid() >> 32;
    event.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;
    bpf_get_current_comm(&event.comm, sizeof(event.comm));

    // Get parent PID (ppid)
    task = (struct task_struct *)bpf_get_current_task_btf();
    struct task_struct *real_parent = BPF_CORE_READ(task, real_parent);
    event.ppid = BPF_CORE_READ(real_parent, pid);

    // Emit metadata
    __builtin_memcpy(event.details, "execve event (check for hollowing)", 35);
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    return 0;
}

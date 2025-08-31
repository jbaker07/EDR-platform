// proc_hollow_monitor.c (ringbuf)

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

/* Ring buffer for events.
 * edr_attach_any will reuse a shared ringbuf named "edr_events_rb" if present.
 */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20);   // 1 MiB
} events SEC(".maps");

// Tracepoint for execve — monitor for process hollowing patterns
SEC("tracepoint/syscalls/sys_enter_execve")
int trace_execve_hollowing(struct trace_event_raw_sys_enter *ctx)
{
    struct hollow_event_t evt = {};
    struct task_struct *task;

    // Basic context
    evt.pid = bpf_get_current_pid_tgid() >> 32;
    evt.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;
    bpf_get_current_comm(&evt.comm, sizeof(evt.comm));

    // Parent TGID as PPID
    task = (struct task_struct *)bpf_get_current_task_btf();
    if (task) {
        struct task_struct *real_parent = BPF_CORE_READ(task, real_parent);
        evt.ppid = BPF_CORE_READ(real_parent, tgid);
    }

    __builtin_memcpy(evt.details, "execve event (check for hollowing)", 35);

    // Emit via ring buffer
    bpf_ringbuf_output(&events, &evt, sizeof(evt), 0);
    return 0;
}


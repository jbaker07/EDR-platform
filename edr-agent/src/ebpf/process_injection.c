#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct injection_evt {
    __u32 pid;
    __u32 target_pid;
    __u64 timestamp;
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} EVENTS SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_process_vm_writev")
int trace_proc_inject(struct trace_event_raw_sys_enter* ctx) {
    struct injection_evt evt = {};
    evt.pid = bpf_get_current_pid_tgid() >> 32;
    evt.target_pid = ctx->args[0]; // target pid
    evt.timestamp = bpf_ktime_get_ns();

    bpf_perf_event_output(ctx, &EVENTS, BPF_F_CURRENT_CPU, &evt, sizeof(evt));
    return 0;
}

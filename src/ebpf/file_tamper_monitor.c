#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct file_tamper_evt {
    __u32 pid;
    __u64 timestamp;
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} EVENTS SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_unlinkat")
int trace_file_tamper(struct trace_event_raw_sys_enter* ctx) {
    struct file_tamper_evt evt = {};
    evt.pid = bpf_get_current_pid_tgid() >> 32;
    evt.timestamp = bpf_ktime_get_ns();

    bpf_perf_event_output(ctx, &EVENTS, BPF_F_CURRENT_CPU, &evt, sizeof(evt));
    return 0;
}

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct ipc_evt {
    __u32 pid;
    __u32 peer_pid;
    __u8 ipc_type; // 0=shm, 1=pipe, etc.
    __u64 timestamp;
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} EVENTS SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_msgsnd")
int trace_suspicious_ipc(struct trace_event_raw_sys_enter* ctx) {
    struct ipc_evt evt = {};
    evt.pid = bpf_get_current_pid_tgid() >> 32;
    evt.peer_pid = ctx->args[0]; // msg queue id or inferred peer
    evt.ipc_type = 1; // msg
    evt.timestamp = bpf_ktime_get_ns();

    bpf_perf_event_output(ctx, &EVENTS, BPF_F_CURRENT_CPU, &evt, sizeof(evt));
    return 0;
}

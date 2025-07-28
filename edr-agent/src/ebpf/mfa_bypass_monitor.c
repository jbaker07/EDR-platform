// edr-agent/src/ebpf/mfa_bypass_monitor.c

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct mfa_bypass_event_t {
    u32 pid;
    u32 uid;
    char comm[64];
    char details[128];
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} events SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_execve")
int trace_mfa_anomaly(struct trace_event_raw_sys_enter *ctx) {
    struct mfa_bypass_event_t event = {};
    event.pid = bpf_get_current_pid_tgid() >> 32;
    event.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;

    bpf_get_current_comm(&event.comm, sizeof(event.comm));
    __builtin_memcpy(event.details, "execve anomaly (potential MFA bypass)", 38);

    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    return 0;
}

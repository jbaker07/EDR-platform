// edr-agent/src/ebpf/ipc_abuse_monitor.c

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct ipc_abuse_event_t {
    u32 pid;
    u32 uid;
    char comm[64];
    char ipc_call[32];
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} events SEC(".maps");

// Watch for suspicious IPC syscalls like shmget or msgsnd

SEC("tracepoint/syscalls/sys_enter_shmget")
int trace_ipc_shmget(struct trace_event_raw_sys_enter *ctx) {
    struct ipc_abuse_event_t event = {};
    event.pid = bpf_get_current_pid_tgid() >> 32;
    event.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;
    bpf_get_current_comm(&event.comm, sizeof(event.comm));
    __builtin_memcpy(event.ipc_call, "shmget", 6);

    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_msgsnd")
int trace_ipc_msgsnd(struct trace_event_raw_sys_enter *ctx) {
    struct ipc_abuse_event_t event = {};
    event.pid = bpf_get_current_pid_tgid() >> 32;
    event.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;
    bpf_get_current_comm(&event.comm, sizeof(event.comm));
    __builtin_memcpy(event.ipc_call, "msgsnd", 6);

    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    return 0;
}

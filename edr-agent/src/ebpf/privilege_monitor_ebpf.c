#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "GPL";

// Output structure for perf events
struct priv_event {
    u32 pid;
    u32 uid;
    u32 old_uid;
    char comm[16];
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} events SEC(".maps");

// Trace setuid syscall
SEC("tracepoint/syscalls/sys_enter_setuid")
int trace_setuid(struct trace_event_raw_sys_enter *ctx) {
    struct priv_event event = {};
    event.pid = bpf_get_current_pid_tgid() >> 32;
    event.uid = ctx->args[0];
    event.old_uid = bpf_get_current_uid_gid() & 0xffffffff;
    bpf_get_current_comm(&event.comm, sizeof(event.comm));
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    return 0;
}

// Trace setresuid syscall
SEC("tracepoint/syscalls/sys_enter_setresuid")
int trace_setresuid(struct trace_event_raw_sys_enter *ctx) {
    struct priv_event event = {};
    event.pid = bpf_get_current_pid_tgid() >> 32;
    event.uid = ctx->args[1];  // effective UID
    event.old_uid = bpf_get_current_uid_gid() & 0xffffffff;
    bpf_get_current_comm(&event.comm, sizeof(event.comm));
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    return 0;
}

// Trace execve for sudo or su
SEC("tracepoint/syscalls/sys_enter_execve")
int trace_execve(struct trace_event_raw_sys_enter *ctx) {
    struct priv_event event = {};
    event.pid = bpf_get_current_pid_tgid() >> 32;
    event.uid = bpf_get_current_uid_gid() & 0xffffffff;
    event.old_uid = event.uid;
    bpf_get_current_comm(&event.comm, sizeof(event.comm));
    if (event.comm[0] == 's' && (event.comm[1] == 'u' || event.comm[1] == 'u')) {
        bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    }
    return 0;
}

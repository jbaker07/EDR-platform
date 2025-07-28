#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "Dual MIT/GPL";

#define MAX_BUF_SIZE 512

struct encrypted_payload_event {
    u32 pid;
    u32 bytes_requested;
    u32 fd;
    u64 timestamp;
    u8  buffer_preview[32]; // first 32 bytes
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} EVENTS SEC(".maps");

// Hook read syscall
SEC("tracepoint/syscalls/sys_enter_read")
int trace_read(struct trace_event_raw_sys_enter* ctx) {
    struct encrypted_payload_event event = {};
    u64 pid_tgid = bpf_get_current_pid_tgid();
    event.pid = pid_tgid >> 32;
    event.fd = ctx->args[0];
    event.bytes_requested = ctx->args[2];
    event.timestamp = bpf_ktime_get_ns();

    // Capture address of buffer to read into
    void* user_buf = (void*)ctx->args[1];
    bpf_probe_read_user(&event.buffer_preview, sizeof(event.buffer_preview), user_buf);

    bpf_perf_event_output(ctx, &EVENTS, BPF_F_CURRENT_CPU, &event, sizeof(event));
    return 0;
}

// Hook recvfrom syscall
SEC("tracepoint/syscalls/sys_enter_recvfrom")
int trace_recvfrom(struct trace_event_raw_sys_enter* ctx) {
    struct encrypted_payload_event event = {};
    u64 pid_tgid = bpf_get_current_pid_tgid();
    event.pid = pid_tgid >> 32;
    event.fd = ctx->args[0];
    event.bytes_requested = ctx->args[2];
    event.timestamp = bpf_ktime_get_ns();

    void* user_buf = (void*)ctx->args[1];
    bpf_probe_read_user(&event.buffer_preview, sizeof(event.buffer_preview), user_buf);

    bpf_perf_event_output(ctx, &EVENTS, BPF_F_CURRENT_CPU, &event, sizeof(event));
    return 0;
}

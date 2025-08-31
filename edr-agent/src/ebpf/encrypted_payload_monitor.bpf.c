// edr-agent/src/ebpf/encrypted_payload_monitor.bpf.c (ringbuf version)

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "Dual MIT/GPL";

struct encrypted_payload_event {
    __u32 pid;
    __u32 bytes_requested;
    __u32 fd;
    __u64 timestamp;
    __u8  buffer_preview[32]; // first 32 bytes
};

/* Ring buffer named "events" so edr_attach_any can reuse the shared
 * /sys/fs/bpf/edr/edr_events_rb if it’s already pinned.
 */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20); // 1 MiB fallback if not reusing shared rb
} events SEC(".maps");

/* sys_enter_read */
SEC("tracepoint/syscalls/sys_enter_read")
int trace_read(struct trace_event_raw_sys_enter* ctx)
{
    struct encrypted_payload_event *event;

    event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event)
        return 0;

    __u64 pid_tgid = bpf_get_current_pid_tgid();
    event->pid = pid_tgid >> 32;
    event->fd  = (__u32)ctx->args[0];
    event->bytes_requested = (__u32)ctx->args[2];
    event->timestamp = bpf_ktime_get_ns();

    void *user_buf = (void *)(unsigned long)ctx->args[1];
    __builtin_memset(event->buffer_preview, 0, sizeof(event->buffer_preview));
    bpf_probe_read_user(event->buffer_preview, sizeof(event->buffer_preview), user_buf);

    bpf_ringbuf_submit(event, 0);
    return 0;
}

/* sys_enter_recvfrom */
SEC("tracepoint/syscalls/sys_enter_recvfrom")
int trace_recvfrom(struct trace_event_raw_sys_enter* ctx)
{
    struct encrypted_payload_event *event;

    event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event)
        return 0;

    __u64 pid_tgid = bpf_get_current_pid_tgid();
    event->pid = pid_tgid >> 32;
    event->fd  = (__u32)ctx->args[0];
    event->bytes_requested = (__u32)ctx->args[2];
    event->timestamp = bpf_ktime_get_ns();

    void *user_buf = (void *)(unsigned long)ctx->args[1];
    __builtin_memset(event->buffer_preview, 0, sizeof(event->buffer_preview));
    bpf_probe_read_user(event->buffer_preview, sizeof(event->buffer_preview), user_buf);

    bpf_ringbuf_submit(event, 0);
    return 0;
}

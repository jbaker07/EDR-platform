// edr-agent/src/ebpf/file_tamper_monitor.c (ringbuf version)

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct file_tamper_evt {
    __u32 pid;
    __u64 timestamp;
};

/* Ring buffer named "events" so the loader can reuse the shared rb if pinned */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 18); // 256 KiB fallback if not reusing shared rb
} events SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_unlinkat")
int trace_file_tamper(struct trace_event_raw_sys_enter *ctx)
{
    struct file_tamper_evt *evt;

    evt = bpf_ringbuf_reserve(&events, sizeof(*evt), 0);
    if (!evt)
        return 0;

    evt->pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    evt->timestamp = bpf_ktime_get_ns();

    bpf_ringbuf_submit(evt, 0);
    return 0;
}

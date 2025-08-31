// dll_injection_monitor.c — ringbuf output
#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct dll_inject_evt {
    __u32 pid;
    __u32 target_pid;
    __u64 timestamp;
};

/*
 * Use a ring buffer map named "events".
 * edr_attach_any will detect this and, if a shared ringbuf
 * /sys/fs/bpf/edr/edr_events_rb is already pinned, it will
 * reuse that FD so all modules publish to the same buffer.
 */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20); // 1 MiB if we don't reuse the shared one
} events SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_ptrace")
int trace_dll_injection(struct trace_event_raw_sys_enter *ctx)
{
    struct dll_inject_evt *evt;

    evt = bpf_ringbuf_reserve(&events, sizeof(*evt), 0);
    if (!evt)
        return 0;

    evt->pid         = (__u32)(bpf_get_current_pid_tgid() >> 32);
    evt->target_pid  = (__u32)ctx->args[1];   // ptrace(pid, ...)
    evt->timestamp   = bpf_ktime_get_ns();

    bpf_ringbuf_submit(evt, 0);
    return 0;
}

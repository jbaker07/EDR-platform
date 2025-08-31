// suspicious_ipc.c (ringbuf)

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct ipc_evt {
    __u32 pid;
    __u32 peer_pid;
    __u8  ipc_type;   // 0=shm, 1=msg, etc.
    __u8  _pad[3];    // keep 8-byte alignment for the next u64
    __u64 timestamp;
};

/* Ring buffer for events.
 * edr_attach_any will reuse a shared ringbuf named "edr_events_rb" if present.
 */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20);   // 1 MiB
} events SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_msgsnd")
int trace_suspicious_ipc(struct trace_event_raw_sys_enter *ctx)
{
    struct ipc_evt evt = {};
    evt.pid       = bpf_get_current_pid_tgid() >> 32;
    evt.peer_pid  = ctx->args[0];            // msg queue id or inferred peer
    evt.ipc_type  = 1;                       // msgsnd
    evt.timestamp = bpf_ktime_get_ns();

    // Emit via ring buffer
    bpf_ringbuf_output(&events, &evt, sizeof(evt), 0);
    return 0;
}

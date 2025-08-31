// edr-agent/src/ebpf/dns_tunnel_monitor.c (ringbuf version)

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct dns_tunnel_event_t {
    __u32 pid;
    __u32 uid;
    char  comm[64];
    char  domain[256];
    __u32 query_type;
};

/* Ring buffer named "events" so edr_attach_any can reuse the shared
 * /sys/fs/bpf/edr/edr_events_rb if it’s already pinned.
 */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20); // 1 MiB fallback if not reusing shared rb
} events SEC(".maps");

// DNS port (placeholder for future parsing)
#define DNS_PORT 53

SEC("tracepoint/syscalls/sys_enter_sendto")
int trace_dns_send(struct trace_event_raw_sys_enter *ctx)
{
    struct dns_tunnel_event_t *event;

    event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event)
        return 0;

    event->pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    event->uid = (__u32)(bpf_get_current_uid_gid() & 0xFFFFFFFF);
    bpf_get_current_comm(&event->comm, sizeof(event->comm));

    /* Heuristic placeholder — real DNS payload inspection would need kprobes/
     * LSM or socket-level hooks to safely peek buffers cross-arch.
     */
    __builtin_memcpy(event->domain, "suspect.longsubdomain.example.com", 35);
    event->query_type = 1; // A record

    bpf_ringbuf_submit(event, 0);
    return 0;
}

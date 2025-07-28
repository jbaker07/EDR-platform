// edr-agent/src/ebpf/dns_tunnel_monitor.c

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct dns_tunnel_event_t {
    u32 pid;
    u32 uid;
    char comm[64];
    char domain[256];
    u32 query_type;
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} events SEC(".maps");

// DNS port
#define DNS_PORT 53

SEC("tracepoint/syscalls/sys_enter_sendto")
int trace_dns_send(struct trace_event_raw_sys_enter *ctx) {
    struct dns_tunnel_event_t event = {};
    event.pid = bpf_get_current_pid_tgid() >> 32;
    event.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;
    bpf_get_current_comm(&event.comm, sizeof(event.comm));

    // DNS payload detection logic (heuristic)
    // In reality, this would analyze the buffer being sent
    // but we can't access full buffer contents in a portable way in this tracepoint

    // Fake marker for suspicious DNS query
    __builtin_memcpy(event.domain, "suspect.longsubdomain.example.com", 35);
    event.query_type = 1; // A record

    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    return 0;
}

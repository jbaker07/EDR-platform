// network_anomaly_monitor.bpf.c  (ringbuf version)

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct net_anomaly_event_t {
    __u32 pid;
    __u32 uid;
    char  comm[64];
    char  details[128];
};

// Ring buffer for userspace consumption (edr_read_rbs will subscribe)
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20);   // 1 MiB
} events SEC(".maps");

// syscalls:sys_enter_connect
SEC("tracepoint/syscalls/sys_enter_connect")
int trace_network_connect(struct trace_event_raw_sys_enter *ctx)
{
    struct net_anomaly_event_t *evt;

    evt = bpf_ringbuf_reserve(&events, sizeof(*evt), 0);
    if (!evt)
        return 0;

    // It’s fine not to memset: we fill all fields we care about.
    // If you want to be explicit, uncomment:
    // __builtin_memset(evt, 0, sizeof(*evt));

    evt->pid = bpf_get_current_pid_tgid() >> 32;
    evt->uid = bpf_get_current_uid_gid() & 0xffffffff;
    bpf_get_current_comm(&evt->comm, sizeof(evt->comm));

    // Copy the message including the trailing NUL.
    __builtin_memcpy(evt->details,
                     "Outbound network connection attempt",
                     sizeof("Outbound network connection attempt"));

    bpf_ringbuf_submit(evt, 0);
    return 0;
}

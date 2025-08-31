// edr-agent/src/ebpf/mfa_bypass_monitor.c

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_tracing.h>

struct mfa_bypass_event_t {
    __u32 pid;
    __u32 uid;
    char  comm[64];
    char  details[128];
};

/* Ring buffer named "events" (shared by our loader if edr_events_rb is pinned) */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20);   // 1 MiB default if not reused
} events SEC(".maps");

/* Execve is a good proxy signal for interactive sessions or MFA bypass attempts */
SEC("tracepoint/syscalls/sys_enter_execve")
int trace_mfa_anomaly(struct trace_event_raw_sys_enter *ctx)
{
    struct mfa_bypass_event_t *evt = bpf_ringbuf_reserve(&events, sizeof(*evt), 0);
    if (!evt)
        return 0;

    evt->pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    evt->uid = (__u32)(bpf_get_current_uid_gid());
    bpf_get_current_comm(&evt->comm, sizeof(evt->comm));
    __builtin_memcpy(evt->details, "execve anomaly (potential MFA bypass)", 38);

    bpf_ringbuf_submit(evt, 0);
    return 0;
}

char LICENSE[] SEC("license") = "Dual BSD/GPL";


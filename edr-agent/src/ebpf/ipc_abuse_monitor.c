// edr-agent/src/ebpf/ipc_abuse_monitor.c (ringbuf version)

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct ipc_abuse_event_t {
    __u32 pid;
    __u32 uid;
    char  comm[64];
    char  ipc_call[32];
};

/* Ring buffer named "events" to allow reuse by the user-space loader */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20); // 1 MiB if not reusing shared rb
} events SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_shmget")
int trace_ipc_shmget(struct trace_event_raw_sys_enter *ctx)
{
    struct ipc_abuse_event_t *event;

    event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event)
        return 0;

    event->pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    event->uid = (__u32)(bpf_get_current_uid_gid());
    bpf_get_current_comm(&event->comm, sizeof(event->comm));
    __builtin_memcpy(event->ipc_call, "shmget", 6);

    bpf_ringbuf_submit(event, 0);
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_msgsnd")
int trace_ipc_msgsnd(struct trace_event_raw_sys_enter *ctx)
{
    struct ipc_abuse_event_t *event;

    event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event)
        return 0;

    event->pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    event->uid = (__u32)(bpf_get_current_uid_gid());
    bpf_get_current_comm(&event->comm, sizeof(event->comm));
    __builtin_memcpy(event->ipc_call, "msgsnd", 6);

    bpf_ringbuf_submit(event, 0);
    return 0;
}

char _license[] SEC("license") = "Dual BSD/GPL";

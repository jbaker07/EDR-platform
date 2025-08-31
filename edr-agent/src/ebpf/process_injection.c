// process_injection.c (ringbuf)
#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct injection_evt {
    __u32 pid;
    __u32 target_pid;
    __u64 timestamp;
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20);  // 1 MiB
} events SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_process_vm_writev")
int trace_proc_inject(struct trace_event_raw_sys_enter *ctx) {
    struct injection_evt *evt;

    evt = bpf_ringbuf_reserve(&events, sizeof(*evt), 0);
    if (!evt)
        return 0;

    evt->pid        = (__u32)(bpf_get_current_pid_tgid() >> 32);
    evt->target_pid = (__u32)ctx->args[0];  // target pid
    evt->timestamp  = bpf_ktime_get_ns();

    bpf_ringbuf_submit(evt, 0);
    return 0;
}

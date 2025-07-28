// edr-agent/src/ebpf/cred_dump_monitor.c

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct cred_dump_event_t {
    u32 pid;
    u32 uid;
    char comm[64];
    char details[128];
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} events SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_ptrace")
int trace_cred_dump_ptrace(struct trace_event_raw_sys_enter *ctx) {
    struct cred_dump_event_t event = {};
    event.pid = bpf_get_current_pid_tgid() >> 32;
    event.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;

    bpf_get_current_comm(&event.comm, sizeof(event.comm));
    __builtin_memcpy(event.details, "syscall ptrace triggered - potential credential dump", 56);

    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_openat")
int trace_open_shadow_file(struct trace_event_raw_sys_enter *ctx) {
    struct cred_dump_event_t event = {};
    char fname[256];

    const char *filename = (const char *)ctx->args[1];
    bpf_core_read_user_str(&fname, sizeof(fname), filename);

    if (__builtin_memcmp(fname, "/etc/shadow", 11) == 0) {
        event.pid = bpf_get_current_pid_tgid() >> 32;
        event.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;
        bpf_get_current_comm(&event.comm, sizeof(event.comm));
        __builtin_memcpy(event.details, "attempt to open /etc/shadow detected", 38);

        bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    }
    return 0;
}

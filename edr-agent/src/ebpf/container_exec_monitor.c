// src/ebpf/container_exec_monitor.c

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

#define MAX_CMD_LEN 256
#define MAX_COMM_LEN 64

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct container_exec_event_t {
    u32 pid;
    u32 uid;
    u32 gid;
    u64 timestamp;
    char comm[MAX_COMM_LEN];
    char filename[MAX_CMD_LEN];
};

// BPF output map
struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} events SEC(".maps");

// Tracepoint handler
SEC("tracepoint/syscalls/sys_enter_execve")
int trace_container_exec(struct trace_event_raw_sys_enter *ctx) {
    struct container_exec_event_t event = {};
    u64 pid_tgid = bpf_get_current_pid_tgid();
    u64 uid_gid = bpf_get_current_uid_gid();

    event.pid = pid_tgid >> 32;
    event.uid = uid_gid & 0xffffffff;
    event.gid = uid_gid >> 32;
    event.timestamp = bpf_ktime_get_ns();

    bpf_get_current_comm(&event.comm, sizeof(event.comm));

    const char *filename = (const char *)ctx->args[0];
    bpf_probe_read_user_str(&event.filename, sizeof(event.filename), filename);

    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    return 0;
}

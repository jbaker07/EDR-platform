#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_tracing.h>
char LICENSE[] SEC("license") = "Dual BSD/GPL";
struct file_access_event_t { u32 pid; u32 uid; char comm[64]; char filename[256]; };
struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u32));
    __uint(max_entries, 1024);
} events SEC(".maps");
SEC("tracepoint/syscalls/sys_enter_openat")
int trace_file_access(struct trace_event_raw_sys_enter *ctx) {
    struct file_access_event_t ev = {};
    ev.pid = bpf_get_current_pid_tgid() >> 32;
    ev.uid = bpf_get_current_uid_gid() & 0xffffffff;
    bpf_get_current_comm(&ev.comm, sizeof(ev.comm));
    const char *fn = (const char *)ctx->args[1];
    bpf_core_read_user_str(&ev.filename, sizeof(ev.filename), fn);
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &ev, sizeof(ev));
    return 0;
}

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

#define MAX_FILENAME_LEN 256
#define MAX_SYSCALL_LEN 16

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct EntropyEvent {
    u32 pid;
    u32 ppid;
    char filename[MAX_FILENAME_LEN];
    char syscall[MAX_SYSCALL_LEN];
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} events SEC(".maps");

static __always_inline int emit_event(struct pt_regs *ctx, const char *filename_ptr, const char *syscall_name) {
    struct EntropyEvent event = {};
    u64 pid_tgid = bpf_get_current_pid_tgid();
    struct task_struct *task = (struct task_struct *)bpf_get_current_task();

    event.pid = pid_tgid >> 32;
    event.ppid = BPF_CORE_READ(task, real_parent, tgid);

    if (filename_ptr) {
        bpf_probe_read_user_str(event.filename, sizeof(event.filename), filename_ptr);
    }

    bpf_probe_read_str(event.syscall, sizeof(event.syscall), syscall_name);

    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    return 0;
}

// Trace execve syscall
SEC("tracepoint/syscalls/sys_enter_execve")
int trace_execve(struct trace_event_raw_sys_enter *ctx) {
    const char *filename = (const char *)ctx->args[0];
    return emit_event((struct pt_regs *)ctx, filename, "execve");
}

// Trace openat syscall
SEC("tracepoint/syscalls/sys_enter_openat")
int trace_openat(struct trace_event_raw_sys_enter *ctx) {
    const char *filename = (const char *)ctx->args[1];
    return emit_event((struct pt_regs *)ctx, filename, "openat");
}

// Trace mmap syscall
SEC("tracepoint/syscalls/sys_enter_mmap")
int trace_mmap(struct trace_event_raw_sys_enter *ctx) {
    // filename not available, so skip it
    return emit_event((struct pt_regs *)ctx, NULL, "mmap");
}
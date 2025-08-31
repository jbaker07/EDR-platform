// entropy_exec_monitor.c  (ringbuf; execve/execveat tracepoints)

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

struct exec_event_t {
    __u64 ts;
    __u32 pid;
    __u32 uid;
    char  comm[16];
    char  filename[256];
};

/* Reuse the shared ringbuf pinned at /sys/fs/bpf/edr/events */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20);
} events SEC(".maps");

/* Optional control map you already had */
struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
} entropy_threshold SEC(".maps");

static __always_inline void emit_exec(struct trace_event_raw_sys_enter *ctx, __u64 filename_arg)
{
    struct exec_event_t *e = bpf_ringbuf_reserve(&events, sizeof(*e), 0);
    if (!e)
        return;

    e->ts  = bpf_ktime_get_ns();
    e->pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    e->uid = (__u32)(bpf_get_current_uid_gid());
    bpf_get_current_comm(&e->comm, sizeof(e->comm));
    bpf_probe_read_user_str(e->filename, sizeof(e->filename), (const void *)filename_arg);

    bpf_ringbuf_submit(e, 0);
}

SEC("tracepoint/syscalls/sys_enter_execve")
int trace_execve(struct trace_event_raw_sys_enter *ctx)
{
    /* execve(filename, argv, envp) -> filename is args[0] */
    emit_exec(ctx, ctx->args[0]);
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_execveat")
int trace_execveat(struct trace_event_raw_sys_enter *ctx)
{
    /* execveat(dirfd, pathname, argv, envp, flags) -> pathname is args[1] */
    emit_exec(ctx, ctx->args[1]);
    return 0;
}

char LICENSE[] SEC("license") = "Dual BSD/GPL";

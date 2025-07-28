// syscall_tracer.bpf.c

#include "vmlinux.h"
#include "bpf_helpers.h"
#include "bpf_tracing.h"

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct syscall_event {
    u32 pid;
    u32 syscall_id;
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 24); // 16MB buffer
} events SEC(".maps");

// Safe wrapper for emitting syscall events
static __always_inline void emit_syscall_event(u32 syscall_id) {
    struct syscall_event *e;

    e = bpf_ringbuf_reserve(&events, sizeof(*e), 0);
    if (!e) return;

    e->pid = bpf_get_current_pid_tgid() >> 32;
    e->syscall_id = syscall_id;

    bpf_ringbuf_submit(e, 0);
}

// Macro to define a tracepoint handler for a specific syscall
#define TRACE_SYSCALL_ENTER(name, id) \
    SEC("tracepoint/syscalls/sys_enter_" #name) \
    int handle_sys_enter_##name(struct trace_event_raw_sys_enter *ctx) { \
        emit_syscall_event(id); \
        return 0; \
    }

// Track common high-value syscalls
TRACE_SYSCALL_ENTER(execve, __NR_execve);
TRACE_SYSCALL_ENTER(open, __NR_open);
TRACE_SYSCALL_ENTER(openat, __NR_openat);
TRACE_SYSCALL_ENTER(unlink, __NR_unlink);
TRACE_SYSCALL_ENTER(rename, __NR_rename);
TRACE_SYSCALL_ENTER(connect, __NR_connect);
TRACE_SYSCALL_ENTER(read, __NR_read);
TRACE_SYSCALL_ENTER(write, __NR_write);
TRACE_SYSCALL_ENTER(fork, __NR_fork);
TRACE_SYSCALL_ENTER(clone, __NR_clone);
TRACE_SYSCALL_ENTER(exit, __NR_exit);
TRACE_SYSCALL_ENTER(setuid, __NR_setuid);
TRACE_SYSCALL_ENTER(mount, __NR_mount);

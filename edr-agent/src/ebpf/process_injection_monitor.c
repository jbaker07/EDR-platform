// process_injection_monitor.c (ringbuf)
#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct injection_event_t {
    __u32 pid;
    __u32 uid;
    char  comm[64];
    char  syscall[32];
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20); // 1 MiB
} events SEC(".maps");

// userspace mmap(2) prot bits
#define PROT_EXEC  0x4

static __always_inline int submit_evt(const char *label, int len)
{
    struct injection_event_t *e = bpf_ringbuf_reserve(&events, sizeof(*e), 0);
    if (!e)
        return 0;

    e->pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    e->uid = (__u32)(bpf_get_current_uid_gid() & 0xffffffff);
    bpf_get_current_comm(&e->comm, sizeof(e->comm));

    /* Ensure syscall field is always NUL-terminated */
    __builtin_memset(e->syscall, 0, sizeof(e->syscall));
    if (label && len > 0) {
        if (len > (int)sizeof(e->syscall)) len = sizeof(e->syscall);
        __builtin_memcpy(e->syscall, label, len);
    }

    bpf_ringbuf_submit(e, 0);
    return 0;
}

// Monitor mmap with PROT_EXEC
SEC("tracepoint/syscalls/sys_enter_mmap")
int trace_mmap_exec(struct trace_event_raw_sys_enter *ctx)
{
    unsigned long prot = ctx->args[2];
    if (!(prot & PROT_EXEC))
        return 0;

    return submit_evt("mmap(PROT_EXEC)", 15);
}

// Monitor ptrace usage
SEC("tracepoint/syscalls/sys_enter_ptrace")
int trace_ptrace(struct trace_event_raw_sys_enter *ctx)
{
    return submit_evt("ptrace", 6);
}

// Monitor cross-process memory writing
SEC("tracepoint/syscalls/sys_enter_process_vm_writev")
int trace_vm_write(struct trace_event_raw_sys_enter *ctx)
{
    return submit_evt("process_vm_writev", 17);
}

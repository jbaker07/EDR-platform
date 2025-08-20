// syscall_tracer.bpf.c  — CO-RE tracepoint hooks for high-signal syscalls.
// Emits compact ringbuf events: { pid, syscall_id }.
//
// Build note: compile with clang -target bpf (CO-RE), link with libbpf.
// The userspace reader should open map "events" (ringbuf) and read SysEvt {u32 pid, u32 syscall_id}.

#include "vmlinux.h"
#include "bpf_helpers.h"
#include "bpf_tracing.h"
#include "bpf_core_read.h"

char LICENSE[] SEC("license") = "Dual BSD/GPL";

// ----- Ring buffer payload -----

struct syscall_event {
    __u32 pid;
    __u32 syscall_id; // ctx->id as seen by the kernel (arch-specific)
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 24); // 16 MiB
} events SEC(".maps");

// Safe wrapper: reserve → fill → submit
static __always_inline void emit_from_ctx(struct trace_event_raw_sys_enter *ctx)
{
    struct syscall_event *e = bpf_ringbuf_reserve(&events, sizeof(*e), 0);
    if (!e)
        return;

    // ctx->id is the syscall number for this tracepoint instance
    __u64 tgid_pid = bpf_get_current_pid_tgid();
    e->pid = (__u32)(tgid_pid >> 32);
    // Use CORE_READ in case layout shifts (CO-RE)
    e->syscall_id = (__u32)BPF_CORE_READ(ctx, id);

    bpf_ringbuf_submit(e, 0);
}

// Macro to define a handler for a specific sys_enter tracepoint.
// We intentionally DO NOT rely on __NR_* compile-time macros —
// the kernel tells us the id via ctx->id and we forward it.
#define TP_ENTER_SYSCALL(sysname)                                              \
SEC("tracepoint/syscalls/sys_enter_" #sysname)                                 \
int BPF_PROG(tp_enter_##sysname, struct trace_event_raw_sys_enter *ctx)        \
{                                                                              \
    emit_from_ctx(ctx);                                                        \
    return 0;                                                                  \
}

// ====== High-signal coverage ======
//
// Process / exec lineage:
TP_ENTER_SYSCALL(execve)
TP_ENTER_SYSCALL(execveat)
TP_ENTER_SYSCALL(fork)
TP_ENTER_SYSCALL(vfork)
TP_ENTER_SYSCALL(clone)

// Memory & WX transitions (paired with wx_exec eBPF for higher fidelity):
TP_ENTER_SYSCALL(mmap)
TP_ENTER_SYSCALL(mprotect)

// Privilege & policy relaxations:
TP_ENTER_SYSCALL(setuid)
TP_ENTER_SYSCALL(setreuid)
TP_ENTER_SYSCALL(setresuid)
TP_ENTER_SYSCALL(capset)

// Network initiation (beacon / C2)
TP_ENTER_SYSCALL(connect)

// Optional file system touchpoints (uncomment if you want file-change context here too)
// TP_ENTER_SYSCALL(open)
// TP_ENTER_SYSCALL(openat)
// TP_ENTER_SYSCALL(unlink)
// TP_ENTER_SYSCALL(rename)

// You can add more as needed:
// TP_ENTER_SYSCALL(prctl)
// TP_ENTER_SYSCALL(ptrace)
// TP_ENTER_SYSCALL(bpf)


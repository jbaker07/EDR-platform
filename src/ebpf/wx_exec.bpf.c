// SPDX-License-Identifier: Dual BSD/GPL
// CO-RE monitor for W→X transitions and memfd → exec chains.

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

#define PROT_READ  0x1
#define PROT_WRITE 0x2
#define PROT_EXEC  0x4

struct wx_evt {
    __u32 pid;
    __u32 kind;    // 1=W->X, 2=memfd_exec
    __u64 ts_ns;
    __u64 addr;    // for m(m)protect/mmap if available
    __u64 len;
    __u64 prot;    // requested prot
    __u64 bytes;   // memfd bytes (for kind=2)
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 22); // 4MB
} wx_events SEC(".maps");

struct pid_u64 {
    __u32 pid;
    __u32 pad;
};

// last writable map time per pid
struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __type(key, __u32);   // pid
    __type(value, __u64); // ts_ns
    __uint(max_entries, 16384);
} last_w_map_ts SEC(".maps");

// per-pid memfd set: fd -> 1
struct pid_fd_key {
    __u32 pid;
    __u32 fd;
};
struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __type(key, struct pid_fd_key);
    __type(value, __u8);
    __uint(max_entries, 32768);
} memfd_set SEC(".maps");

// per-pid memfd bytes + last ts
struct memfd_stat {
    __u64 bytes;
    __u64 ts_ns;
};
struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __type(key, __u32);              // pid
    __type(value, struct memfd_stat);
    __uint(max_entries, 16384);
} memfd_bytes SEC(".maps");

// stash fd seen on sys_enter_write (per pid)
struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __type(key, __u32); // pid
    __type(value, __s32); // fd
    __uint(max_entries, 16384);
} last_fd SEC(".maps");

static __always_inline void emit_evt(__u32 kind, __u64 addr, __u64 len, __u64 prot, __u64 bytes) {
    struct wx_evt *e = bpf_ringbuf_reserve(&wx_events, sizeof(*e), 0);
    if (!e) return;
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    e->pid = pid_tgid >> 32;
    e->kind = kind;
    e->ts_ns = bpf_ktime_get_ns();
    e->addr = addr;
    e->len  = len;
    e->prot = prot;
    e->bytes= bytes;
    bpf_ringbuf_submit(e, 0);
}

// ---- mmap ----
SEC("tracepoint/syscalls/sys_enter_mmap")
int tp_enter_mmap(struct trace_event_raw_sys_enter *ctx) {
    __u64 addr = (unsigned long)ctx->args[0];
    __u64 len  = (unsigned long)ctx->args[1];
    __u64 prot = (unsigned long)ctx->args[2];

    __u32 pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    if (prot & PROT_WRITE) {
        __u64 now = bpf_ktime_get_ns();
        bpf_map_update_elem(&last_w_map_ts, &pid, &now, BPF_ANY);
    }
    if (prot & PROT_EXEC) {
        __u64 *ts = bpf_map_lookup_elem(&last_w_map_ts, &pid);
        if (ts) {
            __u64 now = bpf_ktime_get_ns();
            if (now > *ts && (now - *ts) <= 5ULL * 1000000000ULL) {
                emit_evt(1, addr, len, prot, 0); // W->X within 5s window
            }
        }
    }
    return 0;
}

// ---- mprotect ----
SEC("tracepoint/syscalls/sys_enter_mprotect")
int tp_enter_mprotect(struct trace_event_raw_sys_enter *ctx) {
    __u64 addr = (unsigned long)ctx->args[0];
    __u64 len  = (unsigned long)ctx->args[1];
    __u64 prot = (unsigned long)ctx->args[2];

    __u32 pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    if (prot & PROT_WRITE) {
        __u64 now = bpf_ktime_get_ns();
        bpf_map_update_elem(&last_w_map_ts, &pid, &now, BPF_ANY);
    }
    if (prot & PROT_EXEC) {
        __u64 *ts = bpf_map_lookup_elem(&last_w_map_ts, &pid);
        if (ts) {
            __u64 now = bpf_ktime_get_ns();
            if (now > *ts && (now - *ts) <= 5ULL * 1000000000ULL) {
                emit_evt(1, addr, len, prot, 0); // W->X transition
            }
        }
    }
    return 0;
}

// ---- memfd_create ----
SEC("tracepoint/syscalls/sys_exit_memfd_create")
int tp_exit_memfd_create(struct trace_event_raw_sys_exit *ctx) {
    __s64 fd = ctx->ret;
    if (fd < 0) return 0;
    __u32 pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    struct pid_fd_key k = { .pid = pid, .fd = (__u32)fd };
    __u8 one = 1;
    bpf_map_update_elem(&memfd_set, &k, &one, BPF_ANY);
    return 0;
}

// ---- write enter/exit to capture bytes on memfd ----
SEC("tracepoint/syscalls/sys_enter_write")
int tp_enter_write(struct trace_event_raw_sys_enter *ctx) {
    __s64 fd = (__s64)ctx->args[0];
    __u32 pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    bpf_map_update_elem(&last_fd, &pid, &fd, BPF_ANY);
    return 0;
}

SEC("tracepoint/syscalls/sys_exit_write")
int tp_exit_write(struct trace_event_raw_sys_exit *ctx) {
    __s64 ret = ctx->ret;
    if (ret <= 0) return 0;
    __u32 pid = (__u32)(bpf_get_current_pid_tgid() >> 32);

    __s32 *fdp = bpf_map_lookup_elem(&last_fd, &pid);
    if (!fdp) return 0;

    struct pid_fd_key k = { .pid = pid, .fd = (__u32)(*fdp) };
    __u8 *is_memfd = bpf_map_lookup_elem(&memfd_set, &k);
    if (!is_memfd) return 0;

    struct memfd_stat *st = bpf_map_lookup_elem(&memfd_bytes, &pid);
    if (st) {
        st->bytes += (__u64)ret;
        st->ts_ns  = bpf_ktime_get_ns();
    } else {
        struct memfd_stat ns = {
            .bytes = (__u64)ret,
            .ts_ns = bpf_ktime_get_ns(),
        };
        bpf_map_update_elem(&memfd_bytes, &pid, &ns, BPF_ANY);
    }
    return 0;
}

// ---- execve path: check for recent memfd writes ----
static __always_inline int maybe_emit_memfd_exec() {
    __u32 pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    struct memfd_stat *st = bpf_map_lookup_elem(&memfd_bytes, &pid);
    if (!st) return 0;
    __u64 now = bpf_ktime_get_ns();
    if ((now - st->ts_ns) <= 120ULL * 1000000000ULL && st->bytes >= 4096) {
        emit_evt(2, 0, 0, 0, st->bytes);
        // optional: clear after firing
        struct memfd_stat zero = {};
        bpf_map_update_elem(&memfd_bytes, &pid, &zero, BPF_ANY);
    }
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_execve")
int tp_enter_execve(struct trace_event_raw_sys_enter *ctx) {
    return maybe_emit_memfd_exec();
}

SEC("tracepoint/syscalls/sys_enter_execveat")
int tp_enter_execveat(struct trace_event_raw_sys_enter *ctx) {
    return maybe_emit_memfd_exec();
}

// SPDX-License-Identifier: Dual BSD/GPL
// CO-RE net flow tracer: connect + TX/RX byte counts
// Keeps it lean: IPv4 + IPv6 connect, sendto/recvfrom/sendmsg/recvmsg byte totals.

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct net_evt {
    __u32 pid;
    __u32 kind;      // 1=connect, 2=tx, 3=rx
    __u16 family;    // AF_*
    __u16 dport;     // network byte order for AF_INET/6 connect
    __u8  is_v6;     // 0=IPv4,1=IPv6 (for connect)
    __u32 daddr_v4;  // for IPv4 connect
    __u8  daddr_v6[16]; // for IPv6 connect
    __s64 bytes;     // ret bytes for tx/rx (>=0)
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 24); // 16MB
} net_events SEC(".maps");

// Per-pid scratch to track fd on sys_enter_write/sendto/recvfrom/...
struct key_pid {
    __u32 pid;
};
struct pid_fd {
    __u32 pid;
    __s32 fd;
};
struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __type(key, struct key_pid);
    __type(value, __s32); // last fd
    __uint(max_entries, 16384);
} last_fd_map SEC(".maps");

static __always_inline int emit_connect(__u16 family, __u16 dport_net, void *us_addr, int addrlen) {
    struct net_evt *e = bpf_ringbuf_reserve(&net_events, sizeof(*e), 0);
    if (!e) return 0;
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    e->pid = pid_tgid >> 32;
    e->kind = 1;
    e->family = family;
    e->dport = dport_net;
    e->bytes = 0;

    if (family == AF_INET && addrlen >= sizeof(struct sockaddr_in)) {
        struct sockaddr_in sa4 = {};
        bpf_probe_read_user(&sa4, sizeof(sa4), us_addr);
        e->is_v6 = 0;
        e->daddr_v4 = sa4.sin_addr.s_addr; // already BE
    } else if (family == AF_INET6 && addrlen >= sizeof(struct sockaddr_in6)) {
        struct sockaddr_in6 sa6 = {};
        bpf_probe_read_user(&sa6, sizeof(sa6), us_addr);
        e->is_v6 = 1;
        __builtin_memcpy(e->daddr_v6, &sa6.sin6_addr, 16);
    } else {
        e->is_v6 = 0;
        e->daddr_v4 = 0;
    }

    bpf_ringbuf_submit(e, 0);
    return 0;
}

static __always_inline int emit_bytes(int kind, __s64 bytes) {
    if (bytes <= 0) return 0; // ignore 0/err for noise
    struct net_evt *e = bpf_ringbuf_reserve(&net_events, sizeof(*e), 0);
    if (!e) return 0;
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    e->pid = pid_tgid >> 32;
    e->kind = kind; // 2=tx,3=rx
    e->family = 0;
    e->dport = 0;
    e->is_v6 = 0;
    e->daddr_v4 = 0;
    __builtin_memset(e->daddr_v6, 0, 16);
    e->bytes = bytes;
    bpf_ringbuf_submit(e, 0);
    return 0;
}

// ----- sys_enter helpers to stash fd -----

#define STASH_FD(tp_name) \
SEC("tracepoint/syscalls/sys_enter_" #tp_name) \
int tp_enter_##tp_name(struct trace_event_raw_sys_enter *ctx) { \
    struct key_pid k = { .pid = (__u32)(bpf_get_current_pid_tgid() >> 32) }; \
    __s64 fd = (__s64)ctx->args[0]; \
    bpf_map_update_elem(&last_fd_map, &k, &fd, BPF_ANY); \
    return 0; \
}

STASH_FD(sendto)
STASH_FD(sendmsg)
STASH_FD(recvfrom)
STASH_FD(recvmsg)
STASH_FD(write)     // not strictly net, but harmless to track
STASH_FD(read)

// ----- connect -----

SEC("tracepoint/syscalls/sys_enter_connect")
int tp_enter_connect(struct trace_event_raw_sys_enter *ctx) {
    void *us_addr = (void *)ctx->args[1];
    int addrlen = (int)ctx->args[2];
    __u16 fam = 0, dport = 0;

    // Read family and port from user sockaddr header
    if (addrlen >= sizeof(sa_family_t)) {
        bpf_probe_read_user(&fam, sizeof(fam), us_addr); // sa_family at offset 0
    }
    if (fam == AF_INET && addrlen >= sizeof(struct sockaddr_in)) {
        struct sockaddr_in sa4 = {};
        bpf_probe_read_user(&sa4, sizeof(sa4), us_addr);
        dport = sa4.sin_port; // BE
        return emit_connect(fam, dport, us_addr, addrlen);
    } else if (fam == AF_INET6 && addrlen >= sizeof(struct sockaddr_in6)) {
        struct sockaddr_in6 sa6 = {};
        bpf_probe_read_user(&sa6, sizeof(sa6), us_addr);
        dport = sa6.sin6_port;
        return emit_connect(fam, dport, us_addr, addrlen);
    }
    return 0;
}

// ----- byte accounting on sys_exit -----

#define EMIT_EXIT_BYTES(tp_name, kind_code) \
SEC("tracepoint/syscalls/sys_exit_" #tp_name) \
int tp_exit_##tp_name(struct trace_event_raw_sys_exit *ctx) { \
    __s64 ret = (__s64)ctx->ret; \
    return emit_bytes(kind_code, ret); \
}

EMIT_EXIT_BYTES(sendto, 2)
EMIT_EXIT_BYTES(sendmsg, 2)
EMIT_EXIT_BYTES(recvfrom, 3)
EMIT_EXIT_BYTES(recvmsg, 3)

// Fallbacks (optional) — comment out if too noisy
// EMIT_EXIT_BYTES(write, 2)
// EMIT_EXIT_BYTES(read, 3)

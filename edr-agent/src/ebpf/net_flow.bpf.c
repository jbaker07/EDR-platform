// SPDX-License-Identifier: Dual BSD/GPL
// CO-RE net flow tracer: connect + TX/RX byte counts, and the TLS ClientHello.
// Keeps it lean: IPv4 + IPv6 connect, sendto/recvfrom/sendmsg/recvmsg byte totals.
//
// ClientHello: a connect() to port 443 or 8443 marks the (process, fd) pair. The first
// write() or sendto() on that fd is copied (up to TLS_CAP bytes) when it starts with a
// TLS handshake record, and the mark is removed either way. User space reads the server
// name (SNI) from the copy. The hello is sent before any encryption starts, so this
// needs no key and no hook in a TLS library.

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_endian.h>

// Avoid glibc/uapi includes; provide minimal fallbacks only if missing.
#ifndef AF_INET
#define AF_INET 2
#endif
#ifndef AF_INET6
#define AF_INET6 10
#endif

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct net_evt {
    __u32 pid;
    __u32 kind;         // 1=connect, 2=tx, 3=rx
    __u16 family;       // AF_*
    __u16 dport;        // network byte order for AF_INET/6 connect
    __u8  is_v6;        // 0=IPv4,1=IPv6 (for connect)
    __u32 daddr_v4;     // for IPv4 connect (BE)
    __u8  daddr_v6[16]; // for IPv6 connect
    __u32 uid;          // in what was padding before bytes, so the size stays 48
    __s64 bytes;        // ret bytes for tx/rx (>=0)
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 24); // 16MB
} net_events SEC(".maps");

// kind 4: a net_evt header (bytes = how many bytes were copied) and then the first
// bytes the process wrote on a socket it had connected to a TLS port.
#define NET_KIND_TLS_HELLO 4
#define TLS_CAP 1024
struct tls_evt {
    struct net_evt h;
    __u8 data[TLS_CAP];
};

struct tls_key { __u32 tgid; __u32 fd; };
struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __type(key, struct tls_key);
    __type(value, struct net_evt); // the connect, so the hello names its destination
    __uint(max_entries, 8192);
} tls_pending SEC(".maps");

static __always_inline void mark_tls(int fd, const struct net_evt *e) {
    __u16 port = bpf_ntohs(e->dport);
    if (fd < 0 || (port != 443 && port != 8443)) return;
    struct tls_key k = { .tgid = e->pid, .fd = (__u32)fd };
    struct net_evt v = *e; // map helpers take stack memory, not ring buffer memory
    bpf_map_update_elem(&tls_pending, &k, &v, BPF_ANY);
}

static __always_inline int emit_connect(int fd, __u16 family, __u16 dport_net, void *us_addr, int addrlen) {
    struct net_evt *e = bpf_ringbuf_reserve(&net_events, sizeof(*e), 0);
    if (!e) return 0;

    __u64 pid_tgid = bpf_get_current_pid_tgid();
    e->pid    = pid_tgid >> 32;
    e->uid    = (__u32)bpf_get_current_uid_gid();
    e->kind   = 1;
    e->family = family;
    e->dport  = dport_net;
    e->bytes  = 0;

    if (family == AF_INET && addrlen >= (int)sizeof(struct sockaddr_in)) {
        struct sockaddr_in sa4 = {};
        bpf_probe_read_user(&sa4, sizeof(sa4), us_addr);
        e->is_v6    = 0;
        e->daddr_v4 = sa4.sin_addr.s_addr; // already BE
        __builtin_memset(e->daddr_v6, 0, sizeof(e->daddr_v6));
    } else if (family == AF_INET6 && addrlen >= (int)sizeof(struct sockaddr_in6)) {
        struct sockaddr_in6 sa6 = {};
        bpf_probe_read_user(&sa6, sizeof(sa6), us_addr);
        e->is_v6    = 1;
        __builtin_memcpy(e->daddr_v6, &sa6.sin6_addr, 16);
        e->daddr_v4 = 0;
    } else {
        e->is_v6    = 0;
        e->daddr_v4 = 0;
        __builtin_memset(e->daddr_v6, 0, sizeof(e->daddr_v6));
    }

    mark_tls(fd, e);
    bpf_ringbuf_submit(e, 0);
    return 0;
}

static __always_inline int emit_bytes(int kind, __s64 bytes) {
    if (bytes <= 0) return 0; // ignore 0/err for noise

    struct net_evt *e = bpf_ringbuf_reserve(&net_events, sizeof(*e), 0);
    if (!e) return 0;

    __u64 pid_tgid = bpf_get_current_pid_tgid();
    e->pid       = pid_tgid >> 32;
    e->uid       = (__u32)bpf_get_current_uid_gid();
    e->kind      = kind; // 2=tx,3=rx
    e->family    = 0;
    e->dport     = 0;
    e->is_v6     = 0;
    e->daddr_v4  = 0;
    __builtin_memset(e->daddr_v6, 0, 16);
    e->bytes     = bytes;

    bpf_ringbuf_submit(e, 0);
    return 0;
}

// ----- TLS ClientHello: the first write on a socket connected to a TLS port -----

static __always_inline int tls_capture(int fd, const void *buf, __u64 count) {
    if (fd < 0) return 0;
    struct tls_key k = { .tgid = (__u32)(bpf_get_current_pid_tgid() >> 32), .fd = (__u32)fd };
    struct net_evt *conn = bpf_map_lookup_elem(&tls_pending, &k);
    if (!conn) return 0;
    // Only the first write counts, hello or not.
    struct net_evt c = *conn;
    bpf_map_delete_elem(&tls_pending, &k);

    __u8 first = 0;
    if (count < 6 || bpf_probe_read_user(&first, 1, buf) != 0 || first != 0x16) return 0;

    struct tls_evt *e = bpf_ringbuf_reserve(&net_events, sizeof(*e), 0);
    if (!e) return 0;
    __u32 n = count > TLS_CAP ? TLS_CAP : (__u32)count;
    if (bpf_probe_read_user(e->data, n, buf) != 0) {
        bpf_ringbuf_discard(e, 0);
        return 0;
    }
    e->h = c;
    e->h.kind = NET_KIND_TLS_HELLO;
    e->h.uid = (__u32)bpf_get_current_uid_gid();
    e->h.bytes = n;
    bpf_ringbuf_submit(e, 0);
    return 0;
}

// write and sendto are read on raw_syscalls/sys_enter, not on their own tracepoints.
// On kernel 7.0 a BPF program on syscalls/sys_enter_write or sys_enter_openat (the
// syscalls whose events read a user buffer or path) ran on CPU 0 only, while connect,
// mmap and every sys_exit tracepoint ran on all CPUs. Measured on a 4 CPU x86_64 runner
// with a workload pinned to each CPU in turn. The raw tracepoint runs everywhere.
#if defined(__TARGET_ARCH_x86)
#define NF_NR_WRITE 1
#define NF_NR_SENDTO 44
#else
#define NF_NR_WRITE 64
#define NF_NR_SENDTO 206
#endif

SEC("tracepoint/raw_syscalls/sys_enter")
int tp_raw_enter(struct trace_event_raw_sys_enter *ctx) {
    long id = ctx->id;
    if (id != NF_NR_WRITE && id != NF_NR_SENDTO) return 0;
    return tls_capture((int)ctx->args[0], (const void *)ctx->args[1], (__u64)ctx->args[2]);
}

// ----- connect -----

SEC("tracepoint/syscalls/sys_enter_connect")
int tp_enter_connect(struct trace_event_raw_sys_enter *ctx) {
    int fd        = (int)ctx->args[0];
    void *us_addr = (void *)ctx->args[1];
    int addrlen   = (int)ctx->args[2];

    __u16 fam = 0, dport = 0;

    // sa_family_t is 16-bit at offset 0 in both sockaddr_in/in6
    if (addrlen >= (int)sizeof(__u16)) {
        bpf_probe_read_user(&fam, sizeof(fam), us_addr);
    }

    if (fam == AF_INET && addrlen >= (int)sizeof(struct sockaddr_in)) {
        struct sockaddr_in sa4 = {};
        bpf_probe_read_user(&sa4, sizeof(sa4), us_addr);
        dport = sa4.sin_port; // BE
        return emit_connect(fd, fam, dport, us_addr, addrlen);
    } else if (fam == AF_INET6 && addrlen >= (int)sizeof(struct sockaddr_in6)) {
        struct sockaddr_in6 sa6 = {};
        bpf_probe_read_user(&sa6, sizeof(sa6), us_addr);
        dport = sa6.sin6_port; // BE
        return emit_connect(fd, fam, dport, us_addr, addrlen);
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

EMIT_EXIT_BYTES(sendto,   2)
EMIT_EXIT_BYTES(sendmsg,  2)
EMIT_EXIT_BYTES(recvfrom, 3)
EMIT_EXIT_BYTES(recvmsg,  3)

// Fallbacks (optional): comment out if too noisy
// EMIT_EXIT_BYTES(write, 2)
// EMIT_EXIT_BYTES(read,  3)

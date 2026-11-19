// SPDX-License-Identifier: Dual BSD/GPL
// edr-agent/src/ebpf/dns_tunnel_monitor.bpf.c
//
// Copies the start of every UDP datagram sent to port 53 into a ring buffer.
// Nothing is parsed here. A DNS name is a chain of length-prefixed labels, and
// walking it means a loop with data-dependent offsets, which the verifier makes
// hard to prove safe. User space does the parsing instead
// (src/modules/dns_tunnel.rs), where it is easy to unit test and a bug cannot
// take down the kernel.
//
// Hooks: kprobes on udp_sendmsg and udpv6_sendmsg. send, sendto, sendmsg,
// sendmmsg and io_uring sends all end up in one of these two functions, on
// connected and unconnected sockets alike, so two hooks cover every path.
// Minimum kernel is 5.14 (iov_iter.iter_type). Ring buffers need 5.8.

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_endian.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

#ifndef AF_INET
#define AF_INET 2
#endif
#ifndef AF_INET6
#define AF_INET6 10
#endif

#define DNS_PORT 53

// 12-byte header + longest legal name (255 bytes) + qtype and qclass (4) = 271.
// 320 leaves headroom and keeps each ring buffer record small.
#define DNS_PAYLOAD_CAP 320

// Mirrored by DnsSendEvent in src/modules/dns_tunnel.rs. Change both together.
struct dns_send_event {
    __u32 tgid;      // process id
    __u32 tid;       // thread id
    __u32 uid;
    char  comm[16];
    __u16 family;    // AF_INET or AF_INET6
    __u16 dport;     // destination port, host byte order
    __u32 orig_len;  // bytes the caller asked to send
    __u32 cap_len;   // bytes copied into payload
    __u8  payload[DNS_PAYLOAD_CAP];
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024); // about 700 events of 368 bytes
} dns_events SEC(".maps");

// struct iov_iter changed shape across kernels. These local "flavor" structs
// only list the fields used here. libbpf drops the ___suffix and matches each
// field by name against the running kernel's BTF, then patches the offsets at
// load time (CO-RE). One compiled object works on all of these layouts:
//   5.14 to 6.3   the iovec array pointer is named iov
//   6.0 and later single-buffer sends use ITER_UBUF and the ubuf pointer
//   6.4 and later iov was renamed __iov
struct iov_iter___old {
    u8 iter_type;
    const struct iovec *iov;
} __attribute__((preserve_access_index));

struct iov_iter___new {
    u8 iter_type;
    const struct iovec *__iov;
    void *ubuf;
} __attribute__((preserve_access_index));

// The enum values moved as well (ITER_IOVEC is 0 on 5.15 and 1 on 7.0), so
// they are read from the kernel's BTF instead of being hard coded.
enum iter_type___dns {
    ITER_IOVEC___dns,
    ITER_UBUF___dns,
};

static __always_inline bool is_v4_mapped(const struct in6_addr *a)
{
    return a->in6_u.u6_addr32[0] == 0 &&
           a->in6_u.u6_addr32[1] == 0 &&
           a->in6_u.u6_addr32[2] == bpf_htonl(0x0000ffff);
}

// Finds the user buffer behind msg->msg_iter and copies up to
// DNS_PAYLOAD_CAP bytes of it into a new ring buffer event.
static __always_inline int capture(struct msghdr *msg, size_t len, __u16 family, __u16 dport)
{
    struct iov_iter *iter = &msg->msg_iter;
    struct iov_iter___new *iter_new = (void *)iter;
    struct iov_iter___old *iter_old = (void *)iter;
    const void *ubase;
    __u64 seg_len = len;
    __u8 type = BPF_CORE_READ(iter, iter_type);

    if (bpf_core_enum_value_exists(enum iter_type___dns, ITER_UBUF___dns) &&
        type == bpf_core_enum_value(enum iter_type___dns, ITER_UBUF___dns)) {
        // send() and sendto() on 6.x, and sendmsg() with one buffer since 6.4.
        ubase = BPF_CORE_READ(iter_new, ubuf);
    } else if (type == bpf_core_enum_value(enum iter_type___dns, ITER_IOVEC___dns)) {
        // sendmsg() with several buffers, or any send on 5.x. The iovec array
        // was copied into kernel memory. Only the first buffer is read.
        const struct iovec *iov;

        if (bpf_core_field_exists(iter_new->__iov))
            iov = BPF_CORE_READ(iter_new, __iov);
        else
            iov = BPF_CORE_READ(iter_old, iov);
        ubase = BPF_CORE_READ(iov, iov_base);
        seg_len = BPF_CORE_READ(iov, iov_len);
    } else {
        // kvec, bvec and the rest carry kernel buffers, not a process's query.
        return 0;
    }
    if (!ubase)
        return 0;

    // Clamp before the copy so the verifier can prove the write stays inside
    // payload[]. n stays 64-bit on purpose. With a 32-bit n, clang clamped a
    // truncated copy and passed the unclamped register to the helper, and
    // the verifier rejected it ("R2 min value is negative"). barrier_var
    // stops clang from splitting the clamped value again.
    // At function entry nothing has been consumed from the iterator yet, so
    // the data starts at ubase.
    __u64 n = len < seg_len ? len : seg_len;
    if (n > DNS_PAYLOAD_CAP)
        n = DNS_PAYLOAD_CAP;
    barrier_var(n);

    struct dns_send_event *e = bpf_ringbuf_reserve(&dns_events, sizeof(*e), 0);
    if (!e)
        return 0; // ring buffer full, the event is dropped

    __u64 id = bpf_get_current_pid_tgid();
    e->tgid = id >> 32;
    e->tid = (__u32)id;
    e->uid = (__u32)bpf_get_current_uid_gid();
    bpf_get_current_comm(e->comm, sizeof(e->comm));
    e->family = family;
    e->dport = bpf_ntohs(dport);
    e->orig_len = (__u32)len;
    e->cap_len = (__u32)n;
    // The buffer is the sender's user memory, so it needs the _user helper.
    if (bpf_probe_read_user(e->payload, n, ubase) < 0)
        e->cap_len = 0;

    bpf_ringbuf_submit(e, 0);
    return 0;
}

// int udp_sendmsg(struct sock *sk, struct msghdr *msg, size_t len)
SEC("kprobe/udp_sendmsg")
int BPF_KPROBE(dns_udp_sendmsg, struct sock *sk, struct msghdr *msg, size_t len)
{
    struct sockaddr_in *sin = BPF_CORE_READ(msg, msg_name);
    __u16 dport;

    if (sin)
        // sendto() or sendmsg() with an address. msg_name is the kernel's
        // copy of it, so a plain kernel read is safe.
        dport = BPF_CORE_READ(sin, sin_port);
    else
        // connect() then send(). The peer port is stored on the socket.
        dport = BPF_CORE_READ(sk, __sk_common.skc_dport);

    if (bpf_ntohs(dport) != DNS_PORT)
        return 0;
    return capture(msg, len, AF_INET, dport);
}

// int udpv6_sendmsg(struct sock *sk, struct msghdr *msg, size_t len)
SEC("kprobe/udpv6_sendmsg")
int BPF_KPROBE(dns_udpv6_sendmsg, struct sock *sk, struct msghdr *msg, size_t len)
{
    struct sockaddr_in6 *sin6 = BPF_CORE_READ(msg, msg_name);
    __u16 sa_family = 0;
    struct in6_addr daddr;
    __u16 dport;

    if (sin6)
        sa_family = BPF_CORE_READ(sin6, sin6_family);

    // An IPv6 socket sending to an IPv4 address is handed on to
    // udp_sendmsg, and the other probe reports it there.
    if (sa_family == AF_INET)
        return 0;

    if (sa_family == AF_INET6) {
        dport = BPF_CORE_READ(sin6, sin6_port);
        BPF_CORE_READ_INTO(&daddr, sin6, sin6_addr);
    } else {
        // Connected socket (AF_UNSPEC is treated the same way by the kernel).
        dport = BPF_CORE_READ(sk, __sk_common.skc_dport);
        BPF_CORE_READ_INTO(&daddr, sk, __sk_common.skc_v6_daddr);
    }

    if (bpf_ntohs(dport) != DNS_PORT)
        return 0;
    // ::ffff:a.b.c.d is also handed on to udp_sendmsg. Skipping it here
    // keeps dual-stack clients (Java, for example) from being counted twice.
    if (is_v4_mapped(&daddr))
        return 0;
    return capture(msg, len, AF_INET6, dport);
}

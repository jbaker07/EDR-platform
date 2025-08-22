// ebpf/tcp_state.bpf.c
// CO-RE TCP state & retransmit hooks → edr_event
// Switch to kprobes to avoid fragile tracepoint ctx layouts.
// NOTE: edr_events.h already defines the LICENSE symbol; do not redefine here.

#if __has_include("include/edr_events.h")
#  include "include/edr_events.h"
#elif __has_include("../include/edr_events.h")
#  include "../include/edr_events.h"
#else
#  include "edr_events.h"
#endif

#include <bpf/bpf_tracing.h>   // BPF_KPROBE

// Fallbacks to avoid pulling UAPI headers
#ifndef AF_INET6
#define AF_INET6 10
#endif
#ifndef IPPROTO_TCP
#define IPPROTO_TCP 6
#endif

static __always_inline void fill_sock_tuple(struct edr_event *ev, struct sock *sk)
{
    if (!sk) return;

    // family
    __u16 fam = BPF_CORE_READ(sk, __sk_common.skc_family);
    ev->fam = (fam <= 255) ? (__u8)fam : 0;

    // local (keep kernel’s native byte order; normalize in userspace if desired)
    ev->lport  = BPF_CORE_READ(sk, __sk_common.skc_num);
    ev->laddr4 = BPF_CORE_READ(sk, __sk_common.skc_rcv_saddr);

    // remote
    ev->rport  = BPF_CORE_READ(sk, __sk_common.skc_dport);
    ev->raddr4 = BPF_CORE_READ(sk, __sk_common.skc_daddr);

    // IPv6 if applicable
    if (ev->fam == AF_INET6) {
        struct in6_addr l6 = BPF_CORE_READ(sk, __sk_common.skc_v6_rcv_saddr);
        struct in6_addr r6 = BPF_CORE_READ(sk, __sk_common.skc_v6_daddr);
        __builtin_memcpy(ev->laddr6, &l6, sizeof(ev->laddr6));
        __builtin_memcpy(ev->raddr6, &r6, sizeof(ev->raddr6));
    }

    // protocol: these probes are TCP-oriented
    ev->proto = IPPROTO_TCP;
}

/* kprobe: tcp_set_state(struct sock *sk, int state)
 * Some kernels don’t expose sk->sk_state in BTF; rely on __sk_common.skc_state (u8).
 */
SEC("kprobe/tcp_set_state")
int BPF_KPROBE(k_tcp_set_state, struct sock *sk, int newstate)
{
    if (!sk)
        return 0;

    // Read old state from common header; widen to int
    int oldstate = (int)BPF_CORE_READ(sk, __sk_common.skc_state);

    struct edr_event ev = {};
    ev.type       = EVT_TCP_STATE;
    ev.syscall_id = 0;
    ev.tgid       = edr_tgid();
    ev.fd         = -1;

    // pack old/new into aux_u32: high16=old, low16=new
    ev.aux_u32 = (((unsigned)oldstate & 0xFFFF) << 16) | ((unsigned)newstate & 0xFFFF);

    fill_sock_tuple(&ev, sk);
    edr_emit(&ev);
    return 0;
}

/* kprobe: tcp_retransmit_skb(struct sock *sk, struct sk_buff *skb) */
SEC("kprobe/tcp_retransmit_skb")
int BPF_KPROBE(k_tcp_retransmit_skb, struct sock *sk, struct sk_buff *skb)
{
    struct edr_event ev = {};
    ev.type       = EVT_TCP_RETRANS;
    ev.syscall_id = 0;
    ev.tgid       = edr_tgid();
    ev.fd         = -1;

    if (skb) {
        ev.aux_u64 = BPF_CORE_READ(skb, len);
    }

    fill_sock_tuple(&ev, sk);
    edr_emit(&ev);
    return 0;
}

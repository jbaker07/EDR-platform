// ebpf/tcp_state.bpf.c
#include "include/edr_events.h"

static __always_inline void fill_sock_tuple(struct edr_event *ev, struct sock *sk)
{
    if (!sk) return;

    // family
    __u16 fam = BPF_CORE_READ(sk, __sk_common.skc_family);
    ev->fam = (fam <= 255) ? (__u8)fam : 0;

    // local
    ev->lport  = BPF_CORE_READ(sk, __sk_common.skc_num);   // usually BE already; we keep BE consistently
    ev->laddr4 = BPF_CORE_READ(sk, __sk_common.skc_rcv_saddr);

    // remote
    ev->rport  = BPF_CORE_READ(sk, __sk_common.skc_dport); // BE
    ev->raddr4 = BPF_CORE_READ(sk, __sk_common.skc_daddr);

    // IPv6 if applicable
    if (ev->fam == AF_INET6) {
        // v6 local/remote are in skc_v6_rcv_saddr / skc_v6_daddr
        struct in6_addr l6 = BPF_CORE_READ(sk, __sk_common.skc_v6_rcv_saddr);
        struct in6_addr r6 = BPF_CORE_READ(sk, __sk_common.skc_v6_daddr);
        __builtin_memcpy(ev->laddr6, &l6, sizeof(ev->laddr6));
        __builtin_memcpy(ev->raddr6, &r6, sizeof(ev->raddr6));
    }

    // protocol: kernel doesn’t store IPPROTO_* on sk; assume TCP for this TP
    ev->proto = IPPROTO_TCP;
}

/* tcp/tcp_set_state
 * Context (CO-RE): struct trace_event_raw_tcp_event_sk *
 * Members include: struct sock *sk; int oldstate; int newstate;
 */
SEC("tracepoint/tcp/tcp_set_state")
int tp_tcp_set_state(struct trace_event_raw_tcp_event_sk *ctx)
{
    struct sock *sk = (struct sock *)BPF_CORE_READ(ctx, sk);
    int oldstate    = BPF_CORE_READ(ctx, oldstate);
    int newstate    = BPF_CORE_READ(ctx, newstate);

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

/* tcp/tcp_retransmit_skb
 * Context: struct trace_event_raw_tcp_event_sk_skb *
 * Members include: struct sock *sk; const struct sk_buff *skb; ...
 * We emit EVT_TCP_RETRANS and include skb->len in aux_u64 (best-effort).
 */
SEC("tracepoint/tcp/tcp_retransmit_skb")
int tp_tcp_retransmit_skb(struct trace_event_raw_tcp_event_sk_skb *ctx)
{
    struct sock *sk = (struct sock *)BPF_CORE_READ(ctx, sk);

    struct edr_event ev = {};
    ev.type       = EVT_TCP_RETRANS;
    ev.syscall_id = 0;
    ev.tgid       = edr_tgid();
    ev.fd         = -1;

    // skb->len is kernel memory; core-read if BTF is stable
    const struct sk_buff *skb = (const struct sk_buff *)BPF_CORE_READ(ctx, skbaddr);
    if (skb) {
        ev.aux_u64 = BPF_CORE_READ(skb, len);
    }

    fill_sock_tuple(&ev, sk);
    edr_emit(&ev);
    return 0;
}

char LICENSE[] SEC("license") = "Dual BSD/GPL";

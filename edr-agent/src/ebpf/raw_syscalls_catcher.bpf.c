// ebpf/raw_syscalls_catcher.bpf.c
#if __has_include("include/edr_events.h")
#  include "include/edr_events.h"
#elif __has_include("../include/edr_events.h")
#  include "../include/edr_events.h"
#else
#  include "edr_events.h"
#endif

/* ---- Minimal fallbacks to avoid UAPI headers ---- */
#ifndef AF_INET
#define AF_INET 2
#endif
#ifndef AF_INET6
#define AF_INET6 10
#endif
#ifndef IPPROTO_TCP
#define IPPROTO_TCP 6
#endif

/* ---- arm64 syscall numbers used in this file (no <asm/unistd.h>) ---- */
/* file/io */
#ifndef __NR_openat
#define __NR_openat 56
#endif
#ifndef __NR_openat2
#define __NR_openat2 437
#endif
#ifndef __NR_close
#define __NR_close 57
#endif
#ifndef __NR_read
#define __NR_read 63
#endif
#ifndef __NR_write
#define __NR_write 64
#endif
#ifndef __NR_renameat2
#define __NR_renameat2 276
#endif
#ifndef __NR_unlinkat
#define __NR_unlinkat 35
#endif
#ifndef __NR_fchmodat
#define __NR_fchmodat 53
#endif
#ifndef __NR_fchmodat2
#define __NR_fchmodat2 452
#endif
#ifndef __NR_fchownat
#define __NR_fchownat 54
#endif
#ifndef __NR_dup
#define __NR_dup 23
#endif
/* dup2 is not native on arm64; intentionally not defined */
#ifndef __NR_dup3
#define __NR_dup3 24
#endif

/* xattr */
#ifndef __NR_setxattr
#define __NR_setxattr 5
#endif
#ifndef __NR_lsetxattr
#define __NR_lsetxattr 6
#endif
#ifndef __NR_fsetxattr
#define __NR_fsetxattr 7
#endif

/* net */
#ifndef __NR_socket
#define __NR_socket 198
#endif
#ifndef __NR_bind
#define __NR_bind 200
#endif
#ifndef __NR_listen
#define __NR_listen 201
#endif
#ifndef __NR_accept
#define __NR_accept 202
#endif
#ifndef __NR_accept4
#define __NR_accept4 242
#endif
#ifndef __NR_connect
#define __NR_connect 203
#endif
#ifndef __NR_sendto
#define __NR_sendto 206
#endif
#ifndef __NR_recvfrom
#define __NR_recvfrom 207
#endif
#ifndef __NR_sendmsg
#define __NR_sendmsg 211
#endif
#ifndef __NR_recvmsg
#define __NR_recvmsg 212
#endif

/* proc/mem/priv */
#ifndef __NR_execve
#define __NR_execve 221
#endif
#ifndef __NR_execveat
#define __NR_execveat 281
#endif
#ifndef __NR_mprotect
#define __NR_mprotect 226
#endif
#ifndef __NR_ptrace
#define __NR_ptrace 117
#endif
#ifndef __NR_prctl
#define __NR_prctl 167
#endif
#ifndef __NR_seccomp
#define __NR_seccomp 277
#endif
#ifndef __NR_clone
#define __NR_clone 220
#endif
#ifndef __NR_clone3
#define __NR_clone3 435
#endif

/* ns/mount/bpf/modules */
#ifndef __NR_setns
#define __NR_setns 268
#endif
#ifndef __NR_unshare
#define __NR_unshare 97
#endif
#ifndef __NR_mount
#define __NR_mount 40
#endif
#ifndef __NR_umount2
#define __NR_umount2 39
#endif
#ifndef __NR_pivot_root
#define __NR_pivot_root 41
#endif
#ifndef __NR_bpf
#define __NR_bpf 280
#endif
#ifndef __NR_init_module
#define __NR_init_module 105
#endif
#ifndef __NR_finit_module
#define __NR_finit_module 273
#endif
#ifndef __NR_delete_module
#define __NR_delete_module 106
#endif

/* uid/caps — leave undefined if unsure; guards below will skip blocks if absent */
/* #ifndef __NR_capset
#define __NR_capset 90
#endif */
#ifndef __NR_setuid
#define __NR_setuid 146
#endif

/* -------- pending stash for dup* (oldfd across enter→exit) -------- */
struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, 8192);
    __type(key, __u64);   // pid_tgid
    __type(value, __s32); // oldfd
} edr_pending_dup SEC(".maps");

/* -------- ringbuf helpers to avoid large stack objects -------- */
static __always_inline struct edr_event *ev_reserve_zero(void)
{
    struct edr_event *e = bpf_ringbuf_reserve(&edr_events_rb, sizeof(*e), 0);
    if (!e) return 0;
    __builtin_memset(e, 0, sizeof(*e));
    return e;
}

static __always_inline void ev_fill_common(struct edr_event *e, __u32 type, __u32 id, __u32 tgid)
{
    e->type = type;
    e->syscall_id = id;
    e->tgid = tgid;
    e->ts = bpf_ktime_get_ns();
    e->uid = edr_uid();
    edr_ppid_uid_comm(&e->ppid, e->comm);
}

/* -------- sys_enter -------- */
SEC("tracepoint/raw_syscalls/sys_enter")
int tp_sys_enter(struct trace_event_raw_sys_enter *ctx)
{
    __u32 id = (__u32)ctx->id;
    if (!edr_allowed(id)) return 0;

    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 tgid = (__u32)(pid_tgid >> 32);

    switch (id) {
    /* ---- PROCESS / MEMORY ---- */
    case __NR_execve:
    case __NR_execveat: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_EXEC, id, tgid);
        e->fd = -1;
        const char *filename = (const char *)ctx->args[0];
        edr_copy_user_str(e->path, filename, sizeof(e->path));
        bpf_ringbuf_submit(e, 0);
        break;
    }
    case __NR_mprotect: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_MPROTECT, id, tgid);
        e->fd = -1;
        e->flags = (__u32)ctx->args[2];   // prot
        e->aux_u64 = (__u64)ctx->args[1]; // len
        bpf_ringbuf_submit(e, 0);
        break;
    }
#if defined(__NR_memfd_create)
    case __NR_memfd_create: {
        struct pending_open po = {};
        const char *name = (const char *)ctx->args[0];
        po.flags = (__u32)ctx->args[1];
        edr_copy_user_str(po.path, name, sizeof(po.path));
        bpf_map_update_elem(&edr_pending_open, &pid_tgid, &po, BPF_ANY);
        break;
    }
#endif
    case __NR_ptrace: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_PTRACE, id, tgid);
        e->fd = -1;
        e->flags = (__u32)ctx->args[0];   // request
        e->aux_u32 = (__u32)ctx->args[1]; // pid
        bpf_ringbuf_submit(e, 0);
        break;
    }
#if defined(__NR_prctl)
    case __NR_prctl: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_PRCTL, id, tgid);
        e->fd = -1;
        e->flags = (__u32)ctx->args[0]; // option
        bpf_ringbuf_submit(e, 0);
        break;
    }
#endif
#if defined(__NR_seccomp)
    case __NR_seccomp: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_SECCOMP, id, tgid);
        e->fd = -1;
        e->flags = (__u32)ctx->args[0]; // op
        bpf_ringbuf_submit(e, 0);
        break;
    }
#endif

    /* ---- FILESYSTEM ---- */
    case __NR_openat:
#if defined(__NR_openat2)
    case __NR_openat2:
#endif
    {
        struct pending_open po = {};
        const char *pathname = (const char *)ctx->args[1];
        po.flags = (__u32)ctx->args[2];
        edr_copy_user_str(po.path, pathname, sizeof(po.path));
        bpf_map_update_elem(&edr_pending_open, &pid_tgid, &po, BPF_ANY);
        break;
    }
#if defined(__NR_renameat2)
    case __NR_renameat2: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_RENAME, id, tgid);
        e->fd = -1;
        const char *oldp = (const char *)ctx->args[1];
        const char *newp = (const char *)ctx->args[3];
        edr_copy_user_str(e->path,  oldp, sizeof(e->path));
        edr_copy_user_str(e->path2, newp, sizeof(e->path2));
        bpf_ringbuf_submit(e, 0);
        break;
    }
#endif
    case __NR_unlinkat: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_UNLINK, id, tgid);
        e->fd = -1;
        const char *pathname = (const char *)ctx->args[1];
        edr_copy_user_str(e->path, pathname, sizeof(e->path));
        e->flags = (__u32)ctx->args[2]; // flags (e.g., AT_REMOVEDIR)
        bpf_ringbuf_submit(e, 0);
        break;
    }
#if defined(__NR_chmod)
    case __NR_chmod:
#endif
#if defined(__NR_fchmodat)
    case __NR_fchmodat:
#endif
#if defined(__NR_fchmodat2)
    case __NR_fchmodat2:
#endif
    {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_CHMOD_CHOWN, id, tgid);
        e->fd = -1;
        const char *pathname = (const char *)ctx->args[0 + (id==__NR_fchmodat || id==__NR_fchmodat2)];
        edr_copy_user_str(e->path, pathname, sizeof(e->path));
        e->flags = (__u32)ctx->args[1 + (id==__NR_fchmodat || id==__NR_fchmodat2)];
        bpf_ringbuf_submit(e, 0);
        break;
    }
#if defined(__NR_chown)
    case __NR_chown:
#endif
#if defined(__NR_fchownat)
    case __NR_fchownat:
#endif
    {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_CHMOD_CHOWN, id, tgid);
        e->fd = -1;
        const char *pathname = (const char *)ctx->args[0 + (id==__NR_fchownat)];
        edr_copy_user_str(e->path, pathname, sizeof(e->path));
        bpf_ringbuf_submit(e, 0);
        break;
    }
#if defined(__NR_setxattr)
    case __NR_setxattr:
#endif
#if defined(__NR_lsetxattr)
    case __NR_lsetxattr:
#endif
#if defined(__NR_fsetxattr)
    case __NR_fsetxattr:
#endif
    {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_SETXATTR, id, tgid);
        e->fd = -1;
        const char *path = (const char *)ctx->args[0];
        edr_copy_user_str(e->path, path, sizeof(e->path));
        bpf_ringbuf_submit(e, 0);
        break;
    }
    case __NR_close: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_CLOSE, id, tgid);
        e->fd = (__s32)ctx->args[0];
        bpf_ringbuf_submit(e, 0);
        struct fd_key fk = { .tgid = tgid, .fd = e->fd };
        bpf_map_delete_elem(&edr_fd_table, &fk);
        break;
    }
#if defined(__NR_dup)
    case __NR_dup:
#endif
#if defined(__NR_dup2)
    case __NR_dup2:
#endif
#if defined(__NR_dup3)
    case __NR_dup3:
#endif
    {
        __s32 oldfd = (__s32)ctx->args[0];
        bpf_map_update_elem(&edr_pending_dup, &pid_tgid, &oldfd, BPF_ANY);

        struct edr_event *e = ev_reserve_zero();
        if (e) {
            ev_fill_common(e, EVT_DUP, id, tgid);
            /* finalize on exit when we know newfd */
            bpf_ringbuf_submit(e, 0);
        }
        break;
    }

    /* ---- NETWORKING ---- */
    case __NR_socket: {
        struct pending_socket ps = {};
        ps.domain = (__s32)ctx->args[0];
        ps.type   = (__s32)ctx->args[1];
        ps.proto  = (__s32)ctx->args[2];
        bpf_map_update_elem(&edr_pending_socket, &pid_tgid, &ps, BPF_ANY);
        break;
    }
    case __NR_bind: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_BIND, id, tgid);
        e->fd = (__s32)ctx->args[0];
        const void *uaddr = (const void *)ctx->args[1];
        __s32 addrlen = (__s32)ctx->args[2];
        struct sockaddr sa = {};
        if (addrlen >= (int)sizeof(sa)) {
            bpf_probe_read_user(&sa, sizeof(sa), uaddr);
            e->fam = sa.sa_family;
            if (sa.sa_family == AF_INET && addrlen >= (int)sizeof(struct sockaddr_in)) {
                struct sockaddr_in sin = {};
                bpf_probe_read_user(&sin, sizeof(sin), uaddr);
                e->laddr4 = sin.sin_addr.s_addr; // BE
                e->lport  = sin.sin_port;        // BE
            } else if (sa.sa_family == AF_INET6 && addrlen >= (int)sizeof(struct sockaddr_in6)) {
                struct sockaddr_in6 sin6 = {};
                bpf_probe_read_user(&sin6, sizeof(sin6), uaddr);
                __builtin_memcpy(e->laddr6, &sin6.sin6_addr, 16);
                e->lport = sin6.sin6_port;
            }
        }
        /* update meta if present */
        struct fd_key fk = { .tgid = tgid, .fd = e->fd };
        struct fd_meta *fm = bpf_map_lookup_elem(&edr_fd_table, &fk);
        if (fm && fm->kind == 2) {
            fm->family = e->fam;
            fm->lport  = e->lport;
            fm->laddr4 = e->laddr4;
            __builtin_memcpy(fm->laddr6, e->laddr6, 16);
        }
        bpf_ringbuf_submit(e, 0);
        break;
    }
    case __NR_listen: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_LISTEN, id, tgid);
        e->fd = (__s32)ctx->args[0];
        bpf_ringbuf_submit(e, 0);
        break;
    }
    case __NR_connect: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_CONNECT, id, tgid);
        e->fd = (__s32)ctx->args[0];
        const void *uaddr = (const void *)ctx->args[1];
        __s32 addrlen = (__s32)ctx->args[2];
        struct sockaddr sa = {};
        if (addrlen >= (int)sizeof(sa)) {
            bpf_probe_read_user(&sa, sizeof(sa), uaddr);
            e->fam = sa.sa_family;
            if (sa.sa_family == AF_INET && addrlen >= (int)sizeof(struct sockaddr_in)) {
                struct sockaddr_in sin = {};
                bpf_probe_read_user(&sin, sizeof(sin), uaddr);
                e->raddr4 = sin.sin_addr.s_addr; e->rport = sin.sin_port; e->proto = IPPROTO_TCP;
            } else if (sa.sa_family == AF_INET6 && addrlen >= (int)sizeof(struct sockaddr_in6)) {
                struct sockaddr_in6 sin6 = {};
                bpf_probe_read_user(&sin6, sizeof(sin6), uaddr);
                __builtin_memcpy(e->raddr6, &sin6.sin6_addr, 16);
                e->rport = sin6.sin6_port; e->proto = IPPROTO_TCP;
            }
        }
        /* update fd meta if exists */
        struct fd_key fk = { .tgid = tgid, .fd = e->fd };
        struct fd_meta *fm = bpf_map_lookup_elem(&edr_fd_table, &fk);
        if (fm && fm->kind == 2) {
            fm->family = e->fam; fm->proto = e->proto; fm->rport = e->rport; fm->raddr4 = e->raddr4;
            __builtin_memcpy(fm->raddr6, e->raddr6, 16);
        }
        bpf_ringbuf_submit(e, 0);
        break;
    }
    case __NR_accept:
#if defined(__NR_accept4)
    case __NR_accept4:
#endif
    {
        /* Save peer sock from user pointer at enter (if any) */
        const void *upeer = (const void *)ctx->args[1];
        if (upeer) {
            struct pending_accept pa = {};
            struct sockaddr sa = {};
            bpf_probe_read_user(&sa, sizeof(sa), upeer);
            pa.fam = sa.sa_family;
            if (sa.sa_family == AF_INET) {
                struct sockaddr_in sin = {};
                bpf_probe_read_user(&sin, sizeof(sin), upeer);
                pa.raddr4 = sin.sin_addr.s_addr; pa.rport = sin.sin_port;
            } else if (sa.sa_family == AF_INET6) {
                struct sockaddr_in6 sin6 = {};
                bpf_probe_read_user(&sin6, sizeof(sin6), upeer);
                __builtin_memcpy(pa.raddr6, &sin6.sin6_addr, 16); pa.rport = sin6.sin6_port;
            }
            bpf_map_update_elem(&edr_pending_accept, &pid_tgid, &pa, BPF_ANY);
        }
        break;
    }
    case __NR_sendto: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_SENDTO, id, tgid);
        e->fd = (__s32)ctx->args[0];
        const void *uaddr = (const void *)ctx->args[4];
        __s32 addrlen = (__s32)ctx->args[5];
        if (uaddr && addrlen > 0) {
            struct sockaddr sa = {};
            bpf_probe_read_user(&sa, sizeof(sa), uaddr);
            e->fam = sa.sa_family;
            if (sa.sa_family == AF_INET) {
                struct sockaddr_in sin = {};
                bpf_probe_read_user(&sin, sizeof(sin), uaddr);
                e->raddr4 = sin.sin_addr.s_addr; e->rport = sin.sin_port;
            } else if (sa.sa_family == AF_INET6) {
                struct sockaddr_in6 sin6 = {};
                bpf_probe_read_user(&sin6, sizeof(sin6), uaddr);
                __builtin_memcpy(e->raddr6, &sin6.sin6_addr, 16); e->rport = sin6.sin6_port;
            }
        }
        bpf_ringbuf_submit(e, 0);
        break;
    }
    case __NR_sendmsg: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_SENDMSG, id, tgid);
        e->fd = (__s32)ctx->args[0];
        struct user_msghdr {
            void *msg_name;
            int   msg_namelen;
        } msgh = {};
        bpf_probe_read_user(&msgh, sizeof(msgh), (void*)ctx->args[1]);
        if (msgh.msg_name && msgh.msg_namelen > 0) {
            struct sockaddr sa = {};
            bpf_probe_read_user(&sa, sizeof(sa), msgh.msg_name);
            e->fam = sa.sa_family;
            if (sa.sa_family == AF_INET) {
                struct sockaddr_in sin = {};
                bpf_probe_read_user(&sin, sizeof(sin), msgh.msg_name);
                e->raddr4 = sin.sin_addr.s_addr; e->rport = sin.sin_port;
            } else if (sa.sa_family == AF_INET6) {
                struct sockaddr_in6 sin6 = {};
                bpf_probe_read_user(&sin6, sizeof(sin6), msgh.msg_name);
                __builtin_memcpy(e->raddr6, &sin6.sin6_addr, 16); e->rport = sin6.sin6_port;
            }
        }
        bpf_ringbuf_submit(e, 0);
        break;
    }
    case __NR_recvfrom: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_RECVFROM, id, tgid);
        e->fd = (__s32)ctx->args[0];
        bpf_ringbuf_submit(e, 0);
        break;
    }
    case __NR_recvmsg: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_RECVMSG, id, tgid);
        e->fd = (__s32)ctx->args[0];
        bpf_ringbuf_submit(e, 0);
        break;
    }

    /* ---- NAMESPACE / MOUNT / MODULE / BPF ---- */
#if defined(__NR_setns)
    case __NR_setns: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_SETNS, id, tgid);
        e->fd = -1;
        bpf_ringbuf_submit(e, 0);
        break;
    }
#endif
#if defined(__NR_unshare)
    case __NR_unshare: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_UNSHARE, id, tgid);
        e->fd = -1;
        e->flags = (__u32)ctx->args[0];
        bpf_ringbuf_submit(e, 0);
        break;
    }
#endif
    case __NR_mount: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_MOUNT, id, tgid);
        e->fd = -1;
        bpf_ringbuf_submit(e, 0);
        break;
    }
    case __NR_umount2: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_UMOUNT2, id, tgid);
        e->fd = -1;
        bpf_ringbuf_submit(e, 0);
        break;
    }
#if defined(__NR_pivot_root)
    case __NR_pivot_root: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_PIVOT_ROOT, id, tgid);
        e->fd = -1;
        bpf_ringbuf_submit(e, 0);
        break;
    }
#endif
#if defined(__NR_bpf)
    case __NR_bpf: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_BPF, id, tgid);
        e->fd = -1;
        e->aux_u32 = (__u32)ctx->args[0]; // cmd
        bpf_ringbuf_submit(e, 0);
        break;
    }
#endif
#if defined(__NR_init_module)
    case __NR_init_module:
#endif
#if defined(__NR_finit_module)
    case __NR_finit_module:
#endif
    {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_MOD_LOAD, id, tgid);
        e->fd = -1;
        bpf_ringbuf_submit(e, 0);
        break;
    }
#if defined(__NR_delete_module)
    case __NR_delete_module: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_MOD_DEL, id, tgid);
        e->fd = -1;
        bpf_ringbuf_submit(e, 0);
        break;
    }
#endif

    /* ---- PRIVILEGE ---- */
#if defined(__NR_setuid)
    case __NR_setuid: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_SETUID, id, tgid);
        e->fd = -1;
        e->aux_u32 = (__u32)ctx->args[0]; // new uid
        bpf_ringbuf_submit(e, 0);
        break;
    }
#endif
#if defined(__NR_capset)
    case __NR_capset: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, EVT_CAPSET, id, tgid);
        e->fd = -1;
        bpf_ringbuf_submit(e, 0);
        break;
    }
#endif

    /* ---- R/W sizes ---- */
    case __NR_read:
    case __NR_write: {
        struct edr_event *e = ev_reserve_zero();
        if (!e) break;
        ev_fill_common(e, (id == __NR_read) ? EVT_READ : EVT_WRITE, id, tgid);
        e->fd = (__s32)ctx->args[0];
        e->aux_u64 = (__u64)ctx->args[2]; // count
        bpf_ringbuf_submit(e, 0);
        break;
    }

    default:
        break;
    }

    return 0;
}

/* -------- sys_exit -------- */
SEC("tracepoint/raw_syscalls/sys_exit")
int tp_sys_exit(struct trace_event_raw_sys_exit *ctx)
{
    __u32 id = (__u32)ctx->id;
    if (!edr_allowed(id)) return 0;

    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u32 tgid = (__u32)(pid_tgid >> 32);
    __s64 ret = ctx->ret;

    switch (id) {
    /* open family → fd table */
    case __NR_openat:
#if defined(__NR_openat2)
    case __NR_openat2:
#endif
    {
        struct pending_open *po = bpf_map_lookup_elem(&edr_pending_open, &pid_tgid);
        if (!po) break;

        struct edr_event *e = ev_reserve_zero();
        if (e) {
            ev_fill_common(e, EVT_OPEN, id, tgid);
            e->ret = ret; e->fd = (ret >= 0) ? (__s32)ret : -1;
            e->flags = po->flags;
            __builtin_memcpy(e->path, po->path, sizeof(e->path));
            bpf_ringbuf_submit(e, 0);
        }

        if (ret >= 0) {
            struct fd_key fk = { .tgid = tgid, .fd = (__s32)ret };
            struct fd_meta fm = {};
            fm.kind = 1; // file
            __builtin_memcpy(fm.path, po->path, sizeof(fm.path));
            bpf_map_update_elem(&edr_fd_table, &fk, &fm, BPF_ANY);
        }
        bpf_map_delete_elem(&edr_pending_open, &pid_tgid);
        break;
    }

    /* socket → fd table */
    case __NR_socket: {
        struct pending_socket *ps = bpf_map_lookup_elem(&edr_pending_socket, &pid_tgid);
        if (!ps) break;

        struct edr_event *e = ev_reserve_zero();
        if (e) {
            ev_fill_common(e, EVT_SOCKET, id, tgid);
            e->ret = ret; e->fd = (ret >= 0) ? (__s32)ret : -1;
            e->fam = (ps->domain >= 0 && ps->domain <= 255) ? (__u8)ps->domain : 0;
            e->proto = (ps->proto  >= 0 && ps->proto  <= 255) ? (__u8)ps->proto  : 0;
            bpf_ringbuf_submit(e, 0);
        }

        if (ret >= 0) {
            struct fd_key fk = { .tgid = tgid, .fd = (__s32)ret };
            struct fd_meta fm = {};
            fm.kind   = 2; // socket
            fm.family = (ps->domain >= 0 && ps->domain <= 255) ? (__u8)ps->domain : 0;
            fm.proto  = (ps->proto  >= 0 && ps->proto  <= 255) ? (__u8)ps->proto  : 0;
            bpf_map_update_elem(&edr_fd_table, &fk, &fm, BPF_ANY);
        }
        bpf_map_delete_elem(&edr_pending_socket, &pid_tgid);
        break;
    }

#if defined(__NR_memfd_create)
    case __NR_memfd_create: {
        struct pending_open *po = bpf_map_lookup_elem(&edr_pending_open, &pid_tgid);
        if (!po) break;

        struct edr_event *e = ev_reserve_zero();
        if (e) {
            ev_fill_common(e, EVT_MEMFD_CREATE, id, tgid);
            e->ret = ret; e->fd = (ret >= 0) ? (__s32)ret : -1;
            __builtin_memcpy(e->path, po->path, sizeof(e->path));
            e->flags = po->flags;
            bpf_ringbuf_submit(e, 0);
        }

        if (ret >= 0) {
            struct fd_key fk = { .tgid = tgid, .fd = (__s32)ret };
            struct fd_meta fm = {};
            fm.kind = 1; // treat as file-ish
            __builtin_memcpy(fm.path, po->path, sizeof(fm.path));
            bpf_map_update_elem(&edr_fd_table, &fk, &fm, BPF_ANY);
        }
        bpf_map_delete_elem(&edr_pending_open, &pid_tgid);
        break;
    }
#endif

    /* accept → new fd + peer tuple */
    case __NR_accept:
#if defined(__NR_accept4)
    case __NR_accept4:
#endif
    {
        struct pending_accept *pa = bpf_map_lookup_elem(&edr_pending_accept, &pid_tgid);

        struct edr_event *e = ev_reserve_zero();
        if (e) {
            ev_fill_common(e, EVT_ACCEPT, id, tgid);
            e->ret = ret; e->fd = (ret >= 0) ? (__s32)ret : -1;
            if (pa) {
                e->fam = pa->fam; e->proto = IPPROTO_TCP; // heuristic
                e->rport  = pa->rport; e->raddr4 = pa->raddr4;
                __builtin_memcpy(e->raddr6, pa->raddr6, 16);
            }
            bpf_ringbuf_submit(e, 0);
        }

        if (ret >= 0) {
            struct fd_key fk = { .tgid = tgid, .fd = (__s32)ret };
            struct fd_meta fm = {};
            fm.kind = 2; fm.family = pa ? pa->fam : 0; fm.proto = IPPROTO_TCP;
            fm.rport = pa ? pa->rport : 0; fm.raddr4 = pa ? pa->raddr4 : 0;
            if (pa) __builtin_memcpy(fm.raddr6, pa->raddr6, 16);
            bpf_map_update_elem(&edr_fd_table, &fk, &fm, BPF_ANY);
        }
        if (pa) bpf_map_delete_elem(&edr_pending_accept, &pid_tgid);
        break;
    }

#if defined(__NR_dup) || defined(__NR_dup2) || defined(__NR_dup3)
    case __NR_dup:
#  if defined(__NR_dup2)
    case __NR_dup2:
#  endif
#  if defined(__NR_dup3)
    case __NR_dup3:
#  endif
    {
        if (ret >= 0) {
            __s32 *oldfdp = bpf_map_lookup_elem(&edr_pending_dup, &pid_tgid);
            __s32 oldfd = oldfdp ? *oldfdp : -1;
            __s32 newfd = (__s32)ret;

            if (oldfd >= 0) {
                struct fd_key src = { .tgid = tgid, .fd = oldfd };
                struct fd_key dst = { .tgid = tgid, .fd = newfd };
                struct fd_meta *fm = bpf_map_lookup_elem(&edr_fd_table, &src);
                if (fm) {
                    bpf_map_update_elem(&edr_fd_table, &dst, fm, BPF_ANY);
                }
            }

            struct edr_event *e = ev_reserve_zero();
            if (e) {
                ev_fill_common(e, EVT_DUP, id, tgid);
                e->fd = newfd; e->aux_u32 = (unsigned)(oldfd >= 0 ? oldfd : 0);
                bpf_ringbuf_submit(e, 0);
            }

            if (oldfdp) bpf_map_delete_elem(&edr_pending_dup, &pid_tgid);
        }
        break;
    }
#endif

    case __NR_clone:
#if defined(__NR_clone3)
    case __NR_clone3:
#endif
    {
        struct pending_clone *pc = bpf_map_lookup_elem(&edr_pending_clone, &pid_tgid);
        struct edr_event *e = ev_reserve_zero();
        if (e) {
            ev_fill_common(e, EVT_CLONE, id, tgid);
            e->ret  = ret;  // child pid (>0) in parent
            if (pc) e->aux_u64 = pc->flags;
            bpf_ringbuf_submit(e, 0);
        }
        if (pc) bpf_map_delete_elem(&edr_pending_clone, &pid_tgid);
        break;
    }

    default:
        break;
    }

    return 0;
}

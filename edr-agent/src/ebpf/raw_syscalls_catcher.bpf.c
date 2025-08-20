// ebpf/raw_syscalls_catcher.bpf.c
#include "include/edr_events.h"

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
        struct edr_event ev = {};
        ev.type = EVT_EXEC; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        const char *filename = (const char *)ctx->args[0];
        edr_copy_user_str(ev.path, filename, sizeof(ev.path));
        edr_emit(&ev);
        break;
    }
    case __NR_mprotect: {
        struct edr_event ev = {};
        ev.type = EVT_MPROTECT; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        ev.flags = (__u32)ctx->args[2];   // prot
        ev.aux_u64 = (__u64)ctx->args[1]; // len
        edr_emit(&ev);
        break;
    }
#ifdef __NR_memfd_create
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
        struct edr_event ev = {};
        ev.type = EVT_PTRACE; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        ev.flags = (__u32)ctx->args[0];   // request
        ev.aux_u32 = (__u32)ctx->args[1]; // pid
        edr_emit(&ev);
        break;
    }
#ifdef __NR_prctl
    case __NR_prctl: {
        struct edr_event ev = {};
        ev.type = EVT_PRCTL; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        ev.flags = (__u32)ctx->args[0]; // option
        edr_emit(&ev);
        break;
    }
#endif
#ifdef __NR_seccomp
    case __NR_seccomp: {
        struct edr_event ev = {};
        ev.type = EVT_SECCOMP; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        ev.flags = (__u32)ctx->args[0]; // op
        edr_emit(&ev);
        break;
    }
#endif

    /* ---- FILESYSTEM ---- */
    case __NR_openat:
#ifdef __NR_openat2
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
#ifdef __NR_renameat2
    case __NR_renameat2: {
        struct edr_event ev = {};
        ev.type = EVT_RENAME; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        const char *oldp = (const char *)ctx->args[1];
        const char *newp = (const char *)ctx->args[3];
        edr_copy_user_str(ev.path,  oldp, sizeof(ev.path));
        edr_copy_user_str(ev.path2, newp, sizeof(ev.path2));
        edr_emit(&ev);
        break;
    }
#endif
    case __NR_unlinkat: {
        struct edr_event ev = {};
        ev.type = EVT_UNLINK; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        const char *pathname = (const char *)ctx->args[1];
        edr_copy_user_str(ev.path, pathname, sizeof(ev.path));
        ev.flags = (__u32)ctx->args[2]; // flags (e.g., AT_REMOVEDIR)
        edr_emit(&ev);
        break;
    }
#ifdef __NR_chmod
    case __NR_chmod:
#endif
#ifdef __NR_fchmodat
    case __NR_fchmodat:
#endif
#ifdef __NR_fchmodat2
    case __NR_fchmodat2:
#endif
    {
        struct edr_event ev = {};
        ev.type = EVT_CHMOD_CHOWN; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        const char *pathname = (const char *)ctx->args[0 + (id==__NR_fchmodat || id==__NR_fchmodat2)];
        edr_copy_user_str(ev.path, pathname, sizeof(ev.path));
        ev.flags = (__u32)ctx->args[1 + (id==__NR_fchmodat || id==__NR_fchmodat2)];
        edr_emit(&ev);
        break;
    }
#ifdef __NR_chown
    case __NR_chown:
#endif
#ifdef __NR_fchownat
    case __NR_fchownat:
#endif
    {
        struct edr_event ev = {};
        ev.type = EVT_CHMOD_CHOWN; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        const char *pathname = (const char *)ctx->args[0 + (id==__NR_fchownat)];
        edr_copy_user_str(ev.path, pathname, sizeof(ev.path));
        edr_emit(&ev);
        break;
    }
#ifdef __NR_setxattr
    case __NR_setxattr:
#endif
#ifdef __NR_lsetxattr
    case __NR_lsetxattr:
#endif
#ifdef __NR_fsetxattr
    case __NR_fsetxattr:
#endif
    {
        struct edr_event ev = {};
        ev.type = EVT_SETXATTR; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        const char *path = (const char *)ctx->args[0];
        edr_copy_user_str(ev.path, path, sizeof(ev.path));
        edr_emit(&ev);
        break;
    }
    case __NR_close: {
        struct edr_event ev = {};
        ev.type = EVT_CLOSE; ev.syscall_id = id; ev.tgid = tgid;
        ev.fd = (__s32)ctx->args[0];
        edr_emit(&ev);
        // Remove from FD table immediately; no need to wait for exit
        struct fd_key fk = { .tgid = tgid, .fd = ev.fd };
        bpf_map_delete_elem(&edr_fd_table, &fk);
        break;
    }
#ifdef __NR_dup
    case __NR_dup:
#endif
#ifdef __NR_dup2
    case __NR_dup2:
#endif
#ifdef __NR_dup3
    case __NR_dup3:
#endif
    {
        struct edr_event ev = {};
        ev.type = EVT_DUP; ev.syscall_id = id; ev.tgid = tgid;
        // We’ll finalize new fd on exit; emit enter just for visibility
        edr_emit(&ev);
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
        struct edr_event ev = {};
        ev.type = EVT_BIND; ev.syscall_id = id; ev.tgid = tgid;
        ev.fd = (__s32)ctx->args[0];
        const void *uaddr = (const void *)ctx->args[1];
        __s32 addrlen = (__s32)ctx->args[2];
        struct sockaddr sa = {};
        if (addrlen >= (int)sizeof(sa)) {
            bpf_probe_read_user(&sa, sizeof(sa), uaddr);
            ev.fam = sa.sa_family;
            if (sa.sa_family == AF_INET && addrlen >= (int)sizeof(struct sockaddr_in)) {
                struct sockaddr_in sin = {};
                bpf_probe_read_user(&sin, sizeof(sin), uaddr);
                ev.laddr4 = sin.sin_addr.s_addr; // BE
                ev.lport  = sin.sin_port;        // BE
            } else if (sa.sa_family == AF_INET6 && addrlen >= (int)sizeof(struct sockaddr_in6)) {
                struct sockaddr_in6 sin6 = {};
                bpf_probe_read_user(&sin6, sizeof(sin6), uaddr);
                __builtin_memcpy(ev.laddr6, &sin6.sin6_addr, 16);
                ev.lport = sin6.sin6_port;
            }
        }
        /* update meta if present */
        struct fd_key fk = { .tgid = tgid, .fd = ev.fd };
        struct fd_meta *fm = bpf_map_lookup_elem(&edr_fd_table, &fk);
        if (fm && fm->kind == 2) {
            fm->family = ev.fam;
            fm->lport  = ev.lport;
            fm->laddr4 = ev.laddr4;
            __builtin_memcpy(fm->laddr6, ev.laddr6, 16);
        }
        edr_emit(&ev);
        break;
    }
    case __NR_listen: {
        struct edr_event ev = {};
        ev.type = EVT_LISTEN; ev.syscall_id = id; ev.tgid = tgid;
        ev.fd = (__s32)ctx->args[0];
        edr_emit(&ev);
        break;
    }
    case __NR_connect: {
        struct edr_event ev = {};
        ev.type = EVT_CONNECT; ev.syscall_id = id; ev.tgid = tgid;
        ev.fd = (__s32)ctx->args[0];
        const void *uaddr = (const void *)ctx->args[1];
        __s32 addrlen = (__s32)ctx->args[2];
        struct sockaddr sa = {};
        if (addrlen >= (int)sizeof(sa)) {
            bpf_probe_read_user(&sa, sizeof(sa), uaddr);
            ev.fam = sa.sa_family;
            if (sa.sa_family == AF_INET && addrlen >= (int)sizeof(struct sockaddr_in)) {
                struct sockaddr_in sin = {};
                bpf_probe_read_user(&sin, sizeof(sin), uaddr);
                ev.raddr4 = sin.sin_addr.s_addr; ev.rport = sin.sin_port; ev.proto = IPPROTO_TCP;
            } else if (sa.sa_family == AF_INET6 && addrlen >= (int)sizeof(struct sockaddr_in6)) {
                struct sockaddr_in6 sin6 = {};
                bpf_probe_read_user(&sin6, sizeof(sin6), uaddr);
                __builtin_memcpy(ev.raddr6, &sin6.sin6_addr, 16);
                ev.rport = sin6.sin6_port; ev.proto = IPPROTO_TCP;
            }
        }
        /* update fd meta if exists */
        struct fd_key fk = { .tgid = tgid, .fd = ev.fd };
        struct fd_meta *fm = bpf_map_lookup_elem(&edr_fd_table, &fk);
        if (fm && fm->kind == 2) {
            fm->family = ev.fam; fm->proto = ev.proto; fm->rport = ev.rport; fm->raddr4 = ev.raddr4;
            __builtin_memcpy(fm->raddr6, ev.raddr6, 16);
        }
        edr_emit(&ev);
        break;
    }
    case __NR_accept:
#ifdef __NR_accept4
    case __NR_accept4:
#endif
    {
        /* Save peer sock from user pointer at enter (if any) */
        const void *upeer = (const void *)ctx->args[1];
        __s32 addrlen = 0;
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
        struct edr_event ev = {};
        ev.type = EVT_SENDTO; ev.syscall_id = id; ev.tgid = tgid;
        ev.fd = (__s32)ctx->args[0];
        const void *uaddr = (const void *)ctx->args[4];
        __s32 addrlen = (__s32)ctx->args[5];
        if (uaddr && addrlen > 0) {
            struct sockaddr sa = {};
            bpf_probe_read_user(&sa, sizeof(sa), uaddr);
            ev.fam = sa.sa_family;
            if (sa.sa_family == AF_INET) {
                struct sockaddr_in sin = {};
                bpf_probe_read_user(&sin, sizeof(sin), uaddr);
                ev.raddr4 = sin.sin_addr.s_addr; ev.rport = sin.sin_port;
            } else if (sa.sa_family == AF_INET6) {
                struct sockaddr_in6 sin6 = {};
                bpf_probe_read_user(&sin6, sizeof(sin6), uaddr);
                __builtin_memcpy(ev.raddr6, &sin6.sin6_addr, 16); ev.rport = sin6.sin6_port;
            }
        }
        edr_emit(&ev);
        break;
    }
    case __NR_sendmsg: {
        struct edr_event ev = {};
        ev.type = EVT_SENDMSG; ev.syscall_id = id; ev.tgid = tgid;
        ev.fd = (__s32)ctx->args[0];
        struct user_msghdr {
            void *msg_name;
            int   msg_namelen;
        } msgh;
        bpf_probe_read_user(&msgh, sizeof(msgh), (void*)ctx->args[1]);
        if (msgh.msg_name && msgh.msg_namelen > 0) {
            struct sockaddr sa = {};
            bpf_probe_read_user(&sa, sizeof(sa), msgh.msg_name);
            ev.fam = sa.sa_family;
            if (sa.sa_family == AF_INET) {
                struct sockaddr_in sin = {};
                bpf_probe_read_user(&sin, sizeof(sin), msgh.msg_name);
                ev.raddr4 = sin.sin_addr.s_addr; ev.rport = sin.sin_port;
            } else if (sa.sa_family == AF_INET6) {
                struct sockaddr_in6 sin6 = {};
                bpf_probe_read_user(&sin6, sizeof(sin6), msgh.msg_name);
                __builtin_memcpy(ev.raddr6, &sin6.sin6_addr, 16); ev.rport = sin6.sin6_port;
            }
        }
        edr_emit(&ev);
        break;
    }
    case __NR_recvfrom: {
        struct edr_event ev = {};
        ev.type = EVT_RECVFROM; ev.syscall_id = id; ev.tgid = tgid;
        ev.fd = (__s32)ctx->args[0];
        edr_emit(&ev);
        break;
    }
    case __NR_recvmsg: {
        struct edr_event ev = {};
        ev.type = EVT_RECVMSG; ev.syscall_id = id; ev.tgid = tgid;
        ev.fd = (__s32)ctx->args[0];
        edr_emit(&ev);
        break;
    }

    /* ---- NAMESPACE / MOUNT / MODULE / BPF ---- */
#ifdef __NR_setns
    case __NR_setns: {
        struct edr_event ev = {};
        ev.type = EVT_SETNS; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        edr_emit(&ev);
        break;
    }
#endif
#ifdef __NR_unshare
    case __NR_unshare: {
        struct edr_event ev = {};
        ev.type = EVT_UNSHARE; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        ev.flags = (__u32)ctx->args[0];
        edr_emit(&ev);
        break;
    }
#endif
    case __NR_mount: {
        struct edr_event ev = {};
        ev.type = EVT_MOUNT; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        edr_emit(&ev);
        break;
    }
    case __NR_umount2: {
        struct edr_event ev = {};
        ev.type = EVT_UMOUNT2; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        edr_emit(&ev);
        break;
    }
#ifdef __NR_pivot_root
    case __NR_pivot_root: {
        struct edr_event ev = {};
        ev.type = EVT_PIVOT_ROOT; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        edr_emit(&ev);
        break;
    }
#endif
#ifdef __NR_bpf
    case __NR_bpf: {
        struct edr_event ev = {};
        ev.type = EVT_BPF; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        ev.aux_u32 = (__u32)ctx->args[0]; // cmd
        edr_emit(&ev);
        break;
    }
#endif
#ifdef __NR_init_module
    case __NR_init_module:
#endif
#ifdef __NR_finit_module
    case __NR_finit_module:
#endif
    {
        struct edr_event ev = {};
        ev.type = EVT_MOD_LOAD; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        edr_emit(&ev);
        break;
    }
#ifdef __NR_delete_module
    case __NR_delete_module: {
        struct edr_event ev = {};
        ev.type = EVT_MOD_DEL; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        edr_emit(&ev);
        break;
    }
#endif

    /* ---- PRIVILEGE ---- */
#ifdef __NR_setuid
    case __NR_setuid: {
        struct edr_event ev = {};
        ev.type = EVT_SETUID; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        ev.aux_u32 = (__u32)ctx->args[0]; // new uid
        edr_emit(&ev);
        break;
    }
#endif
#ifdef __NR_capset
    case __NR_capset: {
        struct edr_event ev = {};
        ev.type = EVT_CAPSET; ev.syscall_id = id; ev.tgid = tgid; ev.fd = -1;
        edr_emit(&ev);
        break;
    }
#endif

    /* ---- R/W sizes ---- */
    case __NR_read:
    case __NR_write: {
        struct edr_event ev = {};
        ev.type = (id == __NR_read) ? EVT_READ : EVT_WRITE;
        ev.syscall_id = id; ev.tgid = tgid;
        ev.fd = (__s32)ctx->args[0];
        ev.aux_u64 = (__u64)ctx->args[2]; // count
        edr_emit(&ev);
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
#ifdef __NR_openat2
    case __NR_openat2:
#endif
    {
        struct pending_open *po = bpf_map_lookup_elem(&edr_pending_open, &pid_tgid);
        if (!po) break;

        struct edr_event ev = {};
        ev.type = EVT_OPEN; ev.syscall_id = id; ev.tgid = tgid;
        ev.ret = ret; ev.fd = (ret >= 0) ? (__s32)ret : -1;
        ev.flags = po->flags;
        __builtin_memcpy(ev.path, po->path, sizeof(ev.path));
        edr_emit(&ev);

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

        struct edr_event ev = {};
        ev.type = EVT_SOCKET; ev.syscall_id = id; ev.tgid = tgid;
        ev.ret = ret; ev.fd = (ret >= 0) ? (__s32)ret : -1;
        ev.fam = (ps->domain >= 0 && ps->domain <= 255) ? (__u8)ps->domain : 0;
        ev.proto = (ps->proto  >= 0 && ps->proto  <= 255) ? (__u8)ps->proto  : 0;
        edr_emit(&ev);

        if (ret >= 0) {
            struct fd_key fk = { .tgid = tgid, .fd = (__s32)ret };
            struct fd_meta fm = {};
            fm.kind   = 2; // socket
            fm.family = ev.fam;
            fm.proto  = ev.proto;
            bpf_map_update_elem(&edr_fd_table, &fk, &fm, BPF_ANY);
        }
        bpf_map_delete_elem(&edr_pending_socket, &pid_tgid);
        break;
    }

#ifdef __NR_memfd_create
    case __NR_memfd_create: {
        struct pending_open *po = bpf_map_lookup_elem(&edr_pending_open, &pid_tgid);
        if (!po) break;

        struct edr_event ev = {};
        ev.type = EVT_MEMFD_CREATE; ev.syscall_id = id; ev.tgid = tgid;
        ev.ret = ret; ev.fd = (ret >= 0) ? (__s32)ret : -1;
        __builtin_memcpy(ev.path, po->path, sizeof(ev.path));
        ev.flags = po->flags;
        edr_emit(&ev);

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
#ifdef __NR_accept4
    case __NR_accept4:
#endif
    {
        struct pending_accept *pa = bpf_map_lookup_elem(&edr_pending_accept, &pid_tgid);
        struct edr_event ev = {};
        ev.type = EVT_ACCEPT; ev.syscall_id = id; ev.tgid = tgid;
        ev.ret = ret; ev.fd = (ret >= 0) ? (__s32)ret : -1;
        if (pa) {
            ev.fam = pa->fam; ev.proto = IPPROTO_TCP; // heuristic
            ev.rport  = pa->rport; ev.raddr4 = pa->raddr4;
            __builtin_memcpy(ev.raddr6, pa->raddr6, 16);
        }
        edr_emit(&ev);

        if (ret >= 0) {
            struct fd_key fk = { .tgid = tgid, .fd = (__s32)ret };
            struct fd_meta fm = {};
            fm.kind = 2; fm.family = ev.fam; fm.proto = ev.proto;
            fm.rport = ev.rport; fm.raddr4 = ev.raddr4;
            __builtin_memcpy(fm.raddr6, ev.raddr6, 16);
            bpf_map_update_elem(&edr_fd_table, &fk, &fm, BPF_ANY);
        }
        if (pa) bpf_map_delete_elem(&edr_pending_accept, &pid_tgid);
        break;
    }

#ifdef __NR_dup
    case __NR_dup:
#endif
#ifdef __NR_dup2
    case __NR_dup2:
#endif
#ifdef __NR_dup3
    case __NR_dup3:
#endif
    {
        if (ret >= 0) {
            // source fd is arg0 for dup, dup3; arg0 for dup2 as well
            __s32 oldfd = (__s32)ctx->args[0];
            __s32 newfd = (__s32)ret;
            struct fd_key src = { .tgid = tgid, .fd = oldfd };
            struct fd_key dst = { .tgid = tgid, .fd = newfd };
            struct fd_meta *fm = bpf_map_lookup_elem(&edr_fd_table, &src);
            if (fm) {
                bpf_map_update_elem(&edr_fd_table, &dst, fm, BPF_ANY);
            }
            struct edr_event ev = {};
            ev.type = EVT_DUP; ev.syscall_id = id; ev.tgid = tgid;
            ev.fd = newfd; ev.aux_u32 = (unsigned)oldfd;
            edr_emit(&ev);
        }
        break;
    }

    case __NR_clone:
#ifdef __NR_clone3
    case __NR_clone3:
#endif
    {
        struct pending_clone *pc = bpf_map_lookup_elem(&edr_pending_clone, &pid_tgid);
        struct edr_event ev = {};
        ev.type = EVT_CLONE; ev.syscall_id = id; ev.tgid = tgid;
        ev.ret  = ret;  // child pid (>0) in parent
        if (pc) ev.aux_u64 = pc->flags;
        edr_emit(&ev);
        if (pc) bpf_map_delete_elem(&edr_pending_clone, &pid_tgid);
        break;
    }

    default:
        break;
    }

    return 0;
}

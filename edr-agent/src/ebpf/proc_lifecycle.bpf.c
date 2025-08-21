// include/edr_events.h  (BPF-safe, no /usr/include/linux/*)

#ifndef EDR_EVENTS_H
#define EDR_EVENTS_H

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_endian.h>

/* Fallbacks only; vmlinux.h already has the structs */
#ifndef AF_INET
#define AF_INET 2
#endif
#ifndef AF_INET6
#define AF_INET6 10
#endif

/* Event types (keep or override via -D if you already defined them elsewhere) */
#ifndef EVT_PROC_EXEC_TP
#define EVT_PROC_EXEC_TP  1001
#endif
#ifndef EVT_PROC_FORK
#define EVT_PROC_FORK     1002
#endif
#ifndef EVT_PROC_EXIT
#define EVT_PROC_EXIT     1003
#endif

/* Unified event layout used by your probes */
struct edr_event {
    __u64 ts;            // monotonic ns
    __u32 type;          // EVT_*
    __s32 syscall_id;    // if from sys_enter/exit; 0 for TPs
    __u32 tgid;          // process id
    __u32 ppid;          // parent id (optional if known)
    __u32 uid;           // uid at emit time
    __s32 fd;            // file/socket fd if relevant else -1
    __s32 ret;           // syscall/derived return (e.g., child pid for fork)
    __u32 aux_u32;       // misc small field (e.g., exit_code)
    char  comm[16];      // task comm
    char  path[256];     // best-effort path/argv/comm as text
};

/* Per-object ringbuf map for events (ok to define in each .bpf.c) */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 24); // 16MB
} edr_events_rb SEC(".maps");

/* Tiny helpers */
static __always_inline __u32 edr_tgid(void) {
    return (__u32)(bpf_get_current_pid_tgid() >> 32);
}
static __always_inline __u32 edr_uid(void) {
    return (__u32)(bpf_get_current_uid_gid() & 0xffffffff);
}
static __always_inline int edr_emit(const struct edr_event *ev) {
    return bpf_ringbuf_output(&edr_events_rb, ev, sizeof(*ev), 0);
}

#endif /* EDR_EVENTS_H */

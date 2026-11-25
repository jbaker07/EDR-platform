#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_tracing.h>
char LICENSE[] SEC("license") = "Dual BSD/GPL";
struct file_access_event_t { u32 pid; u32 uid; char comm[64]; char filename[256]; };
struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u32));
    __uint(max_entries, 1024);
} events SEC(".maps");
/* The agent's own tgid, written by the agent at startup (raw_config.rs). Every record
 * the agent writes opens its log files, and without this check those opens filled the
 * perf buffer (10 million in a 10 minute burst) and pushed out other processes' events. */
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 64);
    __type(key, u32);
    __type(value, u8);
} fam_self_tgid SEC(".maps");
/* openat is read on raw_syscalls/sys_enter. On kernel 7.0 a program on
 * syscalls/sys_enter_openat ran on CPU 0 only (see net_flow.bpf.c). */
#if defined(__TARGET_ARCH_x86)
#define FAM_NR_OPENAT 257
#else
#define FAM_NR_OPENAT 56
#endif
SEC("tracepoint/raw_syscalls/sys_enter")
int trace_file_access(struct trace_event_raw_sys_enter *ctx) {
    if (ctx->id != FAM_NR_OPENAT)
        return 0;
    u32 tgid = bpf_get_current_pid_tgid() >> 32;
    if (bpf_map_lookup_elem(&fam_self_tgid, &tgid))
        return 0;
    struct file_access_event_t ev = {};
    ev.pid = tgid;
    ev.uid = bpf_get_current_uid_gid() & 0xffffffff;
    bpf_get_current_comm(&ev.comm, sizeof(ev.comm));
    const char *fn = (const char *)ctx->args[1];
    bpf_core_read_user_str(&ev.filename, sizeof(ev.filename), fn);
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &ev, sizeof(ev));
    return 0;
}

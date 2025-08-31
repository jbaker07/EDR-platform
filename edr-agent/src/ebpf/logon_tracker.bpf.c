// logon_tracker.bpf.c  (ringbuf version)

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

struct logon_event {
    __u32 pid;
    __u32 uid;
    __u32 gid;
    __u32 ppid;
    char  comm[64];
};

/* Ring buffer named "events" so your loader can reuse /sys/fs/bpf/edr/edr_events_rb */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20); // 1 MiB if not reusing the shared rb
} events SEC(".maps");

/* Execve as a proxy for login-ish activity (sshd/login/bash, etc.) */
SEC("tracepoint/syscalls/sys_enter_execve")
int trace_execve(struct trace_event_raw_sys_enter *ctx)
{
    struct task_struct *task = (struct task_struct *)bpf_get_current_task();
    __u32 pid = (__u32)(bpf_get_current_pid_tgid() >> 32);
    __u32 uid = (__u32)(bpf_get_current_uid_gid());
    __u32 gid = (__u32)(bpf_get_current_uid_gid() >> 32);
    __u32 ppid = BPF_CORE_READ(task, real_parent, tgid);

    char comm[64] = {};
    bpf_get_current_comm(&comm, sizeof(comm));

    /* only emit when the comm looks like login/sshd/bash */
    bool looks_login  = (comm[0]=='l' && comm[1]=='o' && comm[2]=='g'); /* login/logind */
    bool looks_sshd   = (comm[0]=='s' && comm[1]=='s' && comm[2]=='h'); /* sshd */
    bool looks_shell  = (comm[0]=='b' && comm[1]=='a' && comm[2]=='s'); /* bash */

    if (!(looks_login || looks_sshd || looks_shell))
        return 0;

    struct logon_event *evt = bpf_ringbuf_reserve(&events, sizeof(*evt), 0);
    if (!evt)
        return 0;

    evt->pid  = pid;
    evt->uid  = uid;
    evt->gid  = gid;
    evt->ppid = ppid;
    __builtin_memcpy(evt->comm, comm, sizeof(evt->comm));

    bpf_ringbuf_submit(evt, 0);
    return 0;
}

char LICENSE[] SEC("license") = "Dual BSD/GPL";

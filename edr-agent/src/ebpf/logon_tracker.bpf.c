// logon_tracker.bpf.c

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct logon_event {
    u32 pid;
    u32 uid;
    u32 gid;
    u32 ppid;
    char comm[64];
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} events SEC(".maps");

// This tracepoint fires on every execve syscall, which is a good proxy for user-initiated logins or SSH sessions.
SEC("tracepoint/syscalls/sys_enter_execve")
int trace_execve(struct trace_event_raw_sys_enter *ctx) {
    struct logon_event evt = {};
    struct task_struct *task = (struct task_struct *)bpf_get_current_task();

    evt.pid = bpf_get_current_pid_tgid() >> 32;
    evt.uid = bpf_get_current_uid_gid() & 0xFFFFFFFF;
    evt.gid = bpf_get_current_uid_gid() >> 32;
    evt.ppid = BPF_CORE_READ(task, real_parent, tgid);
    bpf_get_current_comm(&evt.comm, sizeof(evt.comm));

    // Only emit execs involving login/shell-related processes
    if (!(evt.comm[0] == 'l' && evt.comm[1] == 'o' && evt.comm[2] == 'g') &&  // "login", "logind"
        !(evt.comm[0] == 's' && evt.comm[1] == 's' && evt.comm[2] == 'h') &&   // "sshd"
        !(evt.comm[0] == 'b' && evt.comm[1] == 'a' && evt.comm[2] == 's')) {   // "bash"
        return 0;
    }

    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &evt, sizeof(evt));
    return 0;
}

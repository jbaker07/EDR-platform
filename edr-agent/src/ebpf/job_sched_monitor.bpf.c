// edr-agent/src/ebpf/job_sched_monitor.bpf.c

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

#ifndef __user
#define __user
#endif

char LICENSE[] SEC("license") = "Dual MIT/GPL";

#define MAX_CMD_LEN 256

struct cron_event_t {
    __u64 timestamp;
    __u32 pid;
    __u32 ppid;
    char  filename[MAX_CMD_LEN];
};

/* use the shared ringbuf named "events" */
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20);
} events SEC(".maps");

static __always_inline bool startswith_etc_cron(const char *s)
{
    return s[0]=='/'&&s[1]=='e'&&s[2]=='t'&&s[3]=='c'&&s[4]=='/'&&
           s[5]=='c'&&s[6]=='r'&&s[7]=='o'&&s[8]=='n';
}

static __always_inline bool startswith_var_spool_cron(const char *s)
{
    return s[0]=='/'&&s[1]=='v'&&s[2]=='a'&&s[3]=='r'&&s[4]=='/'&&
           s[5]=='s'&&s[6]=='p'&&s[7]=='o'&&s[8]=='o'&&s[9]=='l'&&
           s[10]=='/'&&s[11]=='c'&&s[12]=='r'&&s[13]=='o'&&s[14]=='n';
}

SEC("tracepoint/syscalls/sys_enter_openat")
int trace_openat(struct trace_event_raw_sys_enter *ctx)
{
    struct cron_event_t ev = {};
    ev.timestamp = bpf_ktime_get_ns();
    ev.pid = (__u32)(bpf_get_current_pid_tgid() >> 32);

    struct task_struct *task = (struct task_struct *)bpf_get_current_task();
    struct task_struct *real_parent = BPF_CORE_READ(task, real_parent);
    ev.ppid = BPF_CORE_READ(real_parent, tgid);

    unsigned long filename_ptr = 0;
    BPF_CORE_READ_INTO(&filename_ptr, ctx, args[1]);
    const char __user *filename = (const char __user *)filename_ptr;
    bpf_probe_read_user_str(ev.filename, sizeof(ev.filename), filename);

    if (startswith_etc_cron(ev.filename) || startswith_var_spool_cron(ev.filename)) {
        bpf_ringbuf_output(&events, &ev, sizeof(ev), 0);
    }
    return 0;
}

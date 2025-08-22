// edr-agent/src/ebpf/persistence_watch_ebpf.c

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <linux/limits.h>

char LICENSE[] SEC("license") = "GPL";

#define MAX_PATH_LEN 256

struct event {
    u32 pid;
    char filename[MAX_PATH_LEN];
    u64 ts;
    char action[16];  // "write", "chmod", etc.
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} events SEC(".maps");

// Monitor file writes (write syscall)
SEC("tracepoint/syscalls/sys_enter_write")
int trace_write(struct trace_event_raw_sys_enter *ctx) {
    u64 pid_tgid = bpf_get_current_pid_tgid();
    u32 pid = pid_tgid >> 32;

    struct event e = {};
    e.pid = pid;
    e.ts = bpf_ktime_get_ns();
    __builtin_memcpy(e.action, "write", 5);

    struct task_struct *task = (struct task_struct *)bpf_get_current_task();
    struct file *file = NULL;
    struct dentry *dentry;
    struct qstr d_name;
    struct path file_path;

    bpf_probe_read_kernel(&file_path, sizeof(file_path), &task->mm->exe_file->f_path);
    bpf_probe_read_kernel(&dentry, sizeof(dentry), &file_path.dentry);
    bpf_probe_read_kernel(&d_name, sizeof(d_name), &dentry->d_name);
    bpf_probe_read_kernel_str(&e.filename, sizeof(e.filename), d_name.name);

    // Basic filter: skip non-persistence paths
    if (!(e.filename[0] == '/' && (
        __builtin_memcmp(e.filename, "/etc/", 5) == 0 ||
        __builtin_memcmp(e.filename, "/var/", 5) == 0 ||
        __builtin_memcmp(e.filename, "/home/", 6) == 0 ||
        __builtin_memcmp(e.filename, "/lib/", 5) == 0 ||
        __builtin_memcmp(e.filename, "/tmp/", 5) == 0))) {
        return 0;
    }

    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &e, sizeof(e));
    return 0;
}

// Monitor chmod or file mode changes
SEC("tracepoint/syscalls/sys_enter_fchmodat")
int trace_chmod(struct trace_event_raw_sys_enter *ctx) {
    u64 pid_tgid = bpf_get_current_pid_tgid();
    u32 pid = pid_tgid >> 32;

    struct event e = {};
    e.pid = pid;
    e.ts = bpf_ktime_get_ns();
    __builtin_memcpy(e.action, "chmod", 6);

    const char *filename = (const char *)ctx->args[1];
    bpf_probe_read_user_str(e.filename, sizeof(e.filename), filename);

    // Filter on common persistence paths
    if (!(e.filename[0] == '/' && (
        __builtin_memcmp(e.filename, "/etc/", 5) == 0 ||
        __builtin_memcmp(e.filename, "/var/", 5) == 0 ||
        __builtin_memcmp(e.filename, "/home/", 6) == 0 ||
        __builtin_memcmp(e.filename, "/tmp/", 5) == 0))) {
        return 0;
    }

    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &e, sizeof(e));
    return 0;
}

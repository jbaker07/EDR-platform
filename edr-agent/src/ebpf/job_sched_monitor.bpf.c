#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "Dual MIT/GPL";

// Maximum length for cron command string
#define MAX_CMD_LEN 256

// Event structure to send to userspace
struct cron_event_t {
    u64 timestamp;
    u32 pid;
    u32 ppid;
    char filename[MAX_CMD_LEN];
};

struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
} events SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_openat")
int trace_openat(struct trace_event_raw_sys_enter *ctx) {
    struct cron_event_t event = {};
    u64 ts = bpf_ktime_get_ns();
    event.timestamp = ts;

    // Extract PID and PPID
    event.pid = bpf_get_current_pid_tgid() >> 32;

    struct task_struct *task = (struct task_struct *)bpf_get_current_task();
    struct task_struct *real_parent = BPF_CORE_READ(task, real_parent);
    event.ppid = BPF_CORE_READ(real_parent, tgid);

    // Extract filename
    const char *filename = (const char *)ctx->args[1];
    bpf_probe_read_user_str(event.filename, sizeof(event.filename), filename);

    // Check if it's likely cron or suspicious
    if (__builtin_memcmp(event.filename, "/etc/cron", 10) == 0 ||
        __builtin_memcmp(event.filename, "/var/spool/cron", 16) == 0) {
        bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event, sizeof(event));
    }

    return 0;
}

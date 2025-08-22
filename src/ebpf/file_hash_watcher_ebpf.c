// src/ebpf/file_hash_watcher_ebpf.c

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";
#define MAX_FILENAME_LEN 256

struct file_event_t {
    u64 timestamp;
    u32 pid;
    char filename[MAX_FILENAME_LEN];
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 20);
} file_events SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_openat")
int trace_openat(struct trace_event_raw_sys_enter *ctx) {
    struct file_event_t *event;

    // Allocate event buffer
    event = bpf_ringbuf_reserve(&file_events, sizeof(struct file_event_t), 0);
    if (!event)
        return 0;

    // Fill in PID and timestamp
    event->pid = bpf_get_current_pid_tgid() >> 32;
    event->timestamp = bpf_ktime_get_ns();

    // Read the filename argument safely
    void *filename_ptr = (void *)ctx->args[1];
    bpf_probe_read_user_str(&event->filename, sizeof(event->filename), filename_ptr);

    bpf_ringbuf_submit(event, 0);
    return 0;
}

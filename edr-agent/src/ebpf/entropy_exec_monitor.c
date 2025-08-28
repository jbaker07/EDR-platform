// src/ebpf/entropy_exec_monitor.c
#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>

#define MAX_FILENAME_LEN 256
#define MAX_SYSCALL_LEN  16

struct EntropyEvent {
    __u32 pid;
    __u32 ppid;
    char  filename[MAX_FILENAME_LEN];
    char  syscall[MAX_SYSCALL_LEN];
};

/* PERF buffer (legacy map def to dodge BTF quirks on older toolchains) */
struct bpf_map_def SEC("maps") events = {
    .type        = BPF_MAP_TYPE_PERF_EVENT_ARRAY,
    .key_size    = sizeof(__u32),
    .value_size  = sizeof(__u32),
    .max_entries = 256,
};

/* Syscall-nr map (index 0=execve, 1=openat, 2=mmap); filled by userspace at startup */
struct bpf_map_def SEC("maps") sysnrs = {
    .type        = BPF_MAP_TYPE_ARRAY,
    .key_size    = sizeof(__u32),
    .value_size  = sizeof(__u64),
    .max_entries = 3,
};

static __always_inline void fill_syscall_name(char dst[MAX_SYSCALL_LEN], int scid)
{
    /* constant-size clear so the verifier can see exact bounds */
    __builtin_memset(dst, 0, MAX_SYSCALL_LEN);

    /* write fixed strings with constant lengths (no pointer bitwise ops) */
    if (scid == 0) {              /* execve */
        __builtin_memcpy(dst, "execve", 6);
    } else if (scid == 1) {       /* openat */
        __builtin_memcpy(dst, "openat", 6);
    } else if (scid == 2) {       /* mmap   */
        __builtin_memcpy(dst, "mmap", 4);
    }
}

/* ctx is the raw_syscalls:sys_enter tracepoint context */
SEC("tracepoint/raw_syscalls/sys_enter")
int on_sys_enter(struct trace_event_raw_sys_enter *ctx)
{
    __u64 id = ctx->id;
    __u32 k;
    __u64 *nr;

    int scid = -1;
    const char *filename = NULL;

    /* execve */
    k = 0; nr = bpf_map_lookup_elem(&sysnrs, &k);
    if (nr && id == *nr) {
        scid = 0;
        filename = (const char *)ctx->args[0];
    }

    /* openat */
    k = 1; nr = bpf_map_lookup_elem(&sysnrs, &k);
    if (nr && id == *nr) {
        scid = 1;
        filename = (const char *)ctx->args[1];
    }

    /* mmap (no filename) */
    k = 2; nr = bpf_map_lookup_elem(&sysnrs, &k);
    if (nr && id == *nr) {
        scid = 2;
        filename = NULL;
    }

    if (scid < 0)
        return 0;

    struct EntropyEvent e = {};
    __u64 pid_tgid = bpf_get_current_pid_tgid();
    e.pid  = pid_tgid >> 32;

    /* task->real_parent->tgid via CO-RE */
    struct task_struct *task = (void *)bpf_get_current_task();
    e.ppid = BPF_CORE_READ(task, real_parent, tgid);

    if (filename) {
        /* safe user copy; returns length or -EFAULT, both fine to ignore */
        bpf_probe_read_user_str(e.filename, sizeof(e.filename), filename);
    }

    fill_syscall_name(e.syscall, scid);

    /* emit */
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &e, sizeof(e));
    return 0;
}

char _license[] SEC("license") = "GPL";

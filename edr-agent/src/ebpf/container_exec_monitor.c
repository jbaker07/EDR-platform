// SPDX-License-Identifier: Dual BSD/GPL
// ebpf/container_exec_monitor.c (CO-RE)
// Emits exec events from containers via ringbuf.
// Hooks: tracepoint/syscalls/sys_enter_execve(+execveat)

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

#ifndef TASK_COMM_LEN
#define TASK_COMM_LEN 16
#endif

#define FNAME_MAX 256

struct container_exec_event {
    __u64 ts_ns;       // monotonic nanoseconds
    __u64 cgroup_id;   // for container resolution in userspace
    __u32 pid;         // tgid
    __u32 ppid;        // parent tgid
    __u32 uid;         // euid
    __u32 mnt_ns;      // mount namespace inode
    __u32 pid_ns;      // pid namespace inode
    __u32 flags;       // bit0: execveat
    char  comm[TASK_COMM_LEN];
    char  filename[FNAME_MAX];
};

// 16MB ring buffer; align with user space map name
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 24);
} container_exec_events SEC(".maps");

static __always_inline __u32 get_ppid_tgid(void)
{
    struct task_struct *task = (struct task_struct *)bpf_get_current_task_btf();
    __u32 ppid = 0;
    if (task) {
        ppid = BPF_CORE_READ(task, real_parent, tgid);
    }
    return ppid;
}

static __always_inline __u32 ns_inum_mount(void)
{
    struct task_struct *task = (struct task_struct *)bpf_get_current_task_btf();
    struct nsproxy *nsp = NULL;
    struct mnt_namespace *mntns = NULL;
    struct ns_common nsc = {};
    BPF_CORE_READ_INTO(&nsp, task, nsproxy);
    if (!nsp) return 0;
    BPF_CORE_READ_INTO(&mntns, nsp, mnt_ns);
    if (!mntns) return 0;
    BPF_CORE_READ_INTO(&nsc, mntns, ns);
    return nsc.inum;
}

static __always_inline __u32 ns_inum_pid(void)
{
    struct task_struct *task = (struct task_struct *)bpf_get_current_task_btf();
    struct nsproxy *nsp = NULL;
    struct pid_namespace *pidns = NULL;
    struct ns_common nsc = {};
    BPF_CORE_READ_INTO(&nsp, task, nsproxy);
    if (!nsp) return 0;
    BPF_CORE_READ_INTO(&pidns, nsp, pid_ns_for_children);
    if (!pidns) return 0;
    BPF_CORE_READ_INTO(&nsc, pidns, ns);
    return nsc.inum;
}

static __always_inline int submit_exec(struct trace_event_raw_sys_enter *ctx, __u32 is_execveat)
{
    struct container_exec_event *e = bpf_ringbuf_reserve(&container_exec_events, sizeof(*e), 0);
    if (!e) return 0;

    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u64 uid_gid  = bpf_get_current_uid_gid();

    e->ts_ns     = bpf_ktime_get_ns();
    e->cgroup_id = bpf_get_current_cgroup_id();
    e->pid       = (__u32)(pid_tgid >> 32);   // tgid
    e->ppid      = get_ppid_tgid();
    e->uid       = (__u32)uid_gid;            // euid
    e->mnt_ns    = ns_inum_mount();
    e->pid_ns    = ns_inum_pid();
    e->flags     = is_execveat ? 1u : 0u;

    __builtin_memset(e->comm, 0, sizeof(e->comm));
    bpf_get_current_comm(&e->comm, sizeof(e->comm));

    // ctx->args[0] => const char *filename
    const char *filename = (const char *)(ctx->args[0]);
    __builtin_memset(e->filename, 0, sizeof(e->filename));
    if (filename) {
        bpf_probe_read_user_str(e->filename, sizeof(e->filename), filename);
    }

    bpf_ringbuf_submit(e, 0);
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_execve")
int handle_sys_enter_execve(struct trace_event_raw_sys_enter *ctx)
{
    return submit_exec(ctx, 0);
}

SEC("tracepoint/syscalls/sys_enter_execveat")
int handle_sys_enter_execveat(struct trace_event_raw_sys_enter *ctx)
{
    return submit_exec(ctx, 1);
}

char LICENSE[] SEC("license") = "Dual BSD/GPL";

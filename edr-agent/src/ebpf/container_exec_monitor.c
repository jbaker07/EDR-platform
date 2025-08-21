// container_exec_monitor.bpf.c (CO-RE)
// Detect exec in container-like namespaces; publish minimal facts to ringbuf.
// User space resolves cgroup/ns → container runtime metadata.

#include "vmlinux.h"
#include "bpf_helpers.h"
#include "bpf_tracing.h"
#include "bpf_core_read.h"

char LICENSE[] SEC("license") = "Dual BSD/GPL";

// conservative caps
#define FNAME_MAX 256

struct container_exec_event {
    __u64 ts;
    __u32 pid;          // tgid (userspace "pid")
    __u32 uid;
    __u64 cgroup_id;    // for user-space container resolution
    __u32 mnt_ns;       // mount ns inode
    __u32 pid_ns;       // pid ns inode
    char  comm[TASK_COMM_LEN];
    char  filename[FNAME_MAX];
    __u32 flags;        // bit0: execveat
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 24); // 16MB
} events SEC(".maps");

static __always_inline __u32 ns_inum_mount(void) {
    struct task_struct *task = (struct task_struct *)bpf_get_current_task();
    struct nsproxy *nsp = NULL;
    struct mnt_namespace *mntns = NULL;
    struct ns_common nsc = {};
    bpf_core_read(&nsp, sizeof(nsp), &task->nsproxy);
    if (!nsp) return 0;
    bpf_core_read(&mntns, sizeof(mntns), &nsp->mnt_ns);
    if (!mntns) return 0;
    bpf_core_read(&nsc, sizeof(nsc), &mntns->ns);
    return nsc.inum;
}

static __always_inline __u32 ns_inum_pid(void) {
    struct task_struct *task = (struct task_struct *)bpf_get_current_task();
    struct nsproxy *nsp = NULL;
    struct pid_namespace *pidns = NULL;
    struct ns_common nsc = {};
    bpf_core_read(&nsp, sizeof(nsp), &task->nsproxy);
    if (!nsp) return 0;
    bpf_core_read(&pidns, sizeof(pidns), &nsp->pid_ns_for_children);
    if (!pidns) return 0;
    bpf_core_read(&nsc, sizeof(nsc), &pidns->ns);
    return nsc.inum;
}

static __always_inline int submit_exec(struct trace_event_raw_sys_enter *ctx, __u32 is_execveat) {
    struct container_exec_event *e = bpf_ringbuf_reserve(&events, sizeof(*e), 0);
    if (!e) return 0;

    __u64 pid_tgid = bpf_get_current_pid_tgid();
    __u64 uid_gid = bpf_get_current_uid_gid();

    e->ts        = bpf_ktime_get_ns();
    e->pid       = pid_tgid >> 32;       // tgid
    e->uid       = uid_gid & 0xffffffff;
    e->cgroup_id = bpf_get_current_cgroup_id();
    e->mnt_ns    = ns_inum_mount();
    e->pid_ns    = ns_inum_pid();
    e->flags     = is_execveat ? 1u : 0u;

    __builtin_memset(e->comm, 0, sizeof(e->comm));
    bpf_get_current_comm(&e->comm, sizeof(e->comm));

    // ctx->args[0] = const char *filename
    const char *filename = (const char *)(ctx->args[0]);
    __builtin_memset(e->filename, 0, sizeof(e->filename));
    if (filename) {
        // Best-effort; ignore failure
        bpf_probe_read_user_str(e->filename, sizeof(e->filename), filename);
    }

    bpf_ringbuf_submit(e, 0);
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_execve")
int handle_sys_enter_execve(struct trace_event_raw_sys_enter *ctx) {
    return submit_exec(ctx, 0);
}

SEC("tracepoint/syscalls/sys_enter_execveat")
int handle_sys_enter_execveat(struct trace_event_raw_sys_enter *ctx) {
    return submit_exec(ctx, 1);
}

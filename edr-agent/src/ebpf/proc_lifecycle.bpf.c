// ebpf/proc_lifecycle.bpf.c
#include "include/edr_events.h"

static __always_inline int read_ctx_loc_str(void *ctx, __u32 loc, char *dst, __u32 len)
{
    if (!loc) return 0;
    // tracepoint __data_loc: lower 16 bits = offset from ctx, upper = len; but len may be 0 here.
    __u32 off = loc & 0xFFFF;
    const char *ptr = (const char *)ctx + off;
    return bpf_probe_read_kernel_str(dst, len, ptr);
}

/* sched/sched_process_exec
 * TP fields (CO-RE): __data_loc char filename[]; pid; ...
 * We prefer this over sys_enter for bullet-proof exec attribution.
 */
SEC("tracepoint/sched/sched_process_exec")
int tp_sched_process_exec(struct trace_event_raw_sched_process_exec *ctx)
{
    struct edr_event ev = {};
    ev.type       = EVT_PROC_EXEC_TP;
    ev.syscall_id = 0;       // not a syscall; keep 0
    ev.tgid       = edr_tgid();
    ev.fd         = -1;

    // Best-effort filename from __data_loc
    read_ctx_loc_str((void *)ctx, ctx->__data_loc_filename, ev.path, sizeof(ev.path));

    edr_emit(&ev);
    return 0;
}

/* sched/sched_process_fork
 * TP fields: parent=task_struct*, child=task_struct*
 * We emit parent→child linkage reliably even without clone().
 */
SEC("tracepoint/sched/sched_process_fork")
int tp_sched_process_fork(struct trace_event_raw_sched_process_fork *ctx)
{
    struct task_struct *parent = (struct task_struct *)BPF_CORE_READ(ctx, parent);
    struct task_struct *child  = (struct task_struct *)BPF_CORE_READ(ctx, child);

    __u32 p_tgid = parent ? BPF_CORE_READ(parent, tgid) : 0;
    __u32 c_tgid = child  ? BPF_CORE_READ(child, tgid)  : 0;

    struct edr_event ev = {};
    ev.type       = EVT_PROC_FORK;
    ev.syscall_id = 0;
    ev.tgid       = p_tgid;     // emitter = parent
    ev.ret        = (__s32)c_tgid; // child pid in 'ret' for uniformity
    // ppid field will be filled by edr_emit() using current task (parent), which matches
    // but we overwrite to ensure accuracy:
    ev.ppid       = p_tgid;

    // Fill child comm into path for convenience (optional)
    if (child) {
        char child_comm[16] = {};
        bpf_core_read_str(&child_comm, sizeof(child_comm), &child->comm);
        __builtin_memcpy(ev.path, child_comm, sizeof(ev.path));
    }

    // Emit without relying on current task (we already set fields)
    ev.ts   = bpf_ktime_get_ns();
    ev.uid  = edr_uid();
    // Keep comm of parent for context
    bpf_get_current_comm(ev.comm, sizeof(ev.comm));
    bpf_ringbuf_output(&edr_events_rb, &ev, sizeof(ev), 0);
    return 0;
}

/* sched/sched_process_exit
 * We provide a final lifecycle event; exit_code via task->exit_code if available.
 */
SEC("tracepoint/sched/sched_process_exit")
int tp_sched_process_exit(struct trace_event_raw_sched_process_template *ctx)
{
    struct task_struct *task = (struct task_struct *)bpf_get_current_task_btf();

    struct edr_event ev = {};
    ev.type       = EVT_PROC_EXIT;
    ev.syscall_id = 0;
    ev.tgid       = edr_tgid();
    ev.fd         = -1;

    // best-effort: exit_code from task_struct
    int code = 0;
    if (task) {
        code = BPF_CORE_READ(task, exit_code);
    }
    ev.aux_u32 = (unsigned)code;

    edr_emit(&ev);
    return 0;
}

char LICENSE[] SEC("license") = "Dual BSD/GPL";

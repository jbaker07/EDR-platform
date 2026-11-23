// edr_guard.bpf.c
// Prevention mode. The other sensors in this directory report what already
// happened. These programs run on Linux Security Module (LSM) hooks, the checks
// the kernel makes right before it allows an action, so they can refuse the
// action itself by returning -EPERM.
//
//   1. Runtime socket guard (lsm/unix_stream_connect). connect() to a container
//      runtime control socket such as /var/run/docker.sock from an executable
//      that is not an allowlisted runtime client. MITRE T1611, T1610.
//   2. Memfd exec guard (lsm/bprm_check_security). execve of a file that lives
//      only in memory, made with memfd_create(). MITRE T1620.
//   3. Self protection (lsm/task_kill, lsm/ptrace_access_check, lsm/bpf).
//      Fatal signals, ptrace attach and BPF object lookups aimed at the agent
//      by any other process. MITRE T1562.001.
//
// Each guard has a mode in guard_config: off, monitor (allow, report what would
// have been blocked) or enforce (refuse and report). Every report is one
// guard_event in the guard_events ring buffer. User space: src/guard/.
//
// The agent attaches these through links it owns and never pins them, so the
// kernel detaches all of them when the agent exits for any reason. A crash can
// not leave the host locked down.

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

// vmlinux.h carries kernel types but not #defines, so these are spelled out.
#define EPERM 1
#define TASK_COMM_LEN 16
#define UNIX_PATH_MAX 108
#define EXE_NAME_LEN 32
#define PTRACE_MODE_ATTACH 0x02
#define SIGCHLD 17
#define SIGCONT 18
#define SIGURG 23
#define SIGWINCH 28

// guard_config slots, written by user space (src/guard/abi.rs).
#define CFG_SOCKET_MODE 0
#define CFG_MEMFD_MODE 1
#define CFG_SELF_MODE 2
#define CFG_AGENT_TGID 3   // host tgid of the agent, 0 until user space sets it
#define CFG_SCOPE_CGROUP 4 // cgroup v2 id the guards act on, 0 means every task
#define CFG_SLOTS 5

#define MODE_OFF 0
#define MODE_MONITOR 1
#define MODE_ENFORCE 2

// guard_event.kind
#define KIND_SOCKET 1
#define KIND_MEMFD_EXEC 2
#define KIND_KILL 3
#define KIND_PTRACE 4
#define KIND_BPF 5

// guard_event.action
#define ACTION_WOULD_BLOCK 1 // monitor: allowed and reported
#define ACTION_BLOCKED 2     // enforce: refused with -EPERM and reported

// An executable, named by inode number and device the way the kernel sees it.
// That is the file the process really runs. comm, the 16 byte task name, is a
// label any process can rewrite with prctl(PR_SET_NAME), so no decision here
// ever uses it.
struct exe_id {
    __u64 ino;
    __u32 dev; // kernel dev_t, major << 20 | minor
    __u32 pad; // always 0, so the 16 byte key compares cleanly
};

// Socket name, zero padded to the full sun_path size, as in suspicious_ipc.bpf.c.
struct sock_path_key {
    char path[UNIX_PATH_MAX];
};

// One of the agent's BPF objects: the bpf() command that opens it by id, and the id.
struct obj_key {
    __u32 cmd;
    __u32 id;
};

// One decision. Mirrored by GuardEvent in src/guard/abi.rs. No implicit padding.
struct guard_event {
    __u64 timestamp_ns; // bpf_ktime_get_ns, nanoseconds since boot
    __u64 exe_ino;      // caller's executable
    __u64 target_ino;   // socket inode, exec file inode or BPF object id, else 0
    __u32 exe_dev;
    __u32 tgid;         // caller
    __u32 tid;
    __u32 uid;
    __u32 target_pid;   // agent tgid for self protection, else 0
    __u32 detail;       // signal number, ptrace mode or bpf command, else 0
    __u8  kind;         // KIND_*
    __u8  action;       // ACTION_*
    __u16 pad;
    char  comm[TASK_COMM_LEN];
    char  exe_name[EXE_NAME_LEN];  // file name of the caller's executable
    char  target[UNIX_PATH_MAX];   // socket name or exec file name, else empty
};

_Static_assert(sizeof(struct guard_event) == 208, "update GuardEvent in src/guard/abi.rs");

struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, CFG_SLOTS);
    __type(key, __u32);
    __type(value, __u64);
} guard_config SEC(".maps");

// Runtime socket names, the same list suspicious_ipc watches.
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 64);
    __type(key, struct sock_path_key);
    __type(value, __u8);
} watched_sockets SEC(".maps");

// Executables allowed to use those sockets: docker, containerd, kubelet and the like.
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 256);
    __type(key, struct exe_id);
    __type(value, __u8);
} socket_clients SEC(".maps");

// Executables allowed to run a memfd. runc can re-execute itself from one.
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 64);
    __type(key, struct exe_id);
    __type(value, __u8);
} memfd_allow SEC(".maps");

// The agent's own programs, maps and links.
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 64);
    __type(key, struct obj_key);
    __type(value, __u8);
} protected_ids SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024);
} guard_events SEC(".maps");

static __always_inline __u64 cfg(__u32 slot)
{
    __u64 *v = bpf_map_lookup_elem(&guard_config, &slot);

    return v ? *v : 0;
}

static __always_inline __u32 current_tgid(void)
{
    return bpf_get_current_pid_tgid() >> 32;
}

// With no scope set the guards judge every task on the host. A scope keeps
// them to one cgroup, which is how the live test keeps the other containers on
// a shared kernel out of reach.
static __always_inline bool in_scope(void)
{
    __u64 scope = cfg(CFG_SCOPE_CGROUP);

    return !scope || bpf_get_current_cgroup_id() == scope;
}

// mm->exe_file is the file the kernel mapped at exec time, the one
// /proc/<pid>/exe points to. A kernel thread has no mm. Every read through
// NULL below then gives 0, and an id of 0 is never on an allowlist.
static __always_inline struct file *current_exe_file(void)
{
    struct task_struct *task = bpf_get_current_task_btf();

    return BPF_CORE_READ(task, mm, exe_file);
}

static __always_inline void file_id(struct file *f, struct exe_id *id)
{
    id->ino = BPF_CORE_READ(f, f_inode, i_ino);
    id->dev = BPF_CORE_READ(f, f_inode, i_sb, s_dev);
}

// Reserves a record and fills the fields every report shares. NULL when the
// ring is full. The hook still returns its decision then, only the report is lost.
static __always_inline struct guard_event *new_event(__u8 kind, __u64 mode)
{
    struct guard_event *e = bpf_ringbuf_reserve(&guard_events, sizeof(*e), 0);
    struct exe_id id = {};
    struct file *exe;
    __u64 pid_tgid;

    if (!e)
        return NULL;
    // Ring memory is not zeroed, and the string copies stop at the NUL.
    __builtin_memset(e, 0, sizeof(*e));
    pid_tgid = bpf_get_current_pid_tgid();
    e->timestamp_ns = bpf_ktime_get_ns();
    e->tgid = pid_tgid >> 32;
    e->tid = (__u32)pid_tgid;
    e->uid = (__u32)bpf_get_current_uid_gid();
    e->kind = kind;
    e->action = mode == MODE_ENFORCE ? ACTION_BLOCKED : ACTION_WOULD_BLOCK;
    bpf_get_current_comm(e->comm, sizeof(e->comm));
    exe = current_exe_file();
    file_id(exe, &id);
    e->exe_ino = id.ino;
    e->exe_dev = id.dev;
    bpf_probe_read_kernel_str(e->exe_name, sizeof(e->exe_name),
                              BPF_CORE_READ(exe, f_path.dentry, d_name.name));
    return e;
}

// Only enforce refuses. Monitor, and any value user space did not define,
// allows.
static __always_inline int verdict(__u64 mode)
{
    return mode == MODE_ENFORCE ? -EPERM : 0;
}

// ---- 1. Runtime socket guard ----

// Runs inside connect() on a unix stream or seqpacket socket, after the path
// was resolved to a listening socket and before the connection is made.
SEC("lsm/unix_stream_connect")
int BPF_PROG(guard_unix_connect, struct sock *sock, struct sock *other, struct sock *newsk, int ret)
{
    struct sock_path_key key = {};
    struct exe_id exe = {};
    struct unix_address *addr;
    struct guard_event *e;
    __u64 mode;

    // A nonzero ret is an earlier BPF program's refusal. Keep it.
    if (ret)
        return ret;
    mode = cfg(CFG_SOCKET_MODE);
    if (mode == MODE_OFF || !in_scope())
        return 0;

    // other is the listening socket, and addr holds the name its server passed
    // to bind(). Matching that name instead of the path the client passed to
    // connect() means a symlink, a relative path or a bind mount of the socket
    // into a container all reach the same decision. An abstract socket name
    // starts with a NUL, reads as empty and matches nothing.
    addr = BPF_CORE_READ((struct unix_sock *)other, addr);
    if (!addr)
        return 0;
    if (bpf_probe_read_kernel_str(key.path, sizeof(key.path), &addr->name[0].sun_path) <= 0)
        return 0;
    if (!bpf_map_lookup_elem(&watched_sockets, &key))
        return 0;

    file_id(current_exe_file(), &exe);
    if (bpf_map_lookup_elem(&socket_clients, &exe))
        return 0;

    e = new_event(KIND_SOCKET, mode);
    if (e) {
        e->target_ino = BPF_CORE_READ((struct unix_sock *)other, path.dentry, d_inode, i_ino);
        __builtin_memcpy(e->target, key.path, sizeof(e->target));
        bpf_ringbuf_submit(e, 0);
    }
    return verdict(mode);
}

// ---- 2. Memfd exec guard ----

// "memfd:" is the prefix memfd_create() puts in front of the name it is given
// (MFD_NAME_PREFIX in mm/memfd.c, unchanged from 3.17 to 7.0).
static __always_inline bool is_memfd_name(const char *n)
{
    return n[0] == 'm' && n[1] == 'e' && n[2] == 'm' && n[3] == 'f' && n[4] == 'd' && n[5] == ':';
}

// Runs once per binary handler round of execve and execveat, with bprm->file
// the file about to run. A memfd script ("#!") is caught in its first round.
SEC("lsm/bprm_check_security")
int BPF_PROG(guard_memfd_exec, struct linux_binprm *bprm, int ret)
{
    char name[8] = {};
    struct exe_id exe = {};
    const unsigned char *dname;
    unsigned int nlink = 1;
    struct guard_event *e;
    struct file *file;
    __u64 mode;

    if (ret)
        return ret;
    mode = cfg(CFG_MEMFD_MODE);
    if (mode == MODE_OFF || !in_scope())
        return 0;

    // A memfd is known by two facts from the kernel's own structures. Its
    // dentry name starts with "memfd:", text the kernel adds and the caller
    // cannot change, and its inode has no links, because a memfd is never in
    // a directory. The name alone would also match a real file called
    // "memfd:x". No links alone would also match any deleted file, which is
    // normal for a daemon that re-executes itself after a package upgrade.
    file = BPF_CORE_READ(bprm, file);
    BPF_CORE_READ_INTO(&nlink, file, f_inode, i_nlink);
    if (nlink != 0)
        return 0;
    dname = BPF_CORE_READ(file, f_path.dentry, d_name.name);
    if (bpf_probe_read_kernel_str(name, sizeof(name), dname) < 7 || !is_memfd_name(name))
        return 0;

    file_id(current_exe_file(), &exe);
    if (bpf_map_lookup_elem(&memfd_allow, &exe))
        return 0;

    e = new_event(KIND_MEMFD_EXEC, mode);
    if (e) {
        e->target_ino = BPF_CORE_READ(file, f_inode, i_ino);
        bpf_probe_read_kernel_str(e->target, sizeof(e->target), dname);
        bpf_ringbuf_submit(e, 0);
    }
    return verdict(mode);
}

// ---- 3. Self protection ----

// The self protection mode when target_tgid is the agent and the caller is
// some other process in scope. MODE_OFF otherwise.
static __always_inline __u64 self_mode(__u32 target_tgid)
{
    __u64 mode = cfg(CFG_SELF_MODE);
    __u32 agent = cfg(CFG_AGENT_TGID);

    if (mode == MODE_OFF || !agent || target_tgid != agent || current_tgid() == agent)
        return MODE_OFF;
    return in_scope() ? mode : MODE_OFF;
}

static __always_inline int report_self(__u8 kind, __u64 mode, __u32 agent, __u32 detail, __u64 obj_id)
{
    struct guard_event *e = new_event(kind, mode);

    if (e) {
        e->target_pid = agent;
        e->detail = detail;
        e->target_ino = obj_id;
        bpf_ringbuf_submit(e, 0);
    }
    return verdict(mode);
}

// A signal is refused when its default action would stop or end the agent.
// 0 (the "is it alive" probe) and the signals whose default is to ignore or to
// continue stay allowed.
static __always_inline bool harmful_signal(int sig)
{
    return sig != 0 && sig != SIGCHLD && sig != SIGCONT && sig != SIGURG && sig != SIGWINCH;
}

// Signals sent from user space: kill, tkill, tgkill and pidfd_send_signal.
// Signals the kernel raises itself, such as Ctrl-C typed on the agent's own
// terminal or a write to cgroup.kill, never reach this hook.
SEC("lsm/task_kill")
int BPF_PROG(guard_task_kill, struct task_struct *p, struct kernel_siginfo *info, int sig,
             const struct cred *cred, int ret)
{
    __u32 target = BPF_CORE_READ(p, tgid);
    __u64 mode;

    if (ret)
        return ret;
    if (!harmful_signal(sig))
        return 0;
    mode = self_mode(target);
    if (mode == MODE_OFF)
        return 0;
    return report_self(KIND_KILL, mode, target, sig, 0);
}

// ptrace attach, and everything the kernel checks with the same attach mode:
// opening /proc/<pid>/mem, process_vm_readv and process_vm_writev, and
// pidfd_getfd. Read mode checks, such as ps reading /proc/<pid>/stat or a
// readlink of /proc/<pid>/exe, stay allowed.
SEC("lsm/ptrace_access_check")
int BPF_PROG(guard_ptrace, struct task_struct *child, unsigned int ptrace_mode, int ret)
{
    __u32 target = BPF_CORE_READ(child, tgid);
    __u64 mode;

    if (ret)
        return ret;
    if (!(ptrace_mode & PTRACE_MODE_ATTACH))
        return 0;
    mode = self_mode(target);
    if (mode == MODE_OFF)
        return 0;
    return report_self(KIND_PTRACE, mode, target, ptrace_mode, 0);
}

// Opening one of the agent's programs, maps or links by id. That is the first
// step of every tool that would detach or rewrite them, such as bpftool map
// update or bpftool link detach. The agent's own fds can not be taken either,
// because pidfd_getfd needs the attach check above.
// No ret argument here on purpose. Newer kernels (7.0 included) pass a fourth
// argument, bool kernel, so the slot ret would read is not the same on every
// kernel. Only cmd and attr are read, and they are the same on 5.15 and 7.0.
SEC("lsm/bpf")
int BPF_PROG(guard_bpf, int cmd, union bpf_attr *attr, unsigned int size)
{
    struct obj_key key = {};
    __u32 agent;
    __u64 mode;

    if (cmd != BPF_PROG_GET_FD_BY_ID && cmd != BPF_MAP_GET_FD_BY_ID && cmd != BPF_LINK_GET_FD_BY_ID)
        return 0;
    agent = cfg(CFG_AGENT_TGID);
    mode = self_mode(agent);
    if (mode == MODE_OFF)
        return 0;
    // prog_id, map_id and link_id share one slot of bpf_attr.
    key.cmd = cmd;
    key.id = BPF_CORE_READ(attr, prog_id);
    if (!bpf_map_lookup_elem(&protected_ids, &key))
        return 0;
    return report_self(KIND_BPF, mode, agent, cmd, key.id);
}

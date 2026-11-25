// edr_shield.bpf.c
// Runtime shields. Four general BPF LSM programs are loaded once. Each shield is
// a row in one of three maps, so adding, promoting or removing a shield is a map
// write and never loads new kernel code.
//
//   exec_deny     lsm/bprm_check_security  refuse execve of a file named by device and inode
//   file_deny     lsm/file_open            refuse opening a file named by device and inode
//   connect_deny  lsm/socket_connect       refuse connect() to an address or network and port
//                 lsm/socket_sendmsg       and sendto()/sendmsg() with a destination on an
//                                          unconnected socket, which never calls connect()
//
// Every row carries its own deadline in CLOCK_BOOTTIME nanoseconds, checked here
// with bpf_ktime_get_boot_ns() on every match. A row past its deadline matches
// nothing, so a block ends on time even when the agent is stopped, hung or dead.
// Boot time keeps counting through suspend and does not move when someone sets
// the wall clock, so neither can stretch or shorten a shield.
//
// The agent pins these programs' links and maps under /sys/fs/bpf/edr_shield, so
// shields keep working after the agent exits, until each row's deadline. The
// guard object (edr_guard.bpf.c, not pinned) protects them while the agent runs.
//
// A row is in monitor mode (allow, count, report would_block) or enforce mode
// (refuse with -EPERM, count, report blocked). User space: src/shield/.
//
// The programs never write to a rule map. Hit counts and the report rate limit
// live in a per CPU array indexed by the shield's number. An earlier version did a
// 64 bit atomic add on the hit count inside the rule value. The verifier accepts
// that, but an LPM trie stores its value right after a key of any length, so the
// value was not 8 byte aligned, and on arm64 an unaligned atomic is a fault in
// kernel mode: the live test panicked the kernel. A per CPU counter needs no
// atomic at all.

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_endian.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

#define EPERM 1
#define AF_INET 2
#define AF_INET6 10
#define TASK_COMM_LEN 16
#define EXE_NAME_LEN 32
#define TARGET_LEN 64

// shield_cfg slots, written by user space (src/shield/abi.rs).
#define SCFG_SCOPE 0      // cgroup v2 id every shield is limited to, 0 for the whole host
#define SCFG_EXEC_ROWS 1  // rows in each map. 0 skips the lookup, so an
#define SCFG_FILE_ROWS 2  // empty kind costs one array read
#define SCFG_CONN_ROWS 3
#define SCFG_SLOTS 8

#define MODE_MONITOR 1
#define MODE_ENFORCE 2

#define KIND_EXEC 1
#define KIND_FILE 2
#define KIND_CONNECT 3

#define ACTION_WOULD_BLOCK 1
#define ACTION_BLOCKED 2

// A cgroup's ancestors are walked from the root down. 16 levels is deeper than
// any systemd, Docker or Kubernetes layout.
#define MAX_CGROUP_DEPTH 16
// At most one report per row per second. Hits are still counted on every match.
#define REPORT_GAP_NS 1000000000ULL
#define ROWS_MAX 16384
// Shield numbers run from 1 to STATS_SLOTS - 1, so every shield has its own counter.
#define STATS_SLOTS 32768

// A file the way the kernel names it: inode number and the superblock's dev_t.
// Same layout as struct exe_id in edr_guard.bpf.c.
struct file_key {
    __u64 ino;
    __u32 dev; // kernel dev_t, major << 20 | minor
    __u32 pad;
};

// LPM trie key. The prefix length counts bits from the start of port, so a rule
// for one port and a /24 has prefixlen 16 + 8 + 8 + 24. port 0 means any port.
// IPv4 addresses sit in the first 4 bytes of addr. tail pads the data after
// prefixlen to 24 bytes, a multiple of 8, so the value the trie stores right
// behind it starts 8 byte aligned. No prefix ever reaches tail.
struct conn_key {
    __u32 prefixlen;
    __u16 port;  // host byte order
    __u8 family; // 4 or 6
    __u8 pad;
    __u8 addr[16];
    __u32 tail;
};

#define CONN_KEY_FULL (32 + 128)

// One shield row, written only by user space. Mirrored by ShieldRule in
// src/shield/abi.rs.
struct shield_rule {
    __u64 expires_ns;    // CLOCK_BOOTTIME deadline
    __u64 not_before_ns; // CLOCK_BOOTTIME start
    __u64 cgroup_id;     // cgroup v2 subtree the row acts in, 0 for every task
    __u32 id;            // the shield's number, 1 to STATS_SLOTS - 1
    __u8 mode;           // MODE_*
    __u8 kind;           // KIND_*
    __u16 pad;
};

_Static_assert(sizeof(struct shield_rule) == 32, "update ShieldRule in src/shield/abi.rs");
_Static_assert(sizeof(struct conn_key) == 28, "update conn_key in src/shield/abi.rs");

// Per CPU, per shield: matches inside the row's time and scope, and the last
// report this CPU sent for the row.
struct shield_stat {
    __u64 hits;
    __u64 last_report_ns;
};

// One decision. Mirrored by ShieldEvent in src/shield/abi.rs.
struct shield_event {
    __u64 timestamp_ns; // CLOCK_BOOTTIME
    __u64 exe_ino;      // caller's executable
    __u64 target_ino;   // file for exec_deny and file_deny, else 0
    __u64 cgroup_id;    // caller's cgroup
    __u32 exe_dev;
    __u32 tgid;
    __u32 tid;
    __u32 uid;
    __u32 shield_id;
    __u32 target_dev;
    __u16 port;         // connect_deny destination port, host order
    __u8 kind;
    __u8 action;
    __u8 family;        // 4 or 6 for connect_deny
    __u8 pad[3];
    __u8 addr[16];
    char comm[TASK_COMM_LEN];
    char exe_name[EXE_NAME_LEN];
    char target[TARGET_LEN]; // file name for exec_deny and file_deny
};

_Static_assert(sizeof(struct shield_event) == 192, "update ShieldEvent in src/shield/abi.rs");

struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, SCFG_SLOTS);
    __type(key, __u32);
    __type(value, __u64);
} shield_cfg SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, ROWS_MAX);
    __type(key, struct file_key);
    __type(value, struct shield_rule);
} shield_exec SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, ROWS_MAX);
    __type(key, struct file_key);
    __type(value, struct shield_rule);
} shield_file SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_LPM_TRIE);
    __uint(max_entries, ROWS_MAX);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, struct conn_key);
    __type(value, struct shield_rule);
} shield_conn SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, STATS_SLOTS);
    __type(key, __u32);
    __type(value, struct shield_stat);
} shield_stats SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024);
} shield_events SEC(".maps");

static __always_inline __u64 cfg(__u32 slot)
{
    __u64 *v = bpf_map_lookup_elem(&shield_cfg, &slot);

    return v ? *v : 0;
}

static __always_inline void file_key_of(struct file *f, struct file_key *k)
{
    k->ino = BPF_CORE_READ(f, f_inode, i_ino);
    k->dev = BPF_CORE_READ(f, f_inode, i_sb, s_dev);
}

static __always_inline struct file *current_exe_file(void)
{
    struct task_struct *task = bpf_get_current_task_btf();

    return BPF_CORE_READ(task, mm, exe_file);
}

// True when the current task's cgroup is id or one of its descendants. Level 0
// is the root, and an ancestor id of 0 means the task's cgroup is not that deep.
static __always_inline bool in_subtree(__u64 id)
{
#pragma unroll
    for (int level = 0; level < MAX_CGROUP_DEPTH; level++) {
        __u64 a = bpf_get_current_ancestor_cgroup_id(level);

        if (a == id)
            return true;
        if (!a)
            return false;
    }
    return false;
}

// No shield acts on init itself. Host pid 1 is never reused while the host
// runs, and init forks before it runs anything, so this exempts no other
// program. There is no exemption by executable on purpose: a launcher such as
// systemd or sshd calls execve in its own child before the new program is
// mapped, so exempting the launcher would let it start anything. What must
// never be blocked is refused as a target in user space instead.
static __always_inline bool exempt_caller(void)
{
    return (bpf_get_current_pid_tgid() >> 32) == 1;
}

// The verdict for one matching row: 0 when the row is outside its time, outside
// its scope or the caller is exempt, else the row's mode. Counts the hit in this
// CPU's counter, and *report says whether this CPU sent no report for the row in
// the last second.
static __always_inline __u8 judge(struct shield_rule *r, __u64 *now, bool *report)
{
    struct shield_stat *st;
    __u64 scope;
    __u32 slot;

    *now = bpf_ktime_get_boot_ns();
    if (*now >= r->expires_ns || *now < r->not_before_ns)
        return 0;
    scope = cfg(SCFG_SCOPE);
    if (scope && !in_subtree(scope))
        return 0;
    if (r->cgroup_id && !in_subtree(r->cgroup_id))
        return 0;
    if (exempt_caller())
        return 0;
    slot = r->id;
    st = bpf_map_lookup_elem(&shield_stats, &slot);
    if (st) {
        // This CPU's own slot, and the program runs with migration off, so a
        // plain add is enough.
        st->hits += 1;
        if (*now - st->last_report_ns >= REPORT_GAP_NS) {
            st->last_report_ns = *now;
            *report = true;
        }
    }
    return r->mode;
}

static __always_inline struct shield_event *new_event(struct shield_rule *r, __u8 mode, __u64 now)
{
    struct shield_event *e = bpf_ringbuf_reserve(&shield_events, sizeof(*e), 0);
    struct file_key exe = {};
    struct file *f;
    __u64 pid_tgid;

    if (!e)
        return NULL;
    __builtin_memset(e, 0, sizeof(*e));
    pid_tgid = bpf_get_current_pid_tgid();
    e->timestamp_ns = now;
    e->tgid = pid_tgid >> 32;
    e->tid = (__u32)pid_tgid;
    e->uid = (__u32)bpf_get_current_uid_gid();
    e->cgroup_id = bpf_get_current_cgroup_id();
    e->shield_id = r->id;
    e->kind = r->kind;
    e->action = mode == MODE_ENFORCE ? ACTION_BLOCKED : ACTION_WOULD_BLOCK;
    bpf_get_current_comm(e->comm, sizeof(e->comm));
    f = current_exe_file();
    file_key_of(f, &exe);
    e->exe_ino = exe.ino;
    e->exe_dev = exe.dev;
    bpf_probe_read_kernel_str(e->exe_name, sizeof(e->exe_name), BPF_CORE_READ(f, f_path.dentry, d_name.name));
    return e;
}

static __always_inline int verdict(__u8 mode)
{
    return mode == MODE_ENFORCE ? -EPERM : 0;
}

// exec_deny and file_deny share everything but the map and the kind.
static __always_inline int file_shield(void *rules, struct file *file)
{
    struct shield_rule *r;
    struct file_key k = {};
    struct shield_event *e;
    bool report = false;
    __u64 now = 0;
    __u8 mode;

    file_key_of(file, &k);
    r = bpf_map_lookup_elem(rules, &k);
    if (!r)
        return 0;
    mode = judge(r, &now, &report);
    if (!mode)
        return 0;
    if (report) {
        e = new_event(r, mode, now);
        if (e) {
            e->target_ino = k.ino;
            e->target_dev = k.dev;
            bpf_probe_read_kernel_str(e->target, sizeof(e->target),
                                      BPF_CORE_READ(file, f_path.dentry, d_name.name));
            bpf_ringbuf_submit(e, 0);
        }
    }
    return verdict(mode);
}

// ---- exec_deny ----

// Runs once per binary handler round of execve and execveat with bprm->file the
// file about to run, so a denied script is caught in its first round and a
// denied interpreter in the round that loads it.
SEC("lsm/bprm_check_security")
int BPF_PROG(shield_exec_check, struct linux_binprm *bprm, int ret)
{
    // A nonzero ret is an earlier BPF program's refusal. Keep it.
    if (ret)
        return ret;
    if (!cfg(SCFG_EXEC_ROWS))
        return 0;
    return file_shield(&shield_exec, BPF_CORE_READ(bprm, file));
}

// ---- file_deny ----

// Every open of a file, exec's own open of the binary included, so a file_deny
// row also stops an exec, and an exec_deny row together with a file_deny row
// stops copying the file to get a new inode.
SEC("lsm/file_open")
int BPF_PROG(shield_file_open, struct file *file, int ret)
{
    if (ret)
        return ret;
    if (!cfg(SCFG_FILE_ROWS))
        return 0;
    return file_shield(&shield_file, file);
}

// ---- connect_deny ----

// Builds the lookup key from a sockaddr the kernel already copied from user
// space. The offsets are the fixed user space ABI of sockaddr_in and
// sockaddr_in6. An IPv4 mapped IPv6 address (::ffff:a.b.c.d) becomes the IPv4
// key, so an AF_INET6 socket cannot walk around an IPv4 rule.
union sa_copy {
    struct {
        __u16 family;
        __u16 port; // network byte order
        __u32 addr;
    } in4;
    struct {
        __u16 family;
        __u16 port;
        __u32 flowinfo;
        __u32 addr[4];
    } in6;
    __u64 raw[3];
};

static __always_inline bool conn_key_of(void *sa, int len, struct conn_key *k)
{
    union sa_copy c = {};

    if (!sa || len < 16)
        return false;
    k->prefixlen = CONN_KEY_FULL;
    if (bpf_probe_read_kernel(&c, 16, sa))
        return false;
    k->port = bpf_ntohs(c.in4.port);
    if (c.in4.family == AF_INET) {
        k->family = 4;
        __builtin_memcpy(k->addr, &c.in4.addr, 4);
        return true;
    }
    if (c.in4.family != AF_INET6 || len < 24)
        return false;
    if (bpf_probe_read_kernel(&c, 24, sa))
        return false;
    if (c.in6.addr[0] == 0 && c.in6.addr[1] == 0 && c.in6.addr[2] == bpf_htonl(0x0000ffff)) {
        k->family = 4;
        __builtin_memcpy(k->addr, &c.in6.addr[3], 4);
        return true;
    }
    k->family = 6;
    __builtin_memcpy(k->addr, c.in6.addr, 16);
    return true;
}

static __always_inline int conn_shield(void *sa, int len)
{
    struct shield_rule *r;
    struct shield_event *e;
    struct conn_key k = {};
    bool report = false;
    __u16 port;
    __u64 now = 0;
    __u8 mode;

    if (!conn_key_of(sa, len, &k))
        return 0;
    port = k.port;
    // The longest match for this port, then for any port.
    r = bpf_map_lookup_elem(&shield_conn, &k);
    if (!r) {
        k.port = 0;
        r = bpf_map_lookup_elem(&shield_conn, &k);
    }
    if (!r)
        return 0;
    mode = judge(r, &now, &report);
    if (!mode)
        return 0;
    if (report) {
        e = new_event(r, mode, now);
        if (e) {
            e->port = port;
            e->family = k.family;
            __builtin_memcpy(e->addr, k.addr, 16);
            bpf_ringbuf_submit(e, 0);
        }
    }
    return verdict(mode);
}

SEC("lsm/socket_connect")
int BPF_PROG(shield_connect, struct socket *sock, struct sockaddr *address, int addrlen, int ret)
{
    if (ret)
        return ret;
    if (!cfg(SCFG_CONN_ROWS))
        return 0;
    return conn_shield(address, addrlen);
}

// sendto() and sendmsg() with a destination address. msg_name is the kernel's
// copy of the address, or NULL for a send on a connected socket, which the
// connect hook already judged.
SEC("lsm/socket_sendmsg")
int BPF_PROG(shield_sendmsg, struct socket *sock, struct msghdr *msg, int size, int ret)
{
    void *name;
    int len;

    if (ret)
        return ret;
    if (!cfg(SCFG_CONN_ROWS))
        return 0;
    name = BPF_CORE_READ(msg, msg_name);
    if (!name)
        return 0;
    len = BPF_CORE_READ(msg, msg_namelen);
    return conn_shield(name, len);
}

// suspicious_ipc.c
// Reports connect() calls to container runtime control sockets such as
// /var/run/docker.sock. A process that can talk to one of these sockets can
// start a privileged container with the host filesystem mounted, so access is
// root on the host in practice (MITRE ATT&CK T1611 Escape to Host, T1610
// Deploy Container).
//
// User space (src/modules/suspicious_ipc.rs) loads this object, fills
// watched_sockets, reads ipc_events and decides which clients are expected.

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

// vmlinux.h carries kernel types but not #defines, so these are spelled out.
#define AF_UNIX 1
#define UNIX_PATH_MAX 108 // size of sockaddr_un.sun_path
#define TASK_COMM_LEN 16
#define SUN_FAMILY_LEN 2  // sockaddr_un is a 2 byte family, then sun_path

// Watch list key: one socket path, zero padded to the full sun_path size.
// User space writes each watched path in the same zero padded form. The
// program builds a key the same way from the caller's sockaddr, so checking
// the whole list is a single hash lookup instead of a string compare loop.
struct sock_path_key {
    char path[UNIX_PATH_MAX];
};

// One record per connect() to a watched path. Mirrored by IpcConnectEvent in
// src/modules/suspicious_ipc.rs. The field order leaves no padding.
struct ipc_connect_event {
    __u32 tgid;         // process id
    __u32 tid;          // thread id
    __u32 uid;          // real uid of the caller
    char  comm[TASK_COMM_LEN];
    char  path[UNIX_PATH_MAX];
    __u64 timestamp_ns; // bpf_ktime_get_ns, nanoseconds since boot
};

_Static_assert(sizeof(struct ipc_connect_event) == 144, "update IpcConnectEvent in suspicious_ipc.rs");

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 64);
    __type(key, struct sock_path_key);
    __type(value, __u8);
} watched_sockets SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024);
} ipc_events SEC(".maps");

// Runs on every connect() on the host, TCP and UDP included. Anything that is
// not AF_UNIX leaves after one 2 byte read.
SEC("tracepoint/syscalls/sys_enter_connect")
int trace_runtime_socket_connect(struct trace_event_raw_sys_enter *ctx)
{
    const char *uaddr = (const char *)ctx->args[1];
    // addrlen is an int in the syscall. Holding it as a 64 bit unsigned value
    // keeps the bounds checks below on one register the verifier can follow.
    __u64 addrlen = ctx->args[2] & 0xffffffff;
    struct sock_path_key key = {};
    struct ipc_connect_event *evt;
    __u16 family = 0;
    __u64 size;
    __u64 id;

    // Need the family plus at least one byte of path.
    if (!uaddr || addrlen <= SUN_FAMILY_LEN)
        return 0;
    if (bpf_probe_read_user(&family, sizeof(family), uaddr))
        return 0;
    if (family != AF_UNIX)
        return 0;

    // The kernel only looks at addrlen bytes of sun_path, so read no more than
    // that: the path bytes passed plus one for the NUL, capped at the key size.
    // The helper stops at the first NUL, ends the copy with a NUL and leaves
    // the rest of key zeroed, which keeps it comparable to the map keys.
    size = addrlen - SUN_FAMILY_LEN + 1;
    if (size > UNIX_PATH_MAX)
        size = UNIX_PATH_MAX;
    if (bpf_probe_read_user_str(key.path, size, uaddr + SUN_FAMILY_LEN) <= 0)
        return 0;

    // sun_path[0] == 0 is an abstract socket, which has no filesystem path.
    if (key.path[0] == 0)
        return 0;

    if (!bpf_map_lookup_elem(&watched_sockets, &key))
        return 0;

    evt = bpf_ringbuf_reserve(&ipc_events, sizeof(*evt), 0);
    if (!evt)
        return 0;

    id = bpf_get_current_pid_tgid();
    evt->tgid = id >> 32;
    evt->tid = (__u32)id;
    evt->uid = (__u32)bpf_get_current_uid_gid();
    bpf_get_current_comm(evt->comm, sizeof(evt->comm));
    __builtin_memcpy(evt->path, key.path, sizeof(evt->path));
    evt->timestamp_ns = bpf_ktime_get_ns();

    bpf_ringbuf_submit(evt, 0);
    return 0;
}

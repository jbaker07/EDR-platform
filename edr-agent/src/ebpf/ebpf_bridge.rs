// src/ebpf_bridge.rs
#![allow(dead_code)]
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry::TelemetryRecord;
use crate::utils::time::now_ts;

// ---- Keep these numeric ids aligned with ebpf/include/edr_events.h ----
pub mod evt {
    pub const EVT_EXEC:            u32 = 30;
    pub const EVT_OPEN:            u32 = 10;
    pub const EVT_RENAME:          u32 = 11;
    pub const EVT_READ:            u32 = 12;
    pub const EVT_WRITE:           u32 = 13;
    pub const EVT_UNLINK:          u32 = 14;
    pub const EVT_CHMOD_CHOWN:     u32 = 15;
    pub const EVT_SETXATTR:        u32 = 16;
    pub const EVT_CLOSE:           u32 = 18;
    pub const EVT_DUP:             u32 = 19;

    pub const EVT_SOCKET:          u32 = 20;
    pub const EVT_CONNECT:         u32 = 21;
    pub const EVT_BIND:            u32 = 22;
    pub const EVT_LISTEN:          u32 = 23;
    pub const EVT_ACCEPT:          u32 = 24;
    pub const EVT_SENDTO:          u32 = 25;
    pub const EVT_SENDMSG:         u32 = 26;
    pub const EVT_RECVFROM:        u32 = 27;
    pub const EVT_RECVMSG:         u32 = 28;

    pub const EVT_MPROTECT:        u32 = 40;
    pub const EVT_MEMFD_CREATE:    u32 = 42;

    pub const EVT_SETUID:          u32 = 50;
    pub const EVT_CAPSET:          u32 = 51;

    pub const EVT_CLONE:           u32 = 60;
    pub const EVT_PTRACE:          u32 = 62;
    pub const EVT_PRCTL:           u32 = 63;
    pub const EVT_SECCOMP:         u32 = 64;

    pub const EVT_SETNS:           u32 = 70;
    pub const EVT_UNSHARE:         u32 = 71;
    pub const EVT_MOUNT:           u32 = 72;
    pub const EVT_UMOUNT2:         u32 = 73;
    pub const EVT_PIVOT_ROOT:      u32 = 74;

    pub const EVT_MOD_LOAD:        u32 = 80;
    pub const EVT_MOD_DEL:         u32 = 81;

    pub const EVT_BPF:             u32 = 90;

    // add-ons
    pub const EVT_PROC_EXEC_TP:    u32 = 101;
    pub const EVT_PROC_FORK:       u32 = 102;
    pub const EVT_PROC_EXIT:       u32 = 103;

    pub const EVT_TCP_STATE:       u32 = 200;
    pub const EVT_TCP_RETRANS:     u32 = 201;
}

// NOTE: Must match C layout exactly.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EdREvent {
    pub ts:         u64,
    pub type_:      u32,   // edr_evt_type
    pub syscall_id: u32,

    pub tgid:       u32,
    pub ppid:       u32,
    pub uid:        u32,

    pub fd:         i32,
    pub ret:        i32,

    pub flags:      u32,
    pub aux_u32:    u32,
    pub aux_u64:    u64,

    pub fam:        u8,
    pub proto:      u8,
    pub lport:      u16,
    pub laddr4:     u32,
    pub laddr6:     [u8; 16],
    pub rport:      u16,
    pub raddr4:     u32,
    pub raddr6:     [u8; 16],

    pub path:       [u8; 128],
    pub path2:      [u8; 128],
    pub comm:       [u8; 16],
}

fn cstr_trunc(bytes: &[u8]) -> String {
    let n = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..n]).to_string()
}

fn ipv4_be_to_string(be: u32) -> String {
    let b = be.to_be_bytes();
    format!("{}.{}.{}.{}", b[0], b[1], b[2], b[3])
}
fn port_be_to_u16(be: u16) -> u16 { u16::from_be(be) }

fn tagset_for_event(e: &EdREvent) -> Vec<String> {
    use evt::*;
    let mut t = Vec::with_capacity(4);
    match e.type_ {
        EVT_EXEC | EVT_PROC_EXEC_TP => t.push("exec".into()),
        EVT_OPEN => t.push("file_open".into()),
        EVT_RENAME => t.push("file_rename".into()),
        EVT_UNLINK => t.push("file_unlink".into()),
        EVT_CHMOD_CHOWN => t.push("file_perm_change".into()),
        EVT_SETXATTR => t.push("file_xattr".into()),
        EVT_CLOSE => t.push("fd_close".into()),
        EVT_DUP => t.push("fd_dup".into()),

        EVT_SOCKET => t.push("sock_create".into()),
        EVT_BIND => t.push("sock_bind".into()),
        EVT_LISTEN => t.push("sock_listen".into()),
        EVT_CONNECT => t.push("net_connect".into()),
        EVT_ACCEPT => t.push("net_accept".into()),
        EVT_SENDTO | EVT_SENDMSG => t.push("net_send".into()),
        EVT_RECVFROM | EVT_RECVMSG => t.push("net_recv".into()),
        EVT_TCP_STATE => t.push("tcp_state".into()),
        EVT_TCP_RETRANS => t.push("tcp_retrans".into()),

        EVT_MPROTECT => {
            t.push("mprotect".into());
            if e.flags & 0x4 != 0 { t.push("mprotect_exec".into()) } // PROT_EXEC
        }
        EVT_MEMFD_CREATE => t.push("memfd_create".into()),

        EVT_SETUID => t.push("priv_setuid".into()),
        EVT_CAPSET => t.push("priv_capset".into()),
        EVT_PTRACE => t.push("ptrace".into()),
        EVT_PRCTL => t.push("prctl".into()),
        EVT_SECCOMP => t.push("seccomp".into()),

        EVT_SETNS => t.push("ns_setns".into()),
        EVT_UNSHARE => t.push("ns_unshare".into()),
        EVT_MOUNT => t.push("fs_mount".into()),
        EVT_UMOUNT2 => t.push("fs_umount".into()),
        EVT_PIVOT_ROOT => t.push("fs_pivot_root".into()),

        EVT_MOD_LOAD => t.push("kernel_module_load".into()),
        EVT_MOD_DEL => t.push("kernel_module_delete".into()),
        EVT_BPF => t.push("bpf_usage".into()),

        EVT_CLONE => t.push("proc_clone".into()),
        EVT_PROC_FORK => t.push("proc_fork".into()),
        EVT_PROC_EXIT => t.push("proc_exit".into()),

        EVT_READ => t.push("fd_read".into()),
        EVT_WRITE => t.push("fd_write".into()),
        _ => t.push("unknown_evt".into()),
    }
    t
}

// Turn one kernel event into a compact TelemetryOutput
pub fn edr_event_to_output(e: &EdREvent) -> TelemetryOutput {
    use std::collections::HashMap;
    let mut data: HashMap<String, String> = HashMap::new();

    // common
    data.insert("ts_ns".into(), e.ts.to_string());
    data.insert("tgid".into(), e.tgid.to_string());
    data.insert("ppid".into(), e.ppid.to_string());
    data.insert("uid".into(), e.uid.to_string());
    data.insert("fd".into(), e.fd.to_string());
    data.insert("ret".into(), e.ret.to_string());
    data.insert("evt_type".into(), e.type_.to_string());
    data.insert("syscall_id".into(), e.syscall_id.to_string());
    data.insert("flags".into(), e.flags.to_string());
    data.insert("aux_u32".into(), e.aux_u32.to_string());
    data.insert("aux_u64".into(), e.aux_u64.to_string());
    data.insert("comm".into(), cstr_trunc(&e.comm));

    // paths
    let p1 = cstr_trunc(&e.path);
    let p2 = cstr_trunc(&e.path2);
    if !p1.is_empty() { data.insert("path".into(), p1); }
    if !p2.is_empty() { data.insert("path2".into(), p2); }

    // net
    if e.fam != 0 {
        data.insert("fam".into(), e.fam.to_string());
    }
    if e.lport != 0 { data.insert("lport".into(), port_be_to_u16(e.lport).to_string()); }
    if e.laddr4 != 0 { data.insert("laddr4".into(), ipv4_be_to_string(e.laddr4)); }
    if e.rport != 0 { data.insert("rport".into(), port_be_to_u16(e.rport).to_string()); }
    if e.raddr4 != 0 { data.insert("raddr4".into(), ipv4_be_to_string(e.raddr4)); }
    if e.proto != 0 { data.insert("proto".into(), e.proto.to_string()); }

    // category/signal
    let tags = tagset_for_event(e);
    // signal = primary tag; category = family
    let (signal, category) = if tags.is_empty() {
        ("unknown_evt".to_string(), "kernel".to_string())
    } else {
        let s = tags[0].clone();
        let cat = if s.starts_with("net_") || s.starts_with("tcp_") || s.starts_with("sock_") {
            "network".to_string()
        } else if s.starts_with("file_") || s.starts_with("fd_") || s.starts_with("fs_") {
            "file".to_string()
        } else if s.starts_with("priv_") || s == "ptrace" || s == "seccomp" || s == "bpf_usage" {
            "privilege".to_string()
        } else if s == "exec" || s.starts_with("proc_") {
            "process".to_string()
        } else if s.starts_with("mprotect") || s.starts_with("memfd") {
            "memory".to_string()
        } else {
            "kernel".to_string()
        };
        (s, cat)
    };

    TelemetryOutput {
        category,
        signal,
        confidence: 0.95, // high-fidelity kernel source
        data,
    }
}

// Convenience for callers who want a TelemetryRecord directly
pub fn edr_event_to_record(e: &EdREvent) -> TelemetryRecord {
    let p1 = cstr_trunc(&e.path);
    let cmd = cstr_trunc(&e.comm);
    TelemetryRecord {
        timestamp: now_ts(),
        pid: e.tgid as i32,
        ppid: e.ppid as i32,
        uid: e.uid,
        binary_path: p1,              // for exec/open this is meaningful
        command_line: cmd,            // short, but better than empty
        cwd: String::new(),
        env_vars: None,
        tags: tagset_for_event(e),
        risk_score: None,
    }
}

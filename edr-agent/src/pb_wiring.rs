// src/pb_wiring.rs
// Adapter facts -> playbook engine facts, and completed playbooks -> hits and incidents.
//
// Facts come from pb_adapter, which sees each decoder's full data map. Sources and the
// facts they become:
//   proc_lifecycle exec                       Exec (exe and argv from /proc at decode, else
//                                             the filename and argv copied in the kernel),
//                                             plus exe_name, cwd, ppid, parent_exe
//   wx_exec memfd_exec, rwx_map               Exec with tag MEMFD_EXEC or RWX_MAP
//   file_access_monitor openat                FileIO op=open
//   raw_syscalls_catcher openat exit          FileIO op=write, open or open_fail plus errno
//   raw_syscalls_catcher renameat(2), unlinkat FileIO op=rename plus ext_new, op=delete
//   raw_syscalls_catcher write                FileIO op=write plus bytes, path from the open
//   net_flow connect                          NetFlow op=connect: dst_ip, dst_port,
//                                             dst_scope, dst_private, dst_new, exe
//   net_flow send, sendto, sendmsg            NetFlow op=send: bytes, and the destination of
//                                             the process's newest connect (with its sni once
//                                             the hello for that destination was seen)
//   net_flow TLS ClientHello                  NetFlow op=tls_hello: sni, alpn, dst_ip, dst_port
//   tcp_state                                 Net op=established, closed (duration_sec) and
//                                             open_flow (sweep_flows, for long-lived ones)
//   rules.json alerts, guard decisions        RuleHit: rule_id, ttp, ttps, source
//   modules::host_perf                        HostPerf: scope, cpu_pct, cpu_busy_pct, and
//                                             scope gpu with gpu_util_pct
//   modules::auth_log                         Auth: method, result, user, src_ip, src_new
// Everything else gives no fact: fork, exit, W then X, recv bytes, DNS and IPC findings.
// raw_syscalls_catcher's execve frame is skipped: it fires at syscall entry, when /proc
// still shows the caller's argv, so it would pair the new exe with old args.
// proc_lifecycle's exec fires after the exec and is the exec source.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use serde_json::{json, Map, Value};

use crate::pb_adapter::AdapterFact;
use crate::pb_engine::{parent_dir, topdir_of, Fact, FactKind, PlaybookEngine, PlaybookHit as EngineHit};
use crate::pb_introspect::{self, PlaybookHit as IntrospectHit};

/// Simple context wrapper for owning a PB engine outside main.
pub struct PbCtx {
    pub engine: PlaybookEngine,
}

impl PbCtx {
    pub fn new(dir: &str) -> Self {
        // Ensure the directory exists so load_dir can succeed even if empty.
        let _ = std::fs::create_dir_all(dir);

        let engine = match PlaybookEngine::load_dir(dir) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("pb_wiring: failed to load playbooks from '{dir}': {e}");
                PlaybookEngine::empty()
            }
        };
        Self { engine }
    }
}

// ---- Mount table: which filesystem a path is on ----

pub trait MountSource: Send {
    /// (mount point, filesystem type) of the mount that holds `path`, as `pid` sees it.
    fn lookup(&mut self, pid: u32, path: &str) -> Option<(String, String)>;
}

/// Mount tables from /proc/<pid>/mountinfo, one per mount namespace, re-read every 30 s.
/// A process that already exited is looked up in the agent's own namespace.
#[derive(Default)]
pub struct ProcMounts {
    tables: HashMap<String, (Instant, Vec<(String, String)>)>,
}

impl MountSource for ProcMounts {
    fn lookup(&mut self, pid: u32, path: &str) -> Option<(String, String)> {
        let (ns, src) = match std::fs::read_link(format!("/proc/{pid}/ns/mnt")) {
            Ok(l) => (l.to_string_lossy().into_owned(), format!("/proc/{pid}/mountinfo")),
            Err(_) => ("self".to_string(), "/proc/self/mountinfo".to_string()),
        };
        let fresh = self
            .tables
            .get(&ns)
            .map(|(t, _)| t.elapsed() < Duration::from_secs(30))
            .unwrap_or(false);
        if !fresh {
            let text = std::fs::read_to_string(&src)
                .or_else(|_| std::fs::read_to_string("/proc/self/mountinfo"))
                .unwrap_or_default();
            if self.tables.len() >= 64 {
                self.tables.clear();
            }
            self.tables
                .insert(ns.clone(), (Instant::now(), parse_mountinfo(&text)));
        }
        mount_for(&self.tables.get(&ns)?.1, path)
    }
}

/// A fixed table, for tests.
pub struct FixedMounts(pub Vec<(String, String)>);

impl MountSource for FixedMounts {
    fn lookup(&mut self, _pid: u32, path: &str) -> Option<(String, String)> {
        mount_for(&self.0, path)
    }
}

/// (mount point, fs type) from /proc/<pid>/mountinfo. Field 5 is the mount point, with
/// spaces and similar bytes written as octal escapes. The fs type follows the " - ".
pub fn parse_mountinfo(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let Some((left, right)) = line.split_once(" - ") else {
            continue;
        };
        let (Some(mp), Some(fstype)) = (left.split(' ').nth(4), right.split(' ').next()) else {
            continue;
        };
        out.push((unescape_octal(mp), fstype.to_string()));
    }
    out
}

fn unescape_octal(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\'
            && i + 3 < b.len()
            && b[i + 1..i + 4].iter().all(|c| (b'0'..=b'7').contains(c))
        {
            out.push((b[i + 1] - b'0') * 64 + (b[i + 2] - b'0') * 8 + (b[i + 3] - b'0'));
            i += 4;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// The mount with the longest mount point that contains `path`, by whole components.
pub fn mount_for(table: &[(String, String)], path: &str) -> Option<(String, String)> {
    table
        .iter()
        .filter(|(mp, _)| {
            mp == "/"
                || path == mp
                || (path.starts_with(mp.as_str()) && path.as_bytes().get(mp.len()) == Some(&b'/'))
        })
        .max_by_key(|(mp, _)| mp.len())
        .cloned()
}

// ---- Mapper ----

const O_ACCMODE: i64 = 0o3;
const O_CREAT: i64 = 0o100;
const O_TRUNC: i64 = 0o1000;
// raw_syscalls_catcher event types, from edr_events.h.
const EVT_EXEC: i64 = 30;
const EVT_OPEN: i64 = 10;
const EVT_RENAME: i64 = 11;
const EVT_WRITE: i64 = 13;
const EVT_UNLINK: i64 = 14;
const EVT_CLOSE: i64 = 18;
const EVT_MOUNT: i64 = 72;
const EVT_TCP_STATE: i64 = 200;
/// Open files remembered so a write(fd) can be tied to a path. Cleared when full.
const FD_TABLE_CAP: usize = 65_536;
/// Processes, destinations and TCP flows remembered. Each table is cleared when full,
/// which costs some attribution for a moment and never unbounded memory.
const PROC_TABLE_CAP: usize = 65_536;
const SEEN_DST_CAP: usize = 65_536;
const FLOW_TABLE_CAP: usize = 16_384;
/// A TCP connection open this long is reported as an open_flow, then again every
/// FLOW_REPORT_SEC while it stays open.
pub const FLOW_REPORT_SEC: u64 = 60;
/// A SYN_SENT that saw no ESTABLISHED within this time is forgotten.
const SYN_PENDING_SEC: u64 = 120;

/// What the mapper knows about a process from its exec fact.
#[derive(Clone, Debug)]
struct ProcEntry {
    exe: String,
    ppid: u32,
    /// From the exec frame's uid. Frames without a uid (net_flow) take it from here,
    /// so playbooks keyed by user can join a process's sends to its exec.
    user: Option<String>,
}

/// The newest connect() of a process, so its sends can name a destination.
#[derive(Clone, Debug)]
struct Dst {
    ip: String,
    port: String,
    scope: &'static str,
    new: bool,
    /// Server name from the TLS ClientHello sent to this destination.
    sni: Option<String>,
}

/// One TCP connection between ESTABLISHED and CLOSE.
#[derive(Clone, Debug)]
struct Flow {
    pid: u32,
    exe: Option<String>,
    start: u64,
    last_report: u64,
    direction: &'static str,
}

/// (local ip, local port, remote ip, remote port)
type FlowKey = (String, u16, String, u16);

pub struct Mapper {
    fds: HashMap<(u32, i64), String>,
    mounts: Box<dyn MountSource>,
    skip_pid: Option<u32>,
    /// /proc/<pid>/exe for a process the mapper saw no exec for. Nothing in tests.
    proc_exe: fn(u32) -> Option<String>,
    /// The package that owns an exe and whether the file matches it (pkg_verify).
    /// Nothing in tests unless a test sets one.
    pub pkg_lookup: fn(&str) -> Option<crate::pkg_verify::Verdict>,
    /// Account name for a uid, from /etc/passwd. Nothing in tests, where users stay
    /// root or uid:N.
    pub uid_names: fn(u32) -> Option<String>,
    /// EDR_HOST_ROLE, such as bastion: what this host is for, as the site states it.
    pub host_role: Option<String>,
    procs: HashMap<u32, ProcEntry>,
    last_dst: HashMap<u32, Dst>,
    seen_dsts: HashSet<String>,
    syn_pending: HashMap<(String, u16), VecDeque<(u32, u64)>>,
    flows: HashMap<FlowKey, Flow>,
}

fn read_proc_exe(pid: u32) -> Option<String> {
    std::fs::read_link(format!("/proc/{pid}/exe"))
        .ok()
        .map(|p| p.to_string_lossy().into_owned())
}

fn no_proc_exe(_pid: u32) -> Option<String> {
    None
}

fn no_pkg(_path: &str) -> Option<crate::pkg_verify::Verdict> {
    None
}

fn no_names(_uid: u32) -> Option<String> {
    None
}

/// uid -> name from an /etc/passwd text.
fn parse_passwd(text: &str) -> HashMap<u32, String> {
    text.lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let mut f = l.split(':');
            let name = f.next()?;
            let uid = f.nth(1)?.parse::<u32>().ok()?;
            (!name.is_empty()).then(|| (uid, name.to_string()))
        })
        .collect()
}

/// The account name of a uid from /etc/passwd, read again when the file changes
/// (checked at most once a minute). Accounts that live only in a directory service
/// (LDAP, SSSD) are not in the file and stay uid:N: asking NSS could block on the network.
fn passwd_name(uid: u32) -> Option<String> {
    struct Cache {
        names: HashMap<u32, String>,
        mtime: Option<std::time::SystemTime>,
        checked: Option<std::time::Instant>,
    }
    static C: OnceLock<Mutex<Cache>> = OnceLock::new();
    let c = C.get_or_init(|| Mutex::new(Cache { names: HashMap::new(), mtime: None, checked: None }));
    let mut c = c.lock().ok()?;
    if c.checked.map(|t| t.elapsed().as_secs() >= 60).unwrap_or(true) {
        c.checked = Some(std::time::Instant::now());
        let mtime = std::fs::metadata("/etc/passwd").and_then(|m| m.modified()).ok();
        if mtime != c.mtime {
            c.mtime = mtime;
            c.names = std::fs::read_to_string("/etc/passwd").map(|t| parse_passwd(&t)).unwrap_or_default();
        }
    }
    c.names.get(&uid).cloned()
}

impl Mapper {
    /// The agent's mapper: real mount tables, /proc for exe paths, and the agent's own
    /// events skipped so its log writes never count toward a burst.
    pub fn new() -> Self {
        let mut m = Self::with_mounts(Box::new(ProcMounts::default()));
        m.skip_pid = Some(std::process::id());
        m.proc_exe = read_proc_exe;
        m.pkg_lookup = crate::pkg_verify::host_verdict;
        m.uid_names = passwd_name;
        m.host_role = std::env::var("EDR_HOST_ROLE").ok().filter(|r| !r.trim().is_empty());
        m
    }

    pub fn with_mounts(mounts: Box<dyn MountSource>) -> Self {
        Self {
            fds: HashMap::new(),
            mounts,
            skip_pid: None,
            proc_exe: no_proc_exe,
            pkg_lookup: no_pkg,
            uid_names: no_names,
            host_role: None,
            procs: HashMap::new(),
            last_dst: HashMap::new(),
            seen_dsts: HashSet::new(),
            syn_pending: HashMap::new(),
            flows: HashMap::new(),
        }
    }

    /// Engine facts for one adapter fact.
    pub fn facts(&mut self, af: &AdapterFact) -> Vec<Fact> {
        let Some(m) = af.meta.as_object() else {
            return Vec::new();
        };
        let text = |k: &str| m.get(k).and_then(value_text).unwrap_or_default();
        let num = |k: &str| m.get(k).and_then(value_i64);
        let pid = num("pid").or_else(|| num("tgid")).unwrap_or(0).max(0) as u32;
        if pid != 0 && Some(pid) == self.skip_pid {
            return Vec::new();
        }
        let mut ctx = Ctx {
            ts: af.ts.max(0) as u64,
            host: af.host.clone(),
            pid,
            user: num("uid").map(|u| self.user_name(u)),
            comm: text("comm"),
        };
        if ctx.user.is_none() {
            ctx.user = self.procs.get(&pid).and_then(|p| p.user.clone());
        }
        let source = text("source");
        let signal = text("signal");

        match source.as_str() {
            "proc_lifecycle" if signal == "process::exec" => {
                let tags = vec![signal.clone()];
                let ppid = num("ppid").map(|p| p.max(0) as u32);
                vec![self.exec_fact(&ctx, ppid, m, tags)]
            }
            "wx_exec" => {
                let marker = match signal.as_str() {
                    "memory::memfd_exec" => "MEMFD_EXEC",
                    "memory::rwx_map" => "RWX_MAP",
                    // w_then_x is the dynamic loader on every process start.
                    _ => return Vec::new(),
                };
                let tags = vec![signal.clone(), marker.to_string()];
                let ppid = self.procs.get(&pid).map(|p| p.ppid).filter(|p| *p != 0);
                vec![self.exec_fact(&ctx, ppid, m, tags)]
            }
            "file_access_monitor" => {
                let path = absolute(&text("path"), &text("cwd"));
                vec![self.file_fact(&ctx, "open", &path, Vec::new())]
            }
            "net_flow" => self.net_flow(&ctx, &signal, m),
            "rule_engine" | "edr_guard" => vec![rule_hit_fact(&ctx, &source, m)],
            "host_perf" => vec![host_perf_fact(&ctx, m)],
            "auth_log" => vec![auth_fact(&ctx, m)],
            "" if m.contains_key("syscall_id") => self.raw_syscall(&ctx, m),
            _ => Vec::new(),
        }
    }

    /// The 376-byte struct edr_event from raw_syscalls_catcher and tcp_state.
    fn raw_syscall(&mut self, ctx: &Ctx, m: &Map<String, Value>) -> Vec<Fact> {
        let text = |k: &str| m.get(k).and_then(value_text).unwrap_or_default();
        let num = |k: &str| m.get(k).and_then(value_i64);
        let path = absolute(&text("path"), &text("cwd"));
        let fd = num("fd").unwrap_or(-1);
        match num("evt_type").unwrap_or(-1) {
            EVT_OPEN => {
                let ret = num("ret").unwrap_or(0);
                let flags = num("flags").unwrap_or(0);
                let mut extra = vec![("flags".to_string(), format!("{flags:#o}"))];
                let op = if ret < 0 {
                    extra.push(("errno".to_string(), errno_name(-ret)));
                    "open_fail"
                } else {
                    if self.fds.len() >= FD_TABLE_CAP {
                        self.fds.clear();
                    }
                    self.fds.insert((ctx.pid, ret), path.clone());
                    let acc = flags & O_ACCMODE;
                    if acc == 1 || acc == 2 || flags & (O_CREAT | O_TRUNC) != 0 {
                        "write"
                    } else {
                        "open"
                    }
                };
                vec![self.file_fact(ctx, op, &path, extra)]
            }
            EVT_RENAME => {
                let new = absolute(&text("path2"), &text("cwd"));
                let extra = vec![
                    ("path2".to_string(), new.clone()),
                    ("ext_new".to_string(), extension(&new)),
                ];
                vec![self.file_fact(ctx, "rename", &path, extra)]
            }
            EVT_UNLINK => vec![self.file_fact(ctx, "delete", &path, Vec::new())],
            EVT_WRITE => {
                // Only writes to a file this process opened while the sensor watched.
                // Pipes, sockets and terminals have no entry and give no fact.
                let Some(p) = self.fds.get(&(ctx.pid, fd)).cloned() else {
                    return Vec::new();
                };
                // aux_u64 is the count passed to write(2), the bytes asked for.
                let bytes = num("aux_u64").unwrap_or(0).max(0);
                vec![self.file_fact(ctx, "write", &p, vec![("bytes".to_string(), bytes.to_string())])]
            }
            EVT_CLOSE => {
                self.fds.remove(&(ctx.pid, fd));
                Vec::new()
            }
            EVT_MOUNT if !path.is_empty() => vec![self.file_fact(ctx, "mount", &path, Vec::new())],
            EVT_TCP_STATE => self.tcp_state(ctx, m),
            EVT_EXEC => Vec::new(),
            _ => Vec::new(),
        }
    }

    fn file_fact(&mut self, ctx: &Ctx, op: &str, path: &str, extra: Vec<(String, String)>) -> Fact {
        let mut fields = HashMap::new();
        fields.insert("op".to_string(), op.to_string());
        fields.insert("path".to_string(), path.to_string());
        // Playbooks use filename both for a basename pattern and for a full path
        // (luks_locker compares it with /etc/crypttab), so it holds the full path.
        fields.insert("filename".to_string(), path.to_string());
        fields.insert("dir".to_string(), parent_dir(path));
        fields.insert("topdir".to_string(), topdir_of(path));
        fields.insert("ext".to_string(), extension(path));
        if path.starts_with('/') {
            if let Some((mp, fstype)) = self.mounts.lookup(ctx.pid, path) {
                fields.insert("mount_point".to_string(), mp);
                fields.insert("mount_fstype".to_string(), fstype);
            }
        }
        for (k, v) in extra {
            fields.insert(k, v);
        }
        // The exe of a process the mapper saw exec. No /proc read per file event.
        let exe = self.procs.get(&ctx.pid).map(|p| p.exe.clone());
        if let Some(e) = &exe {
            fields.insert("exe_name".to_string(), base_name(e));
        }
        ctx.fact(FactKind::FileIO, exe, None, fields)
    }

    fn exec_fact(&mut self, ctx: &Ctx, ppid: Option<u32>, m: &Map<String, Value>, tags: Vec<String>) -> Fact {
        let text = |k: &str| m.get(k).and_then(value_text).filter(|s| !s.is_empty());
        let exe = text("binary_path");
        let mut fields = HashMap::new();
        if let Some(ppid) = ppid {
            fields.insert("ppid".to_string(), ppid.to_string());
            // The parent's exe from its own exec, else from /proc while it still runs.
            let parent = match self.procs.get(&ppid) {
                Some(p) => Some(p.exe.clone()),
                None if ppid > 0 => (self.proc_exe)(ppid),
                None => None,
            };
            if let Some(pe) = parent {
                fields.insert("parent_name".to_string(), base_name(&pe));
                fields.insert("parent_exe".to_string(), pe);
            }
        }
        if let Some(cwd) = text("cwd") {
            fields.insert("cwd".to_string(), cwd);
        }
        if let Some((hour, dow)) = local_hour_dow(ctx.ts) {
            fields.insert("hour".to_string(), hour);
            fields.insert("dow".to_string(), dow);
        }
        if let Some(role) = &self.host_role {
            fields.insert("host_role".to_string(), role.clone());
        }
        // Allowlisted environment variables the adapter read at exec (env.CI and so on).
        for (k, v) in m.iter().filter(|(k, _)| k.starts_with("env.")) {
            if let Some(v) = value_text(v) {
                fields.insert(k.clone(), v);
            }
        }
        if let Some(e) = &exe {
            fields.insert("exe_name".to_string(), base_name(e));
            // Linux binaries carry no code signature. The closest real signal is the
            // package that installed the file and whether its bytes still match it.
            if let Some(v) = (self.pkg_lookup)(e) {
                fields.insert("exe_package".to_string(), v.package);
                fields.insert("exe_pkg_verified".to_string(), v.verified.to_string());
            }
            if self.procs.len() >= PROC_TABLE_CAP {
                self.procs.clear();
            }
            let old_ppid = self.procs.get(&ctx.pid).map(|p| p.ppid).unwrap_or(0);
            self.procs.insert(
                ctx.pid,
                ProcEntry {
                    exe: e.clone(),
                    ppid: ppid.unwrap_or(old_ppid),
                    user: ctx.user.clone(),
                },
            );
        }
        fields.insert("tags".to_string(), tags.join(","));
        ctx.fact(FactKind::Exec, exe, text("command_line"), fields)
    }

    /// The exe of `pid`: from its exec fact, else /proc once, then remembered.
    fn exe_of(&mut self, pid: u32) -> Option<String> {
        if let Some(p) = self.procs.get(&pid) {
            return Some(p.exe.clone());
        }
        let exe = (self.proc_exe)(pid)?;
        if self.procs.len() >= PROC_TABLE_CAP {
            self.procs.clear();
        }
        self.procs.insert(pid, ProcEntry { exe: exe.clone(), ppid: 0, user: None });
        Some(exe)
    }

    /// net_flow: connect() destinations and the bytes of send, sendto and sendmsg.
    /// recv bytes are not facts, because no playbook reads them.
    fn net_flow(&mut self, ctx: &Ctx, signal: &str, m: &Map<String, Value>) -> Vec<Fact> {
        let text = |k: &str| m.get(k).and_then(value_text).unwrap_or_default();
        let mut fields = HashMap::new();
        match signal {
            "network::connect" => {
                let ip = text("dst_addr");
                let port = text("dst_port");
                let scope = ip_scope(&ip);
                if self.seen_dsts.len() >= SEEN_DST_CAP {
                    self.seen_dsts.clear();
                }
                let new = self.seen_dsts.insert(ip.clone());
                let dst = Dst { ip, port, scope, new, sni: None };
                fields.insert("op".to_string(), "connect".to_string());
                fields.insert("family".to_string(), text("family"));
                put_dst(&mut fields, &dst);
                if self.last_dst.len() >= PROC_TABLE_CAP {
                    self.last_dst.clear();
                }
                self.last_dst.insert(ctx.pid, dst);
            }
            "network::tx_bytes" => {
                fields.insert("op".to_string(), "send".to_string());
                fields.insert("bytes".to_string(), text("bytes"));
                if let Some(dst) = self.last_dst.get(&ctx.pid) {
                    put_dst(&mut fields, dst);
                }
            }
            "network::tls_client_hello" => {
                let ip = text("dst_addr");
                let port = text("dst_port");
                let sni = Some(text("sni")).filter(|s| !s.is_empty());
                let scope = ip_scope(&ip);
                let new = !self.seen_dsts.contains(&ip);
                fields.insert("op".to_string(), "tls_hello".to_string());
                for k in ["alpn", "tls_version"] {
                    let v = text(k);
                    if !v.is_empty() {
                        fields.insert(k.to_string(), v);
                    }
                }
                // Later sends to the same destination carry the name too.
                if let Some(d) = self.last_dst.get_mut(&ctx.pid) {
                    if d.ip == ip && d.port == port {
                        d.sni = sni.clone();
                    }
                }
                put_dst(&mut fields, &Dst { ip, port, scope, new, sni });
            }
            _ => return Vec::new(),
        }
        let exe = self.exe_of(ctx.pid);
        if let Some(e) = &exe {
            fields.insert("exe_name".to_string(), base_name(e));
        }
        vec![ctx.fact(FactKind::NetFlow, exe, None, fields)]
    }

    /// tcp_state: aux_u32 holds the old state in the high 16 bits and the new one in the
    /// low 16. SYN_SENT runs in the connecting process, so it names the process. The
    /// ESTABLISHED that follows runs when the SYN-ACK arrives, often in another task's
    /// context, so it takes the process from the matching SYN_SENT instead.
    fn tcp_state(&mut self, ctx: &Ctx, m: &Map<String, Value>) -> Vec<Fact> {
        let text = |k: &str| m.get(k).and_then(value_text).unwrap_or_default();
        let num = |k: &str| m.get(k).and_then(value_i64).unwrap_or(0);
        let aux = num("aux_u32") as u32;
        let (old, new) = ((aux >> 16) as u16, (aux & 0xffff) as u16);
        let (lip, rip) = if text("fam") == "10" {
            (text("laddr6"), text("raddr6"))
        } else {
            (text("laddr4"), text("raddr4"))
        };
        let lport = num("lport") as u16;
        let rport = num("rport") as u16;
        let now = ctx.ts;
        match new {
            TCP_SYN_SENT => {
                if ctx.pid != 0 {
                    if self.syn_pending.len() >= FLOW_TABLE_CAP {
                        self.syn_pending.clear();
                    }
                    let q = self.syn_pending.entry((rip, rport)).or_default();
                    q.retain(|(_, t)| now.saturating_sub(*t) <= SYN_PENDING_SEC);
                    q.push_back((ctx.pid, now));
                }
                Vec::new()
            }
            TCP_ESTABLISHED => {
                let outbound = old == TCP_SYN_SENT;
                let pid = if outbound {
                    self.syn_pending
                        .get_mut(&(rip.clone(), rport))
                        .and_then(|q| q.pop_front())
                        .map(|(p, _)| p)
                        .unwrap_or(0)
                } else {
                    0
                };
                let exe = if pid != 0 { self.exe_of(pid) } else { None };
                let flow = Flow {
                    pid,
                    exe,
                    start: now,
                    last_report: now,
                    direction: if outbound { "outbound" } else { "inbound" },
                };
                if self.flows.len() >= FLOW_TABLE_CAP {
                    self.flows.clear();
                }
                let key = (lip, lport, rip, rport);
                let fact = net_fact(ctx, "established", &key, &flow, None);
                self.flows.insert(key, flow);
                vec![fact]
            }
            TCP_CLOSE => {
                let key = (lip, lport, rip, rport);
                match self.flows.remove(&key) {
                    Some(flow) => {
                        let dur = now.saturating_sub(flow.start);
                        vec![net_fact(ctx, "closed", &key, &flow, Some(dur))]
                    }
                    // A connection that never reached ESTABLISHED while the sensor ran.
                    None => Vec::new(),
                }
            }
            _ => Vec::new(),
        }
    }

    /// open_flow facts for TCP connections open FLOW_REPORT_SEC or longer, at most one
    /// per connection every FLOW_REPORT_SEC. The batch loop calls this every 5 s, so a
    /// long-lived connection shows up while it is still open.
    pub fn sweep_flows(&mut self, now: u64, host: &str) -> Vec<Fact> {
        let mut out = Vec::new();
        for (key, flow) in self.flows.iter_mut() {
            let age = now.saturating_sub(flow.start);
            if age >= FLOW_REPORT_SEC && now.saturating_sub(flow.last_report) >= FLOW_REPORT_SEC {
                flow.last_report = now;
                let ctx = Ctx {
                    ts: now,
                    host: host.to_string(),
                    pid: flow.pid,
                    user: None,
                    comm: String::new(),
                };
                out.push(net_fact(&ctx, "open_flow", key, flow, Some(age)));
            }
        }
        out
    }
}

// TCP states from include/net/tcp_states.h.
const TCP_ESTABLISHED: u16 = 1;
const TCP_SYN_SENT: u16 = 2;
const TCP_CLOSE: u16 = 7;

fn net_fact(ctx: &Ctx, op: &str, key: &FlowKey, flow: &Flow, duration: Option<u64>) -> Fact {
    let (lip, lport, rip, rport) = key;
    let mut fields = HashMap::new();
    fields.insert("op".to_string(), op.to_string());
    fields.insert("src_ip".to_string(), lip.clone());
    fields.insert("src_port".to_string(), lport.to_string());
    let scope = ip_scope(rip);
    fields.insert("dst_ip".to_string(), rip.clone());
    fields.insert("dst_port".to_string(), rport.to_string());
    fields.insert("dst_scope".to_string(), scope.to_string());
    fields.insert("dst_private".to_string(), (scope != "public").to_string());
    fields.insert("direction".to_string(), flow.direction.to_string());
    fields.insert("proto".to_string(), "tcp".to_string());
    if let Some(d) = duration {
        fields.insert("duration_sec".to_string(), d.to_string());
    }
    if let Some(e) = &flow.exe {
        fields.insert("exe_name".to_string(), base_name(e));
    }
    let mut f = ctx.fact(FactKind::Net, flow.exe.clone(), None, fields);
    // The pid is the attributed process, not the task the kernel ran the change in.
    f.fields.insert("pid".to_string(), flow.pid.to_string());
    f.fields.remove("comm");
    f
}

fn put_dst(fields: &mut HashMap<String, String>, dst: &Dst) {
    fields.insert("dst_ip".to_string(), dst.ip.clone());
    fields.insert("dst_port".to_string(), dst.port.clone());
    fields.insert("dst_scope".to_string(), dst.scope.to_string());
    fields.insert("dst_private".to_string(), (dst.scope != "public").to_string());
    fields.insert("dst_new".to_string(), dst.new.to_string());
    if let Some(sni) = &dst.sni {
        fields.insert("sni".to_string(), sni.clone());
    }
}

/// loopback, private (RFC 1918, carrier NAT 100.64/10, IPv6 unique local), link_local,
/// or public for every other address. Documentation ranges such as 203.0.113.0/24 count
/// as public, which the live tests use on a dummy interface.
pub fn ip_scope(ip: &str) -> &'static str {
    use std::net::IpAddr;
    match ip.parse::<IpAddr>() {
        Ok(IpAddr::V4(a)) => {
            let o = a.octets();
            if a.is_loopback() || a.is_unspecified() {
                "loopback"
            } else if a.is_private() || (o[0] == 100 && (o[1] & 0xc0) == 64) {
                "private"
            } else if a.is_link_local() {
                "link_local"
            } else {
                "public"
            }
        }
        Ok(IpAddr::V6(a)) => {
            if let Some(v4) = a.to_ipv4_mapped() {
                return ip_scope(&v4.to_string());
            }
            let seg = a.segments()[0];
            if a.is_loopback() || a.is_unspecified() {
                "loopback"
            } else if (seg & 0xfe00) == 0xfc00 {
                "private"
            } else if (seg & 0xffc0) == 0xfe80 {
                "link_local"
            } else {
                "public"
            }
        }
        Err(_) => "unknown",
    }
}

/// RuleHit from a rules.json alert (pb_adapter::emit_rule_hit) or a guard decision
/// (guard::guard_finding). ttp is the first technique id, ttps all of them.
fn rule_hit_fact(ctx: &Ctx, source: &str, m: &Map<String, Value>) -> Fact {
    let text = |k: &str| m.get(k).and_then(value_text).filter(|s| !s.is_empty());
    let mut fields = HashMap::new();
    let (rule_id, ttps) = if source == "edr_guard" {
        fields.insert("source".to_string(), "guard".to_string());
        for k in ["guard", "action", "target"] {
            if let Some(v) = text(k) {
                fields.insert(k.to_string(), v);
            }
        }
        (
            format!("guard:{}", text("guard").unwrap_or_default()),
            text("mitre").unwrap_or_default(),
        )
    } else {
        fields.insert("source".to_string(), "rules".to_string());
        if let Some(v) = text("rule_name") {
            fields.insert("rule_name".to_string(), v);
        }
        (text("rule_id").unwrap_or_default(), text("ttps").unwrap_or_default())
    };
    fields.insert("rule_id".to_string(), rule_id);
    fields.insert("ttp".to_string(), ttps.split(',').next().unwrap_or("").to_string());
    fields.insert("ttps".to_string(), ttps);
    if let Some(v) = text("severity") {
        fields.insert("severity".to_string(), v);
    }
    let exe = text("binary_path").or_else(|| text("exe"));
    if let Some(e) = &exe {
        fields.insert("exe_name".to_string(), base_name(e));
    }
    ctx.fact(FactKind::RuleHit, exe, text("cmd"), fields)
}

/// HostPerf from modules::host_perf: one sample for the host (scope host) and one for
/// each busy process (scope process). cpu_pct is 100 for one full core.
fn host_perf_fact(ctx: &Ctx, m: &Map<String, Value>) -> Fact {
    let text = |k: &str| m.get(k).and_then(value_text).filter(|s| !s.is_empty());
    let mut fields = HashMap::new();
    for k in ["scope", "cpu_pct", "cpu_busy_pct", "cpu_user_pct", "ncpu", "gpu", "gpu_util_pct"] {
        if let Some(v) = text(k) {
            fields.insert(k.to_string(), v);
        }
    }
    let exe = text("binary_path");
    if let Some(e) = &exe {
        fields.insert("exe_name".to_string(), base_name(e));
    }
    ctx.fact(FactKind::HostPerf, exe, None, fields)
}

/// Auth from modules::auth_log: one sshd login or failure.
fn auth_fact(ctx: &Ctx, m: &Map<String, Value>) -> Fact {
    let text = |k: &str| m.get(k).and_then(value_text).filter(|s| !s.is_empty());
    let mut fields = HashMap::new();
    for k in ["method", "result", "auth_kind", "src_ip", "src_port", "src_scope", "src_private", "src_new"] {
        if let Some(v) = text(k) {
            fields.insert(k.to_string(), v);
        }
    }
    let mut f = ctx.fact(FactKind::Auth, None, None, fields);
    f.user = text("user");
    f
}

impl Default for Mapper {
    fn default() -> Self {
        Self::new()
    }
}

struct Ctx {
    ts: u64,
    host: String,
    pid: u32,
    user: Option<String>,
    comm: String,
}

impl Ctx {
    fn fact(
        &self,
        kind: FactKind,
        exe: Option<String>,
        cmd: Option<String>,
        mut fields: HashMap<String, String>,
    ) -> Fact {
        fields.insert("pid".to_string(), self.pid.to_string());
        if !self.comm.is_empty() {
            fields.insert("comm".to_string(), self.comm.clone());
        }
        Fact {
            ts: self.ts,
            kind,
            host: self.host.clone(),
            user: self.user.clone(),
            exe,
            cmd,
            fields,
        }
    }
}

fn base_name(path: &str) -> String {
    path.rsplit('/').next().unwrap_or(path).to_string()
}

impl Mapper {
    /// root, the /etc/passwd name, or uid:N.
    fn user_name(&self, uid: i64) -> String {
        if uid == 0 {
            return "root".to_string();
        }
        u32::try_from(uid)
            .ok()
            .and_then(self.uid_names)
            .unwrap_or_else(|| format!("uid:{uid}"))
    }
}

/// Local hour (0 to 23) and weekday (Mon to Sun) of a Unix time, for playbooks that
/// leave out a maintenance window.
fn local_hour_dow(ts: u64) -> Option<(String, String)> {
    use chrono::{Datelike, TimeZone, Timelike};
    let t = chrono::Local.timestamp_opt(ts as i64, 0).single()?;
    Some((t.hour().to_string(), t.weekday().to_string()))
}

/// A path relative to the process cwd made absolute. Paths relative to another dirfd
/// cannot be resolved from the frame and stay as they are.
fn absolute(path: &str, cwd: &str) -> String {
    if path.starts_with('/') || path.is_empty() || cwd.is_empty() {
        return path.to_string();
    }
    format!("{}/{}", cwd.trim_end_matches('/'), path.trim_start_matches("./"))
}

/// ".enc" for /x/a.txt.enc, "" when the file name has no dot.
fn extension(path: &str) -> String {
    let name = path.rsplit('/').next().unwrap_or("");
    match name.rfind('.') {
        Some(i) if i > 0 => name[i..].to_ascii_lowercase(),
        _ => String::new(),
    }
}

fn errno_name(e: i64) -> String {
    match e {
        1 => "EPERM".into(),
        2 => "ENOENT".into(),
        13 => "EACCES".into(),
        17 => "EEXIST".into(),
        20 => "ENOTDIR".into(),
        21 => "EISDIR".into(),
        30 => "EROFS".into(),
        n => format!("E{n}"),
    }
}

fn value_text(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn value_i64(v: &Value) -> Option<i64> {
    match v {
        Value::Number(n) => n.as_i64().or_else(|| n.as_u64().map(|u| u as i64)),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

/// True when the mapper turns this output into a fact or needs it for its tables.
/// pb_adapter checks it before an output takes a place in the adapter queue, so frames
/// no playbook can use (fork, exit, the loader's W then X, recv byte counts, unknown
/// frames) never compete with the ones that matter.
pub fn mapper_wants(category: &str, signal: &str, data: &HashMap<String, String>) -> bool {
    let source = data.get("source").map(String::as_str).unwrap_or("");
    match source {
        "proc_lifecycle" => signal == "process::exec",
        "wx_exec" => matches!(signal, "memory::memfd_exec" | "memory::rwx_map"),
        "net_flow" => matches!(signal, "network::connect" | "network::tx_bytes" | "network::tls_client_hello"),
        "file_access_monitor" | "rule_engine" | "edr_guard" | "host_perf" | "auth_log" => true,
        "" => data.contains_key("syscall_id") && category != "debug",
        _ => false,
    }
}

/// The agent's mapper, shared by map_adapter_facts and sweep_flows.
fn shared_mapper() -> &'static Mutex<Mapper> {
    static MAPPER: OnceLock<Mutex<Mapper>> = OnceLock::new();
    MAPPER.get_or_init(|| Mutex::new(Mapper::new()))
}

/// Engine facts for a batch of adapter facts, with one lock of the shared mapper.
pub fn map_adapter_facts(batch: &[AdapterFact]) -> Vec<Fact> {
    match shared_mapper().lock() {
        Ok(mut m) => batch.iter().flat_map(|af| m.facts(af)).collect(),
        Err(_) => Vec::new(),
    }
}

/// open_flow facts from the shared mapper's flow table.
pub fn sweep_flows(now: u64) -> Vec<Fact> {
    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
    match shared_mapper().lock() {
        Ok(mut m) => m.sweep_flows(now, &host),
        Err(_) => Vec::new(),
    }
}

// ---- Completed playbooks ----

const COMPLETED_CAP: usize = 1000;

fn completed() -> &'static Mutex<VecDeque<EngineHit>> {
    static Q: OnceLock<Mutex<VecDeque<EngineHit>>> = OnceLock::new();
    Q.get_or_init(|| Mutex::new(VecDeque::new()))
}

/// A playbook completed: log it, show it under /playbooks/hits, and queue it for the
/// batch loop, which turns it into an incident.
pub fn publish(hit: EngineHit) {
    // One write for the whole block, so other threads' output cannot land inside it.
    let mut msg = format!(
        "playbook fired: {} ({}) on {} at {}\n",
        hit.playbook_id, hit.severity, hit.host, hit.ts
    );
    for line in hit.rationale.iter().skip(1) {
        msg.push_str(&format!("  {line}\n"));
    }
    {
        use std::io::Write;
        let _ = std::io::stderr().write_all(msg.as_bytes());
    }
    pb_introspect::push_hit(to_introspect_hit(&hit));
    if hit.ground_truth {
        use std::sync::atomic::Ordering::Relaxed;
        match write_ground_truth(&hit, ground_truth_path()) {
            Ok(()) => GROUND_TRUTH_WRITTEN.fetch_add(1, Relaxed),
            Err(e) => {
                log::warn!("ground truth write to {} failed: {e}", ground_truth_path().display());
                GROUND_TRUTH_FAILED.fetch_add(1, Relaxed)
            }
        };
    }
    if let Ok(mut q) = completed().lock() {
        if q.len() >= COMPLETED_CAP {
            q.pop_front();
        }
        q.push_back(hit);
    }
}

/// Hits published since the last call.
pub fn take_completed() -> Vec<EngineHit> {
    completed()
        .lock()
        .map(|mut q| q.drain(..).collect())
        .unwrap_or_default()
}

fn to_introspect_hit(h: &EngineHit) -> IntrospectHit {
    let mut tags = h.tags.clone();
    if let Some(ttp) = &h.primary_ttp {
        if !tags.contains(ttp) {
            tags.push(ttp.clone());
        }
    }
    IntrospectHit {
        id: h.id(),
        playbook_id: h.playbook_id.clone(),
        ts: h.ts,
        filled_slots: h.filled_slots.clone(),
        // The hit is complete, so the only slots left are optional ones.
        missing_slots: h.missing_optional.clone(),
        tags,
        rationale: h.rationale.clone(),
    }
}

/// The hit in the alert shape IncidentsMgr::ingest_alert_json reads: ts, severity, risk,
/// technique, findings labels and event.exe. The exe is from the first filled slot that
/// has one, so the incident groups under the tool that was run.
pub fn hit_alert_json(h: &EngineHit) -> Value {
    let exe = h
        .filled_slots
        .iter()
        .filter_map(|s| h.slots.get(s))
        .find_map(|s| s.fields.get("exe").cloned())
        .unwrap_or_else(|| "-".to_string());
    let mut labels: Vec<&String> = h.techniques.iter().collect();
    for t in h.tags.iter().filter(|t| t.starts_with('T')) {
        if !labels.contains(&t) {
            labels.push(t);
        }
    }
    let findings: Vec<Value> = labels.iter().map(|t| json!({ "label": t })).collect();
    let slots: Map<String, Value> = h
        .slots
        .iter()
        .map(|(id, s)| {
            (
                id.clone(),
                json!({ "ts": s.ts, "tags": s.tags, "fields": s.fields }),
            )
        })
        .collect();
    json!({
        "id": h.id(),
        "source": "playbook",
        "title": h.playbook_name,
        "ts": h.ts,
        "playbook_id": h.playbook_id,
        "playbook_name": h.playbook_name,
        "severity": h.severity,
        "risk": h.to_signal().score,
        "technique": h.primary_ttp,
        "findings": findings,
        "event": { "exe": exe, "host": h.host },
        "rationale": h.rationale,
        "narrative": h.narrative,
        "slots": slots,
        "context": context_json(h),
    })
}

fn context_json(h: &EngineHit) -> Value {
    Value::Object(
        h.context
            .iter()
            .map(|(k, v)| (k.clone(), Value::String(v.clone())))
            .collect(),
    )
}

// ---- Ground truth ----

/// Default ground truth file, relative to the agent's working directory like the
/// other logs. EDR_GROUND_TRUTH_PATH moves it.
pub const GROUND_TRUTH_DEFAULT: &str = "logs/ground_truth.jsonl";
static GROUND_TRUTH_WRITTEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static GROUND_TRUTH_FAILED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn ground_truth_path() -> &'static std::path::Path {
    static P: OnceLock<std::path::PathBuf> = OnceLock::new();
    P.get_or_init(|| {
        std::env::var("EDR_GROUND_TRUTH_PATH")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| GROUND_TRUTH_DEFAULT.to_string())
            .into()
    })
}

/// (written, failed) ground truth lines since start.
pub fn ground_truth_stats() -> (u64, u64) {
    use std::sync::atomic::Ordering::Relaxed;
    (GROUND_TRUTH_WRITTEN.load(Relaxed), GROUND_TRUTH_FAILED.load(Relaxed))
}

/// One labelled line for a hit: the playbook, its context and, per slot, the time and
/// the process and file that matched. A model trainer joins these to telemetry by
/// host, pid and time. Command lines are already redacted where they were read.
pub fn ground_truth_record(h: &EngineHit) -> Value {
    let mut slots = Map::new();
    let mut ids: Vec<&String> = h.slots.keys().collect();
    ids.sort();
    for id in ids {
        let s = &h.slots[id];
        let mut o = Map::new();
        o.insert("ts".into(), json!(s.ts));
        for k in ["pid", "exe", "cmd", "path", "dst_ip", "dst_port", "user"] {
            if let Some(v) = s.fields.get(k) {
                o.insert(k.into(), json!(v));
            }
        }
        slots.insert(id.clone(), Value::Object(o));
    }
    json!({
        "id": h.id(),
        "ts": h.ts,
        "label": "attack",
        "source": "playbook",
        "playbook_id": h.playbook_id,
        "severity": h.severity,
        "techniques": h.techniques,
        "host": h.host,
        "context": context_json(h),
        "slots": slots,
    })
}

/// Append the hit's ground truth line to `path`. The file is created 0600.
pub fn write_ground_truth(h: &EngineHit, path: &std::path::Path) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(dir)?;
    }
    let mut line = serde_json::to_string(&ground_truth_record(h))?;
    line.push('\n');
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)?
        .write_all(line.as_bytes())
}

#[cfg(test)]
mod tests;

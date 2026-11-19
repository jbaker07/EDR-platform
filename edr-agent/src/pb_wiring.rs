// src/pb_wiring.rs
// Adapter facts -> playbook engine facts, and completed playbooks -> hits and incidents.
//
// Facts come from pb_adapter, which sees each decoder's full data map. Sources and the
// facts they become:
//   proc_lifecycle exec                       Exec (exe and argv from /proc at decode, else
//                                             the filename and argv copied in the kernel)
//   wx_exec memfd_exec, rwx_map               Exec with tag MEMFD_EXEC or RWX_MAP
//   file_access_monitor openat                FileIO op=open
//   raw_syscalls_catcher openat exit          FileIO op=write, open or open_fail plus errno
//   raw_syscalls_catcher renameat(2), unlinkat FileIO op=rename plus ext_new, op=delete
//   raw_syscalls_catcher write                FileIO op=write plus bytes, path from the open
//   net_flow connect, tcp_state               NetConn (daddr, dport)
// Everything else gives no fact. No loaded playbook reads fork, exit, DNS, IPC or rules
// findings. raw_syscalls_catcher's execve frame is skipped: it fires at syscall entry,
// when /proc still shows the caller's argv, so it would pair the new exe with old args.
// proc_lifecycle's exec fires after the exec and is the exec source.

use std::collections::{HashMap, VecDeque};
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

pub struct Mapper {
    fds: HashMap<(u32, i64), String>,
    mounts: Box<dyn MountSource>,
    skip_pid: Option<u32>,
}

impl Mapper {
    /// The agent's mapper: real mount tables, and the agent's own events skipped so its
    /// log writes never count toward a burst.
    pub fn new() -> Self {
        Self {
            fds: HashMap::new(),
            mounts: Box::new(ProcMounts::default()),
            skip_pid: Some(std::process::id()),
        }
    }

    pub fn with_mounts(mounts: Box<dyn MountSource>) -> Self {
        Self {
            fds: HashMap::new(),
            mounts,
            skip_pid: None,
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
        let ctx = Ctx {
            ts: af.ts.max(0) as u64,
            host: af.host.clone(),
            pid,
            user: num("uid").map(user_name),
            comm: text("comm"),
        };
        let source = text("source");
        let signal = text("signal");

        match source.as_str() {
            "proc_lifecycle" if signal == "process::exec" => {
                let tags = vec![signal.clone()];
                vec![exec_fact(&ctx, num("ppid"), m, tags)]
            }
            "wx_exec" => {
                let marker = match signal.as_str() {
                    "memory::memfd_exec" => "MEMFD_EXEC",
                    "memory::rwx_map" => "RWX_MAP",
                    // w_then_x is the dynamic loader on every process start.
                    _ => return Vec::new(),
                };
                let tags = vec![signal.clone(), marker.to_string()];
                vec![exec_fact(&ctx, None, m, tags)]
            }
            "file_access_monitor" => {
                let path = absolute(&text("path"), &text("cwd"));
                vec![self.file_fact(&ctx, "open", &path, Vec::new())]
            }
            "net_flow" if signal == "network::connect" => {
                vec![net_fact(&ctx, &text("dst_addr"), &text("dst_port"), "connect")]
            }
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
            EVT_TCP_STATE => vec![net_fact(ctx, &text("raddr4"), &text("rport"), "tcp_state")],
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
        ctx.fact(FactKind::FileIO, None, None, fields)
    }
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

fn exec_fact(ctx: &Ctx, ppid: Option<i64>, m: &Map<String, Value>, tags: Vec<String>) -> Fact {
    let text = |k: &str| m.get(k).and_then(value_text).filter(|s| !s.is_empty());
    let mut fields = HashMap::new();
    if let Some(ppid) = ppid {
        fields.insert("ppid".to_string(), ppid.to_string());
    }
    fields.insert("tags".to_string(), tags.join(","));
    ctx.fact(FactKind::Exec, text("binary_path"), text("command_line"), fields)
}

fn net_fact(ctx: &Ctx, daddr: &str, dport: &str, op: &str) -> Fact {
    let mut fields = HashMap::new();
    fields.insert("op".to_string(), op.to_string());
    fields.insert("daddr".to_string(), daddr.to_string());
    fields.insert("dport".to_string(), dport.to_string());
    fields.insert("proto".to_string(), "tcp".to_string());
    ctx.fact(FactKind::NetConn, None, None, fields)
}

fn user_name(uid: i64) -> String {
    if uid == 0 {
        "root".to_string()
    } else {
        format!("uid:{uid}")
    }
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

/// The agent's mapper, shared by every caller of map_adapter_fact.
pub fn map_adapter_fact(af: &AdapterFact) -> Vec<Fact> {
    static MAPPER: OnceLock<Mutex<Mapper>> = OnceLock::new();
    match MAPPER.get_or_init(|| Mutex::new(Mapper::new())).lock() {
        Ok(mut m) => m.facts(af),
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
        id: format!("pb:{}:{}:{}", h.playbook_id, h.host, h.ts),
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
    let findings: Vec<Value> = h
        .tags
        .iter()
        .filter(|t| t.starts_with('T'))
        .map(|t| json!({ "label": t }))
        .collect();
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
        "id": format!("pb:{}:{}:{}", h.playbook_id, h.host, h.ts),
        "source": "playbook",
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
    })
}

#[cfg(test)]
mod tests;

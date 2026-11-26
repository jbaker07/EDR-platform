//! Runtime shields: prevention that changes while the agent runs, without new
//! kernel code. src/ebpf/edr_shield.bpf.c holds four general BPF LSM programs,
//! loaded once. A shield is one row in one of their maps:
//!
//! - exec_deny: refuse execve of one file, named by device and inode.
//! - file_deny: refuse opening one file, named the same way.
//! - connect_deny: refuse connect() (and sendto() on an unconnected socket) to an
//!   address or network, on one port or on every port.
//!
//! Every shield comes from a document signed with the policy key (Ed25519 over the
//! exact bytes, like policy.json), or from a playbook proposal that the signed
//! policy lets the agent apply by itself. Each document names an id, a serial,
//! the kind, the match, an optional cgroup v2 scope, the mode it may reach
//! (monitor or enforce), not_before, expires_at and a reason or incident id.
//!
//! Refused: a bad signature, a serial at or under the last one seen for that id
//! (replay), a lifetime over the policy's maximum, a target on the never-block
//! list (init, systemd, sshd, the agent, the dynamic loader and libc, the
//! firewall tools, loopback, the backend, and what the policy adds), and going
//! over the policy's maximum number of active shields.
//!
//! Monitor first. A new shield always starts in monitor mode: the kernel lets the
//! action through and reports would_block. An operator promotes it to enforce
//! (POST /shields/{id}/promote), or the policy promotes it after a trial period
//! for the kinds it lists. A document signed for monitor never enforces.
//!
//! Every row carries its deadline in CLOCK_BOOTTIME nanoseconds and the kernel
//! checks it on every match, so a shield ends on time even if the agent is
//! stopped or dead. The agent also sweeps expired rows once a second.

pub mod abi;
#[cfg(target_os = "linux")]
pub mod loader;
pub mod propose;

use std::collections::{BTreeMap, HashMap};
use std::fs::OpenOptions;
use std::io::Write;
use std::net::IpAddr;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::config::{write_in_place, NeverBlockPolicy, ShieldPolicy};
use crate::guard::abi::ExeId;
use crate::response::{Net, Outcome};
use abi::{ShieldEvent, ShieldRule};

// ---- Kinds, modes and documents ----

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShieldKind {
    ExecDeny,
    FileDeny,
    ConnectDeny,
}

impl ShieldKind {
    pub const ALL: [ShieldKind; 3] = [Self::ExecDeny, Self::FileDeny, Self::ConnectDeny];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExecDeny => "exec_deny",
            Self::FileDeny => "file_deny",
            Self::ConnectDeny => "connect_deny",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.as_str() == s)
    }

    /// KIND_* in the C file.
    pub fn raw(self) -> u8 {
        match self {
            Self::ExecDeny => abi::KIND_EXEC,
            Self::FileDeny => abi::KIND_FILE,
            Self::ConnectDeny => abi::KIND_CONNECT,
        }
    }

    pub fn from_raw(k: u8) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.raw() == k)
    }

    /// The LSM hooks that enforce it.
    pub fn hook(self) -> &'static str {
        match self {
            Self::ExecDeny => "bprm_check_security",
            Self::FileDeny => "file_open",
            Self::ConnectDeny => "socket_connect, socket_sendmsg",
        }
    }

    /// The shield_cfg slot that counts its rows.
    pub fn rows_slot(self) -> u32 {
        match self {
            Self::ExecDeny => abi::SCFG_EXEC_ROWS,
            Self::FileDeny => abi::SCFG_FILE_ROWS,
            Self::ConnectDeny => abi::SCFG_CONN_ROWS,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShieldMode {
    Monitor,
    Enforce,
}

impl ShieldMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Monitor => "monitor",
            Self::Enforce => "enforce",
        }
    }

    fn raw(self) -> u8 {
        match self {
            Self::Monitor => abi::MODE_MONITOR,
            Self::Enforce => abi::MODE_ENFORCE,
        }
    }
}

/// What a shield names. exec_deny and file_deny take dev and ino (st_dev and
/// st_ino as stat(2) gives them, `stat -c '%d %i' FILE`) and an optional path,
/// kept only as a label. connect_deny takes cidr (an address or a network) and
/// an optional port.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShieldMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dev: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ino: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

/// A cgroup v2 directory. The shield then acts only on tasks in it and below it.
/// No cgroup means the whole host.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShieldScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cgroup: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShieldDoc {
    pub id: String,
    pub serial: u64,
    pub kind: ShieldKind,
    #[serde(rename = "match")]
    pub target: ShieldMatch,
    #[serde(default)]
    pub scope: ShieldScope,
    /// The highest mode the shield may reach. Every shield starts in monitor.
    pub mode: ShieldMode,
    /// Unix seconds.
    pub not_before: u64,
    /// Unix seconds.
    pub expires_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incident_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DocFile {
    shield: ShieldDoc,
}

pub fn parse_document(text: &str) -> Result<ShieldDoc, String> {
    serde_json::from_str::<DocFile>(text)
        .map(|f| f.shield)
        .map_err(|e| format!("not a shield document: {e}"))
}

/// The text an operator signs: {"shield": {...}}.
pub fn document_text(doc: &ShieldDoc) -> String {
    serde_json::to_string_pretty(&DocFile { shield: doc.clone() }).unwrap_or_default() + "\n"
}

// ---- Clock ----

pub trait Clock: Send + Sync {
    fn unix(&self) -> u64;
    /// CLOCK_BOOTTIME, the clock bpf_ktime_get_boot_ns() reads.
    fn boot_ns(&self) -> u64;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn unix(&self) -> u64 {
        crate::utils::time::now_ts()
    }

    fn boot_ns(&self) -> u64 {
        boot_ns()
    }
}

pub fn boot_ns() -> u64 {
    #[cfg(target_os = "linux")]
    let clock = libc::CLOCK_BOOTTIME;
    // Only Linux has CLOCK_BOOTTIME, and only Linux enforces shields.
    #[cfg(not(target_os = "linux"))]
    let clock = libc::CLOCK_MONOTONIC;
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: ts is a live local and clock is a valid clock id.
    unsafe { libc::clock_gettime(clock, &mut ts) };
    ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
}

/// A Unix time as a CLOCK_BOOTTIME deadline, measured from now.
fn to_boot_ns(clock: &dyn Clock, unix: u64) -> u64 {
    let now_unix = clock.unix();
    let now_boot = clock.boot_ns();
    if unix <= now_unix {
        now_boot.saturating_sub((now_unix - unix).saturating_mul(1_000_000_000))
    } else {
        now_boot.saturating_add((unix - now_unix).saturating_mul(1_000_000_000))
    }
}

// ---- The kernel maps ----

/// The shield maps, as the manager uses them. src/shield/loader.rs has the real
/// one, the tests an in-memory one.
pub trait RuleMaps: Send + Sync {
    fn put(&self, kind: ShieldKind, key: &[u8], rule: &ShieldRule) -> Result<(), String>;
    fn get(&self, kind: ShieldKind, key: &[u8]) -> Option<ShieldRule>;
    fn delete(&self, kind: ShieldKind, key: &[u8]) -> Result<(), String>;
    fn keys(&self, kind: ShieldKind) -> Vec<Vec<u8>>;
    fn set_slot(&self, slot: u32, value: u64) -> Result<(), String>;
    /// The hits of shield number `num`, summed over every CPU.
    fn hits(&self, num: u32) -> u64;
    /// Zeroes shield number `num`'s counters, before the number is given out again.
    fn reset_hits(&self, num: u32) -> Result<(), String>;
}

/// In memory maps for tests and for a host without the BPF LSM, where shields
/// are kept and listed but nothing enforces them.
#[derive(Default)]
pub struct MemMaps {
    pub rows: Mutex<HashMap<(ShieldKind, Vec<u8>), ShieldRule>>,
    pub slots: Mutex<HashMap<u32, u64>>,
    pub hits: Mutex<HashMap<u32, u64>>,
}

impl RuleMaps for MemMaps {
    fn put(&self, kind: ShieldKind, key: &[u8], rule: &ShieldRule) -> Result<(), String> {
        self.rows.lock().unwrap().insert((kind, key.to_vec()), *rule);
        Ok(())
    }
    fn get(&self, kind: ShieldKind, key: &[u8]) -> Option<ShieldRule> {
        self.rows.lock().unwrap().get(&(kind, key.to_vec())).copied()
    }
    fn delete(&self, kind: ShieldKind, key: &[u8]) -> Result<(), String> {
        self.rows.lock().unwrap().remove(&(kind, key.to_vec()));
        Ok(())
    }
    fn keys(&self, kind: ShieldKind) -> Vec<Vec<u8>> {
        self.rows
            .lock()
            .unwrap()
            .keys()
            .filter(|(k, _)| *k == kind)
            .map(|(_, key)| key.clone())
            .collect()
    }
    fn set_slot(&self, slot: u32, value: u64) -> Result<(), String> {
        self.slots.lock().unwrap().insert(slot, value);
        Ok(())
    }
    fn hits(&self, num: u32) -> u64 {
        self.hits.lock().unwrap().get(&num).copied().unwrap_or(0)
    }
    fn reset_hits(&self, num: u32) -> Result<(), String> {
        self.hits.lock().unwrap().remove(&num);
        Ok(())
    }
}

// ---- Targets and the never-block list ----

#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    File(ExeId),
    Net { net: Net, port: u16 },
}

impl Target {
    pub fn key(&self) -> Vec<u8> {
        match self {
            Self::File(id) => abi::file_key(id).to_vec(),
            Self::Net { net, port } => abi::conn_key(&net.addr, net.prefix, *port).to_vec(),
        }
    }
}

fn resolve_target(doc: &ShieldDoc) -> Result<Target, String> {
    let m = &doc.target;
    match doc.kind {
        ShieldKind::ExecDeny | ShieldKind::FileDeny => {
            if m.cidr.is_some() || m.port.is_some() {
                return Err(format!("{} matches a file by dev and ino, not cidr or port", doc.kind.as_str()));
            }
            match (m.dev, m.ino) {
                (Some(dev), Some(ino)) if ino != 0 => Ok(Target::File(ExeId::from_stat(dev, ino))),
                _ => Err(format!("{} needs match.dev and match.ino (stat -c '%d %i' FILE)", doc.kind.as_str())),
            }
        }
        ShieldKind::ConnectDeny => {
            if m.dev.is_some() || m.ino.is_some() || m.path.is_some() {
                return Err("connect_deny matches cidr and port, not a file".into());
            }
            let cidr = m.cidr.as_deref().ok_or("connect_deny needs match.cidr")?;
            let mut net = Net::parse(cidr)?;
            if net.prefix == 0 {
                return Err("a connect_deny for every address is isolate_host, not a shield".into());
            }
            net = masked(&net);
            Ok(Target::Net { net, port: m.port.unwrap_or(0) })
        }
    }
}

/// The network address with host bits cleared, so 203.0.113.7/24 is 203.0.113.0/24.
fn masked(n: &Net) -> Net {
    let addr = match n.addr {
        IpAddr::V4(a) => {
            let m = if n.prefix == 0 { 0 } else { u32::MAX << (32 - n.prefix as u32) };
            IpAddr::V4((u32::from(a) & m).into())
        }
        IpAddr::V6(a) => {
            let m = if n.prefix == 0 { 0 } else { u128::MAX << (128 - n.prefix as u32) };
            IpAddr::V6((u128::from(a) & m).into())
        }
    };
    Net { addr, prefix: n.prefix }
}

fn net_text(n: &Net) -> String {
    let max = if n.addr.is_ipv4() { 32 } else { 128 };
    if n.prefix == max {
        n.addr.to_string()
    } else {
        format!("{}/{}", n.addr, n.prefix)
    }
}

fn overlaps(a: &Net, b: &Net) -> bool {
    a.contains(&b.addr) || b.contains(&a.addr)
}

/// Programs and files that keep the host and the agent working. A shield may not
/// name any of them. Found by path, compared by inode.
pub const NEVER_BLOCK_PATHS: &[&str] = &[
    "/sbin/init",
    "/usr/lib/systemd/systemd",
    "/lib/systemd/systemd",
    "/usr/sbin/sshd",
    "/usr/bin/sshd",
    "/usr/sbin/nft",
    "/usr/sbin/iptables",
    "/usr/sbin/ip6tables",
    "/usr/sbin/xtables-nft-multi",
    "/usr/sbin/xtables-legacy-multi",
];

/// Where the dynamic loader and libc live. Denying either would stop every
/// dynamically linked program on the host.
const LIB_DIRS: &[&str] = &["/lib", "/lib64", "/usr/lib", "/usr/lib64"];

fn loader_and_libc() -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut dirs: Vec<PathBuf> = LIB_DIRS.iter().map(PathBuf::from).collect();
    for d in LIB_DIRS {
        if let Ok(rd) = std::fs::read_dir(d) {
            for e in rd.flatten() {
                let name = e.file_name().to_string_lossy().into_owned();
                if name.ends_with("-linux-gnu") || name.ends_with("-linux-musl") {
                    dirs.push(e.path());
                }
            }
        }
    }
    for d in dirs {
        let Ok(rd) = std::fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            if name.starts_with("ld-linux") || name.starts_with("ld-musl") || name == "libc.so.6" || name.starts_with("libc.musl") {
                out.push(e.path());
            }
        }
    }
    out
}

#[derive(Clone, Debug, Default)]
pub struct NeverBlock {
    pub files: Vec<(String, ExeId)>,
    pub nets: Vec<(String, Net)>,
}

impl NeverBlock {
    /// The built in list, the policy's additions, `own_files` (the agent's binary,
    /// key and state) and `keep_ips` (the backend).
    pub fn build(policy: &NeverBlockPolicy, own_files: &[PathBuf], keep_ips: &[IpAddr]) -> Self {
        let mut paths: Vec<PathBuf> = vec![PathBuf::from("/proc/1/exe"), PathBuf::from("/proc/self/exe")];
        paths.extend(NEVER_BLOCK_PATHS.iter().map(PathBuf::from));
        paths.extend(loader_and_libc());
        paths.extend(own_files.iter().cloned());
        paths.extend(policy.paths.iter().map(PathBuf::from));
        let mut files: Vec<(String, ExeId)> = Vec::new();
        for p in paths {
            if let Ok(m) = std::fs::metadata(&p) {
                let id = ExeId::from_stat(m.dev(), m.ino());
                if !files.iter().any(|(_, seen)| *seen == id) {
                    let label = match p.to_str() {
                        Some("/proc/1/exe") => "init (pid 1)".to_string(),
                        Some("/proc/self/exe") => "the agent".to_string(),
                        _ => p.display().to_string(),
                    };
                    files.push((label, id));
                }
            }
        }
        let mut nets = vec![
            ("loopback".to_string(), Net::parse("127.0.0.0/8").unwrap()),
            ("loopback".to_string(), Net::parse("::1").unwrap()),
        ];
        for ip in keep_ips {
            nets.push((format!("the backend {ip}"), Net::parse(&ip.to_string()).unwrap()));
        }
        for c in &policy.cidrs {
            if let Ok(n) = Net::parse(c) {
                nets.push((format!("policy never_block {c}"), n));
            }
        }
        Self { files, nets }
    }

    /// Why a shield may not name `target`, or None.
    pub fn refusal(&self, target: &Target) -> Option<String> {
        match target {
            Target::File(id) => self
                .files
                .iter()
                .find(|(_, f)| f == id)
                .map(|(label, _)| format!("the match is {label}, which is on the never-block list")),
            Target::Net { net, .. } => self
                .nets
                .iter()
                .find(|(_, n)| overlaps(n, net))
                .map(|(label, _)| format!("{} covers {label}, which is on the never-block list", net_text(net))),
        }
    }
}

// ---- The manager ----

/// One active shield as the agent keeps it, in memory and in state/shields.json.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Active {
    pub doc: ShieldDoc,
    pub num_id: u32,
    /// "signed", or "auto:<playbook id>" for a proposal the policy applied.
    pub source: String,
    #[serde(default)]
    pub document: Option<String>,
    #[serde(default)]
    pub signature: Option<String>,
    pub created: u64,
    pub mode_now: ShieldMode,
    #[serde(default)]
    pub promoted_at: Option<u64>,
    #[serde(default)]
    pub auto_promote_at: Option<u64>,
    /// The doc's expires_at, or earlier when the policy capped it.
    pub expires_at: u64,
    pub cgroup_id: u64,
    pub key_hex: String,
}

#[derive(Default, Serialize, Deserialize)]
struct Saved {
    next_id: u32,
    active: Vec<Active>,
}

/// The same file written from borrowed shields, so a snapshot does not clone every
/// active shield first (with 30,000 shields that copy was most of a snapshot's memory).
#[derive(Serialize)]
struct SavedRef<'a> {
    next_id: u32,
    active: Vec<&'a Active>,
}

struct Inner {
    policy: ShieldPolicy,
    never: NeverBlock,
    active: BTreeMap<String, Active>,
    serials: BTreeMap<String, u64>,
    next_id: u32,
    rows: HashMap<ShieldKind, u64>,
    /// Lines in the journal since the last snapshot.
    journal_lines: usize,
}

/// One change, as a journal line. Replayed over the snapshot at startup.
#[derive(Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum Change {
    Put { active: Box<Active> },
    Del { id: String },
}

/// The journal is folded into the snapshot once it holds this many lines.
const JOURNAL_COMPACT: usize = 2000;

pub struct ShieldManager {
    inner: Mutex<Inner>,
    maps: Box<dyn RuleMaps>,
    clock: Box<dyn Clock>,
    pubkey_pem: Mutex<Vec<u8>>,
    state_dir: PathBuf,
    audit_path: PathBuf,
    /// "kernel" when the BPF programs run, "none" when shields are only kept.
    pub enforcement: &'static str,
}

const MAX_REASON: usize = 500;

fn outcome(status: u16, body: Value) -> Outcome {
    Outcome { status, body }
}

fn refused(status: u16, code: &str, msg: impl Into<String>) -> Outcome {
    outcome(status, json!({ "ok": false, "refused": code, "error": msg.into() }))
}

/// The smallest shield number no active shield holds, from 1 to STATS_SLOTS - 1.
/// The number indexes the per CPU hit counters.
fn free_number(inner: &Inner) -> Option<u32> {
    let used: std::collections::HashSet<u32> = inner.active.values().map(|a| a.num_id).collect();
    (1..abi::STATS_SLOTS).find(|n| !used.contains(n))
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | ':'))
}

/// The cgroup id of a cgroup v2 directory: its inode number.
pub fn cgroup_id(dir: &Path) -> Result<u64, String> {
    let m = std::fs::metadata(dir).map_err(|e| format!("scope cgroup {}: {e}", dir.display()))?;
    if !m.is_dir() {
        return Err(format!("scope cgroup {} is not a directory", dir.display()));
    }
    if !dir.join("cgroup.procs").exists() {
        return Err(format!("scope {} is not a cgroup v2 directory", dir.display()));
    }
    Ok(m.ino())
}

impl ShieldManager {
    pub fn new(
        policy: ShieldPolicy,
        never: NeverBlock,
        maps: Box<dyn RuleMaps>,
        clock: Box<dyn Clock>,
        pubkey_pem: Vec<u8>,
        state_dir: &Path,
        audit_path: &Path,
        enforcement: &'static str,
    ) -> Self {
        Self {
            inner: Mutex::new(Inner {
                policy,
                never,
                active: BTreeMap::new(),
                serials: BTreeMap::new(),
                next_id: 1,
                rows: HashMap::new(),
                journal_lines: 0,
            }),
            maps,
            clock,
            pubkey_pem: Mutex::new(pubkey_pem),
            state_dir: state_dir.to_path_buf(),
            audit_path: audit_path.to_path_buf(),
            enforcement,
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn shields_path(&self) -> PathBuf {
        self.state_dir.join("shields.json")
    }

    pub fn serials_path(&self) -> PathBuf {
        self.state_dir.join("shield_serials.json")
    }

    pub fn journal_path(&self) -> PathBuf {
        self.state_dir.join("shields.journal")
    }

    /// The state files the guard write protects.
    pub fn state_files(&self) -> Vec<PathBuf> {
        vec![self.shields_path(), self.serials_path(), self.journal_path()]
    }

    /// One JSON line per change or refusal, owner only, never through a symlink.
    fn audit(&self, action: &str, id: &str, status: u16, detail: Value) {
        let line = json!({ "ts": self.clock.unix(), "action": action, "id": id, "status": status, "detail": detail });
        if let Some(dir) = self.audit_path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        use std::os::unix::fs::OpenOptionsExt;
        match OpenOptions::new()
            .create(true)
            .append(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(&self.audit_path)
        {
            Ok(mut f) => {
                let _ = writeln!(f, "{line}");
            }
            Err(e) => log::warn!("shield audit log {}: {e}", self.audit_path.display()),
        }
        log::info!("shield {action} {id}: {status}");
    }

    /// Writes the whole state (every active shield and every serial seen) and
    /// empties the journal. Each file is rewritten in place, so it keeps its inode
    /// and the guard's write protection.
    fn snapshot(&self, inner: &mut Inner) {
        let _ = std::fs::create_dir_all(&self.state_dir);
        let saved = SavedRef { next_id: inner.next_id, active: inner.active.values().collect() };
        let text = serde_json::to_vec(&saved).unwrap_or_default();
        if let Err(e) = write_in_place(&self.shields_path(), &text) {
            log::warn!("shields: write {}: {e}", self.shields_path().display());
            return;
        }
        let serials = serde_json::to_vec(&inner.serials).unwrap_or_default();
        if let Err(e) = write_in_place(&self.serials_path(), &serials) {
            log::warn!("shields: write {}: {e}", self.serials_path().display());
            return;
        }
        if write_in_place(&self.journal_path(), b"").is_ok() {
            inner.journal_lines = 0;
        }
    }

    /// One change appended and flushed to disk before the API answers, so an
    /// applied shield and its serial survive a crash. Costs one line, not a
    /// rewrite of every shield.
    fn journal(&self, inner: &mut Inner, change: Change) {
        use std::os::unix::fs::OpenOptionsExt;
        let _ = std::fs::create_dir_all(&self.state_dir);
        let line = serde_json::to_string(&change).unwrap_or_default();
        let written = OpenOptions::new()
            .create(true)
            .append(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(self.journal_path())
            .and_then(|mut f| {
                writeln!(f, "{line}")?;
                f.sync_data()
            });
        match written {
            Ok(()) => inner.journal_lines += 1,
            Err(e) => log::warn!("shields: journal {}: {e}", self.journal_path().display()),
        }
        if inner.journal_lines >= JOURNAL_COMPACT {
            self.snapshot(inner);
        }
    }

    fn put(&self, inner: &mut Inner, a: &Active) {
        self.journal(inner, Change::Put { active: Box::new(a.clone()) });
    }

    fn del(&self, inner: &mut Inner, id: &str) {
        self.journal(inner, Change::Del { id: id.to_string() });
    }

    fn set_rows(&self, inner: &mut Inner, kind: ShieldKind) {
        let n = inner.active.values().filter(|a| a.doc.kind == kind).count() as u64;
        inner.rows.insert(kind, n);
        if let Err(e) = self.maps.set_slot(kind.rows_slot(), n) {
            log::warn!("shields: row count for {}: {e}", kind.as_str());
        }
    }

    fn rule_for(&self, a: &Active) -> ShieldRule {
        let not_before_ns = if a.doc.not_before <= self.clock.unix() { 0 } else { to_boot_ns(&*self.clock, a.doc.not_before) };
        ShieldRule {
            expires_ns: to_boot_ns(&*self.clock, a.expires_at),
            not_before_ns,
            cgroup_id: a.cgroup_id,
            id: a.num_id,
            mode: a.mode_now.raw(),
            kind: a.doc.kind.raw(),
        }
    }

    fn key_of(a: &Active) -> Vec<u8> {
        hex::decode(&a.key_hex).unwrap_or_default()
    }

    /// The kernel's row for a shield. An LPM trie lookup answers with the longest
    /// matching prefix, which may be another shield's row, so the id must match.
    fn row(&self, a: &Active) -> Option<ShieldRule> {
        self.maps.get(a.doc.kind, &Self::key_of(a)).filter(|r| r.id == a.num_id)
    }

    /// Restores the shields in state/shields.json at startup. A row the kernel
    /// still has (the maps are pinned) keeps its deadline and hit count. A missing
    /// row is written again. A row no saved shield owns is removed.
    pub fn restore(&self) -> Vec<String> {
        let mut notes = Vec::new();
        let mut inner = self.lock();
        if let Ok(text) = std::fs::read(self.serials_path()) {
            if let Ok(s) = serde_json::from_slice::<BTreeMap<String, u64>>(&text) {
                inner.serials = s;
            }
        }
        let saved: Saved = std::fs::read(self.shields_path())
            .ok()
            .and_then(|t| serde_json::from_slice(&t).ok())
            .unwrap_or_default();
        inner.next_id = saved.next_id.max(1);
        // The snapshot, then every change journaled after it.
        let mut state: BTreeMap<String, Active> = saved.active.into_iter().map(|a| (a.doc.id.clone(), a)).collect();
        let journal = std::fs::read_to_string(self.journal_path()).unwrap_or_default();
        let mut replayed = 0;
        for line in journal.lines().filter(|l| !l.trim().is_empty()) {
            match serde_json::from_str::<Change>(line) {
                Ok(Change::Put { active }) => {
                    let s = inner.serials.entry(active.doc.id.clone()).or_insert(0);
                    *s = (*s).max(active.doc.serial);
                    state.insert(active.doc.id.clone(), *active);
                }
                Ok(Change::Del { id }) => {
                    state.remove(&id);
                }
                Err(_) => {
                    notes.push("skipped a journal line that does not parse (a torn last write)".into());
                    continue;
                }
            }
            replayed += 1;
        }
        if replayed > 0 {
            notes.push(format!("replayed {replayed} journal lines"));
        }
        let now = self.clock.unix();
        for a in state.into_values() {
            if a.expires_at <= now {
                notes.push(format!("{} expired while the agent was down", a.doc.id));
                let _ = self.maps.delete(a.doc.kind, &Self::key_of(&a));
                continue;
            }
            let key = Self::key_of(&a);
            if self.row(&a).is_none() {
                if let Err(e) = self.maps.put(a.doc.kind, &key, &self.rule_for(&a)) {
                    notes.push(format!("{} not restored: {e}", a.doc.id));
                    continue;
                }
                notes.push(format!("{} written again", a.doc.id));
            } else {
                notes.push(format!("{} still in the kernel", a.doc.id));
            }
            inner.next_id = inner.next_id.max(a.num_id + 1);
            inner.active.insert(a.doc.id.clone(), a);
        }
        for kind in ShieldKind::ALL {
            let owned: Vec<Vec<u8>> = inner
                .active
                .values()
                .filter(|a| a.doc.kind == kind)
                .map(Self::key_of)
                .collect();
            for key in self.maps.keys(kind) {
                if !owned.contains(&key) {
                    let _ = self.maps.delete(kind, &key);
                    notes.push(format!("removed a {} row no saved shield owns", kind.as_str()));
                }
            }
            self.set_rows(&mut inner, kind);
        }
        self.snapshot(&mut inner);
        notes
    }

    /// POST /shields: a document and its signature (hex Ed25519 over the exact
    /// document bytes, made with the policy key).
    pub fn apply_signed(&self, document: &str, signature: &str) -> Outcome {
        let pem = self.pubkey_pem.lock().unwrap_or_else(|e| e.into_inner()).clone();
        if let Err(e) = crate::config::verify_policy_signature(document.as_bytes(), signature, &pem) {
            let id = parse_document(document).map(|d| d.id).unwrap_or_else(|_| "-".into());
            let o = refused(403, "bad_signature", format!("{e:#}"));
            self.audit("refuse", &id, 403, json!({ "why": "bad_signature", "error": format!("{e:#}") }));
            return o;
        }
        let doc = match parse_document(document) {
            Ok(d) => d,
            Err(e) => {
                self.audit("refuse", "-", 400, json!({ "why": "bad_document", "error": e }));
                return refused(400, "bad_document", e);
            }
        };
        self.apply(doc, "signed".into(), Some(document.to_string()), Some(signature.trim().to_string()))
    }

    /// A playbook proposal the agent applies by itself. Only kinds the signed
    /// policy lists, and the lifetime is capped at its auto_apply limit.
    pub fn apply_auto(&self, mut doc: ShieldDoc, playbook: &str) -> Outcome {
        let (allowed, cap) = {
            let inner = self.lock();
            (
                inner.policy.auto_apply.kinds.iter().any(|k| k == doc.kind.as_str()),
                inner.policy.auto_apply.max_lifetime_sec,
            )
        };
        if !allowed {
            return refused(403, "auto_apply_not_allowed", format!("the signed policy does not auto apply {}", doc.kind.as_str()));
        }
        let start = doc.not_before.max(self.clock.unix());
        doc.expires_at = doc.expires_at.min(start + cap);
        self.apply(doc, format!("auto:{playbook}"), None, None)
    }

    fn apply(&self, doc: ShieldDoc, source: String, document: Option<String>, signature: Option<String>) -> Outcome {
        let id = doc.id.clone();
        let out = self.apply_inner(doc, source, document, signature);
        let why = out.body.get("refused").cloned().unwrap_or(Value::Null);
        let action = if out.status < 300 { "apply" } else { "refuse" };
        self.audit(action, &id, out.status, json!({ "why": why, "result": out.body }));
        out
    }

    fn apply_inner(&self, doc: ShieldDoc, source: String, document: Option<String>, signature: Option<String>) -> Outcome {
        if self.enforcement == "none" {
            return refused(503, "no_enforcement", "shields need the BPF LSM, and this agent runs detect-only");
        }
        let mut inner = self.lock();
        if !inner.policy.enabled {
            return refused(403, "disabled", "the signed policy turns shields off");
        }
        if !valid_id(&doc.id) {
            return refused(400, "bad_document", "id is 1 to 64 of A-Z a-z 0-9 - _ . :");
        }
        if doc.serial == 0 {
            return refused(400, "bad_document", "serial starts at 1");
        }
        let reason = doc.reason.as_deref().map(str::trim).unwrap_or("");
        let incident = doc.incident_id.as_deref().map(str::trim).unwrap_or("");
        if reason.is_empty() && incident.is_empty() {
            return refused(400, "bad_document", "a reason or an incident_id is required");
        }
        if reason.len() > MAX_REASON || incident.len() > 200 {
            return refused(400, "bad_document", "reason or incident_id too long");
        }
        // Replay: every id's serial only moves up, across deletes and restarts.
        let last = inner.serials.get(&doc.id).copied().unwrap_or(0);
        if doc.serial <= last {
            return refused(409, "replay", format!("serial {} for {} is not above the last one seen, {last}", doc.serial, doc.id));
        }
        let now = self.clock.unix();
        if doc.expires_at <= now {
            return refused(400, "expired", format!("expires_at {} is in the past", doc.expires_at));
        }
        if doc.expires_at <= doc.not_before {
            return refused(400, "bad_document", "expires_at must be after not_before");
        }
        let lifetime = doc.expires_at - doc.not_before.max(now);
        if lifetime > inner.policy.max_lifetime_sec {
            return refused(
                403,
                "lifetime",
                format!("lifetime {lifetime} s is over the policy's maximum of {} s", inner.policy.max_lifetime_sec),
            );
        }
        let target = match resolve_target(&doc) {
            Ok(t) => t,
            Err(e) => return refused(400, "bad_match", e),
        };
        if let Some(why) = inner.never.refusal(&target) {
            return refused(403, "never_block", why);
        }
        let cgroup_id = match &doc.scope.cgroup {
            None => 0,
            Some(dir) => match cgroup_id(Path::new(dir)) {
                Ok(id) => id,
                Err(e) => return refused(400, "bad_scope", e),
            },
        };
        let key = target.key();
        let key_hex = hex::encode(&key);
        // One shield per target and kind. The same id with a higher serial
        // replaces itself.
        if let Some(other) = inner
            .active
            .values()
            .find(|a| a.doc.kind == doc.kind && a.key_hex == key_hex && a.doc.id != doc.id)
        {
            return refused(409, "conflict", format!("{} already shields this target", other.doc.id));
        }
        let replacing = inner.active.get(&doc.id).cloned();
        if replacing.is_none() && inner.active.len() >= inner.policy.max_active {
            return refused(429, "max_active", format!("{} shields are active, the policy's maximum", inner.policy.max_active));
        }
        let rows = inner.rows.get(&doc.kind).copied().unwrap_or(0) as usize;
        if replacing.is_none() && rows >= abi::ROWS_MAX {
            return refused(429, "max_active", format!("the {} map is full", doc.kind.as_str()));
        }
        let auto_promote_at = (doc.mode == ShieldMode::Enforce
            && inner.policy.auto_promote.kinds.iter().any(|k| k == doc.kind.as_str()))
        .then(|| doc.not_before.max(now) + inner.policy.auto_promote.trial_sec);
        let num_id = match replacing.as_ref() {
            Some(a) => a.num_id,
            None => match free_number(&inner) {
                Some(n) => n,
                None => return refused(429, "max_active", "every shield number is in use"),
            },
        };
        if replacing.is_none() {
            if let Err(e) = self.maps.reset_hits(num_id) {
                return refused(500, "map_write", e);
            }
        }
        let a = Active {
            expires_at: doc.expires_at,
            doc,
            num_id,
            source,
            document,
            signature,
            created: now,
            mode_now: ShieldMode::Monitor,
            promoted_at: None,
            auto_promote_at,
            cgroup_id,
            key_hex,
        };
        if let Some(old) = &replacing {
            if old.key_hex != a.key_hex || old.doc.kind != a.doc.kind {
                let _ = self.maps.delete(old.doc.kind, &Self::key_of(old));
            }
        }
        if let Err(e) = self.maps.put(a.doc.kind, &key, &self.rule_for(&a)) {
            return refused(500, "map_write", e);
        }
        if replacing.is_none() {
            inner.next_id = inner.next_id.max(num_id + 1);
        }
        inner.serials.insert(a.doc.id.clone(), a.doc.serial);
        let kind = a.doc.kind;
        let view = self.view(&a);
        self.put(&mut inner, &a);
        inner.active.insert(a.doc.id.clone(), a);
        if let Some(old) = &replacing {
            if old.doc.kind != kind {
                self.set_rows(&mut inner, old.doc.kind);
            }
        }
        self.set_rows(&mut inner, kind);
        outcome(201, json!({ "ok": true, "replaced": replacing.is_some(), "shield": view }))
    }

    /// POST /shields/{id}/promote: monitor to enforce, for a shield whose
    /// document allows enforce.
    pub fn promote(&self, id: &str, why: &str) -> Outcome {
        let out = self.set_mode(id, ShieldMode::Enforce, why);
        self.audit("promote", id, out.status, json!({ "reason": why, "result": out.body }));
        out
    }

    fn set_mode(&self, id: &str, mode: ShieldMode, _why: &str) -> Outcome {
        let mut inner = self.lock();
        let Some(a) = inner.active.get(id).cloned() else {
            return refused(404, "unknown", format!("no shield {id}"));
        };
        if a.doc.mode == ShieldMode::Monitor && mode == ShieldMode::Enforce {
            return refused(409, "monitor_only", format!("{id} was signed for monitor mode, sign a new serial with mode enforce"));
        }
        if a.mode_now == mode {
            return outcome(200, json!({ "ok": true, "unchanged": true, "shield": self.view(&a) }));
        }
        let key = Self::key_of(&a);
        let mut rule = self.row(&a).unwrap_or_else(|| self.rule_for(&a));
        // Only the mode changes. The hit counters are in shield_stats and stay.
        rule.mode = mode.raw();
        if let Err(e) = self.maps.put(a.doc.kind, &key, &rule) {
            return refused(500, "map_write", e);
        }
        let mut a = a;
        a.mode_now = mode;
        a.promoted_at = Some(self.clock.unix());
        a.auto_promote_at = None;
        let view = self.view(&a);
        self.put(&mut inner, &a);
        inner.active.insert(id.to_string(), a);
        outcome(200, json!({ "ok": true, "shield": view }))
    }

    /// DELETE /shields/{id}: lifts a shield now. Its serial stays recorded, so
    /// the same document cannot be applied again.
    pub fn remove(&self, id: &str, why: &str) -> Outcome {
        let out = {
            let mut inner = self.lock();
            match inner.active.remove(id) {
                None => refused(404, "unknown", format!("no shield {id}")),
                Some(a) => {
                    let hits = self.maps.hits(a.num_id);
                    let _ = self.maps.delete(a.doc.kind, &Self::key_of(&a));
                    self.set_rows(&mut inner, a.doc.kind);
                    self.del(&mut inner, id);
                    outcome(200, json!({ "ok": true, "removed": id, "hits": hits }))
                }
            }
        };
        self.audit("delete", id, out.status, json!({ "reason": why, "result": out.body }));
        out
    }

    /// Once a second: rows past their deadline go (the kernel already ignores
    /// them), and shields due for auto promotion are promoted.
    pub fn sweep(&self) {
        let now = self.clock.unix();
        let boot = self.clock.boot_ns();
        let (expired, due): (Vec<Active>, Vec<String>) = {
            let inner = self.lock();
            let expired = inner
                .active
                .values()
                .filter(|a| {
                    let kernel = self.row(a).map(|r| boot >= r.expires_ns);
                    kernel.unwrap_or(false) || a.expires_at <= now
                })
                .cloned()
                .collect();
            let due = inner
                .active
                .values()
                .filter(|a| a.mode_now == ShieldMode::Monitor && a.auto_promote_at.is_some_and(|t| t <= now))
                .map(|a| a.doc.id.clone())
                .collect();
            (expired, due)
        };
        for a in expired {
            let hits = self.maps.hits(a.num_id);
            {
                let mut inner = self.lock();
                if inner.active.remove(&a.doc.id).is_none() {
                    continue;
                }
                let _ = self.maps.delete(a.doc.kind, &Self::key_of(&a));
                self.set_rows(&mut inner, a.doc.kind);
                self.del(&mut inner, &a.doc.id);
            }
            self.audit("expire", &a.doc.id, 200, json!({ "expires_at": a.expires_at, "hits": hits }));
        }
        for id in due {
            let out = self.set_mode(&id, ShieldMode::Enforce, "trial period over");
            self.audit("auto_promote", &id, out.status, json!({ "result": out.body }));
        }
    }

    /// A new signed policy: new limits and never-block list. A shield that now
    /// names a never-block target is removed, and one that outlives the new
    /// maximum lifetime is cut to it.
    pub fn apply_policy(&self, policy: ShieldPolicy, never: NeverBlock) -> Vec<String> {
        let mut notes = Vec::new();
        let now = self.clock.unix();
        let mut audits = Vec::new();
        {
            let mut inner = self.lock();
            inner.policy = policy;
            inner.never = never;
            let ids: Vec<String> = inner.active.keys().cloned().collect();
            for id in ids {
                let a = inner.active[&id].clone();
                let target = resolve_target(&a.doc);
                let blocked = target.as_ref().ok().and_then(|t| inner.never.refusal(t));
                if let Some(why) = blocked.or_else(|| (!inner.policy.enabled).then(|| "the policy turns shields off".to_string())) {
                    inner.active.remove(&id);
                    let _ = self.maps.delete(a.doc.kind, &Self::key_of(&a));
                    self.set_rows(&mut inner, a.doc.kind);
                    self.del(&mut inner, &id);
                    notes.push(format!("removed {id}: {why}"));
                    audits.push(("removed_by_policy", id, json!({ "why": why })));
                    continue;
                }
                let cap = a.doc.not_before.max(now) + inner.policy.max_lifetime_sec;
                if a.expires_at > cap {
                    let mut b = a.clone();
                    b.expires_at = cap;
                    let key = Self::key_of(&b);
                    let mut rule = self.row(&b).unwrap_or_else(|| self.rule_for(&b));
                    rule.expires_ns = to_boot_ns(&*self.clock, cap);
                    let _ = self.maps.put(b.doc.kind, &key, &rule);
                    notes.push(format!("{id} now ends at {cap}, the new maximum lifetime"));
                    audits.push(("capped_by_policy", id.clone(), json!({ "expires_at": cap })));
                    self.put(&mut inner, &b);
                    inner.active.insert(id, b);
                }
            }
        }
        for (action, id, detail) in audits {
            self.audit(action, &id, 200, detail);
        }
        notes
    }

    pub fn set_pubkey(&self, pem: Vec<u8>) {
        *self.pubkey_pem.lock().unwrap_or_else(|e| e.into_inner()) = pem;
    }

    fn view(&self, a: &Active) -> Value {
        let now = self.clock.unix();
        let rule = self.row(a);
        let target = resolve_target(&a.doc).ok();
        // Read once per view. The kernel counts per CPU.
        let target_text = match &target {
            Some(Target::Net { net, port }) => {
                if *port == 0 {
                    format!("{} any port", net_text(net))
                } else {
                    format!("{} port {port}", net_text(net))
                }
            }
            Some(Target::File(id)) => format!(
                "{} (dev {} ino {})",
                a.doc.target.path.as_deref().unwrap_or("file"),
                a.doc.target.dev.unwrap_or(0),
                id.ino
            ),
            None => "-".into(),
        };
        json!({
            "id": a.doc.id,
            "serial": a.doc.serial,
            "kind": a.doc.kind.as_str(),
            "target": target_text,
            "match": a.doc.target,
            "scope": a.doc.scope.cgroup.clone().unwrap_or_else(|| "host".into()),
            "cgroup_id": a.cgroup_id,
            "mode": a.mode_now.as_str(),
            "signed_mode": a.doc.mode.as_str(),
            "hits": self.maps.hits(a.num_id),
            "in_kernel": rule.is_some(),
            "enforcement": self.enforcement,
            "source": a.source,
            "created": a.created,
            "not_before": a.doc.not_before,
            "expires_at": a.expires_at,
            "expires_in_sec": a.expires_at.saturating_sub(now),
            "auto_promote_at": a.auto_promote_at,
            "promoted_at": a.promoted_at,
            "reason": a.doc.reason,
            "incident_id": a.doc.incident_id,
            "hook": a.doc.kind.hook(),
        })
    }

    /// GET /shields.
    pub fn list(&self) -> Value {
        let inner = self.lock();
        let items: Vec<Value> = inner.active.values().map(|a| self.view(a)).collect();
        json!({
            "enforcement": self.enforcement,
            "enabled": inner.policy.enabled,
            "max_active": inner.policy.max_active,
            "max_lifetime_sec": inner.policy.max_lifetime_sec,
            "auto_promote": inner.policy.auto_promote,
            "auto_apply": inner.policy.auto_apply,
            "active": items.len(),
            "rows": ShieldKind::ALL.iter().map(|k| (k.as_str().to_string(), json!(inner.rows.get(k).copied().unwrap_or(0)))).collect::<serde_json::Map<String, Value>>(),
            "shields": items,
        })
    }

    /// The shield behind a kernel report, by the number the row carries.
    pub fn by_num(&self, num: u32) -> Option<Active> {
        self.lock().active.values().find(|a| a.num_id == num).cloned()
    }

    pub fn active_count(&self) -> usize {
        self.lock().active.len()
    }
}

// ---- The process wide manager ----

static MANAGER: OnceLock<Arc<ShieldManager>> = OnceLock::new();

pub fn set_manager(m: Arc<ShieldManager>) -> bool {
    MANAGER.set(m).is_ok()
}

pub fn manager() -> Option<&'static Arc<ShieldManager>> {
    MANAGER.get()
}

// ---- Kernel reports ----

const SHIELD_RISK: f32 = 75.0;

/// One kernel decision as an alert for the incident list, a TrustEvent and a
/// telemetry record. `shield` is the active shield the row belongs to, if the
/// agent still has it.
pub fn shield_finding(e: &ShieldEvent, shield: Option<&Active>, exe_path: Option<&str>) -> Option<(crate::guard::Finding, Value)> {
    let kind = ShieldKind::from_raw(e.kind)?;
    let action = match e.action {
        abi::ACTION_BLOCKED => "blocked",
        abi::ACTION_WOULD_BLOCK => "would_block",
        _ => return None,
    };
    let exe = exe_path.map(str::to_string).unwrap_or_else(|| e.exe_name.clone());
    let target = match kind {
        ShieldKind::ConnectDeny => match e.dest() {
            Some(ip) => format!("{ip}:{}", e.port),
            None => "-".into(),
        },
        _ => {
            let path = shield.and_then(|s| s.doc.target.path.clone()).unwrap_or_else(|| e.target.clone());
            format!("{path} (ino {})", e.target_ino)
        }
    };
    let id = shield.map(|s| s.doc.id.clone()).unwrap_or_else(|| format!("#{}", e.shield_id));
    let incident = shield.and_then(|s| s.doc.incident_id.clone());
    let verb = if action == "blocked" { "blocked" } else { "would block (monitor mode)" };
    let what = match kind {
        ShieldKind::ExecDeny => "execve of",
        ShieldKind::FileDeny => "open of",
        ShieldKind::ConnectDeny => "connect to",
    };
    let reason = format!(
        "shield {id} {verb}: {} (pid {}, uid {}, exe {exe}) {what} {target}",
        e.comm, e.tgid, e.uid
    );
    let signal = format!("shield::{}::{action}", kind.as_str());
    let ts = crate::utils::time::now_ts();
    let tags = vec!["shield".to_string(), format!("shield:{action}"), format!("shield:{}", kind.as_str())];
    let mut data = HashMap::new();
    data.insert("source".into(), "edr_shield".into());
    data.insert("timestamp".into(), ts.to_string());
    data.insert("pid".into(), e.tgid.to_string());
    data.insert("tid".into(), e.tid.to_string());
    data.insert("uid".into(), e.uid.to_string());
    data.insert("comm".into(), e.comm.clone());
    data.insert("binary_path".into(), exe.clone());
    data.insert("exe_ino".into(), e.exe_ino.to_string());
    data.insert("shield".into(), id.clone());
    data.insert("shield_kind".into(), kind.as_str().into());
    data.insert("action".into(), action.into());
    data.insert("target".into(), target.clone());
    data.insert("cgroup_id".into(), e.cgroup_id.to_string());
    data.insert("tags".into(), tags.join(","));
    data.insert("signal".into(), signal.clone());
    data.insert("replay_tag".into(), signal.clone());
    data.insert("reason".into(), reason.clone());
    if let Some(i) = &incident {
        data.insert("incident_id".into(), i.clone());
    }
    let finding = crate::guard::build_finding(signal, e.tgid, e.uid, SHIELD_RISK, tags, reason.clone(), data);
    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
    let (mode, severity) = if action == "blocked" { ("enforce", "high") } else { ("monitor", "medium") };
    let alert = json!({
        "id": format!("shield:{id}:{action}:{}:{ts}", e.tgid),
        "source": "shield",
        "title": format!("Shield {id} {verb}: {target}"),
        "ts": ts,
        "severity": severity,
        "risk": SHIELD_RISK as f64 / 100.0,
        "technique": "-",
        "findings": [],
        "event": { "exe": exe, "pid": e.tgid.to_string(), "uid": e.uid.to_string(), "comm": e.comm, "host": host },
        "details": {
            "shield": id,
            "kind": kind.as_str(),
            "hook": kind.hook(),
            "action": action,
            "mode": mode,
            "caller_exe": exe,
            "caller_pid": e.tgid.to_string(),
            "caller_comm": e.comm,
            "target": target,
            "incident_id": incident,
            "reason": reason,
        },
        "rationale": [reason],
    });
    Some((finding, alert))
}

#[cfg(test)]
mod tests;

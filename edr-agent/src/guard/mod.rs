//! Prevention mode. BPF LSM (Linux Security Module) programs in
//! src/ebpf/edr_guard.bpf.c refuse an attack inside the kernel instead of
//! only reporting it afterwards.
//!
//! - runtime_socket: connect() to a container runtime socket such as
//!   /var/run/docker.sock from an executable that is not an allowlisted
//!   runtime client (T1611, T1610). Executables are named by inode and device,
//!   so renaming python3 to "docker" does not get it through.
//! - memfd_exec: execve of a memory-only file made with memfd_create (T1620).
//! - self_protect: fatal signals, ptrace attach and handles to the agent's BPF
//!   programs, maps and links (the runtime shields' pinned ones included) from
//!   other processes, and writes, unlinks, renames, hard links and truncates of
//!   the agent's own files (T1562.001).
//!
//! Each guard runs off, monitor (the default: allow and report would_block)
//! or enforce (refuse with EPERM and report blocked). start_guard() checks that
//! the kernel has the bpf LSM active and otherwise leaves the agent
//! detect-only with one log line.
//!
//! Environment:
//!   EDR_GUARD_MODE            off, monitor or enforce for every guard (default monitor)
//!   EDR_GUARD_SOCKET_MODE     override for runtime_socket
//!   EDR_GUARD_MEMFD_MODE      override for memfd_exec
//!   EDR_GUARD_SELF_MODE       override for self_protect
//!   EDR_BPF_GUARD_OBJ         object path, else $EDR_BPF_DIR/edr_guard.bpf.o
//!   EDR_GUARD_SOCKET_CLIENTS  more client executables, colon separated paths
//!   EDR_GUARD_MEMFD_ALLOW     more executables allowed to run a memfd, colon separated
//!   EDR_GUARD_SCOPE_CGROUP    a cgroup v2 directory. Every guard then judges only tasks in it
//!   EDR_GUARD_OFF_FILE        self protection off switch (default /run/edr-guard/off)
//!   EDR_GUARD_OFF_SHA256      if set, the off file must hold a token with this SHA-256
//!
//! The signed policy sets the lowest mode (guard_mode) and adds allowlist entries
//! (guard_allow). A new policy applies while the agent runs, through
//! GuardControl::apply, which fills the inactive bank and flips one slot.
//!
//! Self protection off switch, for an operator: create the off file (with the
//! token if EDR_GUARD_OFF_SHA256 is set). Within 200 ms the agent turns self
//! protection off and logs it, and it can then be stopped like any process.
//! GuardHandle::stop() does the same on a graceful shutdown. If the agent dies
//! for any reason the kernel detaches every guard, because nothing is pinned.

pub mod abi;
#[cfg(target_os = "linux")]
mod loader;

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use sha2::{Digest, Sha256};

use crate::modules::suspicious_ipc::allowlist_match;
use crate::telemetry_types::TelemetryOutput;
use crate::trust_hook::{submit_trust_event, TrustEvent};
use crate::utils::time::now_ts;
use abi::{ExeId, GuardAction, GuardEvent, GuardKind, UNIX_PATH_MAX};

// ---- Modes ----

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuardMode {
    Off,
    Monitor,
    Enforce,
}

impl GuardMode {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "off" => Some(Self::Off),
            "monitor" => Some(Self::Monitor),
            "enforce" => Some(Self::Enforce),
            _ => None,
        }
    }

    /// MODE_* in the C file.
    pub fn as_raw(self) -> u64 {
        match self {
            Self::Off => 0,
            Self::Monitor => 1,
            Self::Enforce => 2,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Monitor => "monitor",
            Self::Enforce => "enforce",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuardModes {
    pub socket: GuardMode,
    pub memfd: GuardMode,
    pub self_protect: GuardMode,
}

impl GuardModes {
    /// EDR_GUARD_MODE sets all three (default monitor), and each per guard
    /// variable overrides its own. A value that does not parse keeps the
    /// default and comes back as a warning, so a typo never turns a guard off
    /// or on by surprise.
    pub fn from_vars(var: impl Fn(&str) -> Option<String>) -> (Self, Vec<String>) {
        let mut warnings = Vec::new();
        let mut read = |name: &str, fallback: GuardMode| match var(name) {
            None => fallback,
            Some(value) => GuardMode::parse(&value).unwrap_or_else(|| {
                warnings.push(format!(
                    "{name}={value:?} is not off, monitor or enforce, using {}",
                    fallback.as_str()
                ));
                fallback
            }),
        };
        let all = read("EDR_GUARD_MODE", GuardMode::Monitor);
        let modes = Self {
            socket: read("EDR_GUARD_SOCKET_MODE", all),
            memfd: read("EDR_GUARD_MEMFD_MODE", all),
            self_protect: read("EDR_GUARD_SELF_MODE", all),
        };
        (modes, warnings)
    }

    pub fn all_off(&self) -> bool {
        [self.socket, self.memfd, self.self_protect]
            .iter()
            .all(|m| *m == GuardMode::Off)
    }

    /// Every guard at least `floor` (the signed policy's guard_mode). A variable that
    /// asked for less is overruled, and the warning says so.
    pub fn at_least(self, floor: GuardMode) -> (Self, Vec<String>) {
        let mut warnings = Vec::new();
        let mut lift = |name: &str, m: GuardMode| {
            if m.as_raw() < floor.as_raw() {
                warnings.push(format!(
                    "{name} asked for {}, the signed policy requires at least {}",
                    m.as_str(),
                    floor.as_str()
                ));
                floor
            } else {
                m
            }
        };
        let modes = Self {
            socket: lift("runtime_socket", self.socket),
            memfd: lift("memfd_exec", self.memfd),
            self_protect: lift("self_protect", self.self_protect),
        };
        (modes, warnings)
    }
}

// ---- Settings ----

#[derive(Clone, Debug)]
pub struct GuardSettings {
    pub modes: GuardModes,
    pub obj_path: String,
    pub extra_socket_clients: Vec<PathBuf>,
    pub extra_memfd_allow: Vec<PathBuf>,
    /// guard_allow from the signed policy.
    pub policy_allow: crate::config::GuardAllow,
    pub scope_cgroup: Option<PathBuf>,
    pub off_file: PathBuf,
    pub off_sha256: Option<String>,
}

pub const DEFAULT_OFF_FILE: &str = "/run/edr-guard/off";

impl GuardSettings {
    pub fn from_env() -> Self {
        let var = |name: &str| std::env::var(name).ok();
        let (modes, warnings) = GuardModes::from_vars(var);
        for w in warnings {
            log::warn!("guard: {w}");
        }
        let paths = |name: &str| -> Vec<PathBuf> {
            var(name)
                .map(|v| split_paths(&v))
                .unwrap_or_default()
        };
        Self {
            modes,
            obj_path: object_path(var("EDR_BPF_GUARD_OBJ"), var("EDR_BPF_DIR")),
            extra_socket_clients: paths("EDR_GUARD_SOCKET_CLIENTS"),
            extra_memfd_allow: paths("EDR_GUARD_MEMFD_ALLOW"),
            policy_allow: Default::default(),
            scope_cgroup: var("EDR_GUARD_SCOPE_CGROUP")
                .filter(|v| !v.trim().is_empty())
                .map(PathBuf::from),
            off_file: var("EDR_GUARD_OFF_FILE")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(DEFAULT_OFF_FILE)),
            off_sha256: var("EDR_GUARD_OFF_SHA256").filter(|v| !v.trim().is_empty()),
        }
    }
}

impl GuardSettings {
    /// The runtime socket clients and the memfd allowlist: the programs found on
    /// disk by name, the environment's extra paths and the policy's guard_allow,
    /// each named by inode.
    pub fn allowlists(&self) -> (Vec<(PathBuf, ExeId)>, Vec<(PathBuf, ExeId)>) {
        let mut clients = find_programs(CLIENT_DIRS, crate::modules::suspicious_ipc::RUNTIME_CLIENT_ALLOWLIST);
        clients.extend(self.extra_socket_clients.iter().cloned());
        clients.extend(self.policy_allow.socket_clients.iter().map(PathBuf::from));
        let mut memfd = find_programs(CLIENT_DIRS, MEMFD_EXEC_CLIENTS);
        memfd.extend(self.extra_memfd_allow.iter().cloned());
        memfd.extend(self.policy_allow.memfd_allow.iter().map(PathBuf::from));
        (exe_ids(&clients), exe_ids(&memfd))
    }
}

/// Same lookup order as suspicious_ipc and dns_tunnel: explicit path, then
/// EDR_BPF_DIR, then the build output in the repo.
pub fn object_path(obj: Option<String>, dir: Option<String>) -> String {
    if let Some(path) = obj {
        return path;
    }
    if let Some(dir) = dir {
        return format!("{}/edr_guard.bpf.o", dir.trim_end_matches('/'));
    }
    "src/ebpf/edr_guard.bpf.o".to_string()
}

fn split_paths(value: &str) -> Vec<PathBuf> {
    value
        .split(':')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .map(PathBuf::from)
        .collect()
}

/// True when "bpf" is one of the names in /sys/kernel/security/lsm, the
/// kernel's list of active security modules. A kernel can be built with BPF
/// LSM support and still not run it (Ubuntu 22.04 needs lsm=...,bpf on the
/// boot line). The programs then load and attach but never run, so this list
/// is the only honest check.
pub fn bpf_lsm_listed(lsm_list: &str) -> bool {
    lsm_list.trim().split(',').any(|name| name.trim() == "bpf")
}

// ---- Allowlists ----

/// Where runtime clients are installed. /bin and /usr/bin are often the same
/// directory, so exe_ids() removes duplicates.
pub const CLIENT_DIRS: &[&str] = &[
    "/usr/bin",
    "/usr/sbin",
    "/usr/local/bin",
    "/usr/local/sbin",
    "/bin",
    "/sbin",
    "/usr/libexec/docker/cli-plugins",
    "/usr/lib/docker/cli-plugins",
    "/usr/local/lib/docker/cli-plugins",
];

/// Programs that may run a memfd, matched the same way as the client list.
pub const MEMFD_EXEC_CLIENTS: &[(&str, &str)] = &[(
    "runc",
    "can re-execute itself from a sealed memfd copy, its fix for CVE-2019-5736",
)];

/// Regular files in `dirs` whose names are on `list` (the suspicious_ipc
/// rules, so a trailing '*' matches a prefix). The name is only used to find
/// candidates on disk. The kernel then compares inode and device.
pub fn find_programs(dirs: &[&str], list: &[(&'static str, &'static str)]) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for dir in dirs {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name();
            let Some(name) = name.to_str() else {
                continue;
            };
            let path = entry.path();
            if allowlist_match(name, list).is_some() && path.is_file() {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

/// The kernel id of each path stat() can read, symlinks followed. The first
/// path seen for an id is kept, so /bin/docker and /usr/bin/docker on a
/// merged /usr give one entry.
pub fn exe_ids(paths: &[PathBuf]) -> Vec<(PathBuf, ExeId)> {
    let mut out: Vec<(PathBuf, ExeId)> = Vec::new();
    for path in paths {
        let Ok(meta) = fs::metadata(path) else {
            continue;
        };
        let id = ExeId::from_stat(meta.dev(), meta.ino());
        if !out.iter().any(|(_, seen)| *seen == id) {
            out.push((path.clone(), id));
        }
    }
    out
}

// ---- Off switch ----

/// True when the off file exists and, if a hash is configured, holds a token
/// whose SHA-256 (hex) matches it. Leading and trailing whitespace is ignored.
pub fn off_switch_thrown(file: &Path, sha256_hex: Option<&str>) -> bool {
    let Ok(content) = fs::read(file) else {
        return false;
    };
    match sha256_hex {
        None => true,
        Some(want) => {
            let token = content.trim_ascii();
            hex::encode(Sha256::digest(token)) == want.trim().to_ascii_lowercase()
        }
    }
}

// ---- Findings ----

/// Fixed rule severities on the 0 to 100 TrustEvent scale. They rank the
/// guards against each other and are not measured probabilities. Tampering
/// ranks highest because an attacker does it right before acting.
const SOCKET_RISK: f32 = 60.0;
const MEMFD_RISK: f32 = 70.0;
const TAMPER_RISK: f32 = 80.0;

/// Everything one decision sends downstream.
pub struct Finding {
    pub output: TelemetryOutput,
    pub trust: TrustEvent,
}

fn signal_name(sig: u32) -> String {
    let name = match sig {
        1 => "SIGHUP",
        2 => "SIGINT",
        3 => "SIGQUIT",
        6 => "SIGABRT",
        9 => "SIGKILL",
        10 => "SIGUSR1",
        12 => "SIGUSR2",
        15 => "SIGTERM",
        19 => "SIGSTOP",
        20 => "SIGTSTP",
        _ => return format!("signal {sig}"),
    };
    name.to_string()
}

fn bpf_cmd_name(cmd: u32) -> String {
    match cmd {
        abi::BPF_PROG_GET_FD_BY_ID => "BPF_PROG_GET_FD_BY_ID".into(),
        abi::BPF_MAP_GET_FD_BY_ID => "BPF_MAP_GET_FD_BY_ID".into(),
        abi::BPF_LINK_GET_FD_BY_ID => "BPF_LINK_GET_FD_BY_ID".into(),
        abi::BPF_OBJ_GET => "BPF_OBJ_GET".into(),
        other => format!("bpf command {other}"),
    }
}

fn file_op_name(op: u32) -> &'static str {
    match op {
        abi::FILE_OP_WRITE => "write open",
        abi::FILE_OP_UNLINK => "unlink",
        abi::FILE_OP_RENAME => "rename",
        abi::FILE_OP_TRUNCATE => "truncate",
        abi::FILE_OP_LINK => "hard link",
        _ => "change",
    }
}

/// The tags every guard record carries: "guard", "guard:blocked" or
/// "guard:would_block", "guard:<guard name>", then the MITRE technique ids.
pub fn guard_tags(kind: GuardKind, action: GuardAction) -> Vec<String> {
    let mut tags = vec![
        "guard".to_string(),
        format!("guard:{}", action.as_str()),
        format!("guard:{}", kind.guard()),
    ];
    tags.extend(kind.mitre().iter().map(|t| t.to_string()));
    tags
}

/// Turns one kernel decision into the agent's TelemetryOutput and TrustEvent.
/// exe_path comes from /proc/<pid>/exe when it could be read. None for a
/// record with an unknown kind or action.
pub fn guard_finding(e: &GuardEvent, exe_path: Option<&str>) -> Option<Finding> {
    let kind = e.kind()?;
    let action = e.action()?;
    let comm = e.comm();
    let exe = exe_path.map(str::to_string).unwrap_or_else(|| e.exe_name());
    let (what, target, risk) = match kind {
        GuardKind::RuntimeSocket => {
            let path = e.target();
            (
                format!("connect() to container runtime socket {path}"),
                path,
                SOCKET_RISK,
            )
        }
        GuardKind::MemfdExec => {
            let name = e.target();
            (
                format!("execve of memory-only file {name}"),
                name,
                MEMFD_RISK,
            )
        }
        GuardKind::SignalToAgent => (
            format!("{} to the agent (pid {})", signal_name(e.detail), e.target_pid),
            format!("pid {}", e.target_pid),
            TAMPER_RISK,
        ),
        GuardKind::PtraceAgent => (
            format!("ptrace attach to the agent (pid {}, mode {:#x})", e.target_pid, e.detail),
            format!("pid {}", e.target_pid),
            TAMPER_RISK,
        ),
        GuardKind::BpfAgentObject if e.detail == abi::BPF_OBJ_GET => (
            format!("BPF_OBJ_GET of the shield pin {}", e.target()),
            e.target(),
            TAMPER_RISK,
        ),
        GuardKind::BpfAgentObject => (
            format!("{} on the agent's BPF object id {}", bpf_cmd_name(e.detail), e.target_ino),
            format!("bpf id {}", e.target_ino),
            TAMPER_RISK,
        ),
        GuardKind::FileTamper => (
            format!("{} of the agent's file {} (inode {})", file_op_name(e.detail), e.target(), e.target_ino),
            e.target(),
            TAMPER_RISK,
        ),
    };
    let verb = match action {
        GuardAction::Blocked => "blocked",
        GuardAction::WouldBlock => "would block (monitor mode)",
    };
    let reason = format!(
        "{verb}: {comm} (pid {}, uid {}, exe {exe}) {what}",
        e.tgid, e.uid
    );
    let tags = guard_tags(kind, action);
    let signal = format!("guard::{}::{}", kind.guard(), action.as_str());
    let ts = now_ts();

    let mut data = HashMap::new();
    // pb_wiring turns guard records into RuleHit facts by this source name.
    data.insert("source".into(), "edr_guard".into());
    data.insert("timestamp".into(), ts.to_string());
    data.insert("pid".into(), e.tgid.to_string());
    data.insert("tid".into(), e.tid.to_string());
    data.insert("uid".into(), e.uid.to_string());
    data.insert("comm".into(), comm);
    data.insert("binary_path".into(), exe);
    data.insert("exe_ino".into(), e.exe_ino.to_string());
    data.insert("exe_dev".into(), e.exe_dev.to_string());
    data.insert("guard".into(), kind.guard().into());
    data.insert("hook".into(), kind.hook().into());
    data.insert("action".into(), action.as_str().into());
    data.insert("target".into(), target);
    data.insert("target_ino".into(), e.target_ino.to_string());
    data.insert("target_pid".into(), e.target_pid.to_string());
    data.insert("detail".into(), e.detail.to_string());
    data.insert("mitre".into(), kind.mitre().join(","));
    data.insert("tags".into(), tags.join(","));
    data.insert("signal".into(), signal.clone());
    data.insert("replay_tag".into(), signal.clone());
    data.insert("reason".into(), reason.clone());

    Some(build_finding(signal, e.tgid, e.uid, risk, tags, reason, data))
}

/// The record left when an operator turns self protection off with the off
/// file. An attacker with root can do the same, so it is worth a look.
pub fn off_switch_finding(file: &Path) -> Finding {
    let reason = format!(
        "self protection turned off through the off file {}",
        file.display()
    );
    let tags = vec![
        "guard".to_string(),
        "guard:self_protect".to_string(),
        "guard:off_switch".to_string(),
        "T1562.001".to_string(),
    ];
    let signal = "guard::self_protect::off_switch".to_string();
    let pid = std::process::id();
    let mut data = HashMap::new();
    data.insert("source".into(), "edr_guard".into());
    data.insert("timestamp".into(), now_ts().to_string());
    data.insert("pid".into(), pid.to_string());
    data.insert("guard".into(), "self_protect".into());
    data.insert("action".into(), "off_switch".into());
    data.insert("target".into(), file.display().to_string());
    data.insert("mitre".into(), "T1562.001".into());
    data.insert("tags".into(), tags.join(","));
    data.insert("signal".into(), signal.clone());
    data.insert("replay_tag".into(), signal.clone());
    data.insert("reason".into(), reason.clone());
    build_finding(signal, pid, 0, TAMPER_RISK, tags, reason, data)
}

pub fn build_finding(
    signal: String,
    pid: u32,
    uid: u32,
    risk: f32,
    tags: Vec<String>,
    reason: String,
    data: HashMap<String, String>,
) -> Finding {
    let trust = TrustEvent {
        timestamp: now_ts(),
        pid: pid as i32,
        ppid: 0,
        uid,
        binary_path: data.get("binary_path").cloned().unwrap_or_default(),
        command_line: String::new(),
        cwd: String::new(),
        anomaly_type: "guard".into(),
        component: "edr_guard".into(),
        metadata: data.clone(),
        risk_score: risk,
        source_module: "edr_guard".into(),
        decay_context: Some(signal.clone()),
        module: Some("edr_guard".into()),
        signal: Some(signal.clone()),
        signal_type: Some("process".into()),
        score: Some(risk),
        raw_score: Some(risk),
        tags: Some(tags),
        description: Some(reason),
    };
    let output = TelemetryOutput {
        category: "process".into(),
        signal,
        confidence: risk / 100.0,
        data,
    };
    Finding { output, trust }
}

// ---- Emission ----

/// Repeats of one decision inside this window are counted, not emitted again,
/// so a client retrying in a loop cannot flood the pipeline. The kernel still
/// refuses every attempt.
pub const REPEAT_WINDOW_NS: u64 = 2_000_000_000;
const MAX_TRACKED_DECISIONS: usize = 4096;

type DecisionKey = (u32, u8, u8, u32, [u8; UNIX_PATH_MAX]);

/// A decision is the caller tgid, kind, action, detail and target.
#[derive(Default)]
pub struct RepeatFilter {
    last_seen: HashMap<DecisionKey, u64>,
    pub suppressed: u64,
}

impl RepeatFilter {
    /// True when the event should be emitted. Uses the kernel timestamp.
    pub fn admit(&mut self, e: &GuardEvent) -> bool {
        let key = (e.tgid, e.kind, e.action, e.detail, e.target);
        let now = e.timestamp_ns;
        if let Some(last) = self.last_seen.get(&key) {
            if now.saturating_sub(*last) < REPEAT_WINDOW_NS {
                self.suppressed += 1;
                return false;
            }
        }
        if self.last_seen.len() >= MAX_TRACKED_DECISIONS {
            self.last_seen
                .retain(|_, last| now.saturating_sub(*last) < REPEAT_WINDOW_NS);
            if self.last_seen.len() >= MAX_TRACKED_DECISIONS {
                self.last_seen.clear();
            }
        }
        self.last_seen.insert(key, now);
        true
    }
}

/// Outputs held for the telemetry sweep. Bounded, oldest dropped first.
const MAX_PENDING_OUTPUTS: usize = 1024;
static PENDING_OUTPUTS: Mutex<VecDeque<TelemetryOutput>> = Mutex::new(VecDeque::new());
/// Alerts held for the batch loop, which turns each into an incident.
static PENDING_ALERTS: Mutex<VecDeque<serde_json::Value>> = Mutex::new(VecDeque::new());

/// The TrustEvent goes out at once. The TelemetryOutput waits for the next
/// telemetry sweep, which writes it to the usual sinks with every other
/// module's output, and the alert waits for the batch loop, which makes it an
/// incident on /incidents.
fn emit(finding: Finding) {
    let alert = guard_alert_json(&finding.output);
    emit_with_alert(finding, alert);
}

/// The same, with an alert the caller built (runtime shields, policy reloads).
pub fn emit_with_alert(finding: Finding, alert: serde_json::Value) {
    submit_trust_event(finding.trust);
    {
        let mut queue = PENDING_OUTPUTS.lock().unwrap_or_else(PoisonError::into_inner);
        if queue.len() >= MAX_PENDING_OUTPUTS {
            queue.pop_front();
        }
        queue.push_back(finding.output);
    }
    let mut alerts = PENDING_ALERTS.lock().unwrap_or_else(PoisonError::into_inner);
    if alerts.len() >= MAX_PENDING_OUTPUTS {
        alerts.pop_front();
    }
    alerts.push_back(alert);
}

/// Called by the batch loop in main.rs.
pub fn take_guard_alerts() -> Vec<serde_json::Value> {
    PENDING_ALERTS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .drain(..)
        .collect()
}

/// One guard decision in the alert shape the incident list reads: ts, severity, risk,
/// technique, findings and event.exe, plus a details block with the guard name, the
/// caller, the target and the mode. A block was made in enforce mode, a would_block in
/// monitor mode. The off switch is high severity because root can throw it too.
pub fn guard_alert_json(out: &TelemetryOutput) -> serde_json::Value {
    let d = &out.data;
    let get = |k: &str| d.get(k).cloned().unwrap_or_default();
    let action = get("action");
    let (mode, severity, verb) = match action.as_str() {
        "blocked" => ("enforce", "high", "blocked"),
        "would_block" => ("monitor", "medium", "would block"),
        _ => ("off_switch", "high", "off switch"),
    };
    let mitre: Vec<String> = get("mitre")
        .split(',')
        .filter(|t| !t.is_empty())
        .map(str::to_string)
        .collect();
    let ts: i64 = get("timestamp").parse().unwrap_or(0);
    let guard = get("guard");
    let exe = d.get("binary_path").cloned().unwrap_or_else(|| "-".into());
    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
    serde_json::json!({
        "id": format!("guard:{}:{}:{}:{}", guard, action, get("pid"), ts),
        "source": "guard",
        "title": format!("Guard {guard} {verb}: {}", get("target")),
        "ts": ts,
        "severity": severity,
        "risk": (out.confidence as f64).clamp(0.0, 1.0),
        "technique": mitre.first().cloned().unwrap_or_else(|| "-".into()),
        "findings": mitre.iter().map(|t| serde_json::json!({ "label": t })).collect::<Vec<_>>(),
        "event": { "exe": exe, "pid": get("pid"), "uid": get("uid"), "comm": get("comm"), "host": host },
        "details": {
            "guard": guard,
            "hook": get("hook"),
            "action": action,
            "mode": mode,
            "caller_exe": exe,
            "caller_pid": get("pid"),
            "caller_comm": get("comm"),
            "target": get("target"),
            "reason": get("reason"),
        },
        "rationale": [get("reason")],
    })
}

/// Called by the telemetry sweep (telemetry.rs).
pub fn take_guard_outputs() -> Vec<TelemetryOutput> {
    PENDING_OUTPUTS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .drain(..)
        .collect()
}

// ---- Start and stop ----

/// What start_guard() set up, for logs and the live harness.
#[derive(Clone, Debug)]
pub struct GuardSummary {
    pub obj_path: String,
    pub modes: GuardModes,
    /// The tgid written to the kernel. None when the agent is not in the
    /// host pid namespace, which also turns self protection off.
    pub agent_tgid: Option<u32>,
    /// cgroup v2 id the guards are limited to, 0 for the whole host.
    pub scope_cgroup_id: u64,
    pub socket_clients: Vec<(PathBuf, ExeId)>,
    pub memfd_allow: Vec<(PathBuf, ExeId)>,
    pub protected_ids: usize,
}

/// Changes the loaded guard while the agent runs, from any thread.
pub struct GuardControl {
    #[cfg(target_os = "linux")]
    maps: loader::GuardMaps,
    state: Mutex<ControlState>,
}

struct ControlState {
    bank: u32,
    modes: GuardModes,
    /// The agent is in the host pid namespace, so self protection can work.
    self_possible: bool,
    /// The off switch was thrown. Self protection stays off until a restart.
    self_off: bool,
}

impl GuardControl {
    #[cfg(target_os = "linux")]
    fn new(maps: loader::GuardMaps, modes: GuardModes, self_possible: bool) -> Self {
        Self {
            maps,
            state: Mutex::new(ControlState { bank: 0, modes, self_possible, self_off: false }),
        }
    }

    pub fn modes(&self) -> GuardModes {
        self.state.lock().unwrap_or_else(PoisonError::into_inner).modes
    }

    /// New modes and allowlists in one bank switch. Returns the modes in force,
    /// which keep self protection off when the agent is outside the host pid
    /// namespace or the off switch was thrown.
    pub fn apply(
        &self,
        mut modes: GuardModes,
        socket: &[(PathBuf, ExeId)],
        memfd: &[(PathBuf, ExeId)],
    ) -> anyhow::Result<GuardModes> {
        let mut st = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if !st.self_possible || st.self_off {
            modes.self_protect = GuardMode::Off;
        }
        #[cfg(target_os = "linux")]
        {
            let s: Vec<ExeId> = socket.iter().map(|(_, id)| *id).collect();
            let m: Vec<ExeId> = memfd.iter().map(|(_, id)| *id).collect();
            st.bank = self.maps.switch(st.bank, &modes, &s, &m)?;
        }
        #[cfg(not(target_os = "linux"))]
        let _ = (socket, memfd);
        st.modes = modes;
        Ok(modes)
    }

    pub fn self_off(&self) -> anyhow::Result<()> {
        let mut st = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        st.self_off = true;
        st.modes.self_protect = GuardMode::Off;
        #[cfg(target_os = "linux")]
        self.maps.self_off()?;
        Ok(())
    }

    /// Adds BPF objects (bpf() command that opens them, id) to protected_ids.
    pub fn protect_ids(&self, ids: &[(u32, u32)]) -> anyhow::Result<()> {
        #[cfg(target_os = "linux")]
        self.maps.protect_ids(ids)?;
        #[cfg(not(target_os = "linux"))]
        let _ = ids;
        Ok(())
    }

    /// Write protects files by inode. Paths that do not exist are skipped and
    /// named in the result.
    pub fn protect_files(&self, paths: &[PathBuf]) -> anyhow::Result<Vec<String>> {
        let mut missing = Vec::new();
        let mut ids = Vec::new();
        for p in paths {
            match fs::metadata(p) {
                Ok(m) => ids.push(ExeId::from_stat(m.dev(), m.ino())),
                Err(_) => missing.push(p.display().to_string()),
            }
        }
        #[cfg(target_os = "linux")]
        self.maps.protect_files(&ids)?;
        Ok(missing)
    }

    pub fn unprotect_files(&self, paths: &[PathBuf]) -> anyhow::Result<()> {
        let ids: Vec<ExeId> = paths
            .iter()
            .filter_map(|p| fs::metadata(p).ok())
            .map(|m| ExeId::from_stat(m.dev(), m.ino()))
            .collect();
        #[cfg(target_os = "linux")]
        self.maps.unprotect_files(&ids)?;
        #[cfg(not(target_os = "linux"))]
        let _ = ids;
        Ok(())
    }
}

/// Every BPF object the given fdinfo text names, as (bpf() command that opens it,
/// id). A link's fdinfo names its program too.
pub fn fdinfo_ids(text: &str) -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let (key, value) = match line.split_once(':') {
            Some((k, v)) => (k.trim(), v.trim()),
            None => continue,
        };
        let cmd = match key {
            "prog_id" => abi::BPF_PROG_GET_FD_BY_ID,
            "map_id" => abi::BPF_MAP_GET_FD_BY_ID,
            "link_id" => abi::BPF_LINK_GET_FD_BY_ID,
            _ => continue,
        };
        if let Ok(id) = value.parse::<u32>() {
            if id != 0 && !out.contains(&(cmd, id)) {
                out.push((cmd, id));
            }
        }
    }
    out
}

/// The BPF objects this process holds a descriptor for, from /proc/self/fdinfo: the
/// sensor maps the readers opened, and the objects the agent loaded itself.
pub fn held_bpf_ids() -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    let Ok(rd) = fs::read_dir("/proc/self/fdinfo") else {
        return out;
    };
    for e in rd.flatten() {
        if let Ok(text) = fs::read_to_string(e.path()) {
            for id in fdinfo_ids(&text) {
                if !out.contains(&id) {
                    out.push(id);
                }
            }
        }
    }
    out
}

/// The objects pinned under a bpffs directory and the pin files and directories
/// themselves. Each pin is opened once to read its ids.
#[cfg(target_os = "linux")]
pub fn pinned_objects(dir: &Path) -> (Vec<(u32, u32)>, Vec<PathBuf>) {
    use std::os::unix::ffi::OsStrExt;
    let mut ids = Vec::new();
    let mut paths = vec![dir.to_path_buf()];
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            paths.push(p.clone());
            if p.is_dir() {
                stack.push(p);
                continue;
            }
            let Ok(c) = std::ffi::CString::new(p.as_os_str().as_bytes()) else { continue };
            // SAFETY: c is a NUL terminated path. The fd is closed below.
            let fd = unsafe { libbpf_rs::libbpf_sys::bpf_obj_get(c.as_ptr()) };
            if fd < 0 {
                continue;
            }
            if let Ok(text) = fs::read_to_string(format!("/proc/self/fdinfo/{fd}")) {
                for id in fdinfo_ids(&text) {
                    if !ids.contains(&id) {
                        ids.push(id);
                    }
                }
            }
            // SAFETY: fd came from bpf_obj_get and is closed once.
            unsafe { libc::close(fd) };
        }
    }
    (ids, paths)
}

/// Adds every BPF object the agent holds, and the sensors pinned under `pin_dir`
/// (their links, programs and maps, and the pin files), to the guard's protection.
/// Without this, root could open a sensor map such as the self filter, or remove a
/// sensor's link pin, which detaches the sensor and blinds the agent. Runs again
/// every 30 s, for objects loaded later.
pub fn spawn_protect_held(control: Arc<GuardControl>, pin_dir: PathBuf) {
    let _ = std::thread::Builder::new().name("guard_protect".into()).spawn(move || {
        let mut seen_ids = 0usize;
        let mut seen_pins = 0usize;
        loop {
            let mut ids = held_bpf_ids();
            #[cfg(target_os = "linux")]
            let pins = {
                let (pinned, pins) = pinned_objects(&pin_dir);
                for id in pinned {
                    if !ids.contains(&id) {
                        ids.push(id);
                    }
                }
                pins
            };
            #[cfg(not(target_os = "linux"))]
            let pins: Vec<PathBuf> = Vec::new();
            if ids.len() != seen_ids || pins.len() != seen_pins {
                let a = control.protect_ids(&ids);
                let b = control.protect_files(&pins);
                match (a, b) {
                    (Ok(()), Ok(_)) => log::info!(
                        "guard: {} held or pinned BPF objects and {} sensor pins under {} protected",
                        ids.len(),
                        pins.len(),
                        pin_dir.display()
                    ),
                    (a, b) => log::warn!("guard: protecting held objects: {:?} {:?}", a.err(), b.err()),
                }
                seen_ids = ids.len();
                seen_pins = pins.len();
            }
            std::thread::sleep(std::time::Duration::from_secs(30));
        }
    });
}

/// Owns the guard thread. Dropping it stops the guards the same way stop() does.
pub struct GuardHandle {
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
    pub summary: GuardSummary,
    pub control: Arc<GuardControl>,
}

impl GuardHandle {
    /// Turns self protection off first, then the other guards, then detaches
    /// every program, and returns when that is done. The agent can then be
    /// stopped like any other process.
    pub fn stop(mut self) {
        self.shutdown();
    }

    fn shutdown(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            if thread.join().is_err() {
                log::warn!("guard: the guard thread panicked during shutdown");
            }
        }
    }
}

impl Drop for GuardHandle {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Loads and attaches the guards with settings from the environment.
pub fn start_guard() -> anyhow::Result<GuardHandle> {
    start_guard_with(GuardSettings::from_env(), |_, _| {})
}

/// Same, with every guard at least `floor`, the signed policy's guard_mode.
pub fn start_guard_at_least(floor: GuardMode) -> anyhow::Result<GuardHandle> {
    let mut settings = GuardSettings::from_env();
    let (modes, warnings) = settings.modes.at_least(floor);
    for w in warnings {
        eprintln!("⚠️ guard: {w}");
        log::warn!("guard: {w}");
    }
    settings.modes = modes;
    start_guard_with(settings, |_, _| {})
}

/// Settings for a signed policy: the environment's modes raised to the
/// policy's guard_mode, and the policy's allowlist entries.
pub fn settings_for_policy(policy: &crate::config::RawPolicy) -> (GuardSettings, Vec<String>) {
    let mut settings = GuardSettings::from_env();
    let (modes, warnings) = settings.modes.at_least(policy.guard_floor());
    settings.modes = modes;
    settings.policy_allow = policy.guard_allow.clone();
    (settings, warnings)
}

/// Same, with explicit settings and a callback that sees every decision the
/// agent emits (the live harness prints them). On failure it logs one line
/// saying the agent stays detect-only, and returns the reason.
pub fn start_guard_with(
    settings: GuardSettings,
    on_event: impl FnMut(&GuardEvent, &Finding) + Send + 'static,
) -> anyhow::Result<GuardHandle> {
    let result = start_inner(settings, Box::new(on_event));
    match &result {
        Ok(h) => log::info!(
            "guard: prevention mode on: runtime_socket={} memfd_exec={} self_protect={} ({})",
            h.summary.modes.socket.as_str(),
            h.summary.modes.memfd.as_str(),
            h.summary.modes.self_protect.as_str(),
            h.summary.obj_path
        ),
        Err(e) => log::warn!("guard: prevention mode is off, the agent stays detect-only: {e:#}"),
    }
    result
}

type EventCallback = Box<dyn FnMut(&GuardEvent, &Finding) + Send>;

#[cfg(target_os = "linux")]
fn start_inner(settings: GuardSettings, on_event: EventCallback) -> anyhow::Result<GuardHandle> {
    if settings.modes.all_off() {
        anyhow::bail!("every guard is off (EDR_GUARD_MODE)");
    }
    loader::check_bpf_lsm()?;
    loader::spawn(settings, on_event)
}

#[cfg(not(target_os = "linux"))]
fn start_inner(_settings: GuardSettings, _on_event: EventCallback) -> anyhow::Result<GuardHandle> {
    anyhow::bail!("BPF LSM guards need Linux")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |name: &str| map.get(name).cloned()
    }

    #[test]
    fn fdinfo_names_maps_programs_and_links() {
        let map = "pos:\t0\nflags:\t02000002\nmap_type:\t11\nmap_id:\t189\n";
        assert_eq!(fdinfo_ids(map), vec![(abi::BPF_MAP_GET_FD_BY_ID, 189)]);
        let link = "link_type:\ttracing\nlink_id:\t181\nprog_tag:\tabc\nprog_id:\t459\n";
        assert_eq!(fdinfo_ids(link), vec![(abi::BPF_LINK_GET_FD_BY_ID, 181), (abi::BPF_PROG_GET_FD_BY_ID, 459)]);
        assert!(fdinfo_ids("pos:\t0\nflags:\t0100002\nmnt_id:\t25\n").is_empty());
        assert!(fdinfo_ids("map_id:\t0\n").is_empty());
    }

    #[test]
    fn mode_parsing_accepts_the_three_names_only() {
        assert_eq!(GuardMode::parse("off"), Some(GuardMode::Off));
        assert_eq!(GuardMode::parse(" Monitor "), Some(GuardMode::Monitor));
        assert_eq!(GuardMode::parse("ENFORCE"), Some(GuardMode::Enforce));
        assert_eq!(GuardMode::parse("enfroce"), None);
        assert_eq!(GuardMode::parse(""), None);
        assert_eq!(GuardMode::parse("1"), None);
        // MODE_* values in the C file.
        assert_eq!(
            [GuardMode::Off, GuardMode::Monitor, GuardMode::Enforce].map(GuardMode::as_raw),
            [0, 1, 2]
        );
    }

    #[test]
    fn the_policy_floor_raises_modes_and_never_lowers_them() {
        let (m, _) = GuardModes::from_vars(vars(&[("EDR_GUARD_MODE", "off"), ("EDR_GUARD_SOCKET_MODE", "enforce")]));
        let (lifted, warnings) = m.at_least(GuardMode::Monitor);
        assert_eq!(lifted.socket, GuardMode::Enforce, "a stricter variable stays");
        assert_eq!(lifted.memfd, GuardMode::Monitor);
        assert_eq!(lifted.self_protect, GuardMode::Monitor);
        assert_eq!(warnings.len(), 2, "{warnings:?}");
        let (same, warnings) = lifted.at_least(GuardMode::Off);
        assert_eq!(same, lifted);
        assert!(warnings.is_empty());
    }

    #[test]
    fn modes_default_to_monitor() {
        let (m, warnings) = GuardModes::from_vars(vars(&[]));
        assert_eq!(m.socket, GuardMode::Monitor);
        assert_eq!(m.memfd, GuardMode::Monitor);
        assert_eq!(m.self_protect, GuardMode::Monitor);
        assert!(warnings.is_empty());
    }

    #[test]
    fn per_guard_variables_override_the_global_mode() {
        let (m, _) = GuardModes::from_vars(vars(&[
            ("EDR_GUARD_MODE", "enforce"),
            ("EDR_GUARD_SELF_MODE", "off"),
        ]));
        assert_eq!(m.socket, GuardMode::Enforce);
        assert_eq!(m.memfd, GuardMode::Enforce);
        assert_eq!(m.self_protect, GuardMode::Off);
        assert!(!m.all_off());

        let (m, _) = GuardModes::from_vars(vars(&[("EDR_GUARD_MODE", "off")]));
        assert!(m.all_off());
        let (m, _) = GuardModes::from_vars(vars(&[
            ("EDR_GUARD_MODE", "off"),
            ("EDR_GUARD_MEMFD_MODE", "monitor"),
        ]));
        assert_eq!((m.socket, m.memfd), (GuardMode::Off, GuardMode::Monitor));
    }

    #[test]
    fn a_bad_value_keeps_the_default_and_warns() {
        let (m, warnings) = GuardModes::from_vars(vars(&[
            ("EDR_GUARD_MODE", "enforce"),
            ("EDR_GUARD_SOCKET_MODE", "block"),
        ]));
        // The typo falls back to the global mode, not to off.
        assert_eq!(m.socket, GuardMode::Enforce);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("EDR_GUARD_SOCKET_MODE"));

        let (m, warnings) = GuardModes::from_vars(vars(&[("EDR_GUARD_MODE", "yes")]));
        assert_eq!(m.memfd, GuardMode::Monitor);
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn object_path_lookup_order() {
        assert_eq!(object_path(Some("/x/g.o".into()), Some("/d".into())), "/x/g.o");
        assert_eq!(object_path(None, Some("/opt/bpf/".into())), "/opt/bpf/edr_guard.bpf.o");
        assert_eq!(object_path(None, None), "src/ebpf/edr_guard.bpf.o");
        assert_eq!(
            split_paths(" /a/b : :/c "),
            vec![PathBuf::from("/a/b"), PathBuf::from("/c")]
        );
    }

    #[test]
    fn lsm_list_check() {
        assert!(bpf_lsm_listed("capability,bpf,landlock"));
        assert!(bpf_lsm_listed("lockdown,capability,landlock,yama,apparmor,bpf\n"));
        // Ubuntu 22.04 default: no bpf.
        assert!(!bpf_lsm_listed("lockdown,capability,landlock,yama,apparmor"));
        assert!(!bpf_lsm_listed("bpfx,selinux"));
        assert!(!bpf_lsm_listed(""));
    }

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("edr_guard_{name}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn client_search_uses_the_suspicious_ipc_list_and_dedupes_by_inode() {
        let dir = temp_dir("clients");
        for name in ["docker", "containerd-shim-runc-v2", "python3", "dockerx"] {
            fs::write(dir.join(name), b"x").unwrap();
        }
        fs::create_dir(dir.join("kubelet")).unwrap(); // a directory, not a program
        let d = dir.to_str().unwrap();
        let found = find_programs(&[d], crate::modules::suspicious_ipc::RUNTIME_CLIENT_ALLOWLIST);
        assert_eq!(found, vec![dir.join("containerd-shim-runc-v2"), dir.join("docker")]);

        // A symlink to docker names the same inode, so it adds nothing.
        std::os::unix::fs::symlink(dir.join("docker"), dir.join("docker-link")).unwrap();
        let ids = exe_ids(&[dir.join("docker"), dir.join("docker-link"), dir.join("missing")]);
        assert_eq!(ids.len(), 1);
        let meta = fs::metadata(dir.join("docker")).unwrap();
        assert_eq!(ids[0].1, ExeId::from_stat(meta.dev(), meta.ino()));
        // A copy is a different file, so it is not allowed by the original's entry.
        fs::copy(dir.join("docker"), dir.join("copy")).unwrap();
        assert_eq!(exe_ids(&[dir.join("docker"), dir.join("copy")]).len(), 2);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn off_switch_needs_the_file_and_the_token_when_a_hash_is_set() {
        let dir = temp_dir("off");
        let file = dir.join("off");
        assert!(!off_switch_thrown(&file, None));
        fs::write(&file, b"").unwrap();
        assert!(off_switch_thrown(&file, None));
        let hash = hex::encode(Sha256::digest(b"open sesame"));
        assert!(!off_switch_thrown(&file, Some(&hash)));
        fs::write(&file, b"open sesame\n").unwrap();
        assert!(off_switch_thrown(&file, Some(&hash)));
        assert!(off_switch_thrown(&file, Some(&hash.to_uppercase())));
        fs::write(&file, b"wrong").unwrap();
        assert!(!off_switch_thrown(&file, Some(&hash)));
        let _ = fs::remove_dir_all(&dir);
    }

    fn event(kind: u8, action: u8, comm: &str, target: &str) -> GuardEvent {
        let mut e = GuardEvent {
            timestamp_ns: 5_000_000_000,
            exe_ino: 99,
            target_ino: 7,
            exe_dev: 0x2e,
            tgid: 4321,
            tid: 4321,
            uid: 0,
            target_pid: 0,
            detail: 0,
            kind,
            action,
            pad: 0,
            comm: [0; abi::TASK_COMM_LEN],
            exe_name: [0; abi::EXE_NAME_LEN],
            target: [0; UNIX_PATH_MAX],
        };
        e.comm[..comm.len()].copy_from_slice(comm.as_bytes());
        e.exe_name[..7].copy_from_slice(b"python3");
        e.target[..target.len()].copy_from_slice(target.as_bytes());
        e
    }

    #[test]
    fn socket_finding_carries_guard_tags_and_mitre() {
        let e = event(1, 2, "docker", "/var/run/docker.sock");
        let f = guard_finding(&e, Some("/usr/bin/python3.11")).unwrap();
        assert_eq!(f.output.signal, "guard::runtime_socket::blocked");
        assert_eq!(f.output.data["action"], "blocked");
        assert_eq!(f.output.data["target"], "/var/run/docker.sock");
        assert_eq!(f.output.data["binary_path"], "/usr/bin/python3.11");
        assert_eq!(f.output.data["mitre"], "T1611,T1610");
        let tags = f.trust.tags.unwrap();
        for t in ["guard", "guard:blocked", "guard:runtime_socket", "T1611", "T1610"] {
            assert!(tags.contains(&t.to_string()), "missing tag {t}");
        }
        assert!(f.trust.description.unwrap().starts_with("blocked: docker (pid 4321"));
        assert_eq!(f.trust.pid, 4321);
    }

    #[test]
    fn monitor_and_self_protect_findings() {
        let mut e = event(3, 1, "bash", "");
        e.detail = 9;
        e.target_pid = 100;
        // No /proc path: the kernel's file name stands in.
        let f = guard_finding(&e, None).unwrap();
        assert_eq!(f.output.signal, "guard::self_protect::would_block");
        assert_eq!(f.output.data["binary_path"], "python3");
        assert!(f.output.data["reason"].contains("SIGKILL to the agent (pid 100)"));
        assert!(f.trust.tags.unwrap().contains(&"T1562.001".to_string()));

        let f = guard_finding(&event(2, 2, "python3", "memfd:payload"), None).unwrap();
        assert_eq!(f.output.data["mitre"], "T1620");
        assert_eq!(f.output.data["guard"], "memfd_exec");

        // Unknown kind or action: no finding.
        assert!(guard_finding(&event(9, 2, "x", ""), None).is_none());
        assert!(guard_finding(&event(1, 0, "x", ""), None).is_none());
    }

    #[test]
    fn repeats_inside_the_window_are_counted_not_emitted() {
        let mut filter = RepeatFilter::default();
        let mut e = event(1, 2, "curl", "/run/docker.sock");
        assert!(filter.admit(&e));
        e.timestamp_ns += REPEAT_WINDOW_NS - 1;
        assert!(!filter.admit(&e));
        // Another target or another process is a new decision.
        let other = event(1, 2, "curl", "/run/containerd/containerd.sock");
        assert!(filter.admit(&other));
        let mut other_pid = e;
        other_pid.tgid = 1;
        assert!(filter.admit(&other_pid));
        // After the window the same decision is emitted again.
        e.timestamp_ns += REPEAT_WINDOW_NS;
        assert!(filter.admit(&e));
        assert_eq!(filter.suppressed, 1);
    }

    #[test]
    fn a_decision_becomes_an_incident_alert_with_guard_caller_target_and_mode() {
        let blocked = guard_finding(&event(1, 2, "docker", "/var/run/docker.sock"), Some("/usr/bin/python3.11")).unwrap();
        let a = guard_alert_json(&blocked.output);
        assert_eq!(a["source"], "guard");
        assert_eq!(a["severity"], "high");
        assert_eq!(a["technique"], "T1611");
        assert_eq!(a["event"]["exe"], "/usr/bin/python3.11");
        assert_eq!(a["details"]["guard"], "runtime_socket");
        assert_eq!(a["details"]["caller_exe"], "/usr/bin/python3.11");
        assert_eq!(a["details"]["target"], "/var/run/docker.sock");
        assert_eq!(a["details"]["mode"], "enforce");
        assert_eq!(a["findings"][1]["label"], "T1610");

        let mut e = event(3, 1, "bash", "");
        e.detail = 9;
        e.target_pid = 100;
        let a = guard_alert_json(&guard_finding(&e, Some("/usr/bin/bash")).unwrap().output);
        assert_eq!(a["details"]["mode"], "monitor");
        assert_eq!(a["details"]["action"], "would_block");
        assert_eq!(a["severity"], "medium");
        assert_eq!(a["technique"], "T1562.001");
        assert_eq!(a["details"]["target"], "pid 100");

        // emit() queues the alert for the batch loop.
        let mut unique = event(2, 2, "python3", "memfd:only-in-this-test");
        unique.tgid = 777_777;
        let f = guard_finding(&unique, Some("/usr/bin/python3")).unwrap();
        emit(f);
        let alerts = take_guard_alerts();
        let mine: Vec<_> = alerts.iter().filter(|a| a["details"]["target"] == "memfd:only-in-this-test").collect();
        assert_eq!(mine.len(), 1);
        assert_eq!(mine[0]["details"]["guard"], "memfd_exec");
        assert_eq!(mine[0]["details"]["caller_pid"], "777777");
    }

    #[test]
    fn output_queue_is_bounded_and_drains() {
        for _ in 0..(MAX_PENDING_OUTPUTS + 5) {
            emit(off_switch_finding(Path::new("/run/edr-guard/off")));
        }
        let outputs = take_guard_outputs();
        assert_eq!(outputs.len(), MAX_PENDING_OUTPUTS);
        assert_eq!(outputs[0].signal, "guard::self_protect::off_switch");
        assert!(take_guard_outputs().is_empty());
    }
}

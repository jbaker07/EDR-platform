// src/pb_adapter/mod.rs
// The playbook engine's input. Every decoder and the side-effect monitors call
// emit_adapter_facts with their TelemetryOutput. That output still has the full data map
// (open flags, syscall return, rename target, byte counts), which TelemetryRecord drops,
// so this is the one path facts take into the engine. queue.rs holds them until the
// engine thread in runner.rs turns them into engine facts (pb_wiring) and feeds the
// engine.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use serde_json::Value;

use crate::pb_engine::PlaybookEngine;
use crate::telemetry_types::TelemetryOutput;
use crate::utils::time::now_ts;
use std::sync::OnceLock;

pub mod clock;
mod fact;
pub mod queue;
pub mod router;
pub mod runner;

pub use fact::AdapterFact;
use queue::{FactQueue, QueueStats};

static ENGINE: OnceLock<Arc<Mutex<PlaybookEngine>>> = OnceLock::new();
static QUEUE: OnceLock<Arc<FactQueue<AdapterFact>>> = OnceLock::new();
static METRICS: OnceLock<Arc<crate::metrics::Metrics>> = OnceLock::new();
/// Outputs left out before the queue because no playbook can use them.
static SKIPPED: AtomicU64 = AtomicU64::new(0);

pub fn set_engine_handle(engine: Arc<Mutex<PlaybookEngine>>) {
    let _ = ENGINE.set(engine);
}

pub fn with_engine() -> Option<Arc<Mutex<PlaybookEngine>>> {
    ENGINE.get().cloned()
}

/// The queue the decoders push into. EDR_ADAPTER_QUEUE sets its size (default 32768)
/// and EDR_ADAPTER_PUSH_TIMEOUT_MS how long a producer waits on a full queue (default
/// 250). Called once at startup, before any reader runs.
pub fn init_queue() -> Arc<FactQueue<AdapterFact>> {
    QUEUE
        .get_or_init(|| {
            let cap = std::env::var("EDR_ADAPTER_QUEUE")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .filter(|n| *n > 0)
                .unwrap_or(queue::DEFAULT_CAPACITY);
            let timeout = std::env::var("EDR_ADAPTER_PUSH_TIMEOUT_MS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .map(std::time::Duration::from_millis)
                .unwrap_or(queue::DEFAULT_PUSH_TIMEOUT);
            Arc::new(FactQueue::new(cap, timeout))
        })
        .clone()
}

/// Queue counters, and how many outputs were left out before the queue.
pub fn queue_stats() -> Option<(QueueStats, u64)> {
    QUEUE.get().map(|q| (q.stats(), SKIPPED.load(Ordering::Relaxed)))
}

pub fn set_metrics_handle(metrics: Arc<crate::metrics::Metrics>) {
    let _ = METRICS.set(metrics);
}

pub fn metrics_handle() -> Option<Arc<crate::metrics::Metrics>> {
    METRICS.get().cloned()
}

pub fn emit_adapter_facts(output: &TelemetryOutput) {
    let Some(q) = QUEUE.get() else {
        return;
    };
    if !crate::pb_wiring::mapper_wants(&output.category, &output.signal, &output.data) {
        SKIPPED.fetch_add(1, Ordering::Relaxed);
        return;
    }

    let facts = adapter_facts_with(output, now_ts(), &read_proc);
    for fact in facts {
        if !q.push(fact) {
            crate::metrics::mapper_dropped_facts_total().fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// A rules.json alert as a RuleHit fact. `alert` is the evaluator's alert JSON.
pub fn emit_rule_hit(alert: &Value) {
    let findings = alert.get("findings").and_then(|f| f.as_array()).cloned().unwrap_or_default();
    for f in findings {
        if f.get("source").and_then(|s| s.as_str()) != Some("rules") {
            continue;
        }
        let rule_id = f.get("label").and_then(|l| l.as_str()).unwrap_or("").to_string();
        let ttps: Vec<String> = f
            .pointer("/details/mitre")
            .and_then(|m| m.as_array())
            .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
            .unwrap_or_default();
        let text = |ptr: &str| alert.pointer(ptr).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let mut data = std::collections::HashMap::new();
        data.insert("source".to_string(), "rule_engine".to_string());
        data.insert("rule_id".to_string(), rule_id);
        data.insert(
            "rule_name".to_string(),
            f.pointer("/details/name").and_then(|n| n.as_str()).unwrap_or("").to_string(),
        );
        data.insert("ttps".to_string(), ttps.join(","));
        data.insert(
            "severity".to_string(),
            f.get("score").and_then(|s| s.as_f64()).unwrap_or(0.0).to_string(),
        );
        data.insert("exe".to_string(), text("/event/exe"));
        data.insert("cmd".to_string(), text("/event/cmdline"));
        if let Some(pid) = alert.pointer("/event/pid").and_then(|p| p.as_i64()) {
            data.insert("pid".to_string(), pid.to_string());
        }
        emit_adapter_facts(&TelemetryOutput {
            category: "rules".into(),
            signal: "rules::hit".into(),
            confidence: 1.0,
            data,
        });
    }
}

/// Build the adapter facts for one output and fill in what /proc knows. `now` is used
/// when the output has no stamp. `read` stands in for /proc in tests.
pub fn adapter_facts_with(
    output: &TelemetryOutput,
    now: u64,
    read: &dyn Fn(i64) -> ProcInfo,
) -> Vec<AdapterFact> {
    let mut facts = router::to_adapter_facts_at(output, now);
    for fact in facts.iter_mut() {
        enrich_from_proc(fact, read);
    }
    facts
}

/// What /proc says about a process. Fields are None once it has exited.
#[derive(Debug, Default, Clone)]
pub struct ProcInfo {
    pub exe: Option<String>,
    pub cmdline: Option<String>,
    pub cwd: Option<String>,
    /// Allowlisted environment variables (env_allowlist), name and value.
    pub env: Vec<(String, String)>,
}

/// Environment variables read at exec. Only these names are kept, so the rest of the
/// environment, where secrets live, is never stored. EDR_ENV_ALLOWLIST (comma
/// separated) replaces the list, and a name that looks like a secret is refused.
pub const DEFAULT_ENV_ALLOWLIST: &[&str] = &[
    "CI",
    "GITHUB_ACTIONS",
    "GITLAB_CI",
    "JENKINS_URL",
    "BUILDKITE",
    "TF_BUILD",
    "CIRCLECI",
    "TEAMCITY_VERSION",
    "DRONE",
];

pub fn env_allowlist() -> &'static [String] {
    static L: OnceLock<Vec<String>> = OnceLock::new();
    L.get_or_init(|| {
        let names: Vec<String> = match std::env::var("EDR_ENV_ALLOWLIST") {
            Ok(v) if !v.trim().is_empty() => v.split(',').map(|s| s.trim().to_string()).collect(),
            _ => DEFAULT_ENV_ALLOWLIST.iter().map(|s| s.to_string()).collect(),
        };
        names
            .into_iter()
            .filter(|n| {
                let ok = !n.is_empty() && !crate::redact::is_secret_name(n);
                if !ok && !n.is_empty() {
                    log::warn!("EDR_ENV_ALLOWLIST: {n} looks like a secret and is not read");
                }
                ok
            })
            .collect()
    })
}

/// The allowlisted entries of a NUL separated environ block. Values are capped at 128
/// bytes and go through the redactor anyway.
pub fn allowlisted_env(environ: &[u8], allow: &[String]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for entry in environ.split(|b| *b == 0) {
        let Ok(s) = std::str::from_utf8(entry) else {
            continue;
        };
        let Some((k, v)) = s.split_once('=') else {
            continue;
        };
        if allow.iter().any(|a| a == k) {
            let v: String = v.chars().take(128).collect();
            out.push((k.to_string(), crate::redact::redact_owned(v)));
        }
    }
    out
}

pub fn read_proc(pid: i64) -> ProcInfo {
    if pid <= 0 {
        return ProcInfo::default();
    }
    let exe = std::fs::read_link(format!("/proc/{pid}/exe"))
        .ok()
        .map(|p| p.to_string_lossy().into_owned());
    // Redacted here, before the command line reaches any fact, record or log.
    let cmdline = std::fs::read(format!("/proc/{pid}/cmdline")).ok().and_then(|b| {
        let s = crate::redact::cmdline_from_proc(&b);
        (!s.is_empty()).then_some(s)
    });
    let cwd = std::fs::read_link(format!("/proc/{pid}/cwd"))
        .ok()
        .map(|p| p.to_string_lossy().into_owned());
    let env = std::fs::read(format!("/proc/{pid}/environ"))
        .map(|b| allowlisted_env(&b, env_allowlist()))
        .unwrap_or_default();
    ProcInfo { exe, cmdline, cwd, env }
}

/// Exec and memory frames carry no resolved exe path, and some file paths are relative.
/// /proc is read here, in the decoder thread right after the frame arrives, while the
/// process most likely still exists. Fields a decoder already set are kept.
/// proc_lifecycle exec frames also carry the filename and argv the kernel copied at exec
/// (exec_filename, exec_argv). Those fill in when the process is already gone from /proc,
/// and they win when /proc already shows a later program: a shell that ran its last
/// command in place (`bash -c '...; sleep 75'` becomes sleep in the same pid) within a
/// millisecond, or a pid that was reused. /proc is taken only when its command line
/// starts with the argv the kernel copied (the copy stops at 256 bytes).
pub fn enrich_from_proc(fact: &mut AdapterFact, read: &dyn Fn(i64) -> ProcInfo) {
    let Some(meta) = fact.meta.as_object_mut() else {
        return;
    };
    let text = |k: &str| meta.get(k).and_then(|v| v.as_str()).unwrap_or("").to_string();
    let source = text("source");
    let signal = text("signal");
    let comm = text("comm");
    let non_empty = |s: String| (!s.is_empty()).then_some(s);
    let kernel_exe = non_empty(text("exec_filename"));
    let kernel_argv = non_empty(text("exec_argv"));
    let is_exec = (source == "proc_lifecycle" && signal == "process::exec")
        || (source == "wx_exec"
            && matches!(signal.as_str(), "memory::memfd_exec" | "memory::rwx_map"));
    let path = text("path");
    let relative = !path.is_empty() && !path.starts_with('/') && text("category") == "file";
    if !is_exec && !relative {
        return;
    }
    let pid = ["pid", "tgid"]
        .iter()
        .find_map(|k| meta.get(*k).and_then(|v| v.as_i64()))
        .unwrap_or(0);
    let info = read(pid);
    let mut put = |k: &str, v: Option<String>| {
        if let Some(v) = v {
            if !meta.contains_key(k) {
                meta.insert(k.to_string(), Value::String(v));
            }
        }
    };
    if is_exec {
        let stale = match (&info.cmdline, &kernel_argv) {
            (Some(p), Some(k)) => !p.starts_with(k.as_str()) && !k.starts_with(p.as_str()),
            _ => false,
        };
        if stale {
            note_stale_proc(pid, &comm);
            put("binary_path", kernel_exe.or(info.exe));
            put("command_line", kernel_argv);
            // The cwd in /proc belongs to the later program too.
            return;
        }
        if info.exe.is_none() {
            if kernel_exe.is_some() {
                note_exec_from_kernel_copy(pid, &comm);
            } else {
                note_unresolved_exec(pid, &comm);
            }
        }
        put("binary_path", info.exe.or(kernel_exe));
        put("command_line", info.cmdline.or(kernel_argv));
        for (k, v) in info.env {
            put(&format!("env.{k}"), Some(v));
        }
    }
    put("cwd", info.cwd);
}

static EXEC_STALE_PROC: AtomicU64 = AtomicU64::new(0);

/// Exec frames whose /proc entry already showed a later program, filled from the
/// kernel copy instead.
pub fn exec_stale_proc_total() -> u64 {
    EXEC_STALE_PROC.load(Ordering::Relaxed)
}

fn note_stale_proc(pid: i64, comm: &str) {
    let n = EXEC_STALE_PROC.fetch_add(1, Ordering::Relaxed) + 1;
    if n <= 20 || n % 1000 == 0 {
        use std::io::Write;
        let line = format!(
            "pb_adapter: exec pid {pid} ({comm}) had already run another program by the time /proc was read, used the kernel copy ({n} so far)\n"
        );
        let _ = std::io::stderr().write_all(line.as_bytes());
    }
}

static EXEC_UNRESOLVED: AtomicU64 = AtomicU64::new(0);
static EXEC_FROM_KERNEL: AtomicU64 = AtomicU64::new(0);

/// Exec frames whose process had exited before /proc could be read and that carried no
/// kernel copy either (wx_exec, or an older proc_lifecycle object). Those facts have no
/// exe or argv, so a slot that tests them cannot match.
pub fn exec_unresolved_total() -> u64 {
    EXEC_UNRESOLVED.load(Ordering::Relaxed)
}

/// Exec frames whose process had exited before /proc could be read, filled from the
/// filename and argv proc_lifecycle copied in the kernel.
pub fn exec_from_kernel_copy_total() -> u64 {
    EXEC_FROM_KERNEL.load(Ordering::Relaxed)
}

fn note_exec_from_kernel_copy(pid: i64, comm: &str) {
    let n = EXEC_FROM_KERNEL.fetch_add(1, Ordering::Relaxed) + 1;
    if n <= 20 || n % 1000 == 0 {
        use std::io::Write;
        let line = format!(
            "pb_adapter: exec pid {pid} ({comm}) exited before /proc was read, used the kernel copy ({n} so far)\n"
        );
        let _ = std::io::stderr().write_all(line.as_bytes());
    }
}

fn note_unresolved_exec(pid: i64, comm: &str) {
    let n = EXEC_UNRESOLVED.fetch_add(1, Ordering::Relaxed) + 1;
    if n <= 200 || n % 1000 == 0 {
        // One write per line, so other threads' output cannot split it.
        use std::io::Write;
        let line = format!("pb_adapter: exec pid {pid} ({comm}) exited before /proc was read ({n} so far)\n");
        let _ = std::io::stderr().write_all(line.as_bytes());
    }
}

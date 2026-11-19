// src/pb_adapter/mod.rs
// The playbook engine's input. Every decoder and the side-effect monitors call
// emit_adapter_facts with their TelemetryOutput. That output still has the full data map
// (open flags, syscall return, rename target, byte counts), which TelemetryRecord drops,
// so this is the one path facts take into the engine. runner.rs turns them into engine
// facts (pb_wiring) and feeds the engine.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use serde_json::Value;
use tokio::sync::mpsc::Sender;

use crate::pb_engine::PlaybookEngine;
use crate::telemetry_types::TelemetryOutput;
use crate::utils::time::now_ts;
use std::sync::OnceLock;

pub mod clock;
mod fact;
pub mod router;
pub mod runner;

pub use fact::AdapterFact;

static ENGINE: OnceLock<Arc<Mutex<PlaybookEngine>>> = OnceLock::new();
static SENDER: OnceLock<Sender<AdapterFact>> = OnceLock::new();
static METRICS: OnceLock<Arc<crate::metrics::Metrics>> = OnceLock::new();

pub fn set_engine_handle(engine: Arc<Mutex<PlaybookEngine>>) {
    let _ = ENGINE.set(engine);
}

pub fn with_engine() -> Option<Arc<Mutex<PlaybookEngine>>> {
    ENGINE.get().cloned()
}

pub fn set_sender(tx: Sender<AdapterFact>) {
    let _ = SENDER.set(tx);
}

fn sender() -> Option<Sender<AdapterFact>> {
    SENDER.get().cloned()
}

pub fn set_metrics_handle(metrics: Arc<crate::metrics::Metrics>) {
    let _ = METRICS.set(metrics);
}

pub fn metrics_handle() -> Option<Arc<crate::metrics::Metrics>> {
    METRICS.get().cloned()
}

pub fn emit_adapter_facts(output: &TelemetryOutput) {
    let Some(tx) = sender() else {
        return;
    };

    let facts = adapter_facts_with(output, now_ts(), &read_proc);
    if facts.is_empty() {
        return;
    }

    for fact in facts {
        if let Err(err) = tx.try_send(fact) {
            crate::metrics::mapper_dropped_facts_total().fetch_add(1, Ordering::Relaxed);
            log::debug!("pb_adapter: dropping fact (channel full): {err}");
        }
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
}

pub fn read_proc(pid: i64) -> ProcInfo {
    if pid <= 0 {
        return ProcInfo::default();
    }
    let exe = std::fs::read_link(format!("/proc/{pid}/exe"))
        .ok()
        .map(|p| p.to_string_lossy().into_owned());
    let cmdline = std::fs::read(format!("/proc/{pid}/cmdline")).ok().and_then(|b| {
        let s = b
            .split(|c| *c == 0)
            .filter(|part| !part.is_empty())
            .map(|part| String::from_utf8_lossy(part).into_owned())
            .collect::<Vec<_>>()
            .join(" ");
        (!s.is_empty()).then_some(s)
    });
    let cwd = std::fs::read_link(format!("/proc/{pid}/cwd"))
        .ok()
        .map(|p| p.to_string_lossy().into_owned());
    ProcInfo { exe, cmdline, cwd }
}

/// Exec and memory frames carry no resolved exe path, and some file paths are relative.
/// /proc is read here, in the decoder thread right after the frame arrives, while the
/// process most likely still exists. Fields a decoder already set are kept.
/// proc_lifecycle exec frames also carry the filename and argv the kernel copied at exec
/// (exec_filename, exec_argv). Those fill in when the process is already gone from /proc.
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
        if info.exe.is_none() {
            if kernel_exe.is_some() {
                note_exec_from_kernel_copy(pid, &comm);
            } else {
                note_unresolved_exec(pid, &comm);
            }
        }
        put("binary_path", info.exe.or(kernel_exe));
        put("command_line", info.cmdline.or(kernel_argv));
    }
    put("cwd", info.cwd);
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

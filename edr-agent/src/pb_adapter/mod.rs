use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::Sender;

use crate::pb_engine::PlaybookEngine;
use crate::telemetry::TelemetryRecord;
use crate::telemetry_types::TelemetryOutput;
use std::sync::OnceLock;

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

    let facts = router::to_adapter_facts(output);
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

pub fn to_pb_record(fact: &AdapterFact) -> Option<TelemetryRecord> {
    let meta_map = match fact.meta.as_object() {
        Some(map) => map,
        None => {
            log::debug!(
                "pb_adapter: fact.meta is not an object for kind={} detector={}",
                fact.kind,
                fact.detector
            );
            return None;
        }
    };

    let mut rec = TelemetryRecord::default();
    rec.timestamp = if fact.ts >= 0 { fact.ts as u64 } else { 0 };

    if let Some(pid) = meta_get_i64(meta_map, "pid")
        .or_else(|| meta_get_i64(meta_map, "tgid"))
        .or_else(|| proc_key_pid(&fact.proc_key))
    {
        rec.pid = pid as i32;
    }
    if let Some(ppid) = meta_get_i64(meta_map, "ppid") {
        rec.ppid = ppid as i32;
    }
    if let Some(uid) = meta_get_i64(meta_map, "uid")
        .or_else(|| fact.user_key.as_deref().and_then(parse_uid_from_key))
    {
        rec.uid = uid as u32;
    }

    if let Some(path) = meta_get_string(meta_map, &["binary_path", "exe", "path", "filename"]) {
        rec.binary_path = path;
    }
    if let Some(cmd) = meta_get_string(meta_map, &["command_line", "cmd", "cmdline"]) {
        rec.command_line = cmd;
    }
    if let Some(cwd) = meta_get_string(meta_map, &["cwd", "working_dir"]) {
        rec.cwd = cwd;
    }

    rec.tags.extend(fact.tags.clone());
    if rec.tags.is_empty() && !fact.kind.is_empty() {
        rec.tags.push(fact.kind.clone());
    }

    if rec.risk_score.is_none() {
        if let Some(score) = meta_get_i64(meta_map, "risk_score") {
            rec.risk_score = Some(score as u32);
        } else if let Some(conf) = meta_get_f64(meta_map, "confidence") {
            rec.risk_score = Some((conf * 100.0).clamp(0.0, 100.0) as u32);
        }
    }

    Some(rec)
}

fn meta_get_string(
    map: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<String> {
    for key in keys {
        if let Some(v) = map.get(*key) {
            if let Some(s) = value_as_string(v) {
                if !s.is_empty() {
                    return Some(s);
                }
            }
        }
    }
    None
}

fn meta_get_i64(map: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<i64> {
    map.get(key).and_then(value_as_i64)
}

fn meta_get_f64(map: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<f64> {
    map.get(key).and_then(value_as_f64)
}

fn value_as_string(v: &serde_json::Value) -> Option<String> {
    match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        serde_json::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn value_as_i64(v: &serde_json::Value) -> Option<i64> {
    match v {
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Some(i)
            } else if let Some(u) = n.as_u64() {
                i64::try_from(u).ok()
            } else {
                None
            }
        }
        serde_json::Value::String(s) => s.trim().parse::<i64>().ok(),
        _ => None,
    }
}

fn value_as_f64(v: &serde_json::Value) -> Option<f64> {
    match v {
        serde_json::Value::Number(n) => n.as_f64(),
        serde_json::Value::String(s) => s.trim().parse::<f64>().ok(),
        _ => None,
    }
}

fn proc_key_pid(proc_key: &Option<String>) -> Option<i64> {
    let key = proc_key.as_ref()?;
    for part in key.split('|') {
        if let Some(rest) = part.strip_prefix("pid:") {
            if let Ok(pid) = rest.parse::<i64>() {
                return Some(pid);
            }
        }
    }
    None
}

fn parse_uid_from_key(key: &str) -> Option<i64> {
    if let Some(rest) = key.strip_prefix("uid:") {
        return rest.parse::<i64>().ok();
    }
    None
}

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::net_watch::{log_open_connections, scan_network_anomalies};
use crate::modules::password_spray::scan_password_sprays;
use crate::modules::persistence_watch::scan_persistence_activity;
use crate::modules::privilege_monitor::scan_privilege_activity;
use crate::modules::process_monitor::{gather_processes, scan_processes};
use crate::modules::replay_writer::store_replay_event;
use crate::modules::script_monitor::scan_script_monitor;
use crate::modules::signal_integrity_mapper::scan_signal_integrity;
use crate::modules::suspicious_ipc::scan_ipc_activity;
use crate::modules::usb_monitor::scan_usb_state;
use crate::modules::user_tracker::scan_user_sessions;

use crate::services::trust_engine_final::{evaluate_and_dispatch_trust_score, TelemetryData};
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::{write_telemetry_record, TelemetryWriter};
use crate::utils::time::now_ts;

use crate::pb_adapter;

use crate::traits::FeatureObservation;

// ======================= Data Types =======================

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TelemetryRecord {
    pub timestamp: u64,
    pub pid: i32,
    pub ppid: i32,
    pub uid: u32,
    pub binary_path: String,
    pub command_line: String,
    pub cwd: String,
    pub env_vars: Option<Vec<String>>,
    pub tags: Vec<String>,
    pub risk_score: Option<u32>,
}

// ======================= Utilities =======================

#[inline]
pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[inline]
fn should_log_raw() -> bool {
    // enable with either RUST_LOG containing "telemetry=trace" or EDR_VERBOSE=1
    std::env::var("RUST_LOG")
        .map(|v| v.contains("telemetry=trace"))
        .unwrap_or(false)
        || std::env::var("EDR_VERBOSE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
}

// Convert TelemetryOutput → map while preserving original data
fn output_to_map(output: &TelemetryOutput) -> HashMap<String, String> {
    let mut map = output.data.clone();
    map.insert("category".to_string(), output.category.clone());
    map.insert("signal".to_string(), output.signal.clone());
    map.insert("confidence".to_string(), output.confidence.to_string());
    map
}

// Dedup outputs per-minute by {category,signal,pid,binary_path}
fn deduplicate_outputs(outputs: Vec<TelemetryOutput>) -> Vec<TelemetryOutput> {
    let mut seen: HashSet<String> = HashSet::new();
    let minute = get_current_timestamp() / 60;

    outputs
        .into_iter()
        .filter(|o| {
            let pid = o.data.get("pid").cloned().unwrap_or_default();
            let path = o
                .data
                .get("binary_path")
                .cloned()
                .or_else(|| o.data.get("exe").cloned())
                .unwrap_or_default();
            let key = format!("{}|{}|{}|{}|{}", o.category, o.signal, pid, path, minute);
            seen.insert(key)
        })
        .collect()
}

#[inline]
fn kv_enabled() -> bool {
    !std::env::var("EDR_DISABLE_KV")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

// Centralized gate for KV JSONL writes from this module.
#[inline]
fn persist_side_effect_kv(map: &HashMap<String, String>) {
    if kv_enabled() {
        write_telemetry_record(map.clone());
    }
}

// ======================= Passive Sweep =======================

pub fn run_sideeffect_monitors_and_collect() -> Vec<TelemetryOutput> {
    let mut results = Vec::new();

    // quick process sampling so passive rules have context (no-op return value)
    let _ = scan_processes();

    results.extend(scan_network_anomalies());
    results.extend(scan_password_sprays());
    results.extend(scan_persistence_activity());
    results.extend(scan_privilege_activity());
    results.extend(scan_signal_integrity());
    results.extend(scan_script_monitor());
    results.extend(scan_ipc_activity());
    results.extend(crate::guard::take_guard_outputs());
    results.extend(scan_usb_state());
    results.extend(scan_user_sessions());

    deduplicate_outputs(results)
}

// ======================= Snapshots & Telemetry =======================

pub fn get_current_snapshot() -> Vec<TelemetryRecord> {
    // Always include process snapshot rows so EVENTS_IN can bump even if no side-effect signals fire.
    let process_infos = gather_processes();
    process_infos
        .into_iter()
        .map(|proc| TelemetryRecord {
            timestamp: get_current_timestamp(),
            pid: proc.pid,
            ppid: proc.ppid,
            uid: 0,
            binary_path: proc.exe,
            command_line: proc.command_line,
            cwd: String::new(),
            env_vars: Some(vec!["PATH=/usr/bin".into()]),
            tags: vec!["process_snapshot".into()],
            risk_score: None,
        })
        .collect()
}

pub fn get_current_telemetry_snapshot(writer: Arc<Mutex<TelemetryWriter>>) -> Vec<TelemetryRecord> {
    // Start with process snapshot rows
    let mut records = get_current_snapshot();

    // Pull all “side-effect” outputs and project them into both logs and TelemetryRecord rows
    let all_outputs: Vec<TelemetryOutput> = run_sideeffect_monitors_and_collect();

    for output in all_outputs {
        pb_adapter::emit_adapter_facts(&output);

        let mapped = output_to_map(&output);

        // Optional noisy debug, searchable in `strings` and visible at runtime
        if should_log_raw() {
            if let Ok(js) = serde_json::to_string(&mapped) {
                println!("🟡 Raw Telemetry: {}", js);
            }
        }

        // Persist/forward side-effects (cheap, but now gated)
        persist_side_effect_kv(&mapped);
        push_to_gnn_vector_log(mapped.clone());
        let _ = store_replay_event(mapped.clone());

        // Normalize some common fields
        let pid = output
            .data
            .get("pid")
            .and_then(|v| v.parse().ok())
            .unwrap_or(-1);
        let ppid = output
            .data
            .get("ppid")
            .and_then(|v| v.parse().ok())
            .unwrap_or(-1);
        let uid = output
            .data
            .get("uid")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        let binary_path = output
            .data
            .get("binary_path")
            .cloned()
            .or_else(|| output.data.get("exe").cloned())
            .unwrap_or_default();
        let command_line = output
            .data
            .get("command_line")
            .cloned()
            .or_else(|| output.data.get("cmdline").cloned())
            .unwrap_or_default();
        let cwd = output.data.get("cwd").cloned().unwrap_or_default();

        // Build a row that `main.rs` can count
        let mut tags = vec![output.signal.clone()];
        if output.category != "unknown" {
            tags.push(output.category.clone());
        }

        let rec = TelemetryRecord {
            timestamp: now_ts(),
            pid,
            ppid,
            uid,
            binary_path,
            command_line,
            cwd,
            env_vars: None,
            tags,
            risk_score: Some((output.confidence * 100.0) as u32),
        };

        // inline trust scoring
        let telemetry_data = TelemetryData {
            endpoint_id: format!("endpoint_{}", rec.pid),
            endpoint_role: "default".to_string(),
            cpu_usage: 0.0,
            memory_usage: 0.0,
            risk_score: rec.risk_score.unwrap_or(0) as f64,
            tags: rec.tags.clone(),
        };
        let trust_result = evaluate_and_dispatch_trust_score(&telemetry_data);
        println!("🟢 Trust Result (inline): {:?}", trust_result);

        records.push(rec);
    }

    // Opportunistic network snapshot enrichment
    if let Ok(mut locked_writer) = writer.lock() {
        log_open_connections(&mut *locked_writer);
    }

    records
}

// ======================= Feature bridge =======================

pub fn push_feature_as_signal(f: &FeatureObservation) {
    let mut data = HashMap::new();
    data.insert("feature_key".into(), f.key.clone());
    data.insert("feature_value".into(), f.value.to_string()); // stringify f64

    let output = TelemetryOutput {
        category: "feature".into(),
        signal: "feature_observation".into(),
        confidence: 0.1,
        data,
    };

    pb_adapter::emit_adapter_facts(&output);

    // make it look like a normal output for downstreams
    let mapped = output_to_map(&output);
    persist_side_effect_kv(&mapped);
    push_to_gnn_vector_log(mapped);
}

// --- Entropy helpers ---

/// Byte-oriented Shannon entropy in bits per byte.
pub fn estimate_entropy(bytes: &[u8]) -> f64 {
    if bytes.is_empty() {
        return 0.0;
    }
    let mut freq = [0usize; 256];
    for &b in bytes {
        freq[b as usize] += 1;
    }
    let len = bytes.len() as f64;
    let mut h = 0.0;
    for &count in freq.iter() {
        if count == 0 {
            continue;
        }
        let p = (count as f64) / len;
        h -= p * p.log2();
    }
    h
}

/// Back-compat: string wrapper that defers to `estimate_entropy`.
pub fn calculate_entropy(data: &str) -> f64 {
    estimate_entropy(data.as_bytes())
}

// ======================= Normalized detector event =======================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Telemetry {
    pub ts: DateTime<Utc>,
    pub category: String,
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exe: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmdline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub attrs: Map<String, Value>,
}

impl Telemetry {
    #[inline]
    pub fn attr_str(&self, k: &str) -> Option<&str> {
        self.attrs.get(k)?.as_str()
    }
    #[inline]
    pub fn attr_f64(&self, k: &str) -> Option<f64> {
        self.attrs.get(k)?.as_f64()
    }
}

impl From<TelemetryRecord> for Telemetry {
    fn from(r: TelemetryRecord) -> Self {
        let ts = ts_u64_to_dt(r.timestamp);
        let mut attrs: Map<String, Value> = Map::new();
        if !r.binary_path.is_empty() {
            attrs.insert("binary_path".into(), Value::String(r.binary_path.clone()));
        }
        if !r.command_line.is_empty() {
            attrs.insert("command_line".into(), Value::String(r.command_line.clone()));
        }
        if !r.cwd.is_empty() {
            attrs.insert("cwd".into(), Value::String(r.cwd.clone()));
        }
        if let Some(risk) = r.risk_score {
            attrs.insert("risk_score".into(), Value::from(risk as i64));
        }
        if let Some(env) = r.env_vars.as_ref() {
            attrs.insert("env_vars".into(), Value::from(env.clone()));
        }

        let mut tagmap = std::collections::HashMap::new();
        for t in r.tags.iter() {
            tagmap.insert(t.clone(), "true".to_string());
        }

        Telemetry {
            ts,
            category: "unknown".to_string(),
            event: "unknown".to_string(),
            pid: Some(r.pid),
            ppid: Some(r.ppid),
            uid: Some(r.uid),
            exe: if r.binary_path.is_empty() {
                None
            } else {
                Some(r.binary_path)
            },
            cmdline: if r.command_line.is_empty() {
                None
            } else {
                Some(r.command_line)
            },
            cwd: if r.cwd.is_empty() { None } else { Some(r.cwd) },
            tags: tagmap,
            attrs,
        }
    }
}

fn ts_u64_to_dt(secs: u64) -> DateTime<Utc> {
    Utc.timestamp_opt(secs as i64, 0)
        .single()
        .unwrap_or_else(Utc::now)
}

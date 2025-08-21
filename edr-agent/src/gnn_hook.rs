use crate::telemetry::TelemetryRecord;

use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Optional types used by a few helpers below. If your crate layout differs, adjust these paths.
use crate::services::trust_vector::TrustVector;
use crate::graph_builder::{GraphEdge, GraphNode};
use crate::gnn_models::GnnScoringInput;

/// =========================
/// Configuration & Utilities
/// =========================

/// Base directory for all GNN JSONL logs (env override: `EDR_GNN_DIR`)
const DEFAULT_GNN_DIR: &str = "edr-agent/json_files";

/// FIFO used to stream feature vectors to a daemon (env override: `EDR_GNN_FIFO`)
const DEFAULT_GNN_FIFO: &str = "/tmp/gnn_input.pipe";

fn gnn_dir() -> PathBuf {
    PathBuf::from(env::var("EDR_GNN_DIR").unwrap_or_else(|_| DEFAULT_GNN_DIR.to_string()))
}

fn ensure_parent_dir(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)
    } else {
        Ok(())
    }
}

fn append_jsonl(path: &Path, line: &str) -> std::io::Result<()> {
    ensure_parent_dir(path)?;
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{}", line)?;
    Ok(())
}

fn stringify_map_vals(m: &HashMap<String, f64>) -> HashMap<String, String> {
    m.iter().map(|(k, v)| (k.clone(), v.to_string())).collect()
}

/// ============================
/// Public Logging / Export APIs
/// ============================

/// 🧠 Submit full telemetry record to the GNN telemetry stream (JSONL on disk).
pub fn submit_to_gnn(record: &TelemetryRecord) {
    println!(
        "[GNN HOOK] PID: {}, Risk: {:?}, Binary: {}",
        record.pid, record.risk_score, record.binary_path
    );

    let path = gnn_dir().join("gnn_telemetry_records.jsonl");
    let json_line = json!({
        "timestamp": record.timestamp,
        "pid": record.pid,
        "ppid": record.ppid,
        "uid": record.uid,
        "binary_path": record.binary_path,
        "command_line": record.command_line,
        "cwd": record.cwd,
        "risk_score": record.risk_score,
    })
    .to_string();

    if let Err(e) = append_jsonl(&path, &json_line) {
        eprintln!("[GNN Hook] Failed to write TelemetryRecord: {e} (path: {:?})", path);
    }
}

/// 🚀 Accept a ready-made feature/metadata map and append to the vector log.
pub fn push_feature_map_to_gnn_log(feature_map: HashMap<String, String>) {
    let path = gnn_dir().join("gnn_vector_log.jsonl");
    match serde_json::to_string(&feature_map) {
        Ok(line) => {
            if let Err(e) = append_jsonl(&path, &line) {
                eprintln!("[GNN Hook] Failed to write vector log: {e} (path: {:?})", path);
            }
        }
        Err(_) => eprintln!("[GNN Hook] Failed to serialize feature map to JSON"),
    }
}

/// ✅ Primary helper used across modules (kept for backward-compat).
pub fn push_to_gnn_vector_log(data: HashMap<String, String>) {
    push_feature_map_to_gnn_log(data);
}

/// 🧭 Simple 3-field event writer (handy for coarse trust status beacons).
pub fn push_gnn_event(endpoint_id: &str, score: f64, reason: &str) {
    let mut map = HashMap::new();
    map.insert("timestamp".to_string(), Utc::now().timestamp().to_string());
    map.insert("endpoint_id".to_string(), endpoint_id.to_string());
    map.insert("trust_score".to_string(), format!("{:.2}", score));
    map.insert("reason".to_string(), reason.to_string());
    push_feature_map_to_gnn_log(map);
}

/// 📊 Export a full trust vector snapshot with endpoint labels/tags.
pub fn export_trust_vector_to_gnn(
    endpoint_id: &str,
    endpoint_role: &str,
    trust_score: f64,
    trust_vector: HashMap<String, f64>,
    tags: Vec<String>,
) {
    let mut record = HashMap::new();
    record.insert("timestamp".into(), Utc::now().to_rfc3339());
    record.insert("endpoint_id".into(), endpoint_id.to_string());
    record.insert("endpoint_role".into(), endpoint_role.to_string());
    record.insert("trust_score".into(), format!("{:.4}", trust_score));
    record.insert("tags".into(), format!("{:?}", tags));

    // Flatten vector as trust_<dim>=value
    for (dim, val) in trust_vector.iter() {
        record.insert(format!("trust_{}", dim), format!("{:.6}", val));
    }

    push_feature_map_to_gnn_log(record);
}

/// 📡 Attempt to stream a numeric feature map to a FIFO; fallback to JSONL.
pub fn stream_to_gnn_fifo(feature_map: &HashMap<String, f64>) {
    let fifo_path = env::var("EDR_GNN_FIFO").unwrap_or_else(|_| DEFAULT_GNN_FIFO.to_string());

    match serde_json::to_string(feature_map) {
        Ok(json_line) => match OpenOptions::new().write(true).open(&fifo_path) {
            Ok(mut fifo) => {
                if let Err(e) = writeln!(fifo, "{}", json_line) {
                    eprintln!("[GNN Hook] FIFO write failed: {e}; falling back to file.");
                    push_feature_map_to_gnn_log(stringify_map_vals(feature_map));
                }
            }
            Err(e) => {
                eprintln!("[GNN Hook] FIFO unavailable ({fifo_path}): {e}; falling back to file.");
                push_feature_map_to_gnn_log(stringify_map_vals(feature_map));
            }
        },
        Err(_) => eprintln!("[GNN Hook] Failed to serialize feature map to JSON"),
    }
}

/// Convenience alias (kept for compatibility with older callers).
pub fn push_metadata_to_gnn_vector_log(metadata: HashMap<String, String>) {
    push_feature_map_to_gnn_log(metadata);
}

/// ===============================
/// Higher-level Builder & Streaming
/// ===============================

/// Compose a rich scoring input for a GNN daemon from a subgraph selection.
pub fn build_gnn_scoring_input(
    subgraph_nodes: &[GraphNode],
    subgraph_edges: &[GraphEdge],
    trust_vector: TrustVector,
    snapshot_path: Option<String>,
) -> GnnScoringInput {
    let timestamp = Utc::now().timestamp();

    let semantic_tags: Vec<String> = subgraph_nodes
        .iter()
        .flat_map(|n| n.tags.clone())
        .collect::<std::collections::HashSet<_>>() // dedup
        .into_iter()
        .collect();

    let anchor_hits: Vec<String> = subgraph_nodes
        .iter()
        .flat_map(|n| n.anchor_ids.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let suppressed_by_fingerprint: Vec<String> = subgraph_nodes
        .iter()
        .filter(|n| n.fingerprint_suppressed.unwrap_or(false))
        .map(|n| n.id.clone())
        .collect();

    let decay_score = subgraph_nodes
        .iter()
        .map(|n| 1.0 - n.trust_score)
        .sum::<f32>();

    let session_id = format!("graph_{}", timestamp);

    GnnScoringInput {
        session_id,
        timestamp,
        trust_vector,
        semantic_tags,
        anchor_hits,
        suppressed_by_fingerprint,
        decay_score,
        graph_snapshot_path: snapshot_path,
        notes: Some("Auto-generated from causal subgraph".to_string()),
    }
}

/// Send a ready scoring input to the daemon FIFO.
pub fn send_to_gnn_daemon(trust_vector: TrustVector, tags: Vec<String>) {
    // Minimal, generic payload; prefer `build_gnn_scoring_input` when you have a subgraph.
    let input = GnnScoringInput {
        session_id: format!("session_{}", Utc::now().timestamp()),
        timestamp: Utc::now().timestamp(),
        trust_vector,
        semantic_tags: tags,
        anchor_hits: Vec::new(),
        suppressed_by_fingerprint: Vec::new(),
        decay_score: 0.0,
        graph_snapshot_path: None,
        notes: Some("Direct submit without subgraph".to_string()),
    };

    let fifo_path = env::var("EDR_GNN_FIFO").unwrap_or_else(|_| DEFAULT_GNN_FIFO.to_string());
    match OpenOptions::new().write(true).open(&fifo_path) {
        Ok(mut pipe) => {
            if let Ok(payload) = serde_json::to_string(&input) {
                if let Err(e) = writeln!(pipe, "{}", payload) {
                    eprintln!("[GNN Hook] Could not write to GNN pipe: {e}");
                    // fall back to persisted log
                    let mut map = HashMap::new();
                    map.insert("fallback".into(), "gnn_daemon_pipe_write_failed".into());
                    map.insert("payload".into(), payload);
                    push_feature_map_to_gnn_log(map);
                }
            }
        }
        Err(e) => {
            eprintln!("[GNN Hook] Could not open GNN pipe for writing: {e}");
            // fall back to persisted log
            let mut map = HashMap::new();
            map.insert("fallback".into(), "gnn_daemon_pipe_open_failed".into());
            map.insert("timestamp".into(), Utc::now().to_rfc3339());
            push_feature_map_to_gnn_log(map);
        }
    }
}

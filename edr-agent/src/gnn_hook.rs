use crate::telemetry::TelemetryRecord;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use serde_json::json;
use chrono::Utc;

/// 🧠 Submit full telemetry record to GNN pipeline (legacy file-based)
pub fn submit_to_gnn(record: &TelemetryRecord) {
    println!(
        "[GNN HOOK] PID: {}, Risk: {:?}, Binary: {}",
        record.pid,
        record.risk_score,
        record.binary_path
    );

    let log_path = PathBuf::from("/edr-agent/json_files/gnn_telemetry_records.jsonl");
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_path) {
        let json_line = json!({
            "timestamp": record.timestamp,
            "pid": record.pid,
            "ppid": record.ppid,
            "uid": record.uid,
            "binary_path": record.binary_path,
            "command_line": record.command_line,
            "cwd": record.cwd,
            "risk_score": record.risk_score,
        });

        if let Err(e) = writeln!(file, "{}", json_line.to_string()) {
            eprintln!("[GNN Hook] Failed to write TelemetryRecord to file: {}", e);
        }
    } else {
        eprintln!("[GNN Hook] Failed to open GNN telemetry log file: {:?}", log_path);
    }
}

use std::fs::{OpenOptions, create_dir_all};
use std::path::Path;

pub fn push_feature_map_to_gnn_log(feature_map: HashMap<String, String>) {
    let log_path = PathBuf::from("./edr-agent/json_files/gnn_vector_log.jsonl"); // ← relative path preferred
    let parent = Path::new(&log_path).parent().unwrap();

    // Ensure parent directory exists
    if !parent.exists() {
        if let Err(e) = create_dir_all(parent) {
            eprintln!("[GNN Hook] Failed to create directory {:?}: {}", parent, e);
            return;
        }
    }

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_path) {
        if let Ok(json_line) = serde_json::to_string(&feature_map) {
            if let Err(e) = writeln!(file, "{}", json_line) {
                eprintln!("[GNN Hook] Failed to write to vector log: {}", e);
            }
        } else {
            eprintln!("[GNN Hook] Failed to serialize feature map to JSON");
        }
    } else {
        eprintln!("[GNN Hook] Failed to open GNN vector log file: {:?}", log_path);
    }
}

/// ✅ FIXED: Accept full metadata HashMap directly as 'data'
pub fn push_to_gnn_vector_log(data: HashMap<String, String>) {
    push_feature_map_to_gnn_log(data);
}
/// 🧠 Log raw GNN feature vectors to file (classic 3-argument version)
pub fn push_gnn_event(endpoint_id: &str, score: f64, reason: &str) {
    let mut map = HashMap::new();
    map.insert("timestamp".to_string(), Utc::now().timestamp().to_string());
    map.insert("endpoint_id".to_string(), endpoint_id.to_string());
    map.insert("trust_score".to_string(), format!("{:.2}", score));
    map.insert("reason".to_string(), reason.to_string());

    push_feature_map_to_gnn_log(map);
}


/// 🧠 Trust Engine export: push full trust vector with endpoint details
pub fn export_trust_vector_to_gnn(
    endpoint_id: &str,
    endpoint_role: &str,
    trust_score: f64,
    trust_vector: HashMap<String, f64>,
    tags: Vec<String>,
) {
    let timestamp = chrono::Utc::now().to_rfc3339();
    let mut record = HashMap::new();

    record.insert("timestamp".to_string(), timestamp);
    record.insert("endpoint_id".to_string(), endpoint_id.to_string());
    record.insert("endpoint_role".to_string(), endpoint_role.to_string());
    record.insert("trust_score".to_string(), trust_score.to_string());
    record.insert("tags".to_string(), format!("{:?}", tags));

    for (dim, val) in trust_vector.iter() {
        record.insert(format!("trust_{}", dim), val.to_string());
    }

    push_feature_map_to_gnn_log(record);
}


/// 🧠 Stream trust vector to FIFO (fallback to log on failure)
pub fn stream_to_gnn_fifo(feature_map: &HashMap<String, f64>) {
    let fifo_path = "/tmp/gnn_input.pipe";

    if let Ok(json_line) = serde_json::to_string(feature_map) {
        match OpenOptions::new().write(true).open(fifo_path) {
            Ok(mut fifo) => {
                if let Err(e) = writeln!(fifo, "{}", json_line) {
                    eprintln!("[GNN Hook] Failed to write to FIFO: {}", e);
                    push_feature_map_to_gnn_log(
                        feature_map
                            .iter()
                            .map(|(k, v)| (k.clone(), v.to_string()))
                            .collect(),
                    );
                }
            }
            Err(e) => {
                eprintln!("[GNN Hook] FIFO unavailable, fallback to log: {}", e);
                push_feature_map_to_gnn_log(
                    feature_map
                        .iter()
                        .map(|(k, v)| (k.clone(), v.to_string()))
                        .collect(),
                );
            }
        }
    } else {
        eprintln!("[GNN Hook] Failed to serialize feature map to JSON");
    }
}
use std::fs::OpenOptions;
use std::io::Write;
use crate::gnn_models::GnnScoringInput;
use chrono::Utc;

fn send_to_gnn_daemon(trust_vector: TrustVector, tags: Vec<String>) {
    let pipe_path = "/tmp/gnn_input.pipe";

    let gnn_input = GnnScoringInput {
        trust_vector,
        session_id: "session_xyz".to_string(), // or hash of graph
        timestamp: Utc::now().timestamp(),
        tags,
        graph_snapshot: Some("/tmp/gnn_input_graph_snapshot.json".to_string()), // optional
    };

    if let Ok(mut pipe) = OpenOptions::new().write(true).open(pipe_path) {
        if let Ok(payload) = serde_json::to_string(&gnn_input) {
            let _ = writeln!(pipe, "{}", payload);
        }
    } else {
        eprintln!("[gnn_hook] Could not open GNN pipe for writing");
    }
}

/// ✅ Allows logging a prebuilt HashMap as a GNN vector
pub fn push_metadata_to_gnn_vector_log(metadata: HashMap<String, String>) {
    push_feature_map_to_gnn_log(metadata);
}

use crate::graph_builder::{GraphNode, GraphEdge};
use crate::trust_vector::TrustVector;
use crate::gnn_models::GnnScoringInput;
use chrono::Utc;

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

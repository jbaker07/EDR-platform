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
/// ✅ Allows logging a prebuilt HashMap as a GNN vector
pub fn push_metadata_to_gnn_vector_log(metadata: HashMap<String, String>) {
    push_feature_map_to_gnn_log(metadata);
}

// edr-agent/trust_log_writer.rs

use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::json;

pub fn write_trust_log(endpoint_id: &str, trust_score: f64, risk_level: &str, reason: &str) {
    let log_dir = "logs/trust";
    let log_file = format!("{}/{}.log", log_dir, endpoint_id);

    if let Err(e) = create_dir_all(log_dir) {
        eprintln!("Failed to create trust log directory: {}", e);
        return;
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let entry = json!({
        "timestamp": timestamp,
        "endpoint_id": endpoint_id,
        "trust_score": trust_score,
        "risk_level": risk_level,
        "reason": reason
    });

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_file) {
        if let Err(e) = writeln!(file, "{}", entry) {
            eprintln!("Failed to write trust log: {}", e);
        }
    }
}

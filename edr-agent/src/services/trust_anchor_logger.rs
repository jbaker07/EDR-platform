use chrono::Utc;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize, Debug)]
pub struct AnchorDropEvent {
    pub timestamp: String,
    pub endpoint_id: String,
    pub endpoint_role: String,
    pub dimension: String,
    pub score: f64,
    pub reason: String,
}

pub fn log_anchor_drop(event: AnchorDropEvent) {
    let log_path = "logs/anchor_drop_log.jsonl";
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
        let json = serde_json::to_string(&event).unwrap_or_default();
        let _ = writeln!(file, "{}", json);
    }
}

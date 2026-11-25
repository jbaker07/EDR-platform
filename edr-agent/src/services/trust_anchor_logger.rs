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

/// Append several anchor drops with one open.
pub fn log_anchor_drops(events: &[AnchorDropEvent]) {
    let log_path = "logs/anchor_drop_log.jsonl";
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
        let mut out = String::new();
        for e in events {
            out.push_str(&serde_json::to_string(e).unwrap_or_default());
            out.push('\n');
        }
        let _ = file.write_all(out.as_bytes());
    }
}

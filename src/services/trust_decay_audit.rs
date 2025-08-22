// trust_decay_audit.rs

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustDecayEvent {
    pub endpoint_id: String,
    pub dimension: String,           // e.g., "auth", "net", "process"
    pub decay_type: String,          // e.g., "time-based", "triggered", "anomaly-linked"
    pub decay_amount: f64,           // typically negative
    pub reason: String,              // e.g., "no activity", "missing telemetry"
    pub timestamp: DateTime<Utc>,
}

impl TrustDecayEvent {
    pub fn new(endpoint_id: &str, dimension: &str, decay_type: &str, decay_amount: f64, reason: &str) -> Self {
        TrustDecayEvent {
            endpoint_id: endpoint_id.to_string(),
            dimension: dimension.to_string(),
            decay_type: decay_type.to_string(),
            decay_amount,
            reason: reason.to_string(),
            timestamp: Utc::now(),
        }
    }
}

/// Append the structured event to a persistent JSONL audit log
pub fn log_decay_event(event: &TrustDecayEvent) {
    let json = serde_json::to_string(event).unwrap_or_else(|_| "{}".to_string());
    let path = Path::new("trust_logs/trust_decay_audit.jsonl");

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "{}", json);
    }
}

/// Lightweight helper for writing a fast, human-readable log line
pub fn log_decay_reason(source: &str, user: &str, amount: f64, reason: &str) {
    let timestamp = Utc::now().to_rfc3339();
    let log_line = format!(
        "[TrustDecay] [{}] source={} user={} amount={} reason={}\n",
        timestamp, source, user, amount, reason
    );

    let path = Path::new("/var/log/edr/trust_decay.log");

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = file.write_all(log_line.as_bytes());
    }
}

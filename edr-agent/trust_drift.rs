use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Represents a drift event and its severity
#[derive(Debug, Clone)]
pub struct DriftRecord {
    pub timestamp: DateTime<Utc>,
    pub previous_score: f64,
    pub current_score: f64,
    pub delta: f64,
    pub fingerprint_changed: bool,
}

/// Calculates trust drift between two points in time
pub fn calculate_trust_drift(
    previous_score: f64,
    current_score: f64,
    previous_fingerprint: &str,
    current_fingerprint: &str,
) -> DriftRecord {
    let delta = current_score - previous_score;
    let fingerprint_changed = previous_fingerprint != current_fingerprint;

    DriftRecord {
        timestamp: Utc::now(),
        previous_score,
        current_score,
        delta,
        fingerprint_changed,
    }
}

/// Applies thresholds to detect significant trust drift
pub fn is_drift_significant(record: &DriftRecord, score_threshold: f64) -> bool {
    record.delta.abs() > score_threshold || record.fingerprint_changed
}

/// Optional: store drift records for audit or escalation review
pub fn log_drift_event(record: &DriftRecord) {
    println!(
        "[DRIFT] @{} | ΔScore: {:.2} | Fingerprint Changed: {}",
        record.timestamp,
        record.delta,
        record.fingerprint_changed
    );
}

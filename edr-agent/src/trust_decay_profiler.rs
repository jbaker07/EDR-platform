use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::telemetry_types::TelemetrySnapshot;
use crate::trust_hook::submit_trust_signal;
use crate::services::decay_engine::calculate_decay;
use crate::session_digest_cache::get_recent_session_digest;

/// Thresholds
const DECAY_WINDOW_SECS: u64 = 3600 * 6; // 6 hours
const DRIFT_THRESHOLD: f64 = 0.35;
const DECAY_SIGNAL_THRESHOLD: u8 = 25;

/// Tracks reasons for trust decay
fn identify_decay_reasons(snapshot: &TelemetrySnapshot, baseline: &TelemetrySnapshot) -> Vec<String> {
    let mut reasons = vec![];

    if snapshot.memory_usage > baseline.memory_usage * 2.0 {
        reasons.push("memory spike".into());
    }

    if snapshot.cpu_usage > baseline.cpu_usage * 1.5 {
        reasons.push("cpu anomaly".into());
    }

    if snapshot.network_outbound > baseline.network_outbound * 3.0 {
        reasons.push("outbound network surge".into());
    }

    if snapshot.parent_cmd != baseline.parent_cmd {
        reasons.push("parent command drift".into());
    }

    if snapshot.binary_hash != baseline.binary_hash {
        reasons.push("binary hash changed".into());
    }

    reasons
}

fn get_epoch() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs()
}

/// Analyze trust drift over time
pub fn evaluate_trust_decay(endpoint_id: &str) {
    if let Some((baseline, recent)) = get_recent_session_digest(endpoint_id, DECAY_WINDOW_SECS) {
        let decay_score = calculate_decay(&baseline, &recent);
        if decay_score > DRIFT_THRESHOLD {
            let decay_reasons = identify_decay_reasons(&recent, &baseline);
            let summary = format!(
                "Trust decay observed over {}s. Drift score: {:.2}. Reasons: {:?}",
                DECAY_WINDOW_SECS,
                decay_score,
                decay_reasons
            );

            let signal_strength = if decay_score > 0.75 { 85 } else { 60 };
            let signal_type = if decay_score > 0.75 {
                "critical_decay"
            } else {
                "moderate_decay"
            };

            if signal_strength > DECAY_SIGNAL_THRESHOLD {
                send_trust_signal(signal_type, &summary, signal_strength);
            }
        }
    }
}

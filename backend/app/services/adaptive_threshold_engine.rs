use chrono::{Datelike, Timelike, Utc};
use std::collections::HashMap;

/// Static base thresholds by role
fn base_thresholds() -> HashMap<&'static str, f64> {
    let mut map = HashMap::new();
    map.insert("admin", 85.0);
    map.insert("developer", 70.0);
    map.insert("contractor", 60.0);
    map.insert("finance", 75.0);
    map.insert("default", 65.0);
    map
}

/// Computes a dynamic threshold based on role, time, device, and trust history
pub fn calculate_dynamic_threshold(
    role: &str,
    device_type: &str,
    recent_trust_scores: &[f64],
) -> f64 {
    let base = base_thresholds().get(role).copied().unwrap_or(65.0);

    // Behavior modifier: apply decay if recent trust scores are dropping
    let avg_recent = if !recent_trust_scores.is_empty() {
        recent_trust_scores.iter().sum::<f64>() / recent_trust_scores.len() as f64
    } else {
        base
    };

    let behavior_penalty = if avg_recent < base {
        (base - avg_recent).min(15.0)
    } else {
        0.0
    };

    // Time modifier: apply stricter thresholds during off-hours
    let now = Utc::now();
    let hour = now.hour();
    let off_hours_penalty = if hour < 7 || hour > 20 { 5.0 } else { 0.0 };

    // Device sensitivity modifier
    let device_bonus = match device_type {
        "hardened_laptop" => -5.0,
        "cloud_vm" => 0.0,
        "sensitive_server" => 10.0,
        _ => 0.0,
    };

    let threshold = base + behavior_penalty + off_hours_penalty + device_bonus;

    threshold.clamp(30.0, 100.0)
}

/// Should the system fail closed if trust drops below threshold?
pub fn should_fail_closed(role: &str, trust_score: f64, dynamic_threshold: f64) -> bool {
    let privileged = matches!(role, "admin" | "finance");
    privileged && trust_score < dynamic_threshold
}

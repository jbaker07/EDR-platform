use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct SessionExplanation {
    pub session_id: String,
    pub user: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub key_events: Vec<String>,
    pub triggered_tags: Vec<String>,
    pub trust_score_curve: Vec<(u64, f64)>,
    pub explanation_summary: String,
    pub severity: String,
}

pub fn explain_session(
    session_id: &str,
    user: &str,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
    key_events: Vec<String>,
    triggered_tags: Vec<String>,
    trust_curve: Vec<(u64, f64)>,
) -> SessionExplanation {
    let summary = generate_summary(&key_events, &triggered_tags, &trust_curve);
    let severity = determine_severity(&trust_curve, &triggered_tags);

    SessionExplanation {
        session_id: session_id.to_string(),
        user: user.to_string(),
        start_time,
        end_time,
        key_events,
        triggered_tags,
        trust_score_curve: trust_curve,
        explanation_summary: summary,
        severity,
    }
}

fn generate_summary(
    key_events: &Vec<String>,
    triggered_tags: &Vec<String>,
    trust_curve: &Vec<(u64, f64)>,
) -> String {
    let drop_detected = trust_curve
        .windows(2)
        .any(|pair| pair[1].1 < pair[0].1 && (pair[0].1 - pair[1].1) > 10.0);

    let mut summary = String::new();
    if drop_detected {
        summary.push_str("Significant trust decay detected. ");
    }

    if !triggered_tags.is_empty() {
        summary.push_str(&format!(
            "Triggered tags: {}. ",
            triggered_tags.join(", ")
        ));
    }

    if !key_events.is_empty() {
        summary.push_str(&format!(
            "Notable activity: {}. ",
            key_events.iter().take(3).cloned().collect::<Vec<_>>().join(" | ")
        ));
    }

    if summary.is_empty() {
        summary = "No significant anomalies detected during session.".into();
    }

    summary
}

fn determine_severity(trust_curve: &Vec<(u64, f64)>, tags: &Vec<String>) -> String {
    let lowest = trust_curve.iter().map(|(_, score)| *score).fold(100.0, f64::min);

    if lowest < 25.0 || tags.iter().any(|t| t.contains("privilege") || t.contains("rce")) {
        "critical".into()
    } else if lowest < 50.0 || tags.len() >= 3 {
        "high".into()
    } else if lowest < 75.0 {
        "medium".into()
    } else {
        "low".into()
    }
}

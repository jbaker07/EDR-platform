use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct TrustEvent {
    pub timestamp: DateTime<Utc>,
    pub trust_score: f32,
    pub risk_score: f32,
    pub session_id: String,
}

#[derive(Debug)]
pub struct TrustEscapeReport {
    pub session_id: String,
    pub average_decay: f32,
    pub suspicious_flatline: bool,
    pub event_count: usize,
    pub suspicious: bool,
}

pub fn detect_trust_escape(events: &[TrustEvent]) -> Option<TrustEscapeReport> {
    if events.len() < 3 {
        return None;
    }

    let session_id = &events[0].session_id;
    let mut decay_total = 0.0;
    let mut flatline = true;
    let mut prev_score = events[0].trust_score;

    for event in events.iter().skip(1) {
        let delta = prev_score - event.trust_score;
        decay_total += delta;
        if delta.abs() > 0.05 { flatline = false; }
        prev_score = event.trust_score;
    }

    let average_decay = decay_total / (events.len() - 1) as f32;
    let suspicious = average_decay.abs() < 0.01 || flatline;

    Some(TrustEscapeReport {
        session_id: session_id.clone(),
        average_decay,
        suspicious_flatline: flatline,
        event_count: events.len(),
        suspicious,
    })
}

pub fn analyze_all_sessions(
    session_data: &HashMap<String, Vec<TrustEvent>>
) -> Vec<TrustEscapeReport> {
    session_data.iter()
        .filter_map(|(_, events)| detect_trust_escape(events))
        .collect()
}

// edr-agent/forensic/session_tracker.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, Duration};
use lazy_static::lazy_static;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SessionMeta {
    pub session_id: String,
    pub hostname: String,
    pub user: String,
    pub start_time: SystemTime,
    pub last_activity: SystemTime,
    pub context_tags: Vec<String>,         // e.g. ["persistence", "evasion"]
    pub current_trust_score: f64,
    pub escalation_triggered: bool,
}

lazy_static! {
    static ref SESSION_MAP: Arc<Mutex<HashMap<String, SessionMeta>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Registers or updates a session
pub fn register_or_update_session(hostname: &str, user: &str, context_tags: Vec<String>) -> String {
    let mut map = SESSION_MAP.lock().unwrap();
    let key = format!("{}::{}", hostname, user);

    let now = SystemTime::now();
    let session = map.entry(key.clone()).or_insert_with(|| {
        SessionMeta {
            session_id: Uuid::new_v4().to_string(),
            hostname: hostname.to_string(),
            user: user.to_string(),
            start_time: now,
            last_activity: now,
            context_tags: vec![],
            current_trust_score: 100.0,
            escalation_triggered: false,
        }
    });

    session.last_activity = now;

    for tag in context_tags {
        if !session.context_tags.contains(&tag) {
            session.context_tags.push(tag);
        }
    }

    session.session_id.clone()
}

/// Retrieves the session metadata by key
pub fn get_session_meta(hostname: &str, user: &str) -> Option<SessionMeta> {
    let map = SESSION_MAP.lock().unwrap();
    let key = format!("{}::{}", hostname, user);
    map.get(&key).cloned()
}

/// Updates trust score for a session
pub fn update_trust_score(hostname: &str, user: &str, new_score: f64) {
    let mut map = SESSION_MAP.lock().unwrap();
    let key = format!("{}::{}", hostname, user);
    if let Some(session) = map.get_mut(&key) {
        session.current_trust_score = new_score;
    }
}

/// Marks escalation trigger for session
pub fn trigger_escalation(hostname: &str, user: &str) {
    let mut map = SESSION_MAP.lock().unwrap();
    let key = format!("{}::{}", hostname, user);
    if let Some(session) = map.get_mut(&key) {
        session.escalation_triggered = true;
    }
}

/// Lists all active sessions for audit or export
pub fn list_active_sessions() -> Vec<SessionMeta> {
    let map = SESSION_MAP.lock().unwrap();
    map.values().cloned().collect()
}

/// Periodic decay handler (to be run via background thread or scheduler)
pub fn decay_trust_over_time(decay_rate: f64, decay_interval: Duration) {
    let mut map = SESSION_MAP.lock().unwrap();
    let now = SystemTime::now();

    for session in map.values_mut() {
        if let Ok(elapsed) = now.duration_since(session.last_activity) {
            if elapsed >= decay_interval {
                session.current_trust_score -= decay_rate;
                if session.current_trust_score < 0.0 {
                    session.current_trust_score = 0.0;
                }
            }
        }
    }
}

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::trust_hook::{build_trust_payload, submit_trust_signal};

// Stores last active timestamp per session, user, or endpoint
lazy_static::lazy_static! {
    static ref ACTIVITY_MAP: Mutex<HashMap<String, DateTime<Utc>>> = Mutex::new(HashMap::new());
}

/// Entry point for the decay logic
pub fn start_time_aware_trust_decay() {
    thread::spawn(|| loop {
        perform_decay_check();
        thread::sleep(Duration::from_secs(300)); // Every 5 minutes
    });
}

/// Updates last active timestamp for a given key (e.g., session ID or hostname)
pub fn update_activity_timestamp(entity_id: &str) {
    let mut map = ACTIVITY_MAP.lock().unwrap();
    map.insert(entity_id.to_string(), Utc::now());
}

/// Decays trust if an entity has been dormant for too long
fn perform_decay_check() {
    let mut map = ACTIVITY_MAP.lock().unwrap();
    let now = Utc::now();

    for (entity_id, last_seen) in map.iter() {
        let elapsed = now.signed_duration_since(*last_seen).num_minutes();

        let (decay_level, risk) = if elapsed > 1440 {
            ("severe_decay", 80) // > 24 hours
        } else if elapsed > 360 {
            ("moderate_decay", 60) // > 6 hours
        } else if elapsed > 60 {
            ("light_decay", 45) // > 1 hour
        } else {
            continue;
        };

        let payload = build_trust_payload(
            &format!("{}:{}", decay_level, entity_id),
            risk,
        );
        submit_trust_signal(payload);
    }
}

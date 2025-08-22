use std::thread;
use std::time::Duration;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::services::decay_engine::TrustDecayManager;
use crate::trust_hook::{generate_trust_payload};
use crate::replay_writer::store_replay_event;
use crate::types::MemorySnapshot;

/// Simulated session cache (replace with real agent state or persistent cache)
static SESSION_IDS: &[&str] = &[
    "session-alpha-123",
    "session-beta-456",
    "session-gamma-789",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub session_id: String,
    pub last_seen: u64,
    pub trust_score: f32,
    pub risk_level: u32,
}

/// Starts a background thread that revalidates session trust levels.
pub fn start_session_revalidator(mut trust_cache: TrustDecayManager, hostname: &str) {
    let hostname = hostname.to_string();

    thread::spawn(move || loop {
        println!("[🔄 Session Revalidator] Starting session trust recheck cycle...");

        for session_id in SESSION_IDS {
            if let Some(decayed_score) = trust_cache.get_decayed_score(session_id) {
                // Simulate MemorySnapshot
                let fake_evt = MemorySnapshot {
                    pid: 0,
                    cpu_usage: Some(20.0),
                    memory_usage: Some(60000.0),
                    risk_score: Some(decayed_score as f32),
                    timestamp: std::time::SystemTime::now(),
                    ..Default::default()
                };

                let payload = generate_trust_payload(&fake_evt);
                let serialized_payload = serde_json::to_string(&payload.to_map()).unwrap_or_default();

                println!(
                    "[🚨 Revalidation Alert] Session {} flagged for escalation: {:?}",
                    session_id, payload.to_map()
                );

                if decayed_score < 0.35 {
                    let audit = RevalidationAudit {
                        session_id: session_id.to_string(),
                        hostname: hostname.clone(),
                        timestamp: Utc::now().timestamp(),
                        decayed_score,
                        trust_payload: serialized_payload,
                    };

                    let _ = store_replay_event(audit);
                }
            }
        }

        trust_cache.prune_expired(3600 * 24);
        thread::sleep(Duration::from_secs(180));
    });
}

#[derive(Debug, Serialize, Clone)]
struct RevalidationAudit {
    session_id: String,
    hostname: String,
    timestamp: i64,
    decayed_score: f64,
    trust_payload: String,
}

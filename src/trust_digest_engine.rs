
use std::collections::HashMap;
use chrono::{Utc, DateTime};

/// Represents a digest of trust-relevant behavior across sessions.
pub struct TrustDigestEngine {
    behavior_scores: HashMap<String, f64>,  // session_id → accumulated score
    last_updated: HashMap<String, DateTime<Utc>>, // timestamp of last score update
}

impl TrustDigestEngine {
    /// Initialize the engine.
    pub fn new() -> Self {
        TrustDigestEngine {
            behavior_scores: HashMap::new(),
            last_updated: HashMap::new(),
        }
    }

    /// Ingest a new behavior observation and update the score.
    pub fn ingest_score(&mut self, session_id: &str, delta: f64) {
        let entry = self.behavior_scores.entry(session_id.to_string()).or_insert(0.0);
        *entry += delta;
        self.last_updated.insert(session_id.to_string(), Utc::now());
    }

    /// Retrieve the current cumulative score.
    pub fn get_score(&self, session_id: &str) -> f64 {
        *self.behavior_scores.get(session_id).unwrap_or(&0.0)
    }

    /// Export all tracked session scores.
    pub fn export_all(&self) -> HashMap<String, f64> {
        self.behavior_scores.clone()
    }

    /// Reset score tracking for a specific session.
    pub fn reset_session(&mut self, session_id: &str) {
        self.behavior_scores.remove(session_id);
        self.last_updated.remove(session_id);
    }
}
pub fn process_digest_file(path: &str) {
    println!("Processing digest file at: {}", path);
    // TODO: actual logic here
}
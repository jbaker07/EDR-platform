use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct TrustPoint {
    pub timestamp: u64,
    pub score: f64,
}

#[derive(Debug, Clone)]
pub struct SessionTrustCurve {
    pub session_id: String,
    pub history: Vec<TrustPoint>,
    pub role: String,
    pub decay_profile: DecayProfile,
}

#[derive(Debug, Clone)]
pub struct DecayProfile {
    pub drop_rate: f64,
    pub recovery_rate: f64,
    pub min_score: f64,
    pub max_score: f64,
}

impl SessionTrustCurve {
    pub fn new(session_id: &str, role: &str) -> Self {
        let decay_profile = get_decay_profile_for_role(role);
        SessionTrustCurve {
            session_id: session_id.to_string(),
            history: vec![],
            role: role.to_string(),
            decay_profile,
        }
    }

    pub fn update(&mut self, new_score: f64, event_type: &str) {
        let ts = current_timestamp();
        let last_score = self.history.last().map(|tp| tp.score).unwrap_or(100.0);
        let adjusted_score = match event_type {
            "deviation" => (last_score - self.decay_profile.drop_rate * (100.0 - new_score)).max(self.decay_profile.min_score),
            "recovery" => (last_score + self.decay_profile.recovery_rate * new_score).min(self.decay_profile.max_score),
            _ => new_score.clamp(self.decay_profile.min_score, self.decay_profile.max_score),
        };

        self.history.push(TrustPoint {
            timestamp: ts,
            score: adjusted_score,
        });

        // Optional: emit snapshot to file, visual log, or trigger revalidation
    }

    pub fn current_score(&self) -> f64 {
        self.history.last().map(|tp| tp.score).unwrap_or(100.0)
    }

    pub fn is_decay_triggered(&self, threshold: f64) -> bool {
        self.current_score() < threshold
    }

    pub fn get_trust_trend(&self) -> String {
        if self.history.len() < 2 {
            return "insufficient".to_string();
        }
        let len = self.history.len();
        let recent_delta = self.history[len - 1].score - self.history[len - 2].score;
        if recent_delta < -2.0 {
            "declining".into()
        } else if recent_delta > 2.0 {
            "improving".into()
        } else {
            "stable".into()
        }
    }

    /// ✅ NEW: Determines whether escalation should be triggered
    pub fn escalation_triggered(&self) -> bool {
        self.current_score() < 30.0
    }
}

fn get_decay_profile_for_role(role: &str) -> DecayProfile {
    match role {
        "developer" => DecayProfile { drop_rate: 0.04, recovery_rate: 0.015, min_score: 10.0, max_score: 100.0 },
        "admin" => DecayProfile { drop_rate: 0.06, recovery_rate: 0.02, min_score: 15.0, max_score: 100.0 },
        "executive" => DecayProfile { drop_rate: 0.05, recovery_rate: 0.01, min_score: 5.0, max_score: 100.0 },
        _ => DecayProfile { drop_rate: 0.05, recovery_rate: 0.0125, min_score: 0.0, max_score: 100.0 },
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustState {
    pub last_observed: u64,   // UNIX timestamp
    pub last_score: f64,      // Last computed trust score (0.0 to 1.0)
    pub decay_half_life: u64, // In seconds (e.g., 3600 for 1 hour half-life)
}

impl TrustState {
    /// Applies exponential decay based on half-life.
    pub fn compute_decayed_score(&self) -> f64 {
        let now = current_time();
        let elapsed = now.saturating_sub(self.last_observed) as f64;

        let half_life = self.decay_half_life.max(1) as f64;
        let decay_factor = (-elapsed * std::f64::consts::LN_2 / half_life).exp();

        (self.last_score * decay_factor).clamp(0.0, 1.0)
    }
}

/// Trust decay map for in-memory or persistent agent-level cache
pub struct TrustDecayManager {
    trust_cache: HashMap<String, TrustState>, // Keyed by hostname or session ID
}

impl TrustDecayManager {
    pub fn new() -> Self {
        Self {
            trust_cache: HashMap::new(),
        }
    }

    /// Update trust state with a new score and timestamp
    pub fn update_score(&mut self, id: &str, score: f64, half_life: u64) {
        self.trust_cache.insert(
            id.to_string(),
            TrustState {
                last_observed: current_time(),
                last_score: score.clamp(0.0, 1.0),
                decay_half_life: half_life,
            },
        );
    }

    /// Retrieve decayed trust score if exists
    pub fn get_decayed_score(&self, id: &str) -> Option<f64> {
        self.trust_cache
            .get(id)
            .map(|state| state.compute_decayed_score())
    }

    /// Cleanup entries older than a TTL (for memory-bound agents)
    pub fn prune_expired(&mut self, max_age_secs: u64) {
        let now = current_time();
        self.trust_cache
            .retain(|_, state| now.saturating_sub(state.last_observed) < max_age_secs);
    }

    /// Export current trust cache for replay or persistence
    pub fn export_trust_state(&self) -> HashMap<String, TrustState> {
        self.trust_cache.clone()
    }
}

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}

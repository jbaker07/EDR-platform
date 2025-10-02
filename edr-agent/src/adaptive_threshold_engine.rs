use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

/// Core engine managing adaptive risk thresholds.
pub struct AdaptiveThresholdEngine {
    pub default_threshold: f64,
    pub decay_rate: f64,
    thresholds: HashMap<String, f64>, // session_id → adaptive threshold
    last_updated: HashMap<String, DateTime<Utc>>,
}

/// Parsed role-based thresholds from JSON
#[derive(Debug, Deserialize)]
pub struct RoleRiskProfileMap {
    pub roles: HashMap<String, RoleProfile>,
}

#[derive(Debug, Deserialize)]
pub struct RoleProfile {
    pub trust_decay_rate: f64,
    pub high_risk_threshold: Option<f64>,
    pub critical_risk_threshold: Option<f64>,
}

impl AdaptiveThresholdEngine {
    /// Creates a new engine with a default baseline and decay
    pub fn new(default_threshold: f64, decay_rate: f64) -> Self {
        Self {
            default_threshold,
            decay_rate,
            thresholds: HashMap::new(),
            last_updated: HashMap::new(),
        }
    }

    /// Load role-aware thresholds from config
    pub fn from_role(path: &str, role: &str) -> Self {
        let profile_map = load_role_profile_map(path);
        let profile = profile_map
            .roles
            .get(role)
            .unwrap_or_else(|| panic!("Role '{}' not found in config", role));

        let decay = profile.trust_decay_rate;
        let threshold = profile.high_risk_threshold.unwrap_or(0.25);

        Self::new(threshold, decay)
    }

    /// Get threshold for a session, defaulting if not yet tracked
    pub fn get_threshold(&self, session_id: &str) -> f64 {
        *self
            .thresholds
            .get(session_id)
            .unwrap_or(&self.default_threshold)
    }

    /// Update session threshold with new risk score
    pub fn update(&mut self, session_id: &str, score: f64) {
        let old = self.get_threshold(session_id);
        let new = (old * (1.0 - self.decay_rate)) + (score * self.decay_rate);
        self.thresholds.insert(session_id.to_string(), new);
        self.last_updated.insert(session_id.to_string(), Utc::now());

        println!(
            "[ThresholdEngine] Session={} | Old={:.4} | New={:.4} | Incoming={:.4}",
            session_id, old, new, score
        );
    }

    /// Returns true if incoming score crosses the current threshold
    pub fn should_escalate(&self, session_id: &str, score: f64) -> bool {
        score >= self.get_threshold(session_id)
    }

    /// Manual reset
    pub fn clear_session(&mut self, session_id: &str) {
        self.thresholds.remove(session_id);
        self.last_updated.remove(session_id);
    }

    /// Dump all thresholds (for debug or state persistence)
    pub fn export_state(&self) -> HashMap<String, f64> {
        self.thresholds.clone()
    }
}

/// Load the full role risk profile map from JSON
pub fn load_role_profile_map(path: &str) -> RoleRiskProfileMap {
    let mut file = File::open(path)
        .unwrap_or_else(|_| panic!("❌ Failed to open role profile map at {}", path));

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("❌ Failed to read role profile map file");

    serde_json::from_str(&contents)
        .unwrap_or_else(|_| panic!("❌ Failed to parse role profile map JSON from {}", path))
}

/// Helper function for scoring pipelines
pub fn evaluate_and_escalate(
    session_id: &str,
    score: f64,
    engine: &mut AdaptiveThresholdEngine,
) -> bool {
    engine.update(session_id, score);
    engine.should_escalate(session_id, score)
}

/// Optional score multiplier based on role's high_risk_threshold
pub fn evaluate_role_thresholds(role: &str, score: f64, profile_map: &RoleRiskProfileMap) -> f64 {
    match profile_map.roles.get(role) {
        Some(profile) => {
            if let Some(multiplier) = profile.high_risk_threshold {
                score * (1.0 / multiplier) // adjust based on tighter thresholds
            } else {
                score
            }
        }
        None => score, // fallback
    }
}

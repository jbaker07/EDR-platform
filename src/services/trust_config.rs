use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrustConfig {
    pub decay_rate_per_minute: f64,
    pub trust_floor: f64,
    pub escalation_threshold: f64,
    pub forensic_trigger_score: f64,
    pub minimal_mode_baseline: f64,
    pub gnn_score_weight: f64,
    pub shap_score_weight: f64,
    pub semantic_tag_weights: HashMap<String, f64>,
    pub trust_recovery_grace_period_sec: u64,
}

impl Default for TrustConfig {
    fn default() -> Self {
        TrustConfig {
            decay_rate_per_minute: 0.02,                      // 2% decay per minute
            trust_floor: 0.1,                                  // Lower bound
            escalation_threshold: 0.35,                        // Trust score that triggers replay or isolation
            forensic_trigger_score: 0.45,                      // Trust threshold for switching to forensic mode
            minimal_mode_baseline: 0.9,                        // Score to enter minimal monitoring
            gnn_score_weight: 0.5,
            shap_score_weight: 0.3,
            semantic_tag_weights: HashMap::new(),              // Populated from JSON file
            trust_recovery_grace_period_sec: 900,              // 15 minutes before trust recovers upward
        }
    }
}

impl TrustConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let content = fs::read_to_string(&path);
        match content {
            Ok(data) => {
                match serde_json::from_str::<TrustConfig>(&data) {
                    Ok(config) => config,
                    Err(e) => {
                        eprintln!("[trust_config.rs] Error parsing JSON config: {:?}", e);
                        TrustConfig::default()
                    }
                }
            }
            Err(_) => {
                eprintln!("[trust_config.rs] Could not read config file. Falling back to default.");
                TrustConfig::default()
            }
        }
    }
}

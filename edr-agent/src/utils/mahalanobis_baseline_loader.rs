use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::from_str;
use crate::services::trust_vector::TrustVector;
use crate::logger::log;

lazy_static! {
    pub static ref BASELINES: HashMap<String, TrustVector> = load_baselines();
    pub static ref DEFAULT_BASELINE: TrustVector = TrustVector::new();
}

fn load_baselines() -> HashMap<String, TrustVector> {
    let path = "json_files/mahalanobis_baselines.json";
    let mut map = HashMap::new();

    match fs::read_to_string(path) {
        Ok(content) => {
            if let Ok(parsed) = from_str::<HashMap<String, TrustVector>>(&content) {
                map = parsed;
                log("baseline_loader", &format!("✅ Loaded {} role baselines", map.len()));
            } else {
                log("baseline_loader", "❌ Failed to parse JSON structure for role baselines.");
            }
        }
        Err(_) => {
            log("baseline_loader", "⚠️ No baseline file found, using empty map.");
        }
    }

    map
}

pub fn ensure_baseline_loaded(role: &str) {
    if !BASELINES.contains_key(role) {
        log("baseline_loader", &format!("⚠️ Missing Mahalanobis baseline for role '{}'", role));
    }
}

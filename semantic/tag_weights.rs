use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use serde::Deserialize;

/// Global static cache of tag weights loaded from disk
static TAG_WEIGHTS: Lazy<RwLock<HashMap<String, f64>>> = Lazy::new(|| {
    let path = "ontology/tag_risk_weights.json";
    let data = fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
    let parsed: HashMap<String, f64> = serde_json::from_str(&data).unwrap_or_default();
    RwLock::new(parsed)
});

/// Public interface: get weight of tag (e.g., "execution::process_spawn")
pub fn get_tag_weight(tag: &str) -> f64 {
    TAG_WEIGHTS
        .read()
        .unwrap()
        .get(tag)
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("⚠️ Unknown tag weight for: {}", tag);
            0.0
        })
}

use std::{collections::HashMap, fs};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KnownFingerprint {
    pub hash: String,
}

pub fn load_known_safe_fingerprints(path: &str) -> HashMap<String, KnownFingerprint> {
    let file = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str::<HashMap<String, KnownFingerprint>>(&file).unwrap_or_default()
}

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawPolicy {
    pub collection_interval: u64,
    pub endpoint_role: String,
    pub mode: String,
    pub trust_threshold: f32,
    pub gnn_enabled: bool,
    pub logging: LoggingConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingConfig {
    pub level: String,
}

pub fn load_and_verify_policy<P: AsRef<Path>>(path: P) -> anyhow::Result<RawPolicy> {
    let blob = fs::read_to_string(path)?;
    let wrapper: serde_json::Value = serde_json::from_str(&blob)?;

    // Extract the "policy" field only (ignore signature/pubkey for now)
    let raw_policy = serde_json::from_value(wrapper["policy"].clone())?;
    Ok(raw_policy)
}

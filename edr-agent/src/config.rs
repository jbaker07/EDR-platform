// src/config.rs
use serde::{Deserialize, Serialize};
use std::{fs, path::{Path, PathBuf}};

/// ----------------------------
/// Signed policy (you already use this)
/// ----------------------------
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

/// Reads a JSON wrapper like:
/// { "policy": { ... }, "signature": "...", "pubkey": "..." }
pub fn load_and_verify_policy<P: AsRef<Path>>(path: P) -> anyhow::Result<RawPolicy> {
    let blob = fs::read_to_string(path)?;
    let wrapper: serde_json::Value = serde_json::from_str(&blob)?;
    // Extract the "policy" field only (ignore signature/pubkey for now)
    let raw_policy = serde_json::from_value(wrapper["policy"].clone())?;
    Ok(raw_policy)
}

/// ----------------------------
/// App runtime config (rules, thresholds, etc.)
/// ----------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Thresholds {
    pub low: f32,
    pub medium: f32,
    pub high: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesCfg {
    /// Path to the rules JSON (e.g., "edr-agent/rules.json")
    pub path: PathBuf,
    /// Enable hot-reload watcher (optional)
    #[serde(default)]
    pub hot_reload: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationsCatalogCfg {
    /// Path to integrations catalog JSON (e.g., "integrations/catalog.json")
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub rules: RulesCfg,
    pub thresholds: Thresholds,

    /// Optional extra config; keep it optional so older configs still parse
    #[serde(default)]
    pub integrations_catalog: Option<IntegrationsCatalogCfg>,
}

impl Default for RulesCfg {
    fn default() -> Self {
        Self { path: PathBuf::from("edr-agent/src/rules.json"), hot_reload: true }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rules: RulesCfg::default(),
            thresholds: Thresholds { low: 0.30, medium: 0.55, high: 0.80 },
            integrations_catalog: Some(IntegrationsCatalogCfg { path: PathBuf::from("integrations/catalog.json") }),
        }
    }
}

/// Load a simple JSON config for the app’s runtime knobs.
/// Example:
/// {
///   "rules": { "path": "edr-agent/rules.json", "hot_reload": true },
///   "thresholds": { "low": 0.30, "medium": 0.55, "high": 0.80 },
///   "integrations_catalog": { "path": "integrations/catalog.json" }
/// }
pub fn load_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let s = fs::read_to_string(path)?;
    let cfg: Config = serde_json::from_str(&s)?;
    Ok(cfg)
}

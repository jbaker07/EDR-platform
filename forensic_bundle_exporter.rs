use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};
use chrono::{Utc, SecondsFormat};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForensicBundle {
    pub session_id: String,
    pub trust_score: f32,
    pub timestamp: u64,
    pub tags: Vec<String>,
    pub replay_path: String,
    pub justification: String,
    pub hash: String, // SHA256 of replay + metadata
}

pub fn export_forensic_bundle(
    session_id: &str,
    trust_score: f32,
    tags: Vec<String>,
    replay_data: &str,
    justification: &str,
    export_dir: &str,
) -> Result<ForensicBundle, String> {
    let ts = current_ts();
    let timestamp_str = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

    // Create bundle directory
    let bundle_name = format!("{}_bundle_{}", session_id, timestamp_str.replace(":", "-"));
    let bundle_dir = PathBuf::from(export_dir).join(&bundle_name);
    fs::create_dir_all(&bundle_dir).map_err(|e| e.to_string())?;

    // Write replay file
    let replay_path = bundle_dir.join("replay.json");
    let mut replay_file = File::create(&replay_path).map_err(|e| e.to_string())?;
    replay_file
        .write_all(replay_data.as_bytes())
        .map_err(|e| e.to_string())?;

    // Generate hash of replay + metadata
    let hash_input = format!("{}|{}|{:?}|{}", session_id, trust_score, tags, replay_data);
    let hash = hex::encode(Sha256::digest(hash_input.as_bytes()));

    let bundle = ForensicBundle {
        session_id: session_id.to_string(),
        trust_score,
        timestamp: ts,
        tags,
        replay_path: replay_path.to_string_lossy().into_owned(),
        justification: justification.to_string(),
        hash,
    };

    // Save metadata
    let metadata_path = bundle_dir.join("bundle_metadata.json");
    let metadata_json = serde_json::to_string_pretty(&bundle).map_err(|e| e.to_string())?;
    let mut meta_file = File::create(&metadata_path).map_err(|e| e.to_string())?;
    meta_file.write_all(metadata_json.as_bytes()).map_err(|e| e.to_string())?;

    Ok(bundle)
}

fn current_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

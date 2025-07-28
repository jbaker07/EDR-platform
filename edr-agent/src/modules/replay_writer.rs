use std::fs::{OpenOptions, create_dir_all, File, metadata, rename};
use std::io::{Write, BufWriter};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use chrono::Utc;
use serde::Serialize;
use anyhow::Result;
use crate::services::ontology_mapper::SemanticTagInfo;
use crate::utils::time::now_ts;
use sha2::{Sha256, Digest};
use fs2::FileExt;

const REPLAY_LOG_PATH: &str = "/var/log/edr/replay_archive.jsonl";
const REPLAY_DIR: &str = "/var/log/edr/replays";
const TMP_QUEUE_PATH: &str = "/tmp/replay_queue.log";
const MAX_LOG_SIZE_BYTES: u64 = 10 * 1024 * 1024; // 10 MB

/// Store any serializable telemetry event for replay with hash chaining and locking.
pub fn store_replay_event<T: Serialize>(event: T) -> Result<(), std::io::Error> {
    create_dir_all(Path::new(REPLAY_LOG_PATH).parent().unwrap())?;
    rotate_if_needed(REPLAY_LOG_PATH)?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(REPLAY_LOG_PATH)?;

    file.lock_exclusive()?; // Lock the file
    let previous_hash = compute_last_hash(REPLAY_LOG_PATH)?;
    let serialized = serde_json::to_string(&event)?;
    let chained_payload = serde_json::json!({
        "timestamp": now_ts(),
        "data": event,
        "previous_hash": previous_hash,
        "hash": compute_hash(&serialized)
    });

    writeln!(file, "{}", serde_json::to_string(&chained_payload)?)?;
    file.unlock()?;
    Ok(())
}

/// Write structured replay session file
pub fn queue_replay(
    session_id: &str,
    process_id: u32,
    trust_score: f64,
    reason: &str,
    enriched_tags: Vec<SemanticTagInfo>,
) {
    if let Err(e) = create_dir_all(REPLAY_DIR) {
        eprintln!("❌ Failed to create replay log dir: {}", e);
        return;
    }

    let timestamp = Utc::now().to_rfc3339();
    let path = format!("{}/{}.replay.json", REPLAY_DIR, session_id);

    let tag_info: Vec<_> = enriched_tags
        .iter()
        .map(|t| format!("{} [{}] - {}", t.tag, t.category, t.description))
        .collect();

    let payload = serde_json::json!({
        "session_id": session_id,
        "timestamp": timestamp,
        "trust_score": trust_score,
        "process_id": process_id,
        "reason": reason,
        "ontology_context": tag_info,
    });

    match OpenOptions::new().create(true).append(true).open(&path) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", payload) {
                eprintln!("❌ Error writing replay payload: {}", e);
            }
        }
        Err(e) => eprintln!("❌ Error opening replay file: {}", e),
    };
}

/// Queues minimal replay log
pub fn send_to_replay_queue(data: &str) -> Result<()> {
    create_dir_all(Path::new(TMP_QUEUE_PATH).parent().unwrap())?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(TMP_QUEUE_PATH)?;

    file.lock_exclusive()?;
    writeln!(file, "{}", data)?;
    file.unlock()?;
    Ok(())
}

/// Rotate the file if too large
fn rotate_if_needed(path: &str) -> std::io::Result<()> {
    if metadata(path).map(|m| m.len()).unwrap_or(0) > MAX_LOG_SIZE_BYTES {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S");
        let backup = format!("{}.{}", path, timestamp);
        rename(path, backup)?;
    }
    Ok(())
}

/// Compute SHA256 hash of a string
fn compute_hash(payload: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Get last hash from the replay file for chaining
fn compute_last_hash(path: &str) -> std::io::Result<String> {
    if !Path::new(path).exists() {
        return Ok("GENESIS".into());
    }

    let content = std::fs::read_to_string(path)?;
    if let Some(last_line) = content.lines().last() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(last_line) {
            if let Some(hash) = json.get("hash").and_then(|h| h.as_str()) {
                return Ok(hash.to_string());
            }
        }
    }

    Ok("UNKNOWN".into())
}

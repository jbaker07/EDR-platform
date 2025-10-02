use std::fs::{create_dir_all, metadata, rename, OpenOptions};
use std::io::Write;
use std::path::Path;

use chrono::Utc;
use serde::Serialize;

use fs2::FileExt; // advisory file locks on *nix

use sha2::{Digest, Sha256};

use crate::services::ontology_mapper::SemanticTagInfo;
use crate::utils::time::now_ts;

/// Where we keep the append-only replay chain (rotated at MAX_LOG_SIZE_BYTES).
const REPLAY_LOG_PATH: &str = "/var/log/edr/replay_archive.jsonl";

/// Per-session richer artifacts (human/debug).
const REPLAY_DIR: &str = "/var/log/edr/replays";

/// Small scratch queue for bursts or sidecar shipper.
const TMP_QUEUE_PATH: &str = "/tmp/replay_queue.log";

/// 10 MB per file before we rotate with a timestamp suffix.
const MAX_LOG_SIZE_BYTES: u64 = 10 * 1024 * 1024; // 10 MB

/// Store any serializable telemetry event for replay with
/// *exclusive locking*, *bounded rotation*, and *hash chaining*.
///
/// Chain format (one JSON line per record):
/// {
///   "timestamp": <secs>,
///   "data": <event>,
///   "previous_hash": "<prev>",
///   "hash": "<sha256(timestamp|previous_hash|serialized_data)>"
/// }
pub fn store_replay_event<T: Serialize>(event: T) -> std::io::Result<()> {
    // Ensure log parent exists
    if let Some(parent) = Path::new(REPLAY_LOG_PATH).parent() {
        create_dir_all(parent)?;
    }

    // Open + lock (advisory) the current log file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(REPLAY_LOG_PATH)?;

    file.lock_exclusive()?;

    // ------- Rotation under lock to avoid races -------
    let size = file.metadata()?.len();
    if size > MAX_LOG_SIZE_BYTES {
        // Unlock before renaming the *open* file (some FS balk at renaming open handles)
        file.unlock()?;
        rotate_now(REPLAY_LOG_PATH)?;
        // Reopen and re-lock fresh file
        file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(REPLAY_LOG_PATH)?;
        file.lock_exclusive()?;
    }
    // --------------------------------------------------

    // Compute previous hash from the current file (safe—still locked)
    let previous_hash = compute_last_hash(REPLAY_LOG_PATH)?;

    // Serialize the event cleanly
    let serialized = serde_json::to_string(&event)?;
    let ts = now_ts();

    // Hash covers timestamp + previous_hash + raw serialized data
    let hash_input = format!("{}|{}|{}", ts, previous_hash, serialized);
    let record_hash = compute_hash(&hash_input);

    let chained_payload = serde_json::json!({
        "timestamp": ts,
        "data": event,
        "previous_hash": previous_hash,
        "hash": record_hash
    });

    // Append atomically to the end (we hold the lock)
    let line = serde_json::to_string(&chained_payload)?;
    file.write_all(line.as_bytes())?;
    file.write_all(b"\n")?;

    // Ensure durability (journal/fs dependent; sync_all is stronger than flush)
    file.sync_all()?;

    // Release lock
    file.unlock()?;
    Ok(())
}

/// Write structured replay session file (human-readable / append-only).
pub fn queue_replay(
    session_id: &str,
    process_id: u32,
    trust_score: f64,
    reason: &str,
    enriched_tags: Vec<SemanticTagInfo>,
) {
    if let Err(e) = create_dir_all(REPLAY_DIR) {
        eprintln!("❌ Failed to create replay dir '{}': {}", REPLAY_DIR, e);
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
        Ok(mut f) => {
            if let Err(e) = writeln!(f, "{}", payload) {
                eprintln!("❌ Error writing replay payload: {}", e);
            } else {
                let _ = f.sync_all();
            }
        }
        Err(e) => eprintln!("❌ Error opening replay file '{}': {}", path, e),
    }
}

/// Queue a minimal replay log line to a temp file (for a sidecar shipper).
pub fn send_to_replay_queue(data: &str) -> anyhow::Result<()> {
    if let Some(parent) = Path::new(TMP_QUEUE_PATH).parent() {
        create_dir_all(parent)?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(TMP_QUEUE_PATH)?;

    file.lock_exclusive()?;
    writeln!(file, "{}", data)?;
    file.sync_all()?;
    file.unlock()?;
    Ok(())
}

/// Rotate `path` by renaming to `<path>.<YYYYMMDDHHMMSS>`.
fn rotate_now(path: &str) -> std::io::Result<()> {
    let ts = Utc::now().format("%Y%m%d%H%M%S");
    let backup = format!("{}.{}", path, ts);
    rename(path, backup)?;
    Ok(())
}

/// Legacy check: rotate by metadata length (kept for completeness).
#[allow(dead_code)]
fn rotate_if_needed(path: &str) -> std::io::Result<()> {
    if metadata(path).map(|m| m.len()).unwrap_or(0) > MAX_LOG_SIZE_BYTES {
        rotate_now(path)?;
    }
    Ok(())
}

/// Compute SHA256 hex for an input string (lowercase).
fn compute_hash(payload: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Fetch the most recent `hash` from the JSONL file, or "GENESIS".
///
/// Note: We keep this simple since files are rotated at 10MB.
/// If you want to optimize further, implement a reverse reader to
/// scan from the end instead of loading the whole file.
fn compute_last_hash(path: &str) -> std::io::Result<String> {
    let p = Path::new(path);
    if !p.exists() {
        return Ok("GENESIS".into());
    }

    let content = std::fs::read_to_string(p)?;
    if let Some(last_line) = content.lines().rev().find(|l| !l.trim().is_empty()) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(last_line) {
            if let Some(h) = v.get("hash").and_then(|x| x.as_str()) {
                return Ok(h.to_string());
            }
        }
    }

    Ok("GENESIS".into())
}

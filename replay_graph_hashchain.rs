use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplayEvent {
    pub timestamp: u64,
    pub event_type: String,
    pub session_id: String,
    pub telemetry_data: String, // Canonical JSON or encoded string
    pub prev_hash: String,
    pub hash: String,
}

fn current_ts() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

/// Computes a SHA-256 hash of the input string
fn compute_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}

/// Builds a hash chain element for the given event
fn build_replay_event(
    event_type: &str,
    session_id: &str,
    telemetry_data: &str,
    prev_hash: &str,
) -> ReplayEvent {
    let timestamp = current_ts();
    let payload = format!("{}|{}|{}|{}", timestamp, event_type, telemetry_data, prev_hash);
    let hash = compute_hash(&payload);

    ReplayEvent {
        timestamp,
        event_type: event_type.to_string(),
        session_id: session_id.to_string(),
        telemetry_data: telemetry_data.to_string(),
        prev_hash: prev_hash.to_string(),
        hash,
    }
}

/// Writes a hash-chained replay event to the specified file
pub fn append_to_hashchain_log<P: AsRef<Path>>(
    file_path: P,
    event_type: &str,
    session_id: &str,
    telemetry_data: &str,
) -> Result<(), String> {
    let existing_lines = fs::read_to_string(&file_path).unwrap_or_default();
    let last_hash = match existing_lines.lines().last() {
        Some(line) => {
            serde_json::from_str::<ReplayEvent>(line)
                .map(|e| e.hash)
                .unwrap_or_else(|_| "GENESIS".to_string())
        }
        None => "GENESIS".to_string(),
    };

    let new_event = build_replay_event(event_type, session_id, telemetry_data, &last_hash);

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .map_err(|e| format!("File open error: {}", e))?;

    let mut writer = BufWriter::new(file);
    let json = serde_json::to_string(&new_event).map_err(|e| format!("Serialize error: {}", e))?;
    writer
        .write_all(format!("{}\n", json).as_bytes())
        .map_err(|e| format!("Write error: {}", e))?;

    Ok(())
}

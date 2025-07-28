use sha2::{Sha256, Digest};
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;

/// Represents a single log entry and its integrity state.
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64,
    pub hash: String,
    pub prev_hash: String,
    pub valid: bool,
}

/// Computes the SHA256 hash of given data string.
pub fn hash_entry(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Verifies a directory of log files for hash-chain continuity.
pub fn verify_log_chain<P: AsRef<Path>>(dir_path: P) -> Vec<LogEntry> {
    let mut entries = Vec::new();
    let mut prev_hash = String::from("GENESIS");

    let paths = fs::read_dir(&dir_path).unwrap_or_else(|_| panic!("Cannot read dir {:?}", dir_path.as_ref()));

    let mut logs: Vec<_> = paths.filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "log").unwrap_or(false))
        .collect();

    logs.sort_by_key(|e| e.file_name());

    for file in logs {
        let path = file.path();
        let mut content = String::new();
        let mut reader = BufReader::new(File::open(&path).expect("Failed to open log file"));
        reader.read_to_string(&mut content).expect("Failed to read log");

        let hash = hash_entry(&content);
        let expected_prev = extract_prev_hash(&content).unwrap_or_default();
        let valid = expected_prev == prev_hash;

        entries.push(LogEntry {
            timestamp: extract_timestamp(&content).unwrap_or(0),
            hash: hash.clone(),
            prev_hash: expected_prev.clone(),
            valid,
        });

        prev_hash = hash;
    }

    entries
}

/// Extracts previous hash from log metadata.
fn extract_prev_hash(content: &str) -> Option<String> {
    content.lines()
        .find(|line| line.starts_with("PrevHash:"))
        .map(|line| line.trim_start_matches("PrevHash:").trim().to_string())
}

/// Extracts timestamp from log metadata.
fn extract_timestamp(content: &str) -> Option<u64> {
    content.lines()
        .find(|line| line.starts_with("Timestamp:"))
        .and_then(|line| line.trim_start_matches("Timestamp:").trim().parse().ok())
}

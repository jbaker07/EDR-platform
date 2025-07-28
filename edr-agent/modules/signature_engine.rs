// edr-agent/forensic/signature_engine.rs

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest};
use lazy_static::lazy_static;
use regex::Regex;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref KNOWN_BAD_HASHES: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(load_known_hashes()));
    static ref KNOWN_CMD_PATTERNS: Vec<Regex> = vec![
        Regex::new(r#"wget\s+http.*"#).unwrap(),
        Regex::new(r#"curl\s+.*\|\s*sh"#).unwrap(),
        Regex::new(r#"powershell\s+-enc\s+"#).unwrap(),
        Regex::new(r#"reg\s+add\s+.*Run"#).unwrap(),
    ];
}

/// Load SHA256 hashes from file (e.g., threat signatures)
fn load_known_hashes() -> HashSet<String> {
    let mut set = HashSet::new();
    let path = "/etc/edr/known_bad_hashes.txt";
    if let Ok(contents) = fs::read_to_string(path) {
        for line in contents.lines() {
            set.insert(line.trim().to_string());
        }
    }
    set
}

/// Public interface to refresh signatures at runtime
pub fn refresh_signature_db() {
    let mut hashes = KNOWN_BAD_HASHES.lock().unwrap();
    *hashes = load_known_hashes();
}

/// Hashes the contents of a file and checks for known malicious signatures
pub fn check_file_signature(file_path: &str) -> Option<String> {
    if let Ok(data) = fs::read(file_path) {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash = format!("{:x}", hasher.finalize());

        let db = KNOWN_BAD_HASHES.lock().unwrap();
        if db.contains(&hash) {
            return Some(format!("Known malicious file hash detected: {}", hash));
        }
    }
    None
}

/// Check if a command line string matches known bad patterns
pub fn check_command_signature(cmdline: &str) -> Option<String> {
    for pattern in KNOWN_CMD_PATTERNS.iter() {
        if pattern.is_match(cmdline) {
            return Some(format!("Command pattern matched: {}", pattern.as_str()));
        }
    }
    None
}

/// Hash an in-memory blob and compare against signatures
pub fn check_memory_blob_signature(data: &[u8]) -> Option<String> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = format!("{:x}", hasher.finalize());

    let db = KNOWN_BAD_HASHES.lock().unwrap();
    if db.contains(&hash) {
        return Some(format!("Known malicious memory hash detected: {}", hash));
    }
    None
}

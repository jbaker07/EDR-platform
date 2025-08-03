use chrono::Utc;
use std::collections::HashSet;
use std::sync::Mutex;
use std::fs::OpenOptions;
use std::io::Write;

lazy_static::lazy_static! {
    static ref LOG_CACHE: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}
use crate::logger::{log, log_once, log_suppression_decision};

log("TrustEngine", "Scoring started");
log_once("MemoryScan", "Skipping scoring: missing stats");
log_suppression_decision("privilege_monitor", "/usr/bin/sudo", "Known-good hash");

/// Standard log with timestamp and scope
pub fn log(scope: &str, message: &str) {
    println!("[{}][{}] {}", Utc::now().format("%H:%M:%S"), scope, message);
}

/// Log only once for deduplication across runs
pub fn log_once(scope: &str, message: &str) {
    let key = format!("[{}] {}", scope, message);
    let mut cache = LOG_CACHE.lock().unwrap();
    if cache.insert(key.clone()) {
        println!("[{}][{}] {}", Utc::now().format("%H:%M:%S"), scope, message);
    }
}

/// Writes suppression decisions to audit file
pub fn log_suppression_decision(module: &str, path: &str, reason: &str) {
    let ts = Utc::now().to_rfc3339();
    let log_line = format!(
        "[🔇 Suppression] [{}] Module={} | Path={} | Reason={}\n",
        ts, module, path, reason
    );

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/edr-agent/logs/suppression_audit.log")
    {
        let _ = file.write_all(log_line.as_bytes());
    }
}

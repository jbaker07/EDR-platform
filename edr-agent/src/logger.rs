use chrono::Utc;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::env;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Base directory for logs (can override with env var `EDR_LOG_DIR`)
const DEFAULT_LOG_DIR: &str = "edr-agent/logs";

/// Cap the number of unique `log_once` entries to avoid unbounded growth.
const LOG_ONCE_CACHE_CAP: usize = 5000;

/// In-memory cache to dedupe `log_once` messages across process lifetime.
static LOG_CACHE: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

fn log_dir() -> PathBuf {
    PathBuf::from(env::var("EDR_LOG_DIR").unwrap_or_else(|_| DEFAULT_LOG_DIR.to_string()))
}

fn ensure_dir(path: &Path) {
    if let Some(parent) = path.parent() {
        if let Err(e) = create_dir_all(parent) {
            eprintln!("[logger] failed to create log dir {:?}: {}", parent, e);
        }
    }
}

fn append_line(path: &Path, line: &str) {
    ensure_dir(path);
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{}", line);
    } else {
        eprintln!("[logger] failed to open {:?}", path);
    }
}

/// Standard log with timestamp and scope (also mirrors to file `agent.log`)
pub fn log(scope: &str, message: &str) {
    let ts = Utc::now().format("%H:%M:%S");
    println!("[{}][{}] {}", ts, scope, message);

    let line = format!("[{}][{}] {}", Utc::now().to_rfc3339(), scope, message);
    let path = log_dir().join("agent.log");
    append_line(&path, &line);
}

/// Log only once for deduplication across runs (prints and mirrors to `agent.log`)
pub fn log_once(scope: &str, message: &str) {
    let key = format!("[{}] {}", scope, message);
    let mut cache = LOG_CACHE.lock().expect("log cache poisoned");

    // Basic, cheap cap to avoid unbounded growth
    if cache.len() >= LOG_ONCE_CACHE_CAP {
        cache.clear();
    }

    if cache.insert(key.clone()) {
        let ts = Utc::now().format("%H:%M:%S");
        println!("[{}][{}] {}", ts, scope, message);

        let line = format!("[{}][{}] {}", Utc::now().to_rfc3339(), scope, message);
        let path = log_dir().join("agent.log");
        append_line(&path, &line);
    }
}

/// Writes suppression decisions to audit file (`suppression_audit.log`)
pub fn log_suppression_decision(module: &str, path: &str, reason: &str) {
    let ts = Utc::now().to_rfc3339();
    let log_line = format!(
        "[🔇 Suppression] [{}] Module={} | Path={} | Reason={}",
        ts, module, path, reason
    );

    let audit_path = log_dir().join("suppression_audit.log");
    append_line(&audit_path, &log_line);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_log() {
        log("TestScope", "hello world");
        log_once("TestScope", "print once");
        log_once("TestScope", "print once"); // should dedupe
        log_suppression_decision("privilege_monitor", "/usr/bin/sudo", "Known-good hash");
    }
}

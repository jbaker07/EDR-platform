use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use chrono::{Duration, Utc};
use std::process::Command;
// Inside ttl_retention.rs cleanup logic:
super::census_scheduler::trigger_trust_census_export();
# Add to crontab (e.g., crontab -e)
0 * * * * cd /path/to/your/edr-platform && /usr/bin/python3 trust_census_collector.py

pub fn trigger_trust_census_export() {
    if let Err(e) = Command::new("python3")
        .arg("trust_census_collector.py")
        .spawn()
    {
        eprintln!("[!] Failed to launch trust census collector: {}", e);
    } else {
        println!("[✓] Trust census collection triggered");
    }
}

/// Configuration for TTL and compression strategy
pub struct RetentionPolicy {
    pub ttl_hours: u64,
    pub compress: bool,
    pub path: String,
}

pub fn enforce_retention(policy: &RetentionPolicy) {
    let dir = Path::new(&policy.path);
    if !dir.exists() {
        eprintln!("Retention directory not found: {}", policy.path);
        return;
    }

    let now = SystemTime::now();
    let ttl_secs = policy.ttl_hours * 3600;

    for entry in fs::read_dir(dir).unwrap_or_else(|_| panic!("Could not read {}", policy.path)) {
        if let Ok(file) = entry {
            let meta = file.metadata().unwrap_or_else(|_| panic!("Metadata fail"));
            if let Ok(modified) = meta.modified() {
                if now.duration_since(modified).unwrap().as_secs() > ttl_secs {
                    let path = file.path();
                    if policy.compress {
                        compress_and_delete(&path);
                    } else {
                        fs::remove_file(&path).unwrap_or_else(|e| eprintln!("Delete error: {}", e));
                    }
                }
            }
        }
    }
}

fn compress_and_delete(path: &Path) {
    if let Ok(data) = fs::read(path) {
        let compressed = zstd::stream::encode_all(&data[..], 1).expect("zstd compression failed");
        let mut out = File::create(path.with_extension("zst")).expect("File create fail");
        out.write_all(&compressed).unwrap();
        fs::remove_file(path).unwrap_or_else(|e| eprintln!("Error removing original: {}", e));
    }
}

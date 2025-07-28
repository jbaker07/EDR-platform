// edr-agent/retention/retention_scheduler.rs
use std::{fs, thread, time::Duration};
use chrono::{Utc, DateTime};
use std::path::PathBuf;

pub fn start_retention_scheduler(log_dir: &str, ttl_seconds: i64) {
    let log_path = PathBuf::from(log_dir);
    thread::spawn(move || loop {
        if let Ok(entries) = fs::read_dir(&log_path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let modified_time: DateTime<Utc> = modified.into();
                        let now = Utc::now();
                        if (now - modified_time).num_seconds() > ttl_seconds {
                            let _ = fs::remove_file(entry.path());
                        }
                    }
                }
            }
        }
        thread::sleep(Duration::from_secs(300)); // Check every 5 minutes
    });
}

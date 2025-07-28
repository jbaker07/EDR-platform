use crate::trust_hook::{generate_trust_payload, submit_trust_signal, TrustEvent};
use crate::telemetry::MemorySnapshot;
use crate::telemetry_writer::write_telemetry_record;
use crate::gnn_hook::push_to_gnn_vector_log;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;
use lazy_static::lazy_static;

/// Directories to monitor
const MONITORED_DIRS: &[&str] = &["/home", "/etc", "/var/log", "/opt", "/mnt"];

/// File types often used in staging exfil
const TARGET_EXTENSIONS: &[&str] = &[".zip", ".7z", ".tar", ".gz", ".bak", ".db", ".pem"];

/// Thresholds
const SIZE_DELTA_TRIGGER: u64 = 50_000_000; // 50MB
const TIME_DELTA_TRIGGER: u64 = 60; // 60 seconds

lazy_static! {
    static ref STAGING_MAP: Mutex<HashMap<String, (u64, SystemTime)>> = Mutex::new(HashMap::new());
}

pub fn start_insider_data_staging_monitor() {
    thread::spawn(|| loop {
        for dir in MONITORED_DIRS {
            scan_directory_for_staging(dir);
        }
        thread::sleep(Duration::from_secs(30));
    });
}

fn scan_directory_for_staging(base_path: &str) {
    let walker = WalkDir::new(base_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            entry.file_type().is_file()
                && TARGET_EXTENSIONS.iter().any(|ext| {
                    entry.path().extension().map_or(false, |e| e == &ext[1..])
                })
        });

    for entry in walker {
        let path = entry.path().to_string_lossy().to_string();

        if let Ok(metadata) = fs::metadata(entry.path()) {
            let file_size = metadata.len();
            let modified = metadata.modified().unwrap_or(UNIX_EPOCH);

            let mut map = STAGING_MAP.lock().unwrap();

            if let Some((prev_size, prev_time)) = map.get(&path) {
                let growth = file_size.saturating_sub(*prev_size);
                let time_delta = modified.duration_since(*prev_time).unwrap_or_default().as_secs();

                if growth > SIZE_DELTA_TRIGGER && time_delta >= TIME_DELTA_TRIGGER {
                    let snapshot = MemorySnapshot {
                        pid: 9999,
                        cpu_usage: Some(0.4),
                        memory_usage: Some(file_size as f64),
                        risk_score: Some(87.0),
                        timestamp: SystemTime::now(),
                        ..Default::default()
                    };

                    let trust = generate_trust_payload(&snapshot);

                    let description = format!(
                        "Insider data staging detected: {} grew by {} bytes in {}s",
                        path, growth, time_delta
                    );

                    let event = TrustEvent {
                        signal_type: "insider_staging".into(),
                        description: description.clone(),
                        score: 87.0,
                        tags: vec!["insider".into(), "staging".into(), "exfil-risk".into()],
                        timestamp: now_ts(),
                    };

                    submit_trust_signal(trust.clone());
                    write_telemetry_record(trust.clone());
                    push_to_gnn_vector_log(trust.clone());

                    println!("[🧠 Insider Alert] {}", description);
                }
            }

            map.insert(path.clone(), (file_size, modified));
        }
    }
}

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

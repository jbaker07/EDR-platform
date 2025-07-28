use std::fs;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::path::Path;
use walkdir::WalkDir;

use crate::trust_hook::{submit_trust_event, TrustEvent};
use crate::telemetry_writer::write_telemetry_record;
use crate::gnn_hook::push_to_gnn_vector_log;

/// File staging patterns typical of exfil attempts
const STAGING_DIRS: [&str; 3] = ["/tmp", "/var/tmp", "/dev/shm"];

/// Thresholds
const MAX_STAGE_FILE_SIZE_MB: u64 = 200; // suspiciously large for temp
const MAX_STAGE_LIFESPAN_SECS: u64 = 900; // <15 min from creation to potential exfil

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn is_recent_and_large(file_path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(duration) = SystemTime::now().duration_since(modified) {
                let file_size_mb = metadata.len() / 1_000_000;
                return file_size_mb >= MAX_STAGE_FILE_SIZE_MB && duration.as_secs() < MAX_STAGE_LIFESPAN_SECS;
            }
        }
    }
    false
}

fn detect_staged_files() -> Vec<String> {
    let mut staged = Vec::new();
    for dir in &STAGING_DIRS {
        for entry in WalkDir::new(dir).max_depth(2).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && is_recent_and_large(path) {
                staged.push(path.display().to_string());
            }
        }
    }
    staged
}

fn detect_possible_exfil_behavior() -> bool {
    // TODO: Extend this using process_monitor or bash history parser
    false
}

fn send_event(signal_type: &str, description: String, score: f32, paths: Vec<String>) {
    let event = TrustEvent {
        signal_type: signal_type.to_string(),
        description: description.clone(),
        score,
        tags: vec!["exfil".to_string(), "staging".to_string()],
        timestamp: now_ts(),
    };
    submit_trust_event(event);

    let mut record = std::collections::HashMap::new();
    record.insert("event".into(), signal_type.into());
    record.insert("timestamp".into(), now_ts().to_string());
    record.insert("paths".into(), paths.join(", "));
    record.insert("description".into(), description);

    write_telemetry_record(record.clone());
    push_to_gnn_vector_log(record);
}

pub fn monitor_multi_stage_exfiltration() {
    let staged_files = detect_staged_files();
    let exfil_detected = detect_possible_exfil_behavior();

    if !staged_files.is_empty() && exfil_detected {
        let summary = format!(
            "Multi-stage exfiltration: staged files {:?} + potential exfil tool use",
            staged_files
        );
        send_event("multi_stage_exfil", summary, 90.0, staged_files);
    } else if !staged_files.is_empty() {
        let summary = format!(
            "Staging activity: Large, short-lived temp files: {:?}",
            staged_files
        );
        send_event("staging_detected", summary, 65.0, staged_files);
    }
}

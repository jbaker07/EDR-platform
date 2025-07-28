use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, UNIX_EPOCH};
use std::collections::HashMap;

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::logger::log;
use crate::trust_hook::{generate_feature_vector, generate_trust_payload, submit_trust_event, TrustEvent};
use crate::utils::time::now_ts;
use crate::telemetry_types::{TelemetryOutput, MemoryAnomalyType};
use sha2::{Digest, Sha256};
use entropy::shannon_entropy;
use users::get_user_by_uid;
use crate::telemetry_writer::write_telemetry_record;

/// Whitelisted safe script names (system-owned jobs or known benign scripts)
const WHITELIST: &[&str] = &[
    "/tmp/systemd-private-", "/tmp/pip-", "/tmp/ansible-", "/tmp/crontab"
];

/// Extensions considered suspicious if found in /tmp
const SCRIPT_EXTS: &[&str] = &["sh", "py", "pl", "rb", "ps1", "js", "bash"];

/// Background real-time monitor for suspicious script drops.
pub fn start_script_monitor() {
    thread::spawn(|| loop {
        let results = scan_script_monitor();

        if results.is_empty() {
            log("📜 [ScriptMonitor] No suspicious scripts found in this cycle.");
        } else {
            log(&format!(
                "⚠️ [ScriptMonitor] {} suspicious script events detected.",
                results.len()
            ));
        }

        thread::sleep(Duration::from_secs(40));
    });
}

/// Static wrapper to invoke batch scanning logic
pub fn scan_script_monitor() -> Vec<TelemetryOutput> {
    let mut outputs = Vec::new();

    if let Ok(entries) = fs::read_dir("/tmp") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(output) = check_and_log_script(&path) {
                let mut data = output.data.clone();

                // Submit TrustEvent
                let trust_event = TrustEvent {
                    timestamp: now_ts(),
                    pid: 0,
                    ppid: 0,
                    uid: 0,
                    binary_path: path.display().to_string(),
                    command_line: format!("script drop: {:?}", path.file_name()),
                    cwd: "/tmp".into(),
                    anomaly_type: "script_drop".into(),
                    component: "script_monitor".into(),
                    metadata: data.clone(),
                    risk_score: 22.0_f32,
                    source_module: "script_monitor".into(),
                    decay_context: Some("script_entropy_drop".into()),
                    module: Some("script_monitor".into()),
                    signal: Some("script_drop".into()),
                    signal_type: Some("file".into()),
                    score: Some(22.0_f32),
                    raw_score: Some(22.0_f32),
                    tags: Some(vec!["script".into(), "entropy".into(), "tmp_drop".into()]),
                    description: Some("Suspicious script with high entropy dropped in /tmp.".into()),
                };

                submit_trust_event(trust_event);
                write_telemetry_record(data.clone());
                push_to_gnn_vector_log(data.clone());

                outputs.push(output);
            }
        }
    }

    outputs
}

fn check_and_log_script(path: &PathBuf) -> Option<TelemetryOutput> {
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        if SCRIPT_EXTS.contains(&ext) && !is_whitelisted(&path) {
            if let Ok(meta) = path.metadata() {
                if meta.len() == 0 || meta.uid() == 0 {
                    return None;
                }

                let entropy_score = match File::open(&path) {
                    Ok(mut file) => {
                        let mut contents = Vec::new();
                        if file.read_to_end(&mut contents).is_ok() {
                            shannon_entropy(&contents) as f32
                        } else {
                            return None;
                        }
                    }
                    Err(_) => return None,
                };

                if entropy_score < 3.0 {
                    return None; // not complex enough to flag
                }

                let cpu = 0.5_f32;
                let mem = 90_000;
                let risk = 15.0_f32;
                let ts = now_ts();
                let uid = meta.uid();

                let trust = generate_trust_payload("script_monitor", cpu.into(), mem, risk.into());
                let features = generate_feature_vector(cpu.into(), mem, risk.into());

                let mut map = HashMap::new();
                map.insert("host".into(), "macos-host".into());
                map.insert("features".into(), format!("{:?}", features));
                map.insert("replay_tag".into(), "suspicious_script_drop".into());
                map.insert("path".into(), path.to_string_lossy().to_string());

                let mut hasher = Sha256::new();
                hasher.update(path.to_string_lossy().as_bytes());
                map.insert("script_hash".into(), format!("{:x}", hasher.finalize()));

                // Submit telemetry and trust
                write_telemetry_record(map.clone());
                push_to_gnn_vector_log(map.clone());

                let trust_event = TrustEvent {
                    timestamp: ts,
                    pid: 0,
                    ppid: 0,
                    uid,
                    binary_path: path.display().to_string(),
                    command_line: format!("drop_script {}", path.display()),
                    cwd: "/tmp".into(),
                    anomaly_type: "script_drop".into(),
                    component: "script_monitor".into(),
                    metadata: map.clone(),
                    risk_score: risk,
                    source_module: "script_monitor".into(),
                    decay_context: Some("script_entropy_drop".into()),
                    module: Some("script_monitor".into()),
                    signal: Some("suspicious_tmp_script".into()),
                    signal_type: Some("file".into()),
                    score: Some(risk),
                    raw_score: Some(entropy_score),
                    tags: Some(vec!["script".into(), "entropy".into(), "tmp_drop".into()]),
                    description: Some(format!(
                        "Suspicious script with entropy {:.2} dropped in /tmp",
                        entropy_score
                    )),
                };

                submit_trust_event(trust_event);

                log(&format!(
                    "[📜 Script Monitor] Suspicious script in /tmp → {:?} | Entropy: {:.2} | Trust: {} | Time: {}",
                    path,
                    entropy_score,
                    trust.get("trust_score").unwrap_or(&"N/A".to_string()),
                    ts
                ));

                return Some(TelemetryOutput {
                    category: "script".into(),
                    signal: "suspicious_tmp_script".into(),
                    confidence: 0.93,
                    data: map,
                });
            }
        }
    }

    None
}

fn is_whitelisted(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    WHITELIST.iter().any(|prefix| path_str.starts_with(prefix))
}

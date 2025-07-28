use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::thread;
use std::time::{Duration};
use std::collections::HashMap;
use std::sync::{OnceLock, atomic::{AtomicBool, Ordering}};

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::logger::log;
use crate::trust_hook::{generate_feature_vector, generate_trust_payload, submit_trust_event, TrustEvent};
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::write_telemetry_record;
use crate::utils::time::now_ts;
use sha2::{Digest, Sha256};
use entropy::shannon_entropy;

/// Whitelisted safe script names (system-owned jobs or known benign scripts)
const WHITELIST: &[&str] = &[
    "/tmp/systemd-private-", "/tmp/pip-", "/tmp/ansible-", "/tmp/crontab"
];

/// Extensions considered suspicious if found in /tmp
const SCRIPT_EXTS: &[&str] = &["sh", "py", "pl", "rb", "ps1", "js", "bash"];

pub static SCAN_SCRIPT_MONITOR: OnceLock<AtomicBool> = OnceLock::new();
pub fn start_integrity_monitor() {
    SCAN_SCRIPT_MONITOR.get_or_init(|| AtomicBool::new(true));

    thread::spawn(|| loop {
        if let Ok(entries) = fs::read_dir("/tmp") {
            for entry in entries.flatten() {
                let path = entry.path();

                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if SCRIPT_EXTS.contains(&ext) && !is_whitelisted(&path) {
                        if let Ok(meta) = path.metadata() {
                            if meta.len() == 0 || meta.uid() == 0 {
                                continue;
                            }

                            let entropy_score = match File::open(&path) {
                                Ok(mut file) => {
                                    let mut contents = Vec::new();
                                    if file.read_to_end(&mut contents).is_ok() {
                                        shannon_entropy(&contents)
                                    } else {
                                        continue;
                                    }
                                }
                                Err(_) => continue,
                            };

                            if entropy_score < 3.0 {
                                continue;
                            }

                            let cpu = 0.5;
                            let mem = 90_000;
                            let risk: f64 = 15.0;
                            let ts = now_ts();
                            let uid = meta.uid();

                            let trust = generate_trust_payload("script_monitor", cpu, mem, risk);
                            let features = generate_feature_vector(cpu, mem, risk);

                            let mut map = HashMap::new();
                            map.insert("host".into(), "macos-host".into());
                            map.insert("features".into(), format!("{:?}", features));
                            map.insert("replay_tag".into(), "suspicious_script_drop".into());
                            map.insert("path".into(), path.to_string_lossy().to_string());
                            map.insert("timestamp".into(), ts.to_string());
                            map.insert("entropy".into(), format!("{:.2}", entropy_score));
                            map.insert("soc_note".into(), "Script drop in /tmp detected by signal integrity monitor".into());
                            map.insert("gnn_escalate".into(), "true".into());

                            let mut hasher = Sha256::new();
                            hasher.update(path.to_string_lossy().as_bytes());
                            map.insert("script_hash".into(), format!("{:x}", hasher.finalize()));

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
                                component: "signal_integrity_monitor".into(),
                                metadata: map.clone(),
                                risk_score: risk as f32,
                                source_module: "signal_integrity_monitor".into(),
                                decay_context: Some("script_entropy_drop".into()),
                                module: Some("signal_integrity_mapper".into()),
                                signal: Some("tmp_script_drop".into()),
                                signal_type: Some("script".into()),
                                score: Some(risk as f32),
                                raw_score: Some(entropy_score as f32),
                                tags: Some(vec!["script_drop".into(), "signal_integrity".into()]),
                                description: Some(format!("Suspicious script drop at {:?}", path)),
                            };

                            submit_trust_event(trust_event);

                            log(&format!(
                                "[📜 Script Monitor] Suspicious script in /tmp → {:?} | Entropy: {:.2} | Trust: {} | Time: {}",
                                path,
                                entropy_score,
                                trust.get("trust_score").unwrap_or(&"N/A".to_string()),
                                ts
                            ));
                        }
                    }
                }
            }
        }

        thread::sleep(Duration::from_secs(40));
    });
}
pub fn scan_signal_integrity() -> Vec<TelemetryOutput> {
    let mut outputs = Vec::new();

    if let Ok(entries) = fs::read_dir("/tmp") {
        for entry in entries.flatten() {
            let path = entry.path();

            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if SCRIPT_EXTS.contains(&ext) && !is_whitelisted(&path) {
                    if let Ok(meta) = path.metadata() {
                        if meta.len() == 0 || meta.uid() == 0 {
                            continue;
                        }

                        let entropy_score = match File::open(&path) {
                            Ok(mut file) => {
                                let mut contents = Vec::new();
                                if file.read_to_end(&mut contents).is_ok() {
                                    shannon_entropy(&contents)
                                } else {
                                    continue;
                                }
                            }
                            Err(_) => continue,
                        };

                        if entropy_score < 3.0 {
                            continue;
                        }

                        let cpu = 0.4;
                        let mem = 70_000;
                        let risk: f64 = 12.0;
                        let ts = now_ts();
                        let uid = meta.uid();

                        let trust = generate_trust_payload("signal_integrity_monitor", cpu, mem, risk);
                        let features = generate_feature_vector(cpu, mem, risk);

                        let mut map = HashMap::new();
                        map.insert("host".into(), "macos-host".into());
                        map.insert("features".into(), format!("{:?}", features));
                        map.insert("replay_tag".into(), "signal_integrity_check".into());
                        map.insert("path".into(), path.to_string_lossy().to_string());
                        map.insert("timestamp".into(), ts.to_string());
                        map.insert("entropy".into(), format!("{:.2}", entropy_score));
                        map.insert("soc_note".into(), "Integrity signal from /tmp script entropy match".into());
                        map.insert("gnn_escalate".into(), "true".into());

                        let mut hasher = Sha256::new();
                        hasher.update(path.to_string_lossy().as_bytes());
                        map.insert("signal_hash".into(), format!("{:x}", hasher.finalize()));

                        write_telemetry_record(map.clone());
                        push_to_gnn_vector_log(map.clone());

                        let trust_event = TrustEvent {
                            timestamp: ts,
                            pid: 0,
                            ppid: 0,
                            uid,
                            binary_path: path.display().to_string(),
                            command_line: format!("integrity_check {}", path.display()),
                            cwd: "/tmp".into(),
                            anomaly_type: "signal_integrity".into(),
                            component: "signal_integrity_monitor".into(),
                            metadata: map.clone(),
                            risk_score: risk as f32,
                            source_module: "signal_integrity_monitor".into(),
                            decay_context: Some("signal_hash_entropy".into()),
                            module: Some("signal_integrity_mapper".into()),
                            signal: Some("signal_integrity".into()),
                            signal_type: Some("script".into()),
                            score: Some(risk as f32),
                            raw_score: Some(entropy_score as f32),
                            tags: Some(vec!["signal_integrity".into(), "script_drop".into()]),
                            description: Some(format!("/tmp integrity violation: {:?}", path)),
                        };

                        submit_trust_event(trust_event);

                        outputs.push(TelemetryOutput {
                            category: "integrity".into(),
                            signal: "signal_integrity".into(),
                            confidence: risk as f32,
                            data: map,
                        });
                    }
                }
            }
        }
    }

    outputs
}


fn is_whitelisted(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    WHITELIST.iter().any(|prefix| path_str.starts_with(prefix))
}

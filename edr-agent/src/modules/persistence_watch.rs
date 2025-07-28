use std::fs::{self, Metadata};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, UNIX_EPOCH};

use std::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;
use serde_json::json;
use std::collections::HashSet;
use crate::trust_hook::{generate_trust_payload, generate_feature_vector, TrustEvent, submit_trust_event};
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::telemetry::MemorySnapshot;
use crate::utils::time::now_ts;
use crate::telemetry_writer::write_telemetry_record;
use crate::modules::replay_writer::store_replay_event;
use crate::telemetry_types::TelemetryOutput;
use glob::glob;
use sha2::{Digest, Sha256};
use users::get_user_by_uid;

use aya::{Bpf, include_bytes_aligned};
use aya::programs::TracePoint;



use crate::utils::baseline_filter::{should_suppress_signal, SuppressionDecision};
use crate::logger::log_suppression_decision; // see step 2
use crate::utils::fingerprint::load_known_safe_fingerprints; // or wherever you're storing the hash set

use crate::services::trust_vector::{TrustVector, TRUST_VECTOR_GLOBAL};

use std::sync::MutexGuard;


lazy_static! {
    static ref PERSISTENCE_FOUND: AtomicBool = AtomicBool::new(false);
}

/// Monitored persistence paths
const PERSISTENCE_PATHS: &[&str] = &[
    "/etc/crontab", "/etc/cron.d/*", "/etc/cron.daily/*", "/etc/cron.hourly/*",
    "/etc/cron.monthly/*", "/etc/cron.weekly/*", "/var/spool/cron/crontabs/*",
    "/etc/systemd/system/*", "/usr/lib/systemd/system/*", "/lib/systemd/system/*",
    "/etc/systemd/user/*", "/etc/init.d/*", "/etc/rc.local", "/etc/xdg/autostart/*",
    "/home/*/.config/autostart/*", "/home/*/.bashrc", "/home/*/.bash_profile",
    "/home/*/.zshrc", "/home/*/.profile", "/tmp/*", "/var/tmp/*", "/dev/shm/*",
];
pub fn start_persistence_watch() {
    if let Err(e) = attach_ebpf_monitor() {
        eprintln!("[eBPF] Failed to attach persistence eBPF monitor: {:?}", e);
    }

    thread::spawn(|| loop {
        for pattern in PERSISTENCE_PATHS {
            if let Ok(paths) = glob(pattern) {
                for entry in paths.flatten() {
                    if let Ok(metadata) = fs::metadata(&entry) {
                        if let Some(risk_info) = assess_file(&entry, &metadata) {
                            let now_unix = now_ts();
                            let now = UNIX_EPOCH + Duration::from_secs(now_unix);

                            let path_str = risk_info.path.clone();
                            let score = risk_info.score;
                            let reason = risk_info.reason.clone();

                            let snapshot = MemorySnapshot {
                                id: "persistence_file".into(),
                                path: path_str.clone(),
                                hash: risk_info.hash.clone(),
                                risk_score: score,
                                reason: reason.clone(),
                                timestamp: now,
                                behavior_deviation: Some(true),
                                cpu_usage: Some(0.4),
                                memory_usage: Some(95000.0),
                                pid: 0,
                            };

                            let features = generate_feature_vector(0.4, 95000, score as f64);

                            let mut map = std::collections::HashMap::new();
                            map.insert("path".into(), path_str.clone());
                            map.insert("reason".into(), reason.clone());
                            map.insert("features".into(), format!("{:?}", features));
                            map.insert("replay_tag".into(), "suspicious_persistence".into());
                            map.insert("gnn_escalate".into(), "true".into());
                            map.insert("timestamp".into(), now_unix.to_string());
                            map.insert("soc_note".into(), "Potential persistence artifact discovered".into());

                            push_to_gnn_vector_log(map.clone());
                            write_telemetry_record(map.clone());

                            let trust_event = TrustEvent {
                                timestamp: now_unix,
                                pid: 0,
                                ppid: 0,
                                uid: 0,
                                binary_path: path_str.clone(),
                                command_line: "unknown".into(),
                                cwd: "/".into(),
                                anomaly_type: "persistence_artifact".into(),
                                component: "persistence::watch".into(),
                                metadata: map.clone(),
                                risk_score: score as f32,
                                source_module: "persistence_watch.rs".into(),
                                decay_context: Some("artifact_detected".into()),
                                module: Some("persistence".into()),
                                signal: Some("suspicious_persistence_file".into()),
                                signal_type: Some("file_artifact".into()),
                                score: Some(score as f32),
                                raw_score: Some(score as f32),
                                tags: Some(vec![
                                    "startup_file".into(),
                                    "potential_persistence".into(),
                                    "behavioral_anomaly".into(),
                                ]),
                                description: Some(reason),
                            };

                            submit_trust_event(trust_event);
                            store_replay_event(snapshot.into_hashmap());

                            println!(
                                "[📌 Persistence Watch] {:?} | Score: {} | TrustEvent sent",
                                path_str, score
                            );

                            PERSISTENCE_FOUND.store(true, Ordering::SeqCst);
                        }
                    }
                }
            }
        }
        thread::sleep(Duration::from_secs(180));
    });
}

///Attach precompiled eBPF persistence monitor to sys_enter_write
pub fn attach_ebpf_monitor() -> Result<(), anyhow::Error> {
    // Use literal path directly with the macro
    let mut bpf = Bpf::load(include_bytes_aligned!("../ebpf/persistence_watch_ebpf.o"))
        .map_err(|e| anyhow::anyhow!("Failed to load BPF: {}", e))?;

    // Extract the TracePoint program
    let program: &mut TracePoint = bpf
        .program_mut("trace_persistence_write")
        .ok_or_else(|| anyhow::anyhow!("eBPF program 'trace_persistence_write' not found in object"))?
        .try_into()
        .map_err(|e| anyhow::anyhow!("Failed to cast to TracePoint: {}", e))?;

    // Load the program into the kernel
    program
        .load()
        .map_err(|e| anyhow::anyhow!("Failed to load 'trace_persistence_write': {}", e))?;

    // Attach the program to the write syscall
    program
        .attach("syscalls", "sys_enter_write")
        .map_err(|e| anyhow::anyhow!("Failed to attach to sys_enter_write: {}", e))?;

    println!("[✅ eBPF Persistence Watch] Attached to sys_enter_write successfully");
    Ok(())
}

struct RiskInfo {
    path: String,
    score: u32,
    reason: String,
    hash: String,
}

fn assess_file(path: &PathBuf, metadata: &Metadata) -> Option<RiskInfo> {
    let path_str = path.to_string_lossy().into_owned();
    let filename = path.file_name()?.to_string_lossy();

    let suspicious = filename.ends_with(".service")
        || filename.ends_with(".sh")
        || filename.contains("launch")
        || filename.contains("startup")
        || filename.contains("agent")
        || filename.contains("recon")
        || filename.contains("backdoor");

    if suspicious {
        let hash = hash_file(path).unwrap_or_else(|| "unknown".to_string());
        let uid = metadata.uid();
        let user = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().to_string())
            .unwrap_or_else(|| format!("uid({})", uid));
        let score = 90;

        return Some(RiskInfo {
            path: path_str,
            reason: format!("Suspicious filename owned by '{}'", user),
            score,
            hash,
        });
    }

    None
}

fn hash_file(path: &PathBuf) -> Option<String> {
    fs::read(path).ok().map(|data| {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    })
}


/// Returns the current trust vector or default if unavailable
fn get_current_trust_vector() -> Option<TrustVector> {
    TRUST_VECTOR_GLOBAL
        .lock()
        .ok()
        .and_then(|map| map.get("default").cloned()) // Replace "default" if endpoint ID is known
}

//// Passive fallback scan used by telemetry.rs orchestrator
pub fn scan_persistence_activity() -> Vec<TelemetryOutput> {
    if PERSISTENCE_FOUND.load(Ordering::SeqCst) {
        return vec![];
    }

    let mut results = Vec::new();
    let known_safe_map = load_known_safe_fingerprints("edr-agent/json_files/baseline_fingerprint.json");
    let known_safe_set: HashSet<String> = known_safe_map.keys().cloned().collect();

    for pattern in PERSISTENCE_PATHS {
        if let Ok(paths) = glob(pattern) {
            for entry in paths.flatten() {
                if let Ok(metadata) = fs::metadata(&entry) {
                    if let Some(risk_info) = assess_file(&entry, &metadata) {
                        let ts = now_ts();
                        let hash = risk_info.hash.clone();
                        let path = risk_info.path.clone();

                        let trust_vector = get_current_trust_vector().unwrap_or_default();
                        let decision = should_suppress_signal(
                            &path,
                            Some(&hash),
                            0, // UID unknown statically
                            &["persistence".into(), "artifact".into(), "risky_file".into()],
                            "n/a",
                            &trust_vector,
                            &known_safe_set,
                        );

                        match decision {
                            SuppressionDecision::SuppressReasonably(reason) => {
                                log_suppression_decision("persistence", &path, &reason);
                                continue;
                            }
                            SuppressionDecision::DoNotSuppress(_) => {}
                        }

                        let mut data = std::collections::HashMap::new();
                        data.insert("path".into(), path.clone());
                        data.insert("reason".into(), risk_info.reason.clone());
                        data.insert("hash".into(), hash.clone());
                        data.insert("score".into(), risk_info.score.to_string());
                        data.insert("timestamp".into(), ts.to_string());
                        data.insert("replay_tag".into(), "suspicious_persistence".into());
                        data.insert("soc_note".into(), "Detected suspicious persistence mechanism".into());
                        data.insert("gnn_escalate".into(), "true".into());

                        let trust_event = TrustEvent {
                            timestamp: ts,
                            pid: 0,
                            ppid: 0,
                            uid: 0,
                            binary_path: "n/a".into(),
                            command_line: "n/a".into(),
                            cwd: "n/a".into(),
                            anomaly_type: "persistence_mechanism".into(),
                            component: "persistence::watch".into(),
                            metadata: data.clone(),
                            risk_score: risk_info.score as f32,
                            source_module: "scan_persistence_activity".into(),
                            decay_context: Some("persistence_behavior".into()),
                            module: Some("persistence".into()),
                            signal: Some("scan_persistence_activity".into()),
                            signal_type: Some("static_persistence_artifact".into()),
                            score: Some(risk_info.score as f32),
                            raw_score: Some(risk_info.score as f32),
                            tags: Some(vec!["persistence".into(), "artifact".into(), "risky_file".into()]),
                            description: Some(format!("Suspicious persistence file: {} | {}", path, risk_info.reason)),
                        };

                        submit_trust_event(trust_event);
                        write_telemetry_record(data.clone());
                        push_to_gnn_vector_log(data.clone());

                        results.push(TelemetryOutput {
                            category: "persistence".into(),
                            signal: "scan_persistence_activity".into(),
                            confidence: (risk_info.score as f32) / 100.0,
                            data,
                        });

                        PERSISTENCE_FOUND.store(true, Ordering::SeqCst);
                    }
                }
            }
        }
    }

    results
}

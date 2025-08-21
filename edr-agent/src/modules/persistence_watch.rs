use std::fs::{self, File, Metadata};
use std::io::{Read, BufRead, BufReader};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, UNIX_EPOCH};

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, RwLock};
use lazy_static::lazy_static;

use glob::glob;
use sha2::{Digest, Sha256};
use users::get_user_by_uid;

use serde_json::json;

use anyhow::{Result, anyhow};

use aya::{Bpf, include_bytes_aligned};
use aya::programs::TracePoint;

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::replay_writer::store_replay_event;
use crate::services::trust_vector::{TrustVector, TRUST_VECTOR_GLOBAL};
use crate::telemetry::MemorySnapshot;
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::write_telemetry_record;
use crate::trust_hook::{TrustEvent, submit_trust_event, generate_feature_vector};
use crate::utils::time::now_ts;

use crate::utils::baseline_filter::{should_suppress_signal, SuppressionDecision};
use crate::logger::log_suppression_decision;
use crate::utils::fingerprint::load_known_safe_fingerprints;

lazy_static! {
    static ref PERSISTENCE_FOUND: AtomicBool = AtomicBool::new(false);
    // Path -> (inode, mtime, size, sha256). Prevents duplicate alerts across scans.
    static ref FILE_CACHE: Mutex<HashMap<String, (u64, i64, u64, String)>> = Mutex::new(HashMap::new());
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

#[derive(Clone)]
struct RiskInfo {
    path: String,
    score: u32,
    reason: String,
    hash: String,
}

pub fn start_persistence_watch() {
    // Best-effort eBPF attach; keep running even if it fails
    let _ = attach_ebpf_monitor();

    thread::spawn(|| loop {
        for pattern in PERSISTENCE_PATHS {
            if let Ok(paths) = glob(pattern) {
                for entry in paths.flatten() {
                    if let Ok(metadata) = fs::symlink_metadata(&entry) {
                        // Skip symlinks to reduce trickery/loops
                        if metadata.file_type().is_symlink() {
                            continue;
                        }
                        if let Some(risk_info) = assess_file(&entry, &metadata) {
                            let now_unix = now_ts();
                            let now = UNIX_EPOCH + Duration::from_secs(now_unix);

                            let snapshot = MemorySnapshot {
                                id: "persistence_file".into(),
                                path: risk_info.path.clone(),
                                hash: risk_info.hash.clone(),
                                risk_score: risk_info.score,
                                reason: risk_info.reason.clone(),
                                timestamp: now,
                                behavior_deviation: Some(true),
                                cpu_usage: Some(0.4),
                                memory_usage: Some(95_000.0),
                                pid: 0,
                            };

                            let features = generate_feature_vector(0.4, 95_000, risk_info.score as f64);

                            let mut map = HashMap::new();
                            map.insert("path".into(), risk_info.path.clone());
                            map.insert("reason".into(), risk_info.reason.clone());
                            map.insert("hash".into(), risk_info.hash.clone());
                            map.insert("features".into(), format!("{:?}", features));
                            map.insert("replay_tag".into(), "suspicious_persistence".into());
                            map.insert("gnn_escalate".into(), "true".into());
                            map.insert("timestamp".into(), now_unix.to_string());
                            map.insert("soc_note".into(), "Potential persistence artifact discovered".into());

                            push_to_gnn_vector_log(map.clone());
                            write_telemetry_record(map.clone());

                            submit_trust_event(TrustEvent {
                                timestamp: now_unix,
                                pid: 0,
                                ppid: 0,
                                uid: 0,
                                binary_path: risk_info.path.clone(),
                                command_line: "unknown".into(),
                                cwd: "/".into(),
                                anomaly_type: "persistence_artifact".into(),
                                component: "persistence::watch".into(),
                                metadata: map.clone(),
                                risk_score: risk_info.score as f32,
                                source_module: "persistence_watch.rs".into(),
                                decay_context: Some("artifact_detected".into()),
                                module: Some("persistence".into()),
                                signal: Some("suspicious_persistence_file".into()),
                                signal_type: Some("file_artifact".into()),
                                score: Some(risk_info.score as f32),
                                raw_score: Some(risk_info.score as f32),
                                tags: Some(vec![
                                    "startup_file".into(),
                                    "potential_persistence".into(),
                                    "behavioral_anomaly".into(),
                                ]),
                                description: Some(risk_info.reason.clone()),
                            });

                            store_replay_event(snapshot.into_hashmap());
                            PERSISTENCE_FOUND.store(true, Ordering::SeqCst);
                            println!("[📌 Persistence Watch] {} | Score: {} | TrustEvent sent", risk_info.path, risk_info.score);
                        }
                    }
                }
            }
        }
        thread::sleep(Duration::from_secs(180));
    });
}

/// Attach precompiled eBPF persistence monitor to sys_enter_write
#[cfg(target_os = "linux")]
pub fn attach_ebpf_monitor() -> Result<()> {
    let mut bpf = Bpf::load(include_bytes_aligned!("../ebpf/persistence_watch_ebpf.o"))
        .map_err(|e| anyhow!("Failed to load BPF: {}", e))?;

    let program: &mut TracePoint = bpf
        .program_mut("trace_persistence_write")
        .ok_or_else(|| anyhow!("eBPF program 'trace_persistence_write' not found in object"))?
        .try_into()
        .map_err(|e| anyhow!("Failed to cast to TracePoint: {}", e))?;

    program.load().map_err(|e| anyhow!("Failed to load 'trace_persistence_write': {}", e))?;
    program.attach("syscalls", "sys_enter_write")
        .map_err(|e| anyhow!("Failed to attach to sys_enter_write: {}", e))?;

    println!("[✅ eBPF Persistence Watch] Attached to sys_enter_write successfully");
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn attach_ebpf_monitor() -> Result<()> {
    Ok(())
}

fn assess_file(path: &PathBuf, metadata: &Metadata) -> Option<RiskInfo> {
    let path_str = path.to_string_lossy().into_owned();

    // Quick cache check: if unchanged (inode+mtime+size), skip re-hash/scoring
    let inode = metadata.ino();
    let mtime = metadata.mtime();
    let size = metadata.size();

    if let Some((prev_ino, prev_mtime, prev_size, _)) = FILE_CACHE.lock().unwrap().get(&path_str).cloned() {
        if prev_ino == inode && prev_mtime == mtime && prev_size == size {
            return None;
        }
    }

    // Stream-hash (bounded)
    let hash = stream_sha256(path).unwrap_or_else(|| "unknown".to_string());

    // Heuristics
    let filename = path.file_name().map(|s| s.to_string_lossy()).unwrap_or_default();
    let dir_str = path.parent().map(|p| p.to_string_lossy()).unwrap_or_default();

    let uid = metadata.uid();
    let user = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or_else(|| format!("uid({})", uid));

    let mode = metadata.mode();

    // Content peek (first ~16KB)
    let mut suspicious_tokens = 0;
    let mut shebang = false;
    if let Ok(mut f) = File::open(path) {
        let mut buf = Vec::with_capacity(16 * 1024);
        let _ = (&mut f).take(16 * 1024).read_to_end(&mut buf);
        if buf.starts_with(b"#!") { shebang = true; }
        if let Ok(text) = std::str::from_utf8(&buf) {
            let toks = [
                "curl ", "wget ", "Invoke-WebRequest", "base64 -d", "eval ",
                "nc ", "ncat ", "socat ", "chmod +x", "/tmp/", "http://", "https://",
                "ExecStart=", "WantedBy=multi-user.target",
            ];
            suspicious_tokens = toks.iter().filter(|t| text.contains(*t)).count();
        }
    }

    let mut score: i32 = 0;

    // Location / name
    let looks_persist_name = filename.ends_with(".service")
        || filename.ends_with(".timer")
        || filename.ends_with(".sh")
        || filename.contains("launch")
        || filename.contains("startup")
        || filename.contains("agent")
        || filename.contains("recon")
        || filename.contains("backdoor")
        || filename == ".bashrc"
        || filename == ".profile"
        || filename == ".bash_profile"
        || filename == ".zshrc";

    if looks_persist_name { score += 40; }

    // Suspicious dirs
    if dir_str.contains("/tmp") || dir_str.contains("/var/tmp") || dir_str.contains("/dev/shm") {
        score += 25;
    }

    // Exec bits
    if mode & 0o111 != 0 { score += 15; }

    // Recent change (within 24h)
    let now = now_ts() as i64;
    if now - mtime <= 24 * 3600 { score += 10; }

    // Content indicators
    score += (suspicious_tokens as i32) * 6;
    if shebang { score += 5; }

    // Owner: systemd things owned by non-root are extra weird
    if path_str.contains("/systemd/") && uid != 0 { score += 15; }

    // Bound final score
    let score_u32 = score.clamp(0, 100) as u32;

    // Update cache (after hash so we also keep last hash)
    {
        let mut cache = FILE_CACHE.lock().unwrap();
        cache.insert(path_str.clone(), (inode, mtime, size, hash.clone()));
    }

    if score_u32 >= 60 {
        Some(RiskInfo {
            path: path_str,
            reason: format!(
                "Suspicious persistence candidate (owner '{}', exec={}, tokens={})",
                user, (mode & 0o111 != 0), suspicious_tokens
            ),
            score: score_u32,
            hash,
        })
    } else {
        None
    }
}

fn stream_sha256(path: &Path) -> Option<String> {
    let mut f = File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = f.read(&mut buf).ok()?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    Some(format!("{:x}", hasher.finalize()))
}

/// Returns the current trust vector or default if unavailable
fn get_current_trust_vector() -> Option<TrustVector> {
    TRUST_VECTOR_GLOBAL.lock().ok().and_then(|map| map.get("default").cloned())
}

/// Passive fallback scan used by telemetry.rs orchestrator
pub fn scan_persistence_activity() -> Vec<TelemetryOutput> {
    if PERSISTENCE_FOUND.load(Ordering::SeqCst) {
        return vec![];
    }

    let mut results = Vec::new();
    let known_safe_map = load_known_safe_fingerprints("edr-agent/json_files/baseline_fingerprint.json");
    let known_safe_set: HashSet<String> = known_safe_map.keys().cloned().collect();
    let trust_vector = get_current_trust_vector().unwrap_or_default();

    for pattern in PERSISTENCE_PATHS {
        if let Ok(paths) = glob(pattern) {
            for entry in paths.flatten() {
                if let Ok(metadata) = fs::symlink_metadata(&entry) {
                    if metadata.file_type().is_symlink() { continue; }
                    if let Some(risk_info) = assess_file(&entry, &metadata) {
                        let ts = now_ts();

                        // Baseline suppression
                        let decision = should_suppress_signal(
                            &risk_info.path,
                            Some(&risk_info.hash),
                            0,
                            &["persistence".into(), "artifact".into(), "risky_file".into()],
                            "n/a",
                            &trust_vector,
                            &known_safe_set,
                        );
                        if let SuppressionDecision::SuppressReasonably(reason) = decision {
                            log_suppression_decision("persistence", &risk_info.path, &reason);
                            continue;
                        }

                        let mut data = HashMap::new();
                        data.insert("path".into(), risk_info.path.clone());
                        data.insert("reason".into(), risk_info.reason.clone());
                        data.insert("hash".into(), risk_info.hash.clone());
                        data.insert("score".into(), risk_info.score.to_string());
                        data.insert("timestamp".into(), ts.to_string());
                        data.insert("replay_tag".into(), "suspicious_persistence".into());
                        data.insert("soc_note".into(), "Detected suspicious persistence mechanism".into());
                        data.insert("gnn_escalate".into(), "true".into());

                        submit_trust_event(TrustEvent {
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
                            description: Some(format!("Suspicious persistence file: {} | {}", risk_info.path, risk_info.reason)),
                        });

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

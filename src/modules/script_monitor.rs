use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::logger::log;
use crate::trust_hook::{
    generate_feature_vector, generate_trust_payload, submit_trust_event, TrustEvent,
};
use crate::utils::time::now_ts;
use crate::telemetry_types::TelemetryOutput;
use sha2::{Digest, Sha256};
use entropy::shannon_entropy;
use users::get_user_by_uid;
use crate::telemetry_writer::write_telemetry_record;
use crate::modules::replay_writer::store_replay_event;
use crate::modules::telemetry_fingerprint::{is_known_good, load_fingerprints_from_disk};

/// Whitelisted safe script names (system-owned jobs or known benign scripts)
const WHITELIST: &[&str] = &[
    "/tmp/systemd-private-", "/tmp/pip-", "/tmp/ansible-", "/tmp/crontab",
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

                // Submit TrustEvent (batch wrapper)
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
                store_replay_event(data.clone());

                outputs.push(output);
            }
        }
    }

    outputs
}

fn check_and_log_script(path: &PathBuf) -> Option<TelemetryOutput> {
    let ext_lower = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase());

    if let Some(ext) = ext_lower.as_deref() {
        if SCRIPT_EXTS.contains(&ext) && !is_whitelisted(path) {
            if let Ok(meta) = path.metadata() {
                // Skip empty or root-owned zero-byte placeholders
                if meta.len() == 0 || meta.uid() == 0 {
                    return None;
                }

                let exec_capable = (meta.permissions().mode() & 0o111) != 0;

                // Read file to compute entropy and (later) hash
                let mut contents = Vec::new();
                if let Ok(mut file) = File::open(path) {
                    if file.read_to_end(&mut contents).is_err() {
                        return None;
                    }
                } else {
                    return None;
                }

                let entropy_score = shannon_entropy(&contents) as f32;
                if entropy_score < 3.0 {
                    // Low-entropy scripts (e.g., plain text) are less suspicious
                    return None;
                }

                // Compute file hash from contents
                let file_hash = sha256_of_bytes(&contents);

                // Try to extract shebang
                let shebang_line = contents
                    .get(0..256)
                    .and_then(|slice| std::str::from_utf8(slice).ok())
                    .and_then(|text| text.lines().next().map(|s| s.trim().to_string()));

                // Owner information (best-effort)
                let uid = meta.uid();
                let owner = get_user_by_uid(uid)
                    .map(|u| u.name().to_string_lossy().to_string())
                    .unwrap_or_else(|| format!("uid({})", uid));

                // Fingerprint suppression
                let fingerprints =
                    load_fingerprints_from_disk("src/modules/telemetry_fingerprint.json");
                let mut suppress_probe = HashMap::new();
                suppress_probe.insert("file_path".into(), path.display().to_string());
                suppress_probe.insert("file_hash".into(), file_hash.clone());
                suppress_probe.insert("uid".into(), uid.to_string());
                suppress_probe.insert("exec_capable".into(), exec_capable.to_string());
                suppress_probe.insert("entropy".into(), format!("{:.2}", entropy_score));
                if let Some(sb) = &shebang_line {
                    suppress_probe.insert("shebang".into(), sb.clone());
                }

                if is_known_good(&suppress_probe, &fingerprints) {
                    log(&format!(
                        "[ScriptMonitor] Skipping known-good script: {}",
                        path.display()
                    ));
                    return None;
                }

                // Build features / trust (logged for SOC)
                let cpu = 0.5_f32;
                let mem = 90_000_u64;
                let risk = 15.0_f32;
                let ts = now_ts();

                let trust = generate_trust_payload("script_monitor", cpu.into(), mem, risk.into());
                let features = generate_feature_vector(cpu.into(), mem, risk.into());

                // Telemetry payload
                let mut map = HashMap::new();
                map.insert("path".into(), path.to_string_lossy().to_string());
                map.insert("owner".into(), owner);
                map.insert("file_hash".into(), file_hash.clone());
                map.insert("uid".into(), uid.to_string());
                map.insert("entropy".into(), format!("{:.2}", entropy_score));
                map.insert("exec_capable".into(), exec_capable.to_string());
                if let Some(shebang) = shebang_line {
                    map.insert("shebang".into(), shebang);
                }
                map.insert("features".into(), format!("{:?}", features));
                map.insert("replay_tag".into(), "suspicious_script_drop".into());
                map.insert("gnn_escalate".into(), "true".into());
                map.insert("timestamp".into(), ts.to_string());
                map.insert(
                    "summary".into(),
                    format!(
                        "Suspicious /tmp script (entropy {:.2}, exec={})",
                        entropy_score, exec_capable
                    ),
                );

                // Persist & fan-out
                write_telemetry_record(map.clone());
                push_to_gnn_vector_log(map.clone());
                store_replay_event(map.clone());

                // Event (realtime)
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

fn sha256_of_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

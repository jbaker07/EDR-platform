use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::sync::{OnceLock, atomic::{AtomicBool, Ordering}};

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::logger::log;
use crate::trust_hook::{
    generate_feature_vector, generate_trust_payload, submit_trust_event, TrustEvent,
};
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::write_telemetry_record;
use crate::utils::time::now_ts;
use crate::modules::replay_writer::store_replay_event;
use crate::modules::telemetry_fingerprint::{is_known_good, load_fingerprints_from_disk};
use sha2::{Digest, Sha256};
use entropy::shannon_entropy;

/// Whitelisted safe script names (system-owned jobs or known benign scripts)
const WHITELIST: &[&str] = &[
    "/tmp/systemd-private-", "/tmp/pip-", "/tmp/ansible-", "/tmp/crontab",
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
                // Realtime: higher risk threshold
                if let Some((mut meta_map, uid)) = collect_script_metadata(&path, 15.0) {
                    // add trust/feature context for SOC
                    let cpu = 0.5_f64;
                    let mem = 90_000_u64;
                    let risk = 15.0_f64;
                    let ts = now_ts();

                    let trust = generate_trust_payload("script_monitor", cpu, mem, risk);
                    let features = generate_feature_vector(cpu, mem, risk);

                    meta_map.insert("features".into(), format!("{:?}", features));
                    meta_map.insert("timestamp".into(), ts.to_string());
                    meta_map.insert("soc_note".into(), "Script drop in /tmp detected by signal integrity monitor".into());
                    meta_map.insert("gnn_escalate".into(), "true".into());

                    write_telemetry_record(meta_map.clone());
                    push_to_gnn_vector_log(meta_map.clone());
                    store_replay_event(meta_map.clone());

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
                        metadata: meta_map.clone(),
                        risk_score: risk as f32,
                        source_module: "signal_integrity_monitor".into(),
                        decay_context: Some("script_entropy_drop".into()),
                        module: Some("signal_integrity_monitor".into()),
                        signal: Some("tmp_script_drop".into()),
                        signal_type: Some("script".into()),
                        score: Some(risk as f32),
                        raw_score: meta_map
                            .get("entropy")
                            .and_then(|e| e.parse::<f32>().ok()),
                        tags: Some(vec!["script_drop".into(), "signal_integrity".into()]),
                        description: Some(format!("Suspicious script drop at {:?}", path)),
                    };

                    submit_trust_event(trust_event);

                    log(&format!(
                        "[📜 Script Monitor] Suspicious script in /tmp → {:?} | Entropy: {} | Trust: {} | Time: {}",
                        path,
                        meta_map.get("entropy").cloned().unwrap_or_else(|| "?".into()),
                        trust.get("trust_score").unwrap_or(&"N/A".to_string()),
                        ts
                    ));
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
            // Batch/scan: slightly lower risk than realtime
            if let Some((meta_map, uid)) = collect_script_metadata(&path, 12.0) {
                let ts = now_ts();

                write_telemetry_record(meta_map.clone());
                push_to_gnn_vector_log(meta_map.clone());
                store_replay_event(meta_map.clone());

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
                    metadata: meta_map.clone(),
                    risk_score: 12.0,
                    source_module: "signal_integrity_monitor".into(),
                    decay_context: Some("signal_hash_entropy".into()),
                    module: Some("signal_integrity_monitor".into()),
                    signal: Some("signal_integrity".into()),
                    signal_type: Some("script".into()),
                    score: Some(12.0),
                    raw_score: meta_map
                        .get("entropy")
                        .and_then(|e| e.parse::<f32>().ok()),
                    tags: Some(vec!["signal_integrity".into(), "script_drop".into()]),
                    description: Some(format!("/tmp integrity violation: {:?}", path)),
                };

                submit_trust_event(trust_event);

                outputs.push(TelemetryOutput {
                    category: "integrity".into(),
                    signal: "signal_integrity".into(),
                    confidence: 0.9,
                    data: meta_map,
                });
            }
        }
    }

    outputs
}

fn collect_script_metadata(path: &Path, risk: f32) -> Option<(HashMap<String, String>, u32)> {
    // Filter by extension (case-insensitive) & whitelist
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())?;

    if !SCRIPT_EXTS.contains(&ext.as_str()) || is_whitelisted(path) {
        return None;
    }

    let meta = path.metadata().ok()?;
    if meta.len() == 0 || meta.uid() == 0 {
        return None;
    }

    // Read file once
    let mut contents = Vec::new();
    File::open(path).ok()?.read_to_end(&mut contents).ok()?;

    let entropy_score = shannon_entropy(&contents) as f32;
    if entropy_score < 3.0 {
        return None;
    }

    // exec-capable bit (best-effort)
    let exec_capable = (meta.permissions().mode() & 0o111) != 0;

    // SHA256 of contents (more stable than hashing the path string)
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    let script_hash = format!("{:x}", hasher.finalize());

    // Fingerprint suppression
    let fingerprints = load_fingerprints_from_disk("src/modules/telemetry_fingerprint.json");
    let mut probe = HashMap::new();
    probe.insert("file_path".into(), path.display().to_string());
    probe.insert("file_hash".into(), script_hash.clone());
    probe.insert("uid".into(), meta.uid().to_string());
    probe.insert("entropy".into(), format!("{:.2}", entropy_score));
    probe.insert("exec_capable".into(), exec_capable.to_string());

    if is_known_good(&probe, &fingerprints) {
        log(&format!(
            "[SignalIntegrity] Skipping known-good script: {}",
            path.display()
        ));
        return None;
    }

    // Compose output map
    let ts = now_ts();
    let cpu = if risk >= 15.0 { 0.5 } else { 0.4 };
    let mem = if risk >= 15.0 { 90_000 } else { 70_000 };
    let trust = generate_trust_payload(
        if risk >= 15.0 { "script_monitor" } else { "signal_integrity_monitor" },
        cpu as f64,
        mem as u64,
        risk as f64,
    );

    let mut map = HashMap::new();
    map.insert("path".into(), path.to_string_lossy().to_string());
    map.insert("script_hash".into(), script_hash);
    map.insert("uid".into(), meta.uid().to_string());
    map.insert("entropy".into(), format!("{:.2}", entropy_score));
    map.insert("exec_capable".into(), exec_capable.to_string());
    map.insert("replay_tag".into(), if risk >= 15.0 {
        "suspicious_script_drop".into()
    } else {
        "signal_integrity_check".into()
    });
    map.insert("summary".into(), format!(
        "Suspicious /tmp script (entropy {:.2}, exec={})",
        entropy_score, exec_capable
    ));
    map.insert("trust_score".into(), trust.get("trust_score").cloned().unwrap_or_else(|| "N/A".into()));

    Some((map, meta.uid()))
}

fn is_whitelisted(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    WHITELIST.iter().any(|prefix| path_str.starts_with(prefix))
}

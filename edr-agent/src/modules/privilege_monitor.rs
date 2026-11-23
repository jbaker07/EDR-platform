// src/modules/privilege_monitor.rs
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

use sysinfo::{PidExt, ProcessExt, System, SystemExt};

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::logger::log;
use crate::services::baseline_uid::get_baseline_uid;
use crate::services::trust_kernel::is_known_legit_escalator;
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::write_telemetry_record;
use crate::trust_hook::{generate_feature_vector, submit_trust_event, TrustEvent};
use crate::utils::time::now_ts;

lazy_static! {
    static ref PRIV_ESCALATION_FOUND: AtomicBool = AtomicBool::new(false);
}

pub fn scan_privilege_activity() -> Vec<TelemetryOutput> {
    if PRIV_ESCALATION_FOUND.load(Ordering::SeqCst) {
        return vec![];
    }

    let ts = now_ts();
    let cpu_usage = 0.3_f64;
    let memory_usage = 110_000_u64;
    let risk_score = 20.0_f64;
    let features = generate_feature_vector(cpu_usage, memory_usage, risk_score);

    let mut pid = 0;
    let mut ppid = 0;
    let mut uid = 0;
    let mut binary_path = String::new();
    let mut command_line = String::new();
    let mut cwd = String::new();
    let mut found_priv = false;

    let mut sys = System::new_all();
    sys.refresh_all();

    for (proc_pid, proc) in sys.processes() {
        let cmd = proc.cmd();
        if !cmd.is_empty() && cmd[0].contains("sudo") {
            pid = proc_pid.as_u32() as i32;
            ppid = proc.parent().map(|p| p.as_u32() as i32).unwrap_or(0);
            binary_path = proc.exe().display().to_string();
            command_line = cmd.join(" ");
            cwd = proc.cwd().display().to_string();
            found_priv = true;
            break;
        }
    }

    if !found_priv {
        return vec![];
    }

    // Smart gating
    if !should_trigger_uid_shift(0, &binary_path) {
        log("[⚠️ PrivilegeMonitor] Passive scan found sudo but baseline-approved. Skipping.");
        return vec![];
    }

    PRIV_ESCALATION_FOUND.store(true, Ordering::SeqCst);
    log("[🟡 PrivilegeMonitor] Passive privilege escalation detected via sudo scan");

    let mut data = HashMap::new();
    data.insert("event_type".into(), "privilege_monitor".into());
    data.insert("timestamp".into(), ts.to_string());
    data.insert("replay_tag".into(), "privilege_escalation".into());
    data.insert(
        "soc_note".into(),
        "Privilege escalation detected via passive scan".into(),
    );
    data.insert("gnn_escalate".into(), "true".into());
    data.insert("features".into(), format!("{:?}", features));

    let trust_event = TrustEvent {
        timestamp: ts,
        pid,
        ppid,
        uid,
        binary_path: binary_path.clone(),
        command_line: command_line.clone(),
        cwd: cwd.clone(),
        anomaly_type: "privilege_escalation".into(),
        component: "privilege_monitor".into(),
        metadata: data.clone(),
        risk_score: risk_score as f32,
        source_module: "privilege_monitor".into(),
        decay_context: Some("passive_privilege".into()),
        module: Some("privilege".into()),
        signal: Some("passive_priv_scan".into()),
        signal_type: Some("fallback".into()),
        score: Some(risk_score as f32),
        raw_score: Some(risk_score as f32),
        tags: Some(vec![
            "privilege_escalation".into(),
            "fallback_scan".into(),
            "behavioral".into(),
        ]),
        description: Some(
            "Passive fallback detected privilege escalation (eBPF did not fire)".into(),
        ),
    };

    submit_trust_event(trust_event);
    write_telemetry_record(data.clone());
    push_to_gnn_vector_log(data.clone());

    vec![TelemetryOutput {
        category: "auth".into(),
        signal: "privilege_escalation_scan".into(),
        confidence: 0.8, // normalized confidence (0..1) instead of using raw risk
        data,
    }]
}

/// Determines if the UID shift is suspicious enough to trigger scoring/escalation
fn should_trigger_uid_shift(uid: i32, path: &str) -> bool {
    // Skip if it's a known legit escalation binary
    if is_known_legit_escalator(path) {
        return false;
    }
    // Respect baseline if present
    if let Some(base) = get_baseline_uid(path) {
        if base == uid {
            return false;
        }
    }
    // Trigger if UID is root (0) or otherwise suspicious
    uid == 0 || uid < 10
}


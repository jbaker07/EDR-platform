use std::collections::{HashMap, HashSet};
use std::fs::Metadata;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::auth_monitor::{start_auth_monitor, scan_auth_activity};
use crate::modules::container_monitor::{scan_container_activity, start_ebpf_container_exec_monitor};
use crate::modules::dll_injection_monitor::{start_ebpf_dll_injection_watch, scan_dll_injection_activity};
use crate::modules::encrypted_payload_detector::{
    scan_encrypted_payload_activity, start_encrypted_payload_monitor,
};
use crate::modules::entropy_exec_monitor::start_entropy_exec_monitor;
use crate::modules::file_hash_watcher::{start_file_hash_monitor, scan_file_hash_activity};
use crate::modules::file_tamper_monitor::{start_ebpf_file_tamper_watch, scan_file_tamper_activity};
use crate::modules::geo_ip_anomaly::scan_geo_ip_activity;
use crate::modules::job_sched_monitor::{start_ebpf_kernel_fallout_watch, start_job_sched_monitors, scan_job_sched_activity};
use crate::modules::logon_tracker::{start_logon_tracker, scan_logon_activity};
use crate::modules::mem_scan::{
    scan_memory_health,
    #[cfg(target_os = "linux")] start_ebpf_mem_scan,
    #[cfg(target_os = "linux")] start_ebpf_proc_hollow_scan,
};
use crate::modules::mfa_bypass::{start_mfa_bypass_monitor, scan_mfa_bypass_activity};
use crate::modules::net_watch::{log_open_connections, start_network_monitor, scan_network_anomalies};
use crate::modules::password_spray::{scan_password_sprays};
use crate::modules::persistence_watch::{start_persistence_watch, scan_persistence_activity};
use crate::modules::privilege_monitor::{
    scan_privilege_activity,
    #[cfg(target_os = "linux")] spawn_cred_dump_monitor,
    start_privilege_monitor,
};
use crate::modules::process_injection::{scan_injection_fallback, start_process_injection_monitor};
use crate::modules::process_monitor::{gather_processes, scan_processes, #[cfg(target_os = "linux")] start_ipc_abuse_monitor};
use crate::modules::script_monitor::{scan_script_monitor, start_script_monitor};
use crate::modules::signal_integrity_mapper::{scan_signal_integrity, start_integrity_monitor};
use crate::modules::suspicious_ipc::{scan_ipc_passive, #[cfg(target_os = "linux")] start_ebpf_ipc_abuse_watch};
use crate::modules::usb_monitor::{scan_usb_state, start_usb_monitor};
use crate::modules::user_tracker::{get_logged_in_users, scan_user_sessions};
use crate::modules::replay_writer::store_replay_event;

use crate::services::trust_engine_final::{evaluate_and_dispatch_trust_score, TelemetryData};
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::{write_telemetry_record, TelemetryWriter};
use crate::utils::time::now_ts;

// ======================= Data Types =======================

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TelemetryRecord {
    pub timestamp: u64,
    pub pid: i32,
    pub ppid: i32,
    pub uid: u32,
    pub binary_path: String,
    pub command_line: String,
    pub cwd: String,
    pub env_vars: Option<Vec<String>>,
    pub tags: Vec<String>,
    pub risk_score: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    pub id: String,
    pub path: String,
    pub hash: String,
    pub risk_score: u32,
    pub reason: String,
    pub timestamp: SystemTime,
    pub behavior_deviation: Option<bool>,
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<f64>,
    pub pid: u32,
}
impl Default for MemorySnapshot {
    fn default() -> Self {
        MemorySnapshot {
            id: String::new(),
            path: String::new(),
            hash: String::new(),
            risk_score: 0,
            reason: String::new(),
            timestamp: SystemTime::now(),
            behavior_deviation: None,
            cpu_usage: None,
            memory_usage: None,
            pid: 0,
        }
    }
}
impl MemorySnapshot {
    pub fn into_hashmap(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("id".into(), self.id);
        map.insert("path".into(), self.path);
        map.insert("hash".into(), self.hash);
        map.insert("risk_score".into(), self.risk_score.to_string());
        map.insert("reason".into(), self.reason);
        map.insert(
            "timestamp".into(),
            self.timestamp.duration_since(UNIX_EPOCH).map(|d| d.as_secs().to_string()).unwrap_or_else(|_| "0".into()),
        );
        map.insert("pid".into(), self.pid.to_string());
        if let Some(cpu) = self.cpu_usage { map.insert("cpu_usage".into(), format!("{:.2}", cpu)); }
        if let Some(mem) = self.memory_usage { map.insert("memory_usage".into(), format!("{:.2}", mem)); }
        if let Some(behavior) = self.behavior_deviation { map.insert("behavior_deviation".into(), behavior.to_string()); }
        map
    }
}

// ======================= Utilities =======================

pub fn get_current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub static MEMORY_ANOMALY_DETECTED: AtomicBool = AtomicBool::new(false);
pub fn mark_memory_anomaly_detected() {
    MEMORY_ANOMALY_DETECTED.store(true, Ordering::Relaxed);
}

// Convert TelemetryOutput → map while preserving original data
fn output_to_map(output: &TelemetryOutput) -> HashMap<String, String> {
    let mut map = output.data.clone();
    map.insert("category".to_string(), output.category.clone());
    map.insert("signal".to_string(), output.signal.clone());
    map.insert("confidence".to_string(), output.confidence.to_string());
    map
}

// Dedup outputs per minute by signal to avoid bursts
fn deduplicate_outputs(outputs: Vec<TelemetryOutput>) -> Vec<TelemetryOutput> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut unique_outputs = vec![];
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    for output in outputs {
        let key = format!("{}@{}", output.signal, now / 60);
        if seen.insert(key) { unique_outputs.push(output); }
    }
    unique_outputs
}

// ======================= Realtime Monitors =======================

pub fn start_realtime_monitors(writer: Arc<Mutex<TelemetryWriter>>) {
    start_file_hash_monitor(writer.clone());
    start_usb_monitor();
    start_script_monitor();
    start_network_monitor();
    start_integrity_monitor();
    start_mfa_bypass_monitor();
    start_logon_tracker(Arc::clone(&writer));
    start_persistence_watch();
    start_auth_monitor();
    start_encrypted_payload_monitor();
    start_entropy_exec_monitor();
    start_job_sched_monitors(writer.clone());
    start_ebpf_file_tamper_watch();
    start_ebpf_dll_injection_watch();
    start_ebpf_container_exec_monitor();
    crate::ebpf::syscall_reader::start_syscall_reader(writer.clone());
    crate::ebpf::net_flow_reader::start_net_flow_reader(writer.clone());
    crate::ebpf::wx_exec_reader::start_wx_exec_reader(writer.clone());


        #[cfg(target_os = "linux")]
{
    // 0) Start the episode pipeline so readers can push immediately.
    crate::ingest::start_ingest_pipeline();

    // 1) Syscall backbone (exec/fork/mmap/mprotect/setuid/capset/connect)
    tokio::spawn(async {
        crate::ebpf::syscall_reader::spawn_syscall_reader().await;
    });

    // 2) Network connect destination capture (IPv4)
    tokio::spawn(async {
        crate::ebpf::net_flow_reader::spawn_net_flow_reader().await;
    });

    // 3) W→X transition detector
    tokio::spawn(async {
        crate::ebpf::wx_exec_reader::spawn_wx_exec_reader().await;
    });

    // ... your existing user-space monitors continue unchanged ...
}

    #[cfg(target_os = "linux")]
{
    // 1) Episodes/feature/graph pipeline must exist before readers push.
    crate::ingest::start_ingest_pipeline();

    // 2) Start the syscall reader
    tokio::spawn(async {
        crate::ebpf::syscall_reader::spawn_syscall_reader().await;
    });

    // ...the rest of your monitors (net, usb, script, etc)...
}

    #[cfg(target_os = "linux")]
    {
        let _ = tokio::spawn(async { let _ = start_ebpf_mem_scan().await; });
        let _ = tokio::spawn(async { let _ = start_ebpf_proc_hollow_scan().await; });
        let _ = tokio::spawn(async { let _ = start_privilege_monitor().await; });
        let _ = tokio::spawn(async { let _ = start_process_injection_monitor().await; });
        spawn_cred_dump_monitor();
        start_ipc_abuse_monitor();
        start_ebpf_ipc_abuse_watch();
    }
}

// ======================= Passive Sweep =======================

pub fn run_sideeffect_monitors_and_collect() -> Vec<TelemetryOutput> {
    let mut results = Vec::new();

    results.extend(scan_auth_activity());
    results.extend(scan_container_activity());
    results.extend(scan_dll_injection_activity());
    results.extend(scan_encrypted_payload_activity());
    results.extend(scan_file_hash_activity());
    results.extend(scan_file_tamper_activity());
    results.extend(scan_geo_ip_activity());
    results.extend(scan_job_sched_activity());
    results.extend(scan_logon_activity());
    results.extend(scan_memory_health());
    results.extend(scan_network_anomalies());
    results.extend(scan_password_sprays());
    results.extend(scan_persistence_activity());
    results.extend(scan_privilege_activity());
    results.extend(scan_injection_fallback());
    results.extend(scan_signal_integrity());
    results.extend(scan_script_monitor());
    results.extend(scan_ipc_passive());
    results.extend(scan_usb_state());
    results.extend(scan_user_sessions());

    deduplicate_outputs(results)
}

// ======================= Snapshots & Telemetry =======================

pub fn get_current_snapshot() -> Vec<TelemetryRecord> {
    let process_infos = gather_processes();
    process_infos
        .into_iter()
        .map(|proc| TelemetryRecord {
            timestamp: get_current_timestamp(),
            pid: proc.pid,
            ppid: proc.ppid,
            uid: 0,
            binary_path: proc.exe,
            command_line: proc.command_line,
            cwd: String::new(),
            env_vars: Some(vec!["PATH=/usr/bin".into()]),
            tags: vec![],
            risk_score: None,
        })
        .collect()
}

pub fn get_current_telemetry_snapshot(writer: Arc<Mutex<TelemetryWriter>>) -> Vec<TelemetryRecord> {
    let mut records = get_current_snapshot();

    let all_outputs: Vec<TelemetryOutput> = run_sideeffect_monitors_and_collect();

    for output in all_outputs {
        let mapped = output_to_map(&output);

        write_telemetry_record(mapped.clone());
        push_to_gnn_vector_log(mapped.clone());
        store_replay_event(mapped.clone());

        let pid = output.data.get("pid").and_then(|v| v.parse().ok()).unwrap_or(-1);
        let ppid = output.data.get("ppid").and_then(|v| v.parse().ok()).unwrap_or(-1);
        let uid = output.data.get("uid").and_then(|v| v.parse().ok()).unwrap_or(0);

        let binary_path = output.data.get("binary_path").cloned()
            .or_else(|| output.data.get("exe").cloned()).unwrap_or_default();

        let command_line = output.data.get("command_line").cloned()
            .or_else(|| output.data.get("cmdline").cloned()).unwrap_or_default();

        let cwd = output.data.get("cwd").cloned().unwrap_or_default();

        let rec = TelemetryRecord {
            timestamp: now_ts(),
            pid, ppid, uid,
            binary_path, command_line, cwd,
            env_vars: None,
            tags: vec![output.signal.clone()],
            risk_score: Some((output.confidence * 100.0) as u32),
        };

        let telemetry_data = TelemetryData {
            endpoint_id: format!("endpoint_{}", rec.pid),
            endpoint_role: "default".to_string(),
            cpu_usage: 0.0,
            memory_usage: 0.0,
            risk_score: rec.risk_score.unwrap_or(0) as f64,
            tags: rec.tags.clone(),
        };

        let trust_result = evaluate_and_dispatch_trust_score(&telemetry_data);
        println!("🟢 Trust Result (inline): {:?}", trust_result);

        records.push(rec);
    }

    if let Ok(mut locked_writer) = writer.lock() {
        log_open_connections(&mut *locked_writer);
    }

    records
}

// ======================= Feature bridge =======================

pub fn push_feature_as_signal(f: &crate::features::FeatureObservation) {
    let mut data = HashMap::new();
    data.insert("feature_key".into(), f.key.clone());
    data.insert("feature_value".into(), f.value.clone());
    data.insert("z".into(), format!("{:.4}", f.z));
    data.insert("family".into(), f.family.clone());
    let mapped = data.clone();

    write_telemetry_record(mapped.clone());
    push_to_gnn_vector_log(mapped);
}
// in telemetry.rs
pub fn push_edr_event_record(rec: TelemetryRecord, writer: &Arc<Mutex<TelemetryWriter>>, out: &crate::telemetry_types::TelemetryOutput) {
    // persist + gnn logging (keeps your current behavior)
    let mapped = out.data.clone();
    crate::telemetry_writer::write_telemetry_record(mapped.clone());
    crate::gnn_hook::push_to_gnn_vector_log(mapped);
    // add to batch at caller site; here we only do side effects
    if let Ok(mut w) = writer.lock() {
        let _ = w.flush(); // optional
    }
}

// ======================= User Sessions =======================

pub fn log_user_sessions() {
    let sessions = get_logged_in_users();
    for session in sessions {
        let mut data = HashMap::new();
        data.insert("user".into(), session.username);
        data.insert("terminal".into(), session.terminal);
        data.insert("host".into(), session.host);
        data.insert("login_time".into(), session.login_time);
        data.insert("timestamp".into(), now_ts().to_string());

        let output = TelemetryOutput {
            category: "user_session".into(),
            signal: "active_login".into(),
            confidence: 0.9,
            data,
        };

        let mapped = output_to_map(&output);
        write_telemetry_record(mapped.clone());
        push_to_gnn_vector_log(mapped);
    }
}

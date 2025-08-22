use aya::maps::perf::{AsyncPerfEventArray, PerfEventArray};
use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Bpf};
use aya::util::online_cpus;
use bytes::BytesMut;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{
    thread,
    time::{Duration, Instant},
};
use tokio::{task, time};
use tokio::sync::Mutex;

use anyhow::{anyhow, Result};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::logger::log;
use crate::services::baseline_uid::get_baseline_uid;
use crate::services::trust_kernel::{check_frequent_escalation, is_known_legit_escalator};
use crate::telemetry_types::{MemoryAnomalyType, TelemetryOutput};
use crate::telemetry_writer::{push_memory_telemetry, write_telemetry_record};
use crate::trust_hook::{
    generate_feature_vector, generate_trust_payload, submit_trust_event, TrustEvent,
};
use crate::utils::time::now_ts;

lazy_static! {
    static ref PRIV_ESCALATION_FOUND: AtomicBool = AtomicBool::new(false);
}

/// Userland event used after parsing the raw kernel struct.
#[derive(Debug, Clone)]
pub struct PrivilegeEvent {
    pub pid: i32,
    pub ppid: i32,
    pub uid: i32,
    pub path: String,
    pub reason: String,
}

#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PrivilegeEventRaw {
    pub pid: i32,
    pub ppid: i32,
    pub uid: i32,
    pub path: [u8; 256],
    pub reason: [u8; 128],
}

/// Custom kernel event structure
#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CredDumpEvent {
    pub pid: i32,
    pub timestamp: u64,
    pub summary: [u8; 256], // Fixed-size summary buffer
}

/// Utilities

pub fn get_cmdline(pid: i32) -> Option<String> {
    fs::read_to_string(format!("/proc/{}/cmdline", pid))
        .ok()
        .map(|s| s.replace('\0', " ").trim().to_string())
}

pub fn get_cwd(pid: i32) -> Option<String> {
    fs::read_link(format!("/proc/{}/cwd", pid))
        .ok()
        .map(|path| path.display().to_string())
}

pub fn all_processes() -> std::io::Result<Vec<i32>> {
    let mut result = Vec::new();
    for entry in fs::read_dir("/proc")? {
        if let Ok(entry) = entry {
            if let Some(pid_str) = entry.file_name().to_str() {
                if let Ok(pid) = pid_str.parse::<i32>() {
                    result.push(pid);
                }
            }
        }
    }
    Ok(result)
}

#[cfg(target_os = "linux")]
const DEDUP_TTL: Duration = Duration::from_secs(30);

/// Main eBPF privilege monitor (Linux)
#[cfg(target_os = "linux")]
pub async fn start_privilege_monitor() -> Result<()> {
    use std::collections::HashMap as StdHashMap;
    use tokio::sync::Mutex as TokioMutex;

    let mut bpf = Bpf::load(include_bytes_aligned!("../ebpf/privilege_monitor_ebpf.o"))?;

    let tracepoints = ["trace_setuid", "trace_setresuid", "trace_execve"];
    for prog_name in &tracepoints {
        let program: &mut TracePoint = bpf
            .program_mut(prog_name)
            .ok_or(anyhow!("Program not found: {}", prog_name))?
            .try_into()?;
        program.load()?;
        program.attach(
            "syscalls",
            match *prog_name {
                "trace_setuid" => "sys_enter_setuid",
                "trace_setresuid" => "sys_enter_setresuid",
                "trace_execve" => "sys_enter_execve",
                _ => continue,
            },
        )?;
    }

    let perf = Arc::new(Mutex::new(AsyncPerfEventArray::try_from(
        bpf.map_mut("EVENTS")?,
    )?));

    // TTL-based de-dup map
    let recent_seen: Arc<TokioMutex<StdHashMap<(i32, i32), Instant>>> =
        Arc::new(TokioMutex::new(StdHashMap::new()));

    for cpu_id in online_cpus()? {
        let perf_clone = Arc::clone(&perf);
        let seen_clone = Arc::clone(&recent_seen);

        task::spawn(async move {
            let mut buffers = vec![BytesMut::with_capacity(4096); 16];

            match perf_clone.lock().await.open(cpu_id, None) {
                Ok(mut buf) => loop {
                    match buf.read_events(&mut buffers).await {
                        Ok(events) => {
                            for b in &mut buffers[..events.read] {
                                if b.is_empty() {
                                    continue;
                                }

                                if let Some(event) = parse_privilege_event(b).await {
                                    // TTL de-dup
                                    {
                                        let mut seen = seen_clone.lock().await;
                                        let now = Instant::now();
                                        seen.retain(|_, t| now.duration_since(*t) < DEDUP_TTL);
                                        let key = (event.pid, event.uid);
                                        if seen.contains_key(&key) {
                                            b.clear();
                                            continue;
                                        }
                                        seen.insert(key, now);
                                    }

                                    // Baseline UID gating
                                    if !should_trigger_uid_shift(event.uid, &event.path) {
                                        log("[⚠️ PrivilegeMonitor] UID shift is baseline-approved, skipping.");
                                        b.clear();
                                        continue;
                                    }

                                    // Escalation suppression if recently handled
                                    if has_recent_escalation(event.pid) {
                                        log("[⚠️ PrivilegeMonitor] Recent escalation already handled, skipping.");
                                        b.clear();
                                        continue;
                                    }

                                    let timestamp = now_ts();
                                    let cmdline = get_cmdline(event.pid).unwrap_or_else(|| "n/a".into());
                                    let cwd = get_cwd(event.pid).unwrap_or_else(|| "n/a".into());

                                    record_privilege_escalation(
                                        event.pid,
                                        event.uid,
                                        &event.path,
                                        &event.reason,
                                    );

                                    let mut metadata = HashMap::new();
                                    metadata.insert("binary_path".into(), event.path.clone());
                                    metadata.insert("reason".into(), event.reason.clone());
                                    metadata.insert("pid".into(), event.pid.to_string());
                                    metadata.insert("ppid".into(), event.ppid.to_string());
                                    metadata.insert("uid".into(), event.uid.to_string());
                                    metadata.insert("timestamp".into(), timestamp.to_string());
                                    metadata.insert(
                                        "soc_note".into(),
                                        "Detected potential privilege misuse".into(),
                                    );
                                    metadata.insert("replay_tag".into(), "privilege_escalation".into());
                                    metadata.insert("gnn_escalate".into(), "true".into());

                                    let trust = TrustEvent {
                                        timestamp,
                                        pid: event.pid,
                                        ppid: event.ppid,
                                        uid: event.uid as u32,
                                        binary_path: event.path.clone(),
                                        command_line: cmdline,
                                        cwd,
                                        anomaly_type: "privilege_escalation".into(),
                                        component: "privilege::ebpf".into(),
                                        metadata: metadata.clone(),
                                        risk_score: 85.0,
                                        source_module: "privilege_monitor".into(),
                                        decay_context: Some("privilege_behavior".into()),
                                        module: Some("privilege".into()),
                                        signal: Some("setuid_abuse".into()),
                                        signal_type: Some("ebpf_syscall".into()),
                                        score: Some(85.0),
                                        raw_score: Some(85.0),
                                        tags: Some(vec![
                                            "privilege_escalation".into(),
                                            "ebpf".into(),
                                            "setuid".into(),
                                        ]),
                                        description: Some(format!(
                                            "Potential privilege misuse: {}",
                                            event.reason
                                        )),
                                    };

                                    submit_trust_event(trust);
                                    push_to_gnn_vector_log(metadata.clone());
                                    write_telemetry_record(metadata);
                                    log("[🔐 PrivilegeMonitor] Event processed via eBPF");
                                }

                                // IMPORTANT: clear buffer after processing
                                b.clear();
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "[❌ PrivilegeMonitor] Error reading events on CPU {}: {:?}",
                                cpu_id, e
                            );
                            time::sleep(Duration::from_secs(2)).await;
                        }
                    }
                },
                Err(e) => {
                    eprintln!(
                        "[❌ PrivilegeMonitor] Failed to open perf buffer on CPU {}: {:?}",
                        cpu_id, e
                    );
                }
            }
        });
    }

    Ok(())
}

/// No-op stub on non-Linux platforms
#[cfg(not(target_os = "linux"))]
pub async fn start_privilege_monitor() -> Result<()> {
    log("[ℹ️ PrivilegeMonitor] eBPF not supported on this platform; monitor disabled.");
    Ok(())
}

#[cfg(target_os = "linux")]
fn trim_nul(buf: &[u8]) -> String {
    let s = String::from_utf8_lossy(buf);
    s.trim_end_matches('\0').to_string()
}

#[cfg(target_os = "linux")]
async fn parse_privilege_event(buf: &BytesMut) -> Option<PrivilegeEvent> {
    use std::ptr::read_unaligned;

    if buf.len() < std::mem::size_of::<PrivilegeEventRaw>() {
        log("[❌ PrivilegeMonitor] buffer too small for PrivilegeEventRaw");
        return None;
    }

    let raw = unsafe { read_unaligned(buf.as_ptr() as *const PrivilegeEventRaw) };

    let mut event = PrivilegeEvent {
        pid: raw.pid,
        ppid: raw.ppid,
        uid: raw.uid,
        path: trim_nul(&raw.path),
        reason: trim_nul(&raw.reason),
    };

    // Contextual enrichment
    if event.uid == 0 {
        event.reason = format!("Potential UID 0 escalation from PID {}", event.pid);
    } else if event.uid < 1000 {
        event.reason = format!("System-level UID change detected: {}", event.uid);
    } else {
        event.reason = format!("User-level UID change: {}", event.uid);
    }

    // Frequency-aware tagging
    let history_key = format!("uid_change:{}:{}", event.path, event.uid);
    if check_frequent_escalation(&history_key) {
        event.reason.push_str(" [frequent]");
    }

    // Baseline deviation check
    if let Some(baseline_uid) = get_baseline_uid(&event.path) {
        if baseline_uid != event.uid {
            event.reason
                .push_str(" [deviation from baseline UID]");
        }
    }

    // Known source suppressor marker
    if is_known_legit_escalator(&event.path) {
        event.reason.push_str(" [known escalator]");
    }

    Some(event)
}

async fn simulate_privilege_event() {
    PRIV_ESCALATION_FOUND.store(true, Ordering::SeqCst);

    let cpu_usage: f32 = 0.7;
    let memory_usage: u64 = 130_000;
    let risk_score: f32 = 20.0;
    let ts = now_ts();

    let trust = generate_trust_payload("local", cpu_usage.into(), memory_usage, risk_score.into());
    let features = generate_feature_vector(cpu_usage.into(), memory_usage, risk_score.into());

    let mut map = HashMap::new();
    map.insert("host".into(), "macos-host".into());
    map.insert("features".into(), format!("{:?}", features));
    map.insert("replay_tag".into(), "privilege_escalation".into());
    map.insert("timestamp".into(), ts.to_string());
    map.insert("gnn_escalate".into(), "true".into());
    map.insert("soc_note".into(), "eBPF privilege escalation detected".into());

    push_to_gnn_vector_log(map.clone());
    write_telemetry_record(map.clone());

    let trust_event = TrustEvent {
        timestamp: ts,
        pid: 0,
        ppid: 0,
        uid: 0,
        binary_path: "/usr/bin/sudo".into(),
        command_line: "sudo whoami".into(),
        cwd: "/".into(),
        anomaly_type: "privilege_escalation".into(),
        component: "privilege_monitor".into(),
        metadata: map.clone(),
        risk_score: risk_score.into(),
        source_module: "privilege_monitor".into(),
        decay_context: Some("privilege_behavior".into()),
        module: Some("privilege".into()),
        signal: Some("ebpf_priv_escalation".into()),
        signal_type: Some("ebpf_tracepoint".into()),
        score: Some(risk_score.into()),
        raw_score: Some(risk_score.into()),
        tags: Some(vec![
            "privilege_escalation".into(),
            "ebpf".into(),
            "behavioral".into(),
        ]),
        description: Some("Detected eBPF-level privilege escalation via syscall tracing".into()),
    };

    submit_trust_event(trust_event);

    println!(
        "[🔐 Privilege Monitor] Sudo/similar command executed → Trust Score: {}",
        trust.get("trust_score").unwrap_or(&"N/A".to_string())
    );
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

/// Context-aware gating logic for cred dump events
pub fn cred_dump_context_gater(evt: &CredDumpEvent) -> bool {
    // Add real logic here as needed
    let summary_str = String::from_utf8_lossy(&evt.summary).to_lowercase();

    // Example: bypass known benign openat on temp dirs
    if summary_str.contains("/tmp/") || summary_str.contains("ld.so.cache") {
        return false;
    }

    // Add more context filters here
    true
}

#[cfg(target_os = "linux")]
pub fn spawn_cred_dump_monitor() {
    use lazy_static::lazy_static;
    use std::collections::HashMap as StdHashMap;
    use std::sync::Mutex as StdMutex;

    lazy_static! {
        static ref CRED_DUMP_HITS: StdMutex<StdHashMap<i32, u32>> =
            StdMutex::new(StdHashMap::new());
    }

    thread::spawn(move || {
        let mut bpf = match Bpf::load(include_bytes_aligned!(
            "../ebpf/cred_dump_monitor.bpf.o"
        )) {
            Ok(bpf) => bpf,
            Err(e) => {
                eprintln!("❌ Failed to load cred_dump BPF: {:?}", e);
                return;
            }
        };

        let program: &mut TracePoint = match bpf.program_mut("trace_cred_evt") {
            Some(p) => match p.try_into() {
                Ok(tp) => tp,
                Err(e) => {
                    eprintln!("❌ Failed to cast program to TracePoint: {:?}", e);
                    return;
                }
            },
            None => {
                eprintln!("❌ Missing tracepoint program: trace_cred_evt");
                return;
            }
        };

        if program.load().is_err() || program.attach("syscalls", "sys_enter_openat").is_err() {
            eprintln!("❌ Failed to load or attach cred_dump tracepoint");
            return;
        }

        println!("🔐 [eBPF] Cred dump monitor tracepoint active");

        let mut perf = match bpf.map_mut("EVENTS").and_then(PerfEventArray::try_from) {
            Ok(perf) => perf,
            Err(e) => {
                eprintln!("❌ Failed to get perf array: {:?}", e);
                return;
            }
        };

        for cpu_id in online_cpus().unwrap_or_default() {
            match perf.open(cpu_id, None) {
                Ok(mut buf) => {
                    thread::spawn(move || {
                        let mut buffers = vec![BytesMut::with_capacity(1024); 16];

                        loop {
                            match buf.read_events(&mut buffers) {
                                Ok(events) => {
                                    for b in &mut buffers[..events.read] {
                                        if b.is_empty() {
                                            continue;
                                        }
                                        if let Some(evt) =
                                            crate::modules::privilege_monitor::parse_cred_dump_event(b)
                                        {
                                            if !cred_dump_context_gater(&evt) {
                                                log("[⚠️ CredDumpMonitor] Skipping known benign pattern");
                                                b.clear();
                                                continue;
                                            }

                                            let summary =
                                                String::from_utf8_lossy(&evt.summary).to_string();
                                            let ts = now_ts();
                                            let risk_score = 85.0;

                                            {
                                                let mut hits = CRED_DUMP_HITS.lock().unwrap();
                                                let entry = hits.entry(evt.pid).or_insert(0);
                                                *entry += 1;

                                                if *entry >= 3 {
                                                    log(&format!(
                                                        "[🔥 CredDumpMonitor] Escalating PID {} due to repeated cred dump hits",
                                                        evt.pid
                                                    ));
                                                    escalate_to_isolation(evt.pid);
                                                }
                                            }

                                            let mut data = HashMap::new();
                                            data.insert("pid".into(), evt.pid.to_string());
                                            data.insert("summary".into(), summary.clone());
                                            data.insert("timestamp".into(), ts.to_string());
                                            data.insert("replay_tag".into(), "cred_dump_detected".into());
                                            data.insert(
                                                "soc_note".into(),
                                                "Credential dumping activity detected".into(),
                                            );
                                            data.insert("gnn_escalate".into(), "true".into());

                                            let trust_event = TrustEvent {
                                                timestamp: ts,
                                                pid: evt.pid as i32,
                                                ppid: 0,
                                                uid: 0,
                                                binary_path: "/proc/[pid]/exe".into(),
                                                command_line: "[unknown]".into(),
                                                cwd: "/".into(),
                                                anomaly_type: "cred_dump".into(),
                                                component: "cred_monitor".into(),
                                                metadata: data.clone(),
                                                risk_score,
                                                source_module: "cred_dump_monitor".into(),
                                                decay_context: Some("ebpf_cred".into()),
                                                module: Some("memory".into()),
                                                signal: Some("cred_dump_evt".into()),
                                                signal_type: Some("ebpf".into()),
                                                score: Some(risk_score),
                                                raw_score: Some(risk_score),
                                                tags: Some(vec![
                                                    "credential_access".into(),
                                                    "memory_forensics".into(),
                                                    "ebpf".into(),
                                                ]),
                                                description: Some(
                                                    "Credential dumping activity detected by eBPF monitor".into(),
                                                ),
                                            };

                                            submit_trust_event(trust_event);
                                            push_to_gnn_vector_log(data.clone());
                                            write_telemetry_record(data.clone());

                                            if let Err(e) = push_memory_telemetry(
                                                evt.pid,
                                                0,
                                                0,
                                                "unknown".into(),
                                                "unknown".into(),
                                                "unknown".into(),
                                                MemoryAnomalyType::CredDump,
                                                summary,
                                            ) {
                                                eprintln!("⚠️ Cred dump telemetry failed: {:?}", e);
                                            }
                                        }
                                        // Clear buffer for reuse
                                        b.clear();
                                    }
                                }
                                Err(e) => {
                                    eprintln!("❌ Failed to read perf events: {:?}", e);
                                    thread::sleep(Duration::from_millis(50));
                                }
                            }
                        }
                    });
                }
                Err(e) => {
                    eprintln!("❌ Failed to open perf buffer on CPU {}: {:?}", cpu_id, e);
                }
            }
        }
    });
}

pub fn parse_cred_dump_event(buf: &BytesMut) -> Option<CredDumpEvent> {
    if buf.len() < std::mem::size_of::<CredDumpEvent>() {
        eprintln!("❌ [cred_dump_monitor] Buffer too small: {}", buf.len());
        return None;
    }

    let ptr = buf.as_ptr() as *const CredDumpEvent;
    let event = unsafe { ptr.read_unaligned() };

    // Basic sanity check
    if event.pid <= 0 || event.timestamp == 0 {
        eprintln!("❌ [cred_dump_monitor] Invalid event data: {:?}", event);
        return None;
    }

    Some(event)
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

/// Records that this escalation occurred (to suppress repeats)
fn record_privilege_escalation(pid: i32, uid: i32, path: &str, reason: &str) {
    let key = format!("{}_{}_{}", pid, uid, path);
    let _ = check_frequent_escalation(&key);
    log(&format!(
        "[📌 EscalationMemory] Recorded: PID={}, UID={}, Path={}, Reason={}",
        pid, uid, path, reason
    ));
}

/// Checks if this escalation has been seen recently to avoid repeated scoring
fn has_recent_escalation(pid: i32) -> bool {
    let key = format!("{}", pid);
    check_frequent_escalation(&key)
}

/// Optional isolation logic hook for confirmed privilege abuse
fn escalate_to_isolation(pid: i32) {
    log(&format!(
        "[🚨 Isolation Trigger] Privilege abuse detected — isolating PID {}",
        pid
    ));
    // Hook isolation logic here if implemented elsewhere
}

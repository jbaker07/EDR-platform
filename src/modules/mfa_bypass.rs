// edr-agent/modules/mfa_bypass.rs

use std::{
    fs::File,
    io::{BufReader, BufRead},
    time::{SystemTime, UNIX_EPOCH},
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

use aya::{
    Bpf, include_bytes_aligned,
    maps::perf::AsyncPerfEventArray,
    programs::TracePoint,
    util::online_cpus,
};

use bytes::BytesMut;
use lazy_static::lazy_static;
use tokio::task;

use anyhow::{Result, anyhow};

use crate::modules::replay_writer::store_replay_event;
use crate::{
    telemetry_writer::write_telemetry_record,
    telemetry_types::TelemetryOutput,
    gnn_hook::push_to_gnn_vector_log,
    trust_hook::{submit_trust_event, generate_trust_payload, generate_feature_vector, TrustEvent},
    utils::time::now_ts,
};

lazy_static! {
    static ref MFA_BYPASS_DETECTED: AtomicBool = AtomicBool::new(false);
    static ref MFA_MONITOR_STARTED: AtomicBool = AtomicBool::new(false);
}

#[derive(Debug, Clone)]
pub struct MFABypassEvent {
    pub user: String,
    pub method: String,
    pub reason: String,
    pub timestamp: u64,
}

#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Debug, Clone)]
struct MFABypassKernelEvent {
    pid: u32,
    uid: u32,
    comm: [u8; 64],
    details: [u8; 128],
}

fn current_unix_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Log parsing fallback (userland)
fn detect_mfa_bypass_events() -> Vec<MFABypassEvent> {
    let mut events = Vec::new();
    let log_path = "/var/log/auth.log";

    if let Ok(file) = File::open(log_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if line.contains("Failed publickey") && line.contains("authentication") {
                events.push(MFABypassEvent {
                    user: "unknown".into(),
                    method: "publickey".into(),
                    reason: "Repeated failed attempts".into(),
                    timestamp: current_unix_ts(),
                });
            } else if line.contains("pam_unix") && line.contains("authentication failure") {
                events.push(MFABypassEvent {
                    user: "unknown".into(),
                    method: "fallback".into(),
                    reason: "Bypass via PAM fallback".into(),
                    timestamp: current_unix_ts(),
                });
            } else if line.contains("mfa_push_denied") {
                events.push(MFABypassEvent {
                    user: "unknown".into(),
                    method: "push".into(),
                    reason: "Push fatigue detected".into(),
                    timestamp: current_unix_ts(),
                });
            }
        }
    }

    events
}

fn push_mfa_telemetry(event: MFABypassEvent) {
    let score = 8.5;
    let trust_payload = generate_trust_payload(&event.user, 0.4, 450, score);
    let features = generate_feature_vector(0.4, 450, score);

    let mut metadata = HashMap::new();
    metadata.insert("user".to_string(), event.user.clone());
    metadata.insert("method".to_string(), event.method.clone());
    metadata.insert("reason".to_string(), event.reason.clone());
    metadata.insert("timestamp".to_string(), event.timestamp.to_string());
    metadata.insert("trust_payload".to_string(), format!("{:?}", trust_payload));
    metadata.insert("features".to_string(), format!("{:?}", features));
    // make records uniform with other modules
    metadata.insert("category".into(), "auth".into());
    metadata.insert("signal".into(), "mfa_bypass".into());
    metadata.insert("replay_tag".into(), "mfa_bypass_detected".into());
    metadata.insert("gnn_escalate".into(), "true".into());

    let trust_event = TrustEvent {
        timestamp: event.timestamp,
        pid: -1,
        ppid: -1,
        uid: 0,
        binary_path: "".to_string(),
        command_line: "".to_string(),
        cwd: "".to_string(),
        anomaly_type: "mfa_bypass_execve".to_string(),
        component: "auth".to_string(),
        metadata: metadata.clone(),
        risk_score: score as f32,
        source_module: "mfa_bypass".to_string(),
        decay_context: Some("mfa_bypass_exec".to_string()),
        module: Some("auth".to_string()),
        signal: Some("execve_bypass".to_string()),
        signal_type: Some("auth::mfa_bypass".to_string()),
        score: Some(score as f32),
        raw_score: Some(score as f32),
        tags: Some(vec!["auth".to_string(), "mfa_bypass".to_string()]),
        description: Some(event.reason.clone()),
    };

    submit_trust_event(trust_event);

    let mut gnn_data = HashMap::new();
    gnn_data.insert(
        "vector".into(),
        format!(
            "{{\"user\":\"{}\",\"method\":\"{}\",\"reason\":\"{}\"}}",
            event.user, event.method, event.reason
        ),
    );
    gnn_data.insert("category".into(), "auth".into());
    gnn_data.insert("signal".into(), "mfa_bypass_execve".into());
    gnn_data.insert("confidence".into(), "0.8".into());
    gnn_data.insert("gnn_escalate".into(), "true".into());
    gnn_data.insert(
        "summary".into(),
        format!("MFA bypass anomaly for {} via execve: {}", event.user, event.reason),
    );
    gnn_data.insert("replay_tag".into(), "mfa_execve".into());

    push_to_gnn_vector_log(gnn_data.clone());
    let _ = store_replay_event(gnn_data.clone());

    write_telemetry_record(metadata);
    MFA_BYPASS_DETECTED.store(true, Ordering::SeqCst);
}

pub fn start_mfa_bypass_monitor() {
    if MFA_MONITOR_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    // Log-based detector (no Tokio needed)
    thread::spawn(|| loop {
        for event in detect_mfa_bypass_events() {
            // push telemetry + trust for each parsed event
            push_mfa_telemetry(event.clone());

            // also emit a log-source trust event with distinct signal
            let mut metadata = HashMap::new();
            metadata.insert("user".to_string(), event.user.clone());
            metadata.insert("method".to_string(), event.method.clone());
            metadata.insert("reason".to_string(), event.reason.clone());
            metadata.insert("timestamp".to_string(), event.timestamp.to_string());
            metadata.insert("category".into(), "auth".into());
            metadata.insert("signal".into(), "log_mfa_bypass".into());

            let score = 7.0_f32;

            submit_trust_event(TrustEvent {
                timestamp: event.timestamp,
                pid: -1,
                ppid: -1,
                uid: 0,
                binary_path: "".to_string(),
                command_line: "".to_string(),
                cwd: "".to_string(),
                anomaly_type: "mfa_bypass_log".to_string(),
                component: "auth".to_string(),
                metadata,
                risk_score: score,
                source_module: "mfa_bypass".to_string(),
                decay_context: Some("log_mfa_bypass".to_string()),
                module: Some("auth".to_string()),
                signal: Some("log_mfa_bypass".to_string()),
                signal_type: Some("log_monitor".to_string()),
                score: Some(score),
                raw_score: Some(score),
                tags: Some(vec!["auth".to_string(), "mfa_bypass".to_string()]),
                description: Some(event.reason.clone()),
            });

            let mut gnn_data = HashMap::new();
            gnn_data.insert(
                "vector".into(),
                format!(
                    "{{\"user\":\"{}\",\"method\":\"{}\",\"reason\":\"{}\"}}",
                    event.user, event.method, event.reason
                ),
            );
            gnn_data.insert("category".into(), "auth".into());
            gnn_data.insert("signal".into(), "log_mfa_bypass".into());
            gnn_data.insert("confidence".into(), "0.75".into());
            gnn_data.insert("gnn_escalate".into(), "true".into());
            gnn_data.insert(
                "summary".into(),
                format!("Log-detected MFA bypass attempt by {} via {}", event.user, event.method),
            );
            gnn_data.insert("replay_tag".into(), "mfa_log".into());

            push_to_gnn_vector_log(gnn_data.clone());
            let _ = store_replay_event(gnn_data);
        }

        thread::sleep(std::time::Duration::from_secs(30));
    });

    // eBPF kernel listener in its own Tokio runtime
    thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().expect("tokio runtime for mfa_bypass");
        if let Err(e) = rt.block_on(start_ebpf_mfa_trace()) {
            eprintln!("[eBPF MFA BYPASS] Init error: {:?}", e);
        }
    });
}

#[cfg(target_os = "linux")]
pub async fn start_ebpf_mfa_trace() -> Result<()> {
    let mut bpf = Bpf::load(include_bytes_aligned!("../ebpf/mfa_bypass_monitor.bpf.o"))
        .map_err(|e| anyhow!("load mfa_bypass_monitor.bpf.o: {e:?}"))?;

    let program: &mut TracePoint = bpf
        .program_mut("trace_mfa_anomaly")
        .ok_or_else(|| anyhow!("program not found: trace_mfa_anomaly"))?
        .try_into()?;
    program.load()?;
    program.attach("syscalls", "sys_enter_execve")?;

    let mut perf_array = AsyncPerfEventArray::try_from(bpf.map_mut("events")?)
        .map_err(|e| anyhow!("AsyncPerfEventArray init failed: {e:?}"))?;

    for cpu_id in online_cpus()? {
        let mut buf = perf_array.open(cpu_id, None)?;
        task::spawn(async move {
            let mut buffers: Vec<BytesMut> = vec![BytesMut::with_capacity(1024); 16];

            loop {
                match buf.read_events(&mut buffers).await {
                    Ok(events) => {
                        for i in 0..events.read {
                            let slot = &buffers[i];
                            if slot.len() < std::mem::size_of::<MFABypassKernelEvent>() {
                                continue;
                            }

                            let event = unsafe {
                                std::ptr::read_unaligned(slot.as_ptr() as *const MFABypassKernelEvent)
                            };

                            let comm = String::from_utf8_lossy(&event.comm)
                                .trim_end_matches('\0')
                                .to_string();

                            let details = String::from_utf8_lossy(&event.details)
                                .trim_end_matches('\0')
                                .to_string();

                            let enriched = MFABypassEvent {
                                user: format!("uid_{}", event.uid),
                                method: "execve".to_string(),
                                reason: details.clone(),
                                timestamp: now_ts(),
                            };

                            // Push telemetry + trust
                            push_mfa_telemetry(enriched.clone());
                            println!("[eBPF MFA BYPASS] {} ({})", comm, details);

                            // Additional trust/gnn metadata specific to the eBPF path
                            let mut metadata = HashMap::new();
                            metadata.insert("user".to_string(), enriched.user.clone());
                            metadata.insert("method".to_string(), enriched.method.clone());
                            metadata.insert("reason".to_string(), enriched.reason.clone());
                            metadata.insert("timestamp".to_string(), enriched.timestamp.to_string());
                            metadata.insert("category".into(), "auth".into());
                            metadata.insert("source".into(), "ebpf".into());

                            let trust_event = TrustEvent {
                                timestamp: enriched.timestamp,
                                pid: event.pid as i32,
                                ppid: event.pid as i32, // no PPID in event
                                uid: event.uid,
                                binary_path: comm.clone(),
                                command_line: "execve anomaly".to_string(),
                                cwd: "/".to_string(),
                                anomaly_type: "mfa_bypass_execve".to_string(),
                                component: "auth".to_string(),
                                metadata: metadata.clone(),
                                risk_score: 8.5,
                                source_module: "mfa_bypass".to_string(),
                                decay_context: Some("mfa_bypass_exec".to_string()),
                                module: Some("auth".to_string()),
                                signal: Some("execve_bypass".to_string()),
                                signal_type: Some("ebpf".to_string()),
                                score: Some(8.5),
                                raw_score: Some(8.5),
                                tags: Some(vec!["mfa_bypass".to_string(), "execve".to_string(), "auth".to_string()]),
                                description: Some("Possible MFA bypass attempt via execve-based suspicious process".to_string()),
                            };
                            submit_trust_event(trust_event);

                            let mut gnn_data = HashMap::new();
                            gnn_data.insert(
                                "vector".into(),
                                format!(
                                    "{{\"user\":\"{}\",\"method\":\"{}\",\"reason\":\"{}\"}}",
                                    enriched.user, enriched.method, enriched.reason
                                ),
                            );
                            gnn_data.insert("category".into(), "auth".into());
                            gnn_data.insert("signal".into(), "execve_bypass".into());
                            gnn_data.insert("confidence".into(), "0.85".into());
                            gnn_data.insert("gnn_escalate".into(), "true".into());
                            gnn_data.insert(
                                "summary".into(),
                                format!("Execve-based MFA bypass attempt by {} using {}", enriched.user, comm),
                            );
                            gnn_data.insert("replay_tag".into(), "mfa_execve".into());

                            push_to_gnn_vector_log(gnn_data.clone());
                            let _ = store_replay_event(gnn_data);
                        }
                    }
                    Err(e) => {
                        eprintln!("[eBPF MFA BYPASS] Error reading events: {:?}", e);
                        break;
                    }
                }
            }
        });
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub async fn start_ebpf_mfa_trace() -> Result<()> {
    // Non-Linux platforms: no-op so the crate still builds.
    Ok(())
}

/// Passive scan for fallback telemetry
pub fn scan_mfa_bypass_activity() -> Vec<TelemetryOutput> {
    if MFA_BYPASS_DETECTED.load(Ordering::Relaxed) {
        // suppress the heartbeat right after a real detection
        MFA_BYPASS_DETECTED.store(false, Ordering::Relaxed);
        return vec![];
    }

    let ts = now_ts();

    let mut data = HashMap::new();
    data.insert("event_type".into(), "mfa_bypass_monitor".into());
    data.insert("timestamp".into(), ts.to_string());
    data.insert("status".into(), "no_anomalies_detected".into());

    let trust_event = TrustEvent {
        timestamp: ts,
        pid: -1,
        ppid: -1,
        uid: 0,
        binary_path: "n/a".to_string(),
        command_line: "none".to_string(),
        cwd: "/".to_string(),
        anomaly_type: "mfa_bypass_passive_scan".to_string(),
        component: "auth".to_string(),
        metadata: data.clone(),
        risk_score: 0.0,
        source_module: "mfa_bypass".to_string(),
        decay_context: Some("mfa_bypass_passive".to_string()),
        module: Some("auth".to_string()),
        signal: Some("no_mfa_bypass_detected".to_string()),
        signal_type: Some("auth::mfa_bypass".to_string()),
        score: Some(0.0),
        raw_score: Some(0.0),
        tags: Some(vec!["auth".to_string(), "mfa".to_string()]),
        description: Some("Passive MFA bypass scan: no anomalies detected".to_string()),
    };

    submit_trust_event(trust_event);

    vec![TelemetryOutput {
        category: "auth".into(),
        signal: "no_mfa_bypass_detected".into(),
        confidence: 0.0,
        data,
    }]
}



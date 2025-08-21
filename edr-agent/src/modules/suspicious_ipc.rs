use aya::{include_bytes_aligned, Bpf};
use aya::maps::perf::PerfEventArray;
use aya::programs::TracePoint;
use aya::util::online_cpus;
use bytes::BytesMut;
use std::convert::TryInto;
use std::{thread, time::Duration};
use std::sync::{OnceLock, atomic::{AtomicBool, Ordering}};
use std::collections::HashMap;

use crate::telemetry_writer::{push_memory_telemetry, write_telemetry_record};
use crate::telemetry_types::{MemoryAnomalyType, TelemetryOutput};
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::trust_hook::{generate_feature_vector, generate_trust_payload, TrustEvent, submit_trust_event};
use crate::modules::replay_writer::store_replay_event;
use crate::utils::time::now_ts;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct IPCAbuseEvent {
    pub pid: u32,
    pub target_pid: u32,
    pub syscall_id: u32,
    pub timestamp: u64,
}

pub static SCAN_SUSPICIOUS_IPC: OnceLock<AtomicBool> = OnceLock::new();

#[inline]
fn parse_ipc_event(buf: &[u8]) -> Option<IPCAbuseEvent> {
    use std::{mem, ptr::read_unaligned};
    if buf.len() < mem::size_of::<IPCAbuseEvent>() {
        return None;
    }
    let ptr = buf.as_ptr() as *const IPCAbuseEvent;
    Some(unsafe { read_unaligned(ptr) })
}

#[cfg(target_os = "linux")]
pub fn start_ebpf_ipc_abuse_watch() {
    SCAN_SUSPICIOUS_IPC.get_or_init(|| AtomicBool::new(true));

    thread::spawn(move || {
        // Load BPF object
        let mut bpf = match Bpf::load(include_bytes_aligned!("../ebpf/suspicious_ipc.bpf.o")) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("❌ Failed to load suspicious_ipc BPF: {:?}", e);
                return;
            }
        };

        // Program lookup / attach
        let program = match bpf.program_mut("trace_suspicious_ipc") {
            Some(p) => p,
            None => {
                eprintln!("❌ Missing program: trace_suspicious_ipc");
                return;
            }
        };

        let tp: &mut TracePoint = match program.try_into() {
            Ok(tp) => tp,
            Err(e) => {
                eprintln!("❌ Could not convert program to TracePoint: {:?}", e);
                return;
            }
        };

        if let Err(e) = tp.load() {
            eprintln!("❌ Failed to load tracepoint: {:?}", e);
            return;
        }

        if let Err(e) = tp.attach("syscalls", "sys_enter_msgsnd") {
            eprintln!("❌ Failed to attach to sys_enter_msgsnd: {:?}", e);
            return;
        }

        println!("💬 [eBPF] Suspicious IPC monitor tracepoint active");

        // Open perf array
        let perf_map = match bpf.map_mut("EVENTS") {
            Ok(m) => m,
            Err(e) => {
                eprintln!("❌ No perf map 'EVENTS' found: {:?}", e);
                return;
            }
        };

        let mut perf_array = match PerfEventArray::try_from(perf_map) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("❌ PerfEventArray error: {:?}", e);
                return;
            }
        };

        for cpu_id in online_cpus().unwrap_or_default() {
            match perf_array.open(cpu_id, None) {
                Ok(mut buf) => {
                    thread::spawn(move || {
                        // Use multiple buffers; clear per use, not the whole Vec
                        let mut buffers: Vec<BytesMut> =
                            (0..16).map(|_| BytesMut::with_capacity(1024)).collect();

                        loop {
                            match buf.read_events(&mut buffers) {
                                Ok(events) => {
                                    for b in buffers.iter_mut().take(events.read) {
                                        if let Some(evt) = parse_ipc_event(&b[..]) {
                                            handle_ipc_event(evt);
                                        }
                                        b.clear(); // ready buffer for next read
                                    }
                                }
                                Err(e) => {
                                    eprintln!("⚠️ Read events error (CPU {}): {:?}", cpu_id, e);
                                    thread::sleep(Duration::from_millis(25));
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

fn handle_ipc_event(evt: IPCAbuseEvent) {
    let ts = now_ts();
    let cpu = 0.5_f64;
    let mem = 100_000_u64;
    let risk = 55.0_f64;

    let summary = format!(
        "Suspicious IPC: pid={} → pid={} via syscall_id={}",
        evt.pid, evt.target_pid, evt.syscall_id
    );

    // Best-effort memory telemetry (variant name unified with rest of the codebase)
    let _ = push_memory_telemetry(
        evt.pid as i32,
        evt.target_pid as i32,
        0,
        "unknown".into(),
        "unknown".into(),
        "unknown".into(),
        MemoryAnomalyType::IPCAbuse,
        summary.clone(),
    ).map_err(|e| eprintln!("⚠️ IPC telemetry failed: {:?}", e));

    // Trust / features / GNN / replay
    let trust = generate_trust_payload("suspicious_ipc_monitor", cpu, mem, risk);
    let features = generate_feature_vector(cpu, mem, risk);

    let mut data = HashMap::new();
    data.insert("host".into(), "macos-host".into());
    data.insert("features".into(), format!("{:?}", features));
    data.insert("replay_tag".into(), "ipc_abuse_detected".into());
    data.insert("ipc_summary".into(), summary.clone());
    data.insert("timestamp".into(), ts.to_string());
    data.insert("soc_note".into(), "IPC abuse detected via syscall msgsnd".into());
    data.insert("gnn_escalate".into(), "true".into());
    data.insert("cpu".into(), format!("{:.2}", cpu));
    data.insert("mem_kb".into(), format!("{}", mem));

    write_telemetry_record(data.clone());
    push_to_gnn_vector_log(data.clone());
    store_replay_event(data.clone());

    let trust_event = TrustEvent {
        timestamp: ts,
        pid: evt.pid as i32,
        ppid: evt.target_pid as i32,
        uid: u32::MAX, // unknown UID
        binary_path: format!("/proc/{}/exe", evt.pid),
        command_line: format!("syscall_id={}", evt.syscall_id),
        cwd: "/".into(),
        anomaly_type: "ipc_abuse".into(),
        component: "ipc_abuse_detector".into(),
        metadata: data.clone(),
        risk_score: risk as f32,
        source_module: "suspicious_ipc_monitor".into(),
        decay_context: Some("ipc_syscall_id".into()),
        module: Some("suspicious_ipc_monitor".into()),
        signal: Some("ipc_abuse_detected".into()),
        signal_type: Some("process".into()),
        score: Some(risk as f32),
        raw_score: Some(risk as f32),
        tags: Some(vec!["ipc".into(), "abuse".into(), "suspicious".into()]),
        description: Some(summary.clone()),
    };

    submit_trust_event(trust_event);

    println!(
        "🔁 [IPC Abuse] Detected → Trust Score: {}",
        trust.get("trust_score").unwrap_or(&"N/A".to_string())
    );
}

/// Passive fallback function for suspicious IPC scan
pub fn scan_ipc_passive() -> Vec<TelemetryOutput> {
    let ts = now_ts();
    let cpu = 0.2_f64;
    let mem = 10_000_u64;
    let risk = 5.0_f64;

    let features = generate_feature_vector(cpu, mem, risk);

    let mut data = HashMap::new();
    data.insert("host".into(), "macos-host".into());
    data.insert("note".into(), "Passive IPC scan fallback - no live BPF".into());
    data.insert("method".into(), "scan_ipc_passive".into());
    data.insert("severity".into(), "low".into());
    data.insert("replay_tag".into(), "simulated_ipc_abuse".into());
    data.insert("timestamp".into(), ts.to_string());
    data.insert("source".into(), "ipc_passive".into());
    data.insert("features".into(), format!("{:?}", features));
    data.insert("cpu".into(), format!("{:.2}", cpu));
    data.insert("mem_kb".into(), format!("{}", mem));
    data.insert("soc_note".into(), "Simulated fallback IPC abuse".into());
    data.insert("gnn_escalate".into(), "false".into());

    push_to_gnn_vector_log(data.clone());
    write_telemetry_record(data.clone());
    store_replay_event(data.clone());

    submit_trust_event(TrustEvent {
        timestamp: ts,
        pid: 0,
        ppid: 0,
        uid: 0,
        binary_path: "/dev/null".into(),
        command_line: "scan_ipc_passive".into(),
        cwd: "/".into(),
        anomaly_type: "ipc_abuse_simulated".into(),
        component: "ipc_passive_scanner".into(),
        metadata: data.clone(),
        risk_score: risk as f32,
        source_module: "ipc_passive".into(),
        decay_context: Some("passive_ipc".into()),
        module: Some("ipc_passive".into()),
        signal: Some("simulated_ipc_abuse".into()),
        signal_type: Some("memory_anomaly".into()),
        score: Some(risk as f32),
        raw_score: Some(risk as f32),
        tags: Some(vec!["ipc".into(), "passive".into(), "fallback".into()]),
        description: Some("Simulated IPC abuse event from passive fallback".into()),
    });

    vec![TelemetryOutput {
        category: "memory_anomaly".into(),
        signal: "simulated_ipc_abuse".into(),
        confidence: (risk / 100.0) as f32,
        data,
    }]
}
use aya::{include_bytes_aligned, Bpf};
use aya::maps::perf::PerfEventArray;
use aya::programs::TracePoint;
use aya::util::online_cpus;
use bytes::BytesMut;
use std::convert::TryInto;
use std::{thread, time::Duration};
use std::sync::{OnceLock, atomic::{AtomicBool, Ordering}};
use std::collections::HashMap;

use crate::telemetry_writer::{push_memory_telemetry, write_telemetry_record};
use crate::telemetry_types::{MemoryAnomalyType, TelemetryOutput};
use crate::trust_hook::{generate_feature_vector, generate_trust_payload, TrustEvent, submit_trust_event};
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::replay_writer::store_replay_event;
use crate::utils::time::now_ts;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct IPCAbuseEvent {
    pub pid: u32,
    pub target_pid: u32,
    pub syscall_id: u32,
    pub timestamp: u64,
}

pub static SCAN_SUSPICIOUS_IPC: OnceLock<AtomicBool> = OnceLock::new();

#[inline]
fn parse_ipc_event(buf: &[u8]) -> Option<IPCAbuseEvent> {
    use std::{mem, ptr::read_unaligned};
    if buf.len() < mem::size_of::<IPCAbuseEvent>() {
        return None;
    }
    let ptr = buf.as_ptr() as *const IPCAbuseEvent;
    Some(unsafe { read_unaligned(ptr) })
}

#[cfg(target_os = "linux")]
/// Starts the eBPF-powered suspicious IPC monitor
pub fn start_ebpf_ipc_abuse_watch() {
    SCAN_SUSPICIOUS_IPC.get_or_init(|| AtomicBool::new(true));

    thread::spawn(move || {
        // Load BPF object
        let mut bpf = match Bpf::load(include_bytes_aligned!("../ebpf/suspicious_ipc.bpf.o")) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("❌ Failed to load suspicious_ipc BPF: {:?}", e);
                return;
            }
        };

        // Program lookup / attach
        let program = match bpf.program_mut("trace_suspicious_ipc") {
            Some(p) => p,
            None => {
                eprintln!("❌ Missing trace_suspicious_ipc program");
                return;
            }
        };

        let tp: &mut TracePoint = match program.try_into() {
            Ok(tp) => tp,
            Err(e) => {
                eprintln!("❌ Could not convert to TracePoint: {:?}", e);
                return;
            }
        };

        if let Err(e) = tp.load() {
            eprintln!("❌ Failed to load tracepoint: {:?}", e);
            return;
        }

        if let Err(e) = tp.attach("syscalls", "sys_enter_msgsnd") {
            eprintln!("❌ Failed to attach syscall: {:?}", e);
            return;
        }

        println!("💬 [eBPF] Suspicious IPC monitor tracepoint active");

        // Open perf array
        let perf_map = match bpf.map_mut("EVENTS") {
            Ok(m) => m,
            Err(e) => {
                eprintln!("❌ No perf map found in suspicious_ipc_monitor: {:?}", e);
                return;
            }
        };

        let mut perf_array = match PerfEventArray::try_from(perf_map) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("❌ PerfEventArray error: {:?}", e);
                return;
            }
        };

        for cpu_id in online_cpus().unwrap_or_default() {
            match perf_array.open(cpu_id, None) {
                Ok(mut buf) => {
                    thread::spawn(move || {
                        // Pre-allocate a handful of buffers and reuse them
                        let mut buffers: Vec<BytesMut> =
                            (0..16).map(|_| BytesMut::with_capacity(1024)).collect();

                        loop {
                            match buf.read_events(&mut buffers) {
                                Ok(events) => {
                                    for b in buffers.iter_mut().take(events.read) {
                                        if let Some(evt) = parse_ipc_event(&b[..]) {
                                            handle_ipc_event(evt);
                                        }
                                        b.clear(); // ready buffer for next read
                                    }
                                }
                                Err(e) => {
                                    eprintln!("⚠️ Read events error (CPU {}): {:?}", cpu_id, e);
                                    thread::sleep(Duration::from_millis(25));
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

fn handle_ipc_event(evt: IPCAbuseEvent) {
    let ts = now_ts();
    let cpu = 0.5_f64;
    let mem = 100_000_u64;
    let risk = 55.0_f64;

    let summary = format!(
        "Suspicious IPC: pid={} → pid={} via syscall_id={}",
        evt.pid, evt.target_pid, evt.syscall_id
    );

    // Best-effort memory telemetry
    let _ = push_memory_telemetry(
        evt.pid as i32,
        evt.target_pid as i32,
        0,
        "unknown".into(),
        "unknown".into(),
        "unknown".into(),
        MemoryAnomalyType::IPCAbuse,
        summary.clone(),
    )
    .map_err(|e| eprintln!("⚠️ IPC telemetry failed: {:?}", e));

    // Trust / features / GNN / replay
    let trust = generate_trust_payload("suspicious_ipc_monitor", cpu, mem, risk);
    let features = generate_feature_vector(cpu, mem, risk);

    let mut data = HashMap::new();
    data.insert("ipc_summary".into(), summary.clone());
    data.insert("timestamp".into(), ts.to_string());
    data.insert("cpu".into(), format!("{:.2}", cpu));
    data.insert("mem_kb".into(), format!("{}", mem));
    data.insert("features".into(), format!("{:?}", features));
    data.insert("replay_tag".into(), "ipc_abuse_detected".into());
    data.insert("soc_note".into(), "IPC abuse detected via syscall msgsnd".into());
    data.insert("gnn_escalate".into(), "true".into());

    write_telemetry_record(data.clone());
    push_to_gnn_vector_log(data.clone());
    store_replay_event(data.clone());

    let trust_event = TrustEvent {
        timestamp: ts,
        pid: evt.pid as i32,
        ppid: evt.target_pid as i32,
        uid: u32::MAX, // unknown UID
        binary_path: format!("/proc/{}/exe", evt.pid),
        command_line: format!("syscall_id={}", evt.syscall_id),
        cwd: "/".into(),
        anomaly_type: "ipc_abuse".into(),
        component: "ipc_abuse_detector".into(),
        metadata: data.clone(),
        risk_score: risk as f32,
        source_module: "suspicious_ipc_monitor".into(),
        decay_context: Some("ipc_syscall_id".into()),
        module: Some("suspicious_ipc_monitor".into()),
        signal: Some("ipc_abuse_detected".into()),
        signal_type: Some("process".into()),
        score: Some(risk as f32),
        raw_score: Some(risk as f32),
        tags: Some(vec!["ipc".into(), "abuse".into(), "suspicious".into()]),
        description: Some(summary),
    };

    submit_trust_event(trust_event);

    println!(
        "🔁 [IPC Abuse] Detected → Trust Score: {}",
        trust.get("trust_score").unwrap_or(&"N/A".to_string())
    );
}

/// Passive fallback function for suspicious IPC scan
pub fn scan_ipc_passive() -> Vec<TelemetryOutput> {
    let ts = now_ts();
    let cpu = 0.2_f64;
    let mem = 10_000_u64;
    let risk = 5.0_f64;

    let features = generate_feature_vector(cpu, mem, risk);

    let mut data = HashMap::new();
    data.insert("note".into(), "Passive IPC scan fallback - no live BPF".into());
    data.insert("method".into(), "scan_ipc_passive".into());
    data.insert("severity".into(), "low".into());
    data.insert("replay_tag".into(), "simulated_ipc_abuse".into());
    data.insert("timestamp".into(), ts.to_string());
    data.insert("features".into(), format!("{:?}", features));
    data.insert("cpu".into(), format!("{:.2}", cpu));
    data.insert("mem_kb".into(), format!("{}", mem));
    data.insert("soc_note".into(), "Simulated fallback IPC abuse".into());
    data.insert("gnn_escalate".into(), "false".into());

    // Keep passive path consistent with other modules
    write_telemetry_record(data.clone());
    push_to_gnn_vector_log(data.clone());
    store_replay_event(data.clone());

    submit_trust_event(TrustEvent {
        timestamp: ts,
        pid: 0,
        ppid: 0,
        uid: 0,
        binary_path: "/dev/null".into(),
        command_line: "scan_ipc_passive".into(),
        cwd: "/".into(),
        anomaly_type: "ipc_abuse_simulated".into(),
        component: "ipc_passive_scanner".into(),
        metadata: data.clone(),
        risk_score: risk as f32,
        source_module: "ipc_passive".into(),
        decay_context: Some("passive_ipc".into()),
        module: Some("ipc_passive".into()),
        signal: Some("simulated_ipc_abuse".into()),
        signal_type: Some("memory_anomaly".into()),
        score: Some(risk as f32),
        raw_score: Some(risk as f32),
        tags: Some(vec!["ipc".into(), "passive".into(), "fallback".into()]),
        description: Some("Simulated IPC abuse event from passive fallback".into()),
    });

    vec![TelemetryOutput {
        category: "memory_anomaly".into(),
        signal: "simulated_ipc_abuse".into(),
        confidence: (risk / 100.0) as f32,
        data,
    }]
}

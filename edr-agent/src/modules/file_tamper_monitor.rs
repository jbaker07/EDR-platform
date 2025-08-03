use aya::{include_bytes_aligned, Bpf};
use aya::maps::perf::PerfEventArray;
use aya::programs::TracePoint;
use aya::util::online_cpus;
use bytes::BytesMut;
use chrono::Utc;
use crate::forensic::utils::{read_proc_value, get_ppid_and_uid};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    convert::TryInto,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use crate::{
    gnn_hook::push_to_gnn_vector_log,
    modules::replay_writer::store_replay_event,
    telemetry_types::{MemoryAnomalyType, TelemetryOutput},
    telemetry_writer::{push_memory_telemetry, write_telemetry_record},
    trust_hook::{submit_trust_event, TrustEvent},
    utils::time::now_ts,
};

// Atomic flag
lazy_static! {
    static ref FILE_TAMPER_FOUND: AtomicBool = AtomicBool::new(false);
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct FileTamperEvent {
    pub pid: u32,
    pub timestamp: u64,
    pub file_path_ptr: u64,
    pub flags: u32,
}

fn parse_file_tamper_event(buf: &[u8]) -> Option<FileTamperEvent> {
    use std::ptr::read_unaligned;
    let ptr = buf.as_ptr() as *const FileTamperEvent;
    Some(unsafe { read_unaligned(ptr) })
}

pub fn start_ebpf_file_tamper_watch() {
    thread::spawn(move || {
        let mut bpf = match Bpf::load(include_bytes_aligned!(
            "../ebpf/file_tamper_monitor.bpf.o"
        )) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("❌ Failed to load BPF: {:?}", e);
                return;
            }
        };

        let tp: &mut TracePoint = match bpf.program_mut("trace_file_tamper")
            .and_then(|p| p.try_into().ok()) {
            Some(tp) => tp,
            None => {
                eprintln!("❌ Failed to load or convert tracepoint program");
                return;
            }
        };

        if let Err(e) = tp.load().and_then(|_| tp.attach("syscalls", "sys_enter_unlink")) {
            eprintln!("❌ TracePoint attach/load error: {:?}", e);
            return;
        }

        let mut perf_array = match bpf.map_mut("EVENTS")
            .and_then(PerfEventArray::try_from) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("❌ Could not get perf array from EVENTS map: {:?}", e);
                return;
            }
        };

        for cpu_id in online_cpus().unwrap_or_default() {
            match perf_array.open(cpu_id, None) {
                Ok(mut buf) => {
                    thread::spawn(move || {
                        let mut buffers = vec![BytesMut::with_capacity(1024)];
                        loop {
                            match buf.read_events(&mut buffers) {
                                Ok(_events) => {
                                    for buffer in &buffers {
                                        if let Some(evt) = parse_file_tamper_event(buffer) {
                                            let details = format!(
                                                "File tamper detected: pid={} flags={}",
                                                evt.pid, evt.flags
                                            );

                                            let pid = evt.pid;
                                            let binary_path = read_proc_value(pid, "exe").unwrap_or_else(|_| "unknown".into());
                                            let cmdline     = read_proc_value(pid, "cmdline").unwrap_or_else(|_| "unknown".into());
                                            let cwd         = read_proc_value(pid, "cwd").unwrap_or_else(|_| "unknown".into());
                                            let file_path = read_proc_value(pid, "exe").unwrap_or_else(|_| "unknown".into());
                                            let uid = get_ppid_and_uid(pid).1;
                                            let hash = compute_file_hash(&file_path).unwrap_or_else(|_| "N/A".into()); // Optional, if you support it

                                            let mut fingerprint_data = HashMap::new();
                                            fingerprint_data.insert("file_path".into(), file_path.clone());
                                            fingerprint_data.insert("file_hash".into(), hash.clone());
                                            fingerprint_data.insert("uid".into(), uid.to_string());

                                            let fingerprints = load_fingerprints_from_disk("src/modules/telemetry_fingerprint.json");

                                            if is_known_good(&fingerprint_data, &fingerprints) {
                                                log(&format!(
                                                    "[FileTamperMonitor] Suppressed known-good file tamper: {}",
                                                    file_path
                                                ));
                                                return; // Skip processing this event
                                            }

                                            let mut data = HashMap::new();
                                            data.insert("timestamp".into(), evt.timestamp.to_string());
                                            data.insert("pid".into(), pid.to_string());
                                            data.insert("flags".into(), evt.flags.to_string());
                                            data.insert("category".into(), "integrity".into());
                                            data.insert("signal".into(), "file_tamper".into());
                                            data.insert("event_type".into(), "file_tamper".into());
                                            data.insert("confidence".into(), "0.95".into());
                                            data.insert("summary".into(), details.clone());
                                            data.insert("replay_tag".into(), "file_tamper_detected".into());
                                            data.insert("gnn_escalate".into(), "true".into());
                                            data.insert("binary_path".into(), binary_path.clone());
                                            data.insert("command_line".into(), cmdline.clone());
                                            data.insert("cwd".into(), cwd.clone());

                                            let telemetry_output = TelemetryOutput {
                                                category: "integrity".into(),
                                                signal: "file_tamper".into(),
                                                confidence: 0.95,
                                                data: data.clone(),
                                            };

                                            write_telemetry_record(data.clone());
                                            push_to_gnn_vector_log(data.clone());
                                            crate::gnn_hook::push_metadata_to_gnn_vector_log(data.clone());
                                            store_replay_event(data.clone());

                                            FILE_TAMPER_FOUND.store(true, Ordering::SeqCst);
                                            let (ppid, uid) = get_ppid_and_uid(evt.pid);

                                            submit_trust_event(TrustEvent {
                                                timestamp: evt.timestamp,
                                                pid: evt.pid as i32,
                                                ppid,
                                                uid,
                                                binary_path: binary_path.clone(),
                                                command_line: cmdline.clone(),
                                                cwd: cwd.clone(),
                                                anomaly_type: "FileTampering".into(),
                                                component: "integrity".into(),
                                                metadata: data.clone(),
                                                risk_score: 70.0,
                                                source_module: "file_tamper_monitor".into(),
                                                decay_context: Some("unlink_syscall".into()),
                                                module: Some("file_tamper_monitor".into()),
                                                signal: Some("file_tamper".into()),
                                                signal_type: Some("unlink".into()),
                                                score: Some(70.0),
                                                raw_score: Some(70.0),
                                                tags: Some(vec![
                                                    "tag:unlink_detected".into(),
                                                    "tag:file_integrity".into(),
                                                ]),
                                                description: Some(details.clone()),
                                            });

                                            log_file_tamper(&details);
                                        }
                                    }
                                    buffers.clear();
                                }
                                Err(e) => {
                                    eprintln!("⚠️ Read events error: {:?}", e);
                                }
                            }

                            thread::sleep(Duration::from_millis(25));
                        }
                    });
                }
                Err(e) => {
                    eprintln!("❌ Could not open perf buffer on CPU {}: {:?}", cpu_id, e);
                }
            }
        }
    });
}

/// Called when anomaly is detected and fallback should stop
pub fn log_file_tamper(details: &str) {
    println!("[🛑 File Tamper] {}", details);
    FILE_TAMPER_FOUND.store(true, Ordering::SeqCst);
}

pub fn scan_file_tamper_activity() -> Vec<TelemetryOutput> {
    if FILE_TAMPER_FOUND.load(Ordering::SeqCst) {
        return vec![]; // ✅ Suppress fallback output if real anomaly already triggered
    }

    let timestamp = Utc::now().timestamp() as u64;

    let mut data = HashMap::new();
    data.insert("event_type".into(), "file_tamper_monitor_active".into());
    data.insert("timestamp".into(), timestamp.to_string());
    data.insert("category".into(), "file".into());
    data.insert("signal".into(), "file_tamper_monitor_active".into());
    data.insert("confidence".into(), "0.0".into());
    data.insert("summary".into(), "Passive file tamper scan active".into());
    data.insert("replay_tag".into(), "monitor_heartbeat".into());
    data.insert("gnn_escalate".into(), "false".into());
    data.insert("soc_note".into(), "Heartbeat: File tamper monitor active".into());

    let trust_event = TrustEvent {
        timestamp,
        pid: 0,
        ppid: 0,
        uid: 0,
        binary_path: "monitor:passive_scan".to_string(),
        command_line: "monitor_heartbeat".to_string(),
        cwd: "/etc/edr/".to_string(),
        anomaly_type: "MonitorHeartbeat".to_string(),
        component: "file".to_string(),
        metadata: data.clone(),
        risk_score: 0.0,
        source_module: "file_tamper_monitor".to_string(),
        decay_context: Some("monitor_health".to_string()),
        module: Some("file_tamper_monitor".to_string()),
        signal: Some("file_tamper_monitor_active".to_string()),
        signal_type: Some("heartbeat".to_string()),
        score: Some(100.0),
        raw_score: Some(0.0),
        tags: Some(vec![
            "heartbeat".to_string(),
            "tag:file_integrity".to_string(),
        ]),
        description: Some("File tamper monitor heartbeat signal".to_string()),
    };

    submit_trust_event(trust_event);

    vec![TelemetryOutput {
        category: "file".into(),
        signal: "file_tamper_monitor_active".into(),
        confidence: 0.0,
        data,
    }]
}

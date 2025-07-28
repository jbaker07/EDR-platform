use std::{
    collections::HashMap,
    fs,
    mem,
    sync::{atomic::{AtomicBool, Ordering}, Once},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use aya::{include_bytes_aligned, Bpf, maps::perf::PerfEventArray, programs::TracePoint, util::online_cpus};
use bytes::BytesMut;
use chrono::{Timelike, Local, Utc};
use walkdir::WalkDir;
use crate::utils::time::now_ts;

use crate::{
    telemetry_types::TelemetryOutput,
    telemetry::{calculate_entropy, get_file_metadata, is_memory_only, TelemetryRecord},
    logger::log,
    trust_hook::{submit_trust_event, TrustEvent},
    gnn_hook::push_to_gnn_vector_log,
    telemetry_writer::write_telemetry_record,
};
use crate::modules::replay_writer::store_replay_event;

static ENCRYPTED_MONITOR_STARTED: AtomicBool = AtomicBool::new(false);
static ENCRYPTED_MONITOR_ONCE: Once = Once::new();

#[repr(C)]
#[derive(Clone, Copy)]
struct EncryptedPayloadEvent {
    pid: u32,
    ppid: u32,
    filename: [u8; 256],
    entropy: f32,
}
// Adding dynamic PID, PPID, and UID values
use std::process::{Command, id};

pub fn detect_encrypted_payloads() -> Vec<HashMap<String, String>> {
    let mut records = Vec::new();

    let suspicious_dirs = vec![
        "/tmp", "/var/tmp", "/dev/shm", "/run/user", "/home", "/opt", "/usr/local/bin",
    ];

    for dir in &suspicious_dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Ok(content) = fs::read(&path) {
                    let entropy = calculate_entropy(&content);
                    if entropy > 9.0 {
                        let filename = path.file_name().unwrap_or_default().to_string_lossy();
                        let is_mem_only = is_memory_only(&path);
                        let metadata = get_file_metadata(&path);
                        let size = metadata.len();
                        let hour = Local::now().hour();

                        let is_weird_name = filename.contains(".enc") || filename.contains(".bin") || filename.contains(".dat");
                        let is_obfuscated_name = filename.len() > 30 && !filename.contains('.');
                        let is_off_hours = hour < 6 || hour > 22;

                        let mut trust_score = if entropy > 9.0 && is_mem_only {
                            15.0
                        } else if entropy > 8.0 && is_weird_name {
                            10.0
                        } else {
                            5.0
                        };
                        if is_obfuscated_name { trust_score += 2.0; }
                        if is_off_hours { trust_score += 2.5; }

                        let score_f32 = (100.0_f32 - trust_score).max(0.0_f32);
                        let now = now_ts();

                        // Dynamically get PID, PPID, UID here
                        let pid = id();
                        let ppid = get_ppid(pid); // You may need to write a helper function to fetch PPID
                        let uid = get_uid(pid); // You can get UID based on the process ID

                        let mut metadata_map = HashMap::new();
                        metadata_map.insert("path".into(), path.display().to_string());
                        metadata_map.insert("entropy".into(), format!("{:.2}", entropy));
                        metadata_map.insert("mem_only".into(), is_mem_only.to_string());
                        metadata_map.insert("size_bytes".into(), size.to_string());
                        metadata_map.insert("off_hours".into(), is_off_hours.to_string());
                        metadata_map.insert("weird_name".into(), is_weird_name.to_string());
                        metadata_map.insert("obfuscated_name".into(), is_obfuscated_name.to_string());

                        let trust_event = TrustEvent {
                            timestamp: now,
                            pid: pid as i32,           // Dynamically set PID
                            ppid: ppid as i32,         // Dynamically set PPID
                            uid: uid,                  // Dynamically set UID
                            binary_path: path.display().to_string(),
                            command_line: "file_drop".into(),
                            cwd: dir.to_string(),
                            anomaly_type: "EncryptedPayload".into(),
                            component: "file".into(),
                            metadata: metadata_map.clone(),
                            risk_score: trust_score as f32,
                            source_module: "encrypted_payload_detector".into(),
                            decay_context: Some("file_entropy".into()),
                            module: Some("encrypted_payload_detector".into()),
                            signal: Some("encrypted_payload".into()),
                            signal_type: Some("file_drop".into()),
                            score: Some(score_f32),
                            raw_score: Some(trust_score as f32),
                            tags: Some(vec![
                                "encrypted_file".into(),
                                "high_entropy".into(),
                                if is_off_hours { "off_hours".into() } else { "normal_hours".into() },
                                if is_mem_only { "memory_only".into() } else { "disk_backed".into() },
                            ]),
                            description: Some(format!(
                                "Encrypted file detected: {} (entropy {:.2}, mem_only={}, size={}B)",
                                path.display(), entropy, is_mem_only, size
                            )),
                        };

                        submit_trust_event(trust_event);

                        let mut record = metadata_map.clone();
                        record.insert("timestamp".into(), now.to_string());
                        record.insert("event".into(), "encrypted_payload".into());
                        record.insert("severity".into(), if trust_score > 10.0 { "high".into() } else { "medium".into() });
                        record.insert("replay_tag".into(), "encrypted_file_drop".into());
                        record.insert("soc_note".into(), "High-entropy file detected in sensitive directory".into());
                        record.insert("gnn_escalate".into(), "true".into());
                        record.insert("category".into(), "file".into());
                        record.insert("signal".into(), "encrypted_payload".into());

                        write_telemetry_record(record.clone());
                        push_to_gnn_vector_log(record.clone());
                        crate::gnn_hook::push_metadata_to_gnn_vector_log(record.clone());
                        store_replay_event(record.clone()).ok();

                        records.push(record);
                    }
                }
            }
        }
    }

    records
}


fn get_ppid(pid: u32) -> u32 {
    let path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(path).unwrap_or_default();
    content.lines().find(|line| line.starts_with("PPid:")).and_then(|line| {
        line.split_whitespace().nth(1)?.parse().ok()
    }).unwrap_or(0)
}


fn get_uid(pid: u32) -> u32 {
    let path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(path).unwrap_or_default();
    content.lines().find(|line| line.starts_with("Uid:")).and_then(|line| {
        line.split_whitespace().nth(1)?.parse().ok()
    }).unwrap_or(0)
}

pub fn start_encrypted_payload_monitor() {
    ENCRYPTED_MONITOR_ONCE.call_once(|| {
        thread::spawn(move || {
            let mut bpf = match Bpf::load(include_bytes_aligned!(
                "../../src/ebpf/encrypted_payload_monitor.bpf.o"
            )) {
                Ok(b) => b,
                Err(e) => return eprintln!("❌ Failed to load eBPF: {:?}", e),
            };

            let program = match bpf.program_mut("trace_execve_entropy") {
                Some(p) => p,
                None => return eprintln!("❌ Missing eBPF program 'trace_execve_entropy'"),
            };

            let tp: &mut TracePoint = match program.try_into() {
                Ok(tp) => tp,
                Err(e) => return eprintln!("❌ Could not convert program: {:?}", e),
            };

            if tp.load().is_err() || tp.attach("syscalls", "sys_enter_execve").is_err() {
                return eprintln!("❌ Failed to attach eBPF tracepoint");
            }

            println!("📦 [eBPF] Encrypted Payload Monitor attached.");

            let mut perf_array = match PerfEventArray::try_from(bpf.map_mut("events").unwrap()) {
                Ok(p) => p,
                Err(e) => return eprintln!("❌ PerfEventArray error: {:?}", e),
            };

            for cpu_id in online_cpus().unwrap_or_default() {
                if let Ok(mut buf) = perf_array.open(cpu_id, None) {
                    thread::spawn(move || {
                        let mut buffers = vec![BytesMut::with_capacity(1024); 32];
                        loop {
                            match buf.read_events(&mut buffers) {
                                Ok(events) => {
                                    for buf in &buffers[..events.read] {
                                        if buf.len() < mem::size_of::<EncryptedPayloadEvent>() {
                                            continue;
                                        }
                                        let ptr = buf.as_ptr() as *const EncryptedPayloadEvent;
                                        let evt = unsafe { ptr.read_unaligned() };

                                        let fname = String::from_utf8_lossy(&evt.filename)
                                            .trim_matches(char::from(0))
                                            .to_string();

                                        println!(
                                            "[eBPF] Encrypted payload: pid={} name={} entropy={}",
                                            evt.pid, fname, evt.entropy
                                        );

                                        let now = now_ts();
                                        let mut metadata = HashMap::new();
                                        metadata.insert("pid".into(), evt.pid.to_string());
                                        metadata.insert("ppid".into(), evt.ppid.to_string());
                                        metadata.insert("filename".into(), fname.clone());
                                        metadata.insert("entropy".into(), format!("{:.2}", evt.entropy));
                                        metadata.insert("detected_by".into(), "ebpf_entropy_monitor".into());

                                        let trust_score = if evt.entropy > 7.9 {
                                            14.0
                                        } else if evt.entropy > 7.7 {
                                            9.5
                                        } else {
                                            5.0
                                        };

                                        let trust_event = TrustEvent {
                                            timestamp: now,
                                            pid: evt.pid as i32,
                                            ppid: evt.ppid as i32,
                                            uid: 0,
                                            binary_path: fname.clone(),
                                            command_line: "execve".into(),
                                            cwd: "".into(),
                                            anomaly_type: "EncryptedPayloadRealtime".into(),
                                            component: "memory".into(),
                                            metadata: metadata.clone(),
                                            risk_score: trust_score,
                                            source_module: "encrypted_payload_monitor".into(),
                                            decay_context: Some("ebpf_exec_entropy".into()),
                                            module: Some("encrypted_payload_monitor".into()),
                                            signal: Some("encrypted_payload".into()),
                                            signal_type: Some("execve_entropy".into()),
                                            score: Some(100.0 - trust_score),
                                            raw_score: Some(trust_score),
                                            tags: Some(vec!["encrypted_exec".into(), "high_entropy".into()]),
                                            description: Some(format!(
                                                "High-entropy payload exec: {} (entropy {:.2})",
                                                fname, evt.entropy
                                            )),
                                        };

                                        submit_trust_event(trust_event);

                                        let mut record = metadata.clone();
                                        record.insert("timestamp".into(), now.to_string());
                                        record.insert("event".into(), "encrypted_payload_realtime".into());
                                        record.insert("category".into(), "memory".into());
                                        record.insert("signal".into(), "encrypted_payload".into());
                                        record.insert("confidence".into(), "0.9".into());
                                        record.insert("gnn_escalate".into(), "true".into());
                                        record.insert("replay_tag".into(), "realtime_entropy".into());
                                        record.insert("soc_note".into(), "eBPF execve trace caught encrypted payload".into());

                                        write_telemetry_record(record.clone());
                                        push_to_gnn_vector_log(record.clone());
                                        crate::gnn_hook::push_metadata_to_gnn_vector_log(record.clone());
                                        store_replay_event(record.clone()).ok();
                                    }
                                }
                                Err(e) => {
                                    eprintln!("⚠️ eBPF read error: {:?}", e);
                                    thread::sleep(Duration::from_millis(50));
                                }
                            }
                        }
                    });
                }
            }
        });
    });
}

/// Emits a heartbeat record and ensures eBPF thread is launched
pub fn scan_encrypted_payload_activity() -> Vec<TelemetryOutput> {
    if !ENCRYPTED_MONITOR_STARTED.load(Ordering::Relaxed) {
        start_encrypted_payload_monitor();
        ENCRYPTED_MONITOR_STARTED.store(true, Ordering::Relaxed);
    }

    let ts = now_ts();

    let mut data = HashMap::new();
    data.insert("timestamp".into(), ts.to_string());
    data.insert("event_type".into(), "encrypted_payload_monitor_active".into());
    data.insert("category".into(), "memory".into());
    data.insert("signal".into(), "encrypted_payload_monitor_active".into());
    data.insert("confidence".into(), "0.0".into());
    data.insert("replay_tag".into(), "monitor_heartbeat".into());
    data.insert("gnn_escalate".into(), "false".into());
    data.insert("soc_note".into(), "Heartbeat: Encrypted payload monitor active".into());

    let trust_event = TrustEvent {
        timestamp: ts,
        pid: 1, // Dummy, but stable
        ppid: 0,
        uid: 0,
        binary_path: "kernel".into(),
        command_line: "start_encrypted_payload_monitor".into(),
        cwd: "/".into(),
        anomaly_type: "Status".into(), // Normalize for consistency
        component: "memory".into(),
        metadata: data.clone(),
        risk_score: 0.0,
        source_module: "encrypted_payload_monitor".into(),
        decay_context: Some("monitor_health".into()),
        module: Some("encrypted_payload_monitor".into()),
        signal: Some("encrypted_payload_monitor_active".into()),
        signal_type: Some("monitor_heartbeat".into()),
        score: Some(100.0),
        raw_score: Some(0.0),
        tags: Some(vec!["heartbeat".into(), "monitor_check".into()]),
        description: Some("Encrypted payload monitor heartbeat signal".into()),
    };

    submit_trust_event(trust_event);

    vec![TelemetryOutput {
        category: "memory".into(),
        signal: "encrypted_payload_monitor_active".into(),
        confidence: 0.0,
        data,
    }]
}

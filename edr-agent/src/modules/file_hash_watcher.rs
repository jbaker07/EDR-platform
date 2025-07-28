use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Read,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use aya::{
    include_bytes_aligned,
    maps::perf::PerfEventArray,
    programs::TracePoint,
    util::online_cpus,
    Bpf,
};
use bytes::BytesMut;
use chrono::Utc;
use sha2::{Digest, Sha256};
use crate::forensic::utils::read_proc_value;

use crate::telemetry::TelemetryRecord;
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::{TelemetryWriter, write_telemetry_record};
use crate::trust_hook::{submit_trust_event, TrustEvent};
use crate::utils::time::now_ts;
use crate::modules::replay_writer::store_replay_event;
use crate::gnn_hook::{push_to_gnn_vector_log, push_metadata_to_gnn_vector_log};

static FILE_HASH_FOUND: AtomicBool = AtomicBool::new(false);

#[repr(C)]
#[derive(Clone, Copy)]
struct FileEvent {
    timestamp: u64,
    pid: u32,
    filename: [u8; 256],
}

#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FileAccessEvent {
    pub pid: u32,
    pub access_type: u8, // 0=read, 1=write, 2=exec
    pub file_path_hash: u64,
    pub file_path: [u8; 256],
    pub timestamp: u64,
}

fn load_hash_list_from_file(path: &str) -> HashSet<String> {
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str::<Vec<String>>(&data)
            .unwrap_or_default()
            .into_iter()
            .collect()
    } else {
        println!("[⚠️ file_hash_watcher] Threat hash file not found: {}", path);
        HashSet::new()
    }
}

fn compute_sha256(path: &Path) -> Option<String> {
    let mut file = fs::File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 4096];

    loop {
        let bytes_read = file.read(&mut buffer).ok()?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Some(format!("{:x}", hasher.finalize()))
}

pub fn start_file_hash_monitor(writer: Arc<Mutex<TelemetryWriter>>) {
    let writer_clone = Arc::clone(&writer);
    thread::spawn(move || loop {
        let scan_dirs = vec!["/bin", "/usr/bin", "/usr/local/bin", "/opt", "/tmp", "/etc"];
        let threat_hashes = load_hash_list_from_file("/etc/edr/threat_hashes.json");
        let timestamp = now_ts();

        for dir in &scan_dirs {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(hash) = compute_sha256(&path) {
                            let is_threat = threat_hashes.contains(&hash);
                            let risk = if is_threat { 95.0 } else { 10.0 };

                            let canonical_path = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
                            let binary_path_str = canonical_path.to_string_lossy().into_owned();

                            // Default values
                            let mut pid = 0;
                            let mut ppid = 0;
                            let mut uid = 0;
                            let mut cwd = dir.to_string();
                            let mut cmdline = "hash_check".to_string();

                            // Attempt enrichment if any PIDs map to the file
                            // (In the future, inode matching or LSOF-style lookup)
                            // Placeholder here: keep default

                            let record = TelemetryRecord {
                                timestamp,
                                pid,
                                ppid,
                                uid,
                                binary_path: binary_path_str.clone(),
                                command_line: cmdline.clone(),
                                cwd: cwd.clone(),
                                env_vars: None,
                                risk_score: Some(risk as u32),
                                tags: vec![
                                    format!("sha256:{}", hash),
                                    if is_threat { "malicious:true".into() } else { "malicious:false".into() },
                                    "tag:file_integrity".into(),
                                ],
                            };

                            if let Ok(mut w) = writer_clone.lock() {
                                w.write_record(record.clone());
                            }

                            let mut data = HashMap::new();
                            data.insert("timestamp".into(), timestamp.to_string());
                            data.insert("pid".into(), pid.to_string());
                            data.insert("ppid".into(), ppid.to_string());
                            data.insert("uid".into(), uid.to_string());
                            data.insert("binary_path".into(), binary_path_str.clone());
                            data.insert("sha256".into(), hash.clone());
                            data.insert("risk_score".into(), risk.to_string());
                            data.insert("event_type".into(), "file_hash".into());
                            data.insert("signal".into(), "file_hash_check".into());
                            data.insert("category".into(), "file".into());
                            data.insert("cwd".into(), cwd.clone());
                            data.insert("command_line".into(), cmdline.clone());
                            data.insert("confidence".into(), if is_threat { "0.95" } else { "0.3" }.into());
                            data.insert("replay_tag".into(), "hash_match".into());
                            data.insert("gnn_escalate".into(), if is_threat { "true" } else { "false" }.into());

                            store_replay_event(data.clone());
                            push_to_gnn_vector_log(data.clone());
                            push_metadata_to_gnn_vector_log(data.clone());

                            if is_threat {
                                FILE_HASH_FOUND.store(true, Ordering::SeqCst);
                                submit_trust_event(TrustEvent {
                                    timestamp,
                                    pid,
                                    ppid,
                                    uid,
                                    binary_path: binary_path_str.clone(),
                                    command_line: cmdline.clone(),
                                    cwd,
                                    anomaly_type: "FileHashMatch".into(),
                                    component: "file".into(),
                                    metadata: data.clone(),
                                    risk_score: risk,
                                    source_module: "file_hash_watcher".into(),
                                    decay_context: Some("file_integrity".into()),
                                    module: Some("file_hash_watcher".into()),
                                    signal: Some("file_hash_check".into()),
                                    signal_type: Some("file_hash_match".into()),
                                    score: Some(risk),
                                    raw_score: Some(risk),
                                    tags: Some(vec![
                                        format!("sha256:{}", hash),
                                        "tag:file_integrity".into(),
                                        "tag:threat_intel_match".into(),
                                    ]),
                                    description: Some("Known malicious file detected via SHA256 match".into()),
                                });
                            }
                        }
                    }
                }
            }
        }

        thread::sleep(Duration::from_secs(180));
    });
}

#[cfg(target_os = "linux")]
pub fn start_ebpf_file_watch() -> Vec<TelemetryOutput> {
    use std::sync::mpsc::{channel, Sender};

    let mut results = Vec::new();

    let data = std::fs::read("/opt/edr-ebpf/file_access_monitor.bpf.o").expect("Missing eBPF object");
    let mut bpf = Bpf::load(&data).expect("Failed to load eBPF");

    let program = bpf.program_mut("trace_file_access").expect("Missing program");
    let prog: &mut TracePoint = program.try_into().expect("Program cast failed");
    prog.load().expect("Program load failed");
    prog.attach("syscalls", "sys_enter_openat").expect("Attach failed");

    let mut perf = PerfEventArray::try_from(bpf.map_mut("EVENTS").unwrap())
        .expect("Failed to access EVENTS map");

    let (tx, rx) = channel();

    for cpu_id in online_cpus().unwrap_or_default() {
        let mut buf = perf.open(cpu_id, None).expect("Failed to open perf buffer");
        let tx = tx.clone();

        thread::spawn(move || {
            let mut buffers = vec![BytesMut::with_capacity(1024); 32];
            loop {
                match buf.read_events(&mut buffers) {
                    Ok(events) => {
                        for buf in &buffers[..events.read] {
                            if let Some(parsed) = parse_file_access_event(buf) {
                                FILE_HASH_FOUND.store(true, Ordering::SeqCst);
                                let _ = tx.send(parsed);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("perf read error: {:?}", e);
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }
        });
    }

    let timeout = Instant::now() + Duration::from_millis(100);
    while Instant::now() < timeout {
        if let Ok(parsed) = rx.try_recv() {
            let data = parsed.data.clone();

            write_telemetry_record(data.clone());
            push_to_gnn_vector_log(data.clone());
            store_replay_event(data.clone());
            crate::gnn_hook::push_metadata_to_gnn_vector_log(data.clone());
            results.push(parsed.clone());

            // Emit TrustEvent
            if let Some(pid_str) = data.get("pid") {
                let pid = pid_str.parse::<u32>().unwrap_or(0);

                // ✅ Define path_str from data
                let path_str = data
                    .get("path")
                    .cloned()
                    .unwrap_or_else(|| "[unknown path]".into());

                // ✅ Define risk_score explicitly
                let risk_score = 90.0;

                // ✅ Construct metadata HashMap
                let mut metadata = HashMap::new();
                metadata.insert("path".into(), path_str.clone());
                metadata.insert("reason".into(), "Matched known malicious hash".into());
                metadata.insert("scanner".into(), "local_hash_db".into());

                // ✅ Now safe to call
                submit_trust_event(TrustEvent {
                    timestamp: now_ts(),
                    pid: pid as i32,
                    ppid: 0,
                    uid: 0,
                    binary_path: path_str.clone(),
                    command_line: "[unknown]".into(),
                    cwd: "/".into(),
                    anomaly_type: "file_hash_detected".into(),
                    component: "file_hash_watcher".into(),
                    metadata,
                    risk_score,
                    source_module: "file_hash_watcher".into(),
                    decay_context: Some("known_malware".into()),
                    module: Some("file".into()),
                    signal: Some("malicious_file".into()),
                    signal_type: Some("hash_match".into()),
                    score: Some(risk_score),
                    raw_score: Some(risk_score),
                    tags: Some(vec!["malware".into(), "file".into(), "hash".into()]),
                    description: Some("Known malicious file hash detected".into()),
                });
            }
        } else {
            thread::sleep(Duration::from_millis(10));
        }
    }

    results
}

#[cfg(target_os = "linux")]
fn parse_file_access_event(buf: &[u8]) -> Option<TelemetryOutput> {
    use std::ptr::read_unaligned;
    use std::str;

    let ptr = buf.as_ptr() as *const FileAccessEvent;
    let evt = unsafe { read_unaligned(ptr) };

    let access_str = match evt.access_type {
        0 => "read",
        1 => "write",
        2 => "exec",
        _ => "unknown",
    };

    let file_path = String::from_utf8_lossy(&evt.file_path)
        .trim_end_matches(char::from(0))
        .to_string();

    let mut data = HashMap::new();
    data.insert("timestamp".into(), evt.timestamp.to_string());
    data.insert("pid".into(), evt.pid.to_string());
    data.insert("access_type".into(), access_str.to_string());
    data.insert("file_path_hash".into(), format!("{:x}", evt.file_path_hash));
    data.insert("file_path".into(), file_path.clone());
    data.insert("summary".into(), format!(
        "File {} access by pid {} (hash={})",
        access_str, evt.pid, evt.file_path_hash
    ));

    if let Ok(binary_path) = read_proc_value(evt.pid, "exe") {
        data.insert("binary_path".into(), binary_path);
    }
    if let Ok(cmdline) = read_proc_value(evt.pid, "cmdline") {
        data.insert("command_line".into(), cmdline);
    }
    if let Ok(cwd) = read_proc_value(evt.pid, "cwd") {
        data.insert("cwd".into(), cwd);
    }

    Some(TelemetryOutput {
        category: "file".into(),
        signal: "file_access".into(),
        confidence: 0.7,
        data,
    })
}


/// Passive fallback scan signal for heartbeat and trust audit
pub fn scan_file_hash_activity() -> Vec<TelemetryOutput> {
    if FILE_HASH_FOUND.load(Ordering::SeqCst) {
        return vec![]; // Don't emit heartbeat if detection already occurred
    }

    let ts = now_ts();
    let mut data = HashMap::new();
    data.insert("event_type".into(), "file_hash_monitor_active".into());
    data.insert("timestamp".into(), ts.to_string());
    data.insert("category".into(), "file".into());
    data.insert("signal".into(), "file_hash_monitor_active".into());
    data.insert("confidence".into(), "0.0".into());
    data.insert("replay_tag".into(), "monitor_heartbeat".into());
    data.insert("gnn_escalate".into(), "false".into());
    data.insert("soc_note".into(), "Heartbeat: file hash monitor active".into());

    submit_trust_event(TrustEvent {
        timestamp: ts,
        pid: 1,
        ppid: 0,
        uid: 0,
        binary_path: "kernel".into(),
        command_line: "start_file_hash_monitor".into(),
        cwd: "/".into(),
        anomaly_type: "Status".into(),
        component: "file".into(),
        metadata: data.clone(),
        risk_score: 0.0,
        source_module: "file_hash_watcher".into(),
        decay_context: Some("monitor_health".into()),
        module: Some("file_hash_watcher".into()),
        signal: Some("file_hash_monitor_active".into()),
        signal_type: Some("monitor_heartbeat".into()),
        score: Some(100.0),
        raw_score: Some(0.0),
        tags: Some(vec!["heartbeat".into(), "monitor_check".into()]),
        description: Some("File hash monitor heartbeat signal".into()),
    });

    vec![TelemetryOutput {
        category: "file".into(),
        signal: "file_hash_monitor_active".into(),
        confidence: 0.0,
        data,
    }]
}

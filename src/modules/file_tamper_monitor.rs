use aya::{include_bytes_aligned, Bpf};
use aya::maps::perf::PerfEventArray;
use aya::programs::TracePoint;
use aya::util::online_cpus;
use bytes::BytesMut;
use chrono::Utc;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    fs,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Once,
    },
    thread,
    time::Duration,
};

use crate::forensic::utils::{get_ppid_and_uid, read_proc_value};
use crate::{
    gnn_hook::{push_to_gnn_vector_log, push_metadata_to_gnn_vector_log},
    logger::log,
    modules::replay_writer::store_replay_event,
    telemetry_types::TelemetryOutput,
    telemetry_writer::write_telemetry_record,
    trust_hook::{submit_trust_event, TrustEvent},
    utils::time::now_ts,
};

/* ------------------ start/health flags ------------------ */

lazy_static! {
    static ref FILE_TAMPER_FOUND: AtomicBool = AtomicBool::new(false);
}
static TAMPER_ONCE: Once = Once::new();
static TAMPER_STARTED: AtomicBool = AtomicBool::new(false);

/* ------------------ event layouts ------------------ */

#[repr(C)]
#[derive(Clone, Debug)]
pub struct FileTamperEventV1 {
    pub pid: u32,
    pub timestamp: u64,
    pub file_path_ptr: u64, // pointer captured by eBPF (cannot be dereferenced here)
    pub flags: u32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct FileTamperEventV2 {
    pub pid: u32,
    pub timestamp: u64,
    pub flags: u32,
    pub file_path: [u8; 256], // preferred: path copied by eBPF with bpf_probe_read_user_str
}

/* ------------------ safe parsers ------------------ */

fn parse_v1(buf: &[u8]) -> Option<FileTamperEventV1> {
    use std::ptr::read_unaligned;
    if buf.len() < std::mem::size_of::<FileTamperEventV1>() {
        return None;
    }
    let ptr = buf.as_ptr() as *const FileTamperEventV1;
    Some(unsafe { read_unaligned(ptr) })
}

fn parse_v2(buf: &[u8]) -> Option<FileTamperEventV2> {
    use std::ptr::read_unaligned;
    if buf.len() < std::mem::size_of::<FileTamperEventV2>() {
        return None;
    }
    let ptr = buf.as_ptr() as *const FileTamperEventV2;
    Some(unsafe { read_unaligned(ptr) })
}

/* ------------------ optional hashing & fingerprints ------------------ */

fn compute_file_hash(path: &Path) -> Option<String> {
    use sha2::{Digest, Sha256};
    use std::io::Read;

    const MAX_SIZE: u64 = 64 * 1024 * 1024; // 64MB guard
    let meta = fs::metadata(path).ok()?;
    if !meta.is_file() || meta.len() > MAX_SIZE {
        return None;
    }

    let mut f = fs::File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = f.read(&mut buf).ok()?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Some(format!("{:x}", hasher.finalize()))
}

#[derive(Default)]
struct Fingerprints {
    hashes: HashSet<String>,
    paths: HashSet<String>,
}
fn load_fingerprints_from_disk(path: &str) -> Fingerprints {
    if let Ok(s) = fs::read_to_string(path) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) {
            let mut fp = Fingerprints::default();
            match v {
                serde_json::Value::Array(arr) => {
                    for x in arr {
                        if let Some(h) = x.as_str() {
                            fp.hashes.insert(h.to_string());
                        }
                    }
                }
                serde_json::Value::Object(obj) => {
                    if let Some(h) = obj.get("hashes").and_then(|x| x.as_array()) {
                        for x in h {
                            if let Some(s) = x.as_str() {
                                fp.hashes.insert(s.to_string());
                            }
                        }
                    }
                    if let Some(p) = obj.get("paths").and_then(|x| x.as_array()) {
                        for x in p {
                            if let Some(s) = x.as_str() {
                                fp.paths.insert(s.to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
            return fp;
        }
    }
    Fingerprints::default()
}
fn is_known_good(meta: &HashMap<String, String>, fp: &Fingerprints) -> bool {
    if let Some(h) = meta.get("file_hash") {
        if fp.hashes.contains(h) {
            return true;
        }
    }
    if let Some(p) = meta.get("file_path") {
        if fp.paths.contains(p) {
            return true;
        }
    }
    false
}

/* ------------------ eBPF bootstrap ------------------ */

pub fn start_ebpf_file_tamper_watch() {
    TAMPER_ONCE.call_once(|| {
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

            let tp: &mut TracePoint = match bpf
                .program_mut("trace_file_tamper")
                .and_then(|p| p.try_into().ok())
            {
                Some(tp) => tp,
                None => {
                    eprintln!("❌ Failed to load/convert tracepoint program");
                    return;
                }
            };

            if let Err(e) = tp.load().and_then(|_| tp.attach("syscalls", "sys_enter_unlink")) {
                eprintln!("❌ TracePoint attach/load error: {:?}", e);
                return;
            }
            // Optionally, also monitor unlinkat/rename if your BPF supports it:
            // tp.attach("syscalls", "sys_enter_unlinkat").ok();
            // tp.attach("syscalls", "sys_enter_rename").ok();

            let mut perf_array = match bpf.map_mut("EVENTS").and_then(PerfEventArray::try_from) {
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
                            let mut buffers = vec![BytesMut::with_capacity(1024); 32];
                            loop {
                                match buf.read_events(&mut buffers) {
                                    Ok(events) => {
                                        if events.lost > 0 {
                                            log(&format!("⚠️ Lost {} file-tamper events", events.lost));
                                        }
                                        for b in &buffers[..events.read] {
                                            // Try v2 (with inlined path) first, then v1 (pointer)
                                            if let Some(evt2) = parse_v2(b) {
                                                handle_event_v2(evt2);
                                            } else if let Some(evt1) = parse_v1(b) {
                                                handle_event_v1(evt1);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("⚠️ Read events error: {:?}", e);
                                        thread::sleep(Duration::from_millis(50));
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("❌ Could not open perf buffer on CPU {}: {:?}", cpu_id, e);
                    }
                }
            }

            TAMPER_STARTED.store(true, Ordering::Relaxed);
            log("✅ file_tamper_monitor attached");
        });
    });
}

/* ------------------ handlers ------------------ */

fn handle_event_v1(evt: FileTamperEventV1) {
    let pid = evt.pid;
    let (ppid, uid) = get_ppid_and_uid(pid);

    // We cannot safely dereference evt.file_path_ptr in user space. Fall back:
    let file_path = read_proc_value(pid, "exe").unwrap_or_else(|_| "unknown".into());
    let cmdline = read_proc_value(pid, "cmdline").unwrap_or_else(|_| "unknown".into());
    let cwd = read_proc_value(pid, "cwd").unwrap_or_else(|_| "unknown".into());

    let details = format!(
        "File tamper detected: pid={} flags={} ptr=0x{:x}",
        pid, evt.flags, evt.file_path_ptr
    );

    let mut fingerprint_data = HashMap::new();
    fingerprint_data.insert("file_path".into(), file_path.clone());
    if let Some(h) = compute_file_hash(Path::new(&file_path)) {
        fingerprint_data.insert("file_hash".into(), h);
    }
    fingerprint_data.insert("uid".into(), uid.to_string());

    let fingerprints = load_fingerprints_from_disk("src/modules/telemetry_fingerprint.json");
    if is_known_good(&fingerprint_data, &fingerprints) {
        log(&format!(
            "[FileTamperMonitor] Suppressed known-good file tamper: {}",
            file_path
        ));
        return;
    }

    let mut data = HashMap::new();
    data.insert("timestamp".into(), evt.timestamp.to_string());
    data.insert("pid".into(), pid.to_string());
    data.insert("ppid".into(), ppid.to_string());
    data.insert("uid".into(), uid.to_string());
    data.insert("flags".into(), evt.flags.to_string());
    data.insert("ptr".into(), format!("0x{:x}", evt.file_path_ptr));
    data.insert("category".into(), "integrity".into());
    data.insert("signal".into(), "file_tamper".into());
    data.insert("event_type".into(), "file_tamper".into());
    data.insert("confidence".into(), "0.95".into());
    data.insert("summary".into(), details.clone());
    data.insert("replay_tag".into(), "file_tamper_detected".into());
    data.insert("gnn_escalate".into(), "true".into());
    data.insert("binary_path".into(), file_path.clone());
    data.insert("command_line".into(), cmdline.clone());
    data.insert("cwd".into(), cwd.clone());

    write_telemetry_record(data.clone());
    push_to_gnn_vector_log(data.clone());
    push_metadata_to_gnn_vector_log(data.clone());
    store_replay_event(data.clone());

    FILE_TAMPER_FOUND.store(true, Ordering::SeqCst);

    submit_trust_event(TrustEvent {
        timestamp: evt.timestamp,
        pid: pid as i32,
        ppid,
        uid,
        binary_path: file_path.clone(),
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
        tags: Some(vec!["tag:unlink_detected".into(), "tag:file_integrity".into()]),
        description: Some(details),
    });

    log_file_tamper("tamper_v1");
}

fn handle_event_v2(evt: FileTamperEventV2) {
    let pid = evt.pid;
    let (ppid, uid) = get_ppid_and_uid(pid);

    let path_str = String::from_utf8_lossy(&evt.file_path)
        .trim_end_matches(char::from(0))
        .to_string();

    let cmdline = read_proc_value(pid, "cmdline").unwrap_or_else(|_| "unknown".into());
    let cwd = read_proc_value(pid, "cwd").unwrap_or_else(|_| "unknown".into());

    let mut fingerprint_data = HashMap::new();
    fingerprint_data.insert("file_path".into(), path_str.clone());
    if let Some(h) = compute_file_hash(Path::new(&path_str)) {
        fingerprint_data.insert("file_hash".into(), h);
    }
    fingerprint_data.insert("uid".into(), uid.to_string());

    let fingerprints = load_fingerprints_from_disk("src/modules/telemetry_fingerprint.json");
    if is_known_good(&fingerprint_data, &fingerprints) {
        log(&format!(
            "[FileTamperMonitor] Suppressed known-good file tamper: {}",
            path_str
        ));
        return;
    }

    let details = format!("File tamper detected: pid={} flags={} path={}", pid, evt.flags, path_str);

    let mut data = HashMap::new();
    data.insert("timestamp".into(), now_ts().to_string());
    data.insert("pid".into(), pid.to_string());
    data.insert("ppid".into(), ppid.to_string());
    data.insert("uid".into(), uid.to_string());
    data.insert("flags".into(), evt.flags.to_string());
    data.insert("category".into(), "integrity".into());
    data.insert("signal".into(), "file_tamper".into());
    data.insert("event_type".into(), "file_tamper".into());
    data.insert("confidence".into(), "0.95".into());
    data.insert("summary".into(), details.clone());
    data.insert("replay_tag".into(), "file_tamper_detected".into());
    data.insert("gnn_escalate".into(), "true".into());
    data.insert("binary_path".into(), path_str.clone());
    data.insert("command_line".into(), cmdline.clone());
    data.insert("cwd".into(), cwd.clone());

    write_telemetry_record(data.clone());
    push_to_gnn_vector_log(data.clone());
    push_metadata_to_gnn_vector_log(data.clone());
    store_replay_event(data.clone());

    FILE_TAMPER_FOUND.store(true, Ordering::SeqCst);

    submit_trust_event(TrustEvent {
        timestamp: now_ts(),
        pid: pid as i32,
        ppid,
        uid,
        binary_path: path_str.clone(),
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
        tags: Some(vec!["tag:unlink_detected".into(), "tag:file_integrity".into()]),
        description: Some(details),
    });

    log_file_tamper("tamper_v2");
}

/* ------------------ public helpers ------------------ */

/// Called when anomaly is detected and fallback should stop
pub fn log_file_tamper(details: &str) {
    println!("[🛑 File Tamper] {}", details);
    FILE_TAMPER_FOUND.store(true, Ordering::SeqCst);
}

/// Passive heartbeat (emitted only if no real tamper seen)
pub fn scan_file_tamper_activity() -> Vec<TelemetryOutput> {
    if FILE_TAMPER_FOUND.load(Ordering::SeqCst) || TAMPER_STARTED.load(Ordering::Relaxed) {
        return vec![];
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
        tags: Some(vec!["heartbeat".to_string(), "tag:file_integrity".to_string()]),
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

use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::Read,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, Once,
    },
    thread,
    time::{Duration, Instant},
};

use aya::{
    include_bytes_aligned, maps::perf::PerfEventArray, programs::TracePoint, util::online_cpus, Bpf,
};
use bytes::BytesMut;
use chrono::Utc;
use sha2::{Digest, Sha256};

use crate::forensic::utils::read_proc_value;
use crate::logger::log;

use crate::gnn_hook::{push_metadata_to_gnn_vector_log, push_to_gnn_vector_log};
use crate::modules::replay_writer::store_replay_event;
use crate::telemetry::TelemetryRecord;
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::{write_telemetry_record, TelemetryWriter};
use crate::trust_hook::{submit_trust_event, TrustEvent};
use crate::utils::strnorm::cbytes_to_string_lossy;
use crate::utils::time::now_ts;

static FILE_HASH_FOUND: AtomicBool = AtomicBool::new(false);

/* ---------- eBPF one-time + outbox ----------- */
static FILE_EBPF_ONCE: Once = Once::new();
static FILE_EBPF_STARTED: AtomicBool = AtomicBool::new(false);
lazy_static::lazy_static! {
    static ref FILE_EBPF_OUTBOX: Mutex<Vec<TelemetryOutput>> = Mutex::new(Vec::new());
    // De-dupe and rate-limit state for file_hash_check
    static ref FH_DEDUPE: Mutex<HashMap<String, Instant>> = Mutex::new(HashMap::new());
    static ref FH_RATE: Mutex<VecDeque<Instant>> = Mutex::new(VecDeque::new());
}

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

/* ------------ threat-hash & hashing utils ------------- */

fn load_hash_list_from_file(path: &str) -> HashSet<String> {
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str::<Vec<String>>(&data)
            .unwrap_or_default()
            .into_iter()
            .collect()
    } else {
        println!(
            "[⚠️ file_hash_watcher] Threat hash file not found: {}",
            path
        );
        HashSet::new()
    }
}

fn compute_sha256(path: &Path) -> Option<String> {
    // guardrail: skip huge files (e.g., >100MB)
    const MAX_SIZE: u64 = 100 * 1024 * 1024;
    if let Ok(meta) = fs::metadata(path) {
        if !meta.is_file() || meta.len() > MAX_SIZE {
            return None;
        }
    }

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

use mime_guess::MimeGuess;

/* ------------- fingerprint fallbacks (safe defaults) --------------
   If you already have richer helpers, replace these with:
   use crate::forensic::utils::{load_fingerprints_from_disk, is_known_good};
   use crate::forensic::utils::{load_fingerprints, is_known_good_fingerprint};
*/
#[derive(Default)]
struct Fingerprints {
    hashes: HashSet<String>,
    paths: HashSet<String>,
}

fn fp_from_file(path: &str) -> Fingerprints {
    if let Ok(s) = fs::read_to_string(path) {
        // Allow either: ["hash1", "hash2", ...] OR {"hashes":[...], "paths":[...]}
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
fn load_fingerprints_from_disk(path: &str) -> Fingerprints {
    fp_from_file(path)
}
fn is_known_good(meta: &HashMap<String, String>, fp: &Fingerprints) -> bool {
    if let Some(h) = meta.get("hash") {
        if fp.hashes.contains(h) {
            return true;
        }
    }
    if let Some(p) = meta.get("path") {
        if fp.paths.contains(p) {
            return true;
        }
    }
    false
}
fn load_fingerprints() -> Fingerprints {
    // try repo path then system path
    let a = load_fingerprints_from_disk("src/modules/telemetry_fingerprint.json");
    if !a.hashes.is_empty() || !a.paths.is_empty() {
        return a;
    }
    load_fingerprints_from_disk("/etc/edr/telemetry_fingerprint.json")
}
fn is_known_good_fingerprint(meta: &HashMap<String, String>, fp: &Fingerprints) -> bool {
    is_known_good(meta, fp)
}

/* ---------------- gating helpers: de-dupe, rate, heuristics ---------------- */

const FH_DEDUPE_TTL_SECS: u64 = 120;
const FH_RATE_WINDOW_SECS: u64 = 60;
const FH_RATE_LIMIT_PER_MIN: usize = 200;

fn fh_should_emit_now(key: &str) -> bool {
    let now = Instant::now();
    // de-dupe window
    {
        let mut m = FH_DEDUPE.lock().unwrap();
        if let Some(last) = m.get(key) {
            if now.duration_since(*last) < Duration::from_secs(FH_DEDUPE_TTL_SECS) {
                return false;
            }
        }
        m.insert(key.to_string(), now);
    }
    // rate cap
    {
        let mut q = FH_RATE.lock().unwrap();
        let cutoff = now - Duration::from_secs(FH_RATE_WINDOW_SECS);
        while matches!(q.front(), Some(t) if *t < cutoff) {
            q.pop_front();
        }
        if q.len() >= FH_RATE_LIMIT_PER_MIN {
            return false;
        }
        q.push_back(now);
    }
    true
}

fn path_is_risky(p: &str) -> bool {
    p.starts_with("/tmp/") || p.starts_with("/dev/shm/") || p.contains("(deleted)")
}

fn owned_by_root_and_exec(meta: &fs::Metadata) -> bool {
    let exec_capable = meta.permissions().mode() & 0o111 != 0;
    let uid = meta.uid();
    exec_capable && uid == 0
}

fn should_stream(signal: &str, confidence: f32, gnn_escalate: bool) -> bool {
    if signal.ends_with("_monitor_active") || signal.ends_with("_no_anomaly") {
        return false;
    }
    if signal == "file_hash_check" && confidence < 0.5 && !gnn_escalate {
        return false;
    }
    true
}

/* ---------------- periodic file-system hashing scan ---------------- */

pub fn start_file_hash_monitor(writer: Arc<Mutex<TelemetryWriter>>) {
    let writer_clone = Arc::clone(&writer);
    thread::spawn(move || loop {
        let scan_dirs = vec!["/bin", "/usr/bin", "/usr/local/bin", "/opt", "/tmp", "/etc"];
        let threat_hashes = load_hash_list_from_file("/etc/edr/threat_hashes.json");
        let timestamp = now_ts();
        // Load fingerprints once per scan iteration
        let fingerprints = load_fingerprints();

        for dir in &scan_dirs {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(hash) = compute_sha256(&path) {
                            let is_threat = threat_hashes.contains(&hash);
                            let mut risk: f32 = if is_threat { 95.0 } else { 10.0 };

                            let canonical_path =
                                fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
                            let binary_path_str = canonical_path.to_string_lossy().into_owned();

                            // Extract file metadata
                            let metadata = match fs::metadata(&path) {
                                Ok(meta) => meta,
                                Err(_) => continue,
                            };
                            let permissions = metadata.permissions().mode();
                            let uid = metadata.uid();
                            let exec_capable = permissions & 0o111 != 0;
                            let category = MimeGuess::from_path(&path)
                                .first_raw()
                                .unwrap_or("application/octet-stream")
                                .to_string();

                            // Fingerprint suppression (known-good)
                            let mut fingerprint_data = HashMap::new();
                            fingerprint_data.insert("path".into(), binary_path_str.clone());
                            fingerprint_data.insert("hash".into(), hash.clone());
                            fingerprint_data.insert("trusted_uid".into(), uid.to_string());
                            fingerprint_data.insert("permissions".into(), permissions.to_string());
                            fingerprint_data
                                .insert("exec_capable".into(), exec_capable.to_string());
                            fingerprint_data.insert("category".into(), category.clone());
                            fingerprint_data
                                .insert("source_module".into(), "file_hash_watcher".into());
                            fingerprint_data.insert("timestamp".into(), timestamp.to_string());

                            if is_known_good(&fingerprint_data, &fingerprints) && !is_threat {
                                log(&format!(
                                    "[FileHashWatcher] Suppressed known-good file: {}",
                                    binary_path_str
                                ));
                                continue;
                            }

                            // De-dupe & rate-cap per (path|sha256)
                            let key = format!("{}|{}", binary_path_str, hash);
                            if !fh_should_emit_now(&key) {
                                continue;
                            }

                            // Confidence/risk gating
                            let mut confidence: f32 = if is_threat { 0.95 } else { 0.30 };
                            let mut gnn_escalate = is_threat;
                            if !is_threat {
                                if path_is_risky(&binary_path_str) {
                                    confidence = confidence.max(0.60);
                                    risk = risk.max(40.0_f32);
                                }
                                if owned_by_root_and_exec(&metadata) {
                                    confidence = confidence.max(0.70);
                                    risk = risk.max(55.0_f32);
                                }
                            }

                            // Default enrichment
                            let pid = 0;
                            let ppid = 0;
                            let cwd = dir.to_string();
                            let cmdline = "hash_check".to_string();

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
                                    if is_threat {
                                        "malicious:true".into()
                                    } else {
                                        "malicious:false".into()
                                    },
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
                            data.insert("risk_score".into(), format!("{:.0}", risk));
                            data.insert("event_type".into(), "file_hash".into());
                            data.insert("signal".into(), "file_hash_check".into());
                            data.insert("category".into(), "file".into());
                            data.insert("cwd".into(), cwd.clone());
                            data.insert("command_line".into(), cmdline.clone());
                            data.insert("confidence".into(), format!("{:.2}", confidence));
                            data.insert("replay_tag".into(), "hash_match".into());
                            data.insert(
                                "gnn_escalate".into(),
                                if gnn_escalate { "true" } else { "false" }.into(),
                            );

                            // persist + gnn + replay
                            write_telemetry_record(data.clone());
                            store_replay_event(data.clone());
                            if should_stream("file_hash_check", confidence, gnn_escalate) {
                                push_to_gnn_vector_log(data.clone());
                                push_metadata_to_gnn_vector_log(data.clone());
                            }

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
                                    description: Some(
                                        "Known malicious file detected via SHA256 match".into(),
                                    ),
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

/* ---------------- eBPF realtime file-access watcher ---------------- */

#[cfg(target_os = "linux")]
pub fn start_ebpf_file_watch() -> Vec<TelemetryOutput> {
    FILE_EBPF_ONCE.call_once(|| {
        // best-effort attach only once
        let data = match std::fs::read("/opt/edr-ebpf/file_access_monitor.bpf.o") {
            Ok(d) => d,
            Err(e) => {
                eprintln!(
                    "Missing eBPF object (/opt/edr-ebpf/file_access_monitor.bpf.o): {:?}",
                    e
                );
                return;
            }
        };

        // Leak BPF so perf readers can run on 'static threads without lifetime issues
        let mut tmp = match Bpf::load(&data) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Failed to load eBPF: {:?}", e);
                return;
            }
        };
        let bpf: &'static mut Bpf = Box::leak(Box::new(tmp));

        let program = match bpf.program_mut("trace_file_access") {
            Some(p) => p,
            None => {
                eprintln!("Missing program trace_file_access");
                return;
            }
        };

        let prog: &mut TracePoint = match program.try_into() {
            Ok(tp) => tp,
            Err(e) => {
                eprintln!("Program cast failed: {:?}", e);
                return;
            }
        };

        if let Err(e) = prog
            .load()
            .and_then(|_| prog.attach("syscalls", "sys_enter_openat"))
        {
            eprintln!("Attach failed: {:?}", e);
            return;
        }

        // Open the PerfEventArray without overlapping borrows of `bpf`
        let mut perf = match bpf
            .map_mut("EVENTS")
            .and_then(|m| PerfEventArray::try_from(m).ok())
        {
            Some(arr) => arr,
            None => {
                eprintln!("Failed to access EVENTS map (PerfEventArray)");
                return;
            }
        };

        for cpu_id in online_cpus().unwrap_or_default() {
            match perf.open(cpu_id, None) {
                Ok(mut buf) => {
                    thread::spawn(move || {
                        let mut buffers = vec![BytesMut::with_capacity(1024); 32];
                        loop {
                            match buf.read_events(&mut buffers) {
                                Ok(events) => {
                                    if events.lost > 0 {
                                        log(&format!("⚠️ Lost {} file-access events", events.lost));
                                    }
                                    for buf in &buffers[..events.read] {
                                        if let Some(parsed) = parse_file_access_event(buf) {
                                            // persist + gnn + replay immediately
                                            write_telemetry_record(parsed.data.clone());
                                            push_to_gnn_vector_log(parsed.data.clone());
                                            store_replay_event(parsed.data.clone());
                                            crate::gnn_hook::push_metadata_to_gnn_vector_log(
                                                parsed.data.clone(),
                                            );

                                            FILE_HASH_FOUND.store(true, Ordering::SeqCst);
                                            if let Ok(mut q) = FILE_EBPF_OUTBOX.lock() {
                                                q.push(parsed);
                                            }
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
                Err(e) => {
                    eprintln!("Failed to open perf buffer on CPU {}: {:?}", cpu_id, e);
                }
            }
        }

        FILE_EBPF_STARTED.store(true, Ordering::Relaxed);
        log("✅ eBPF file-access watcher started");
    });

    // Drain outbox (non-blocking) and return those outputs
    let mut out = Vec::new();
    if let Ok(mut q) = FILE_EBPF_OUTBOX.lock() {
        if !q.is_empty() {
            out.append(&mut *q);
        } else if FILE_EBPF_STARTED.load(Ordering::Relaxed) {
            // emit a tiny heartbeat if started but no events this tick
            let mut data = HashMap::new();
            data.insert("event_type".into(), "file_access_monitor_active".into());
            data.insert("timestamp".into(), now_ts().to_string());
            data.insert("category".into(), "file".into());
            data.insert("signal".into(), "file_access_monitor_active".into());
            data.insert("confidence".into(), "0.0".into());
            out.push(TelemetryOutput {
                category: "file".into(),
                signal: "file_access_monitor_active".into(),
                confidence: 0.0,
                data,
            });
        }
    }
    out
}

#[cfg(target_os = "linux")]
fn parse_file_access_event(buf: &[u8]) -> Option<TelemetryOutput> {
    use std::ptr::read_unaligned;

    if buf.len() < std::mem::size_of::<FileAccessEvent>() {
        return None;
    }

    let ptr = buf.as_ptr() as *const FileAccessEvent;
    let evt = unsafe { read_unaligned(ptr) };

    let access_str = match evt.access_type {
        0 => "read",
        1 => "write",
        2 => "exec",
        _ => "unknown",
    };

    let file_path = cbytes_to_string_lossy(&evt.file_path);

    let mut data = HashMap::new();
    data.insert("timestamp".into(), evt.timestamp.to_string());
    data.insert("pid".into(), evt.pid.to_string());
    data.insert("access_type".into(), access_str.to_string());
    data.insert("file_path_hash".into(), format!("{:x}", evt.file_path_hash));
    data.insert("file_path".into(), file_path.clone());
    data.insert(
        "summary".into(),
        format!(
            "File {} access by pid {} (hash={})",
            access_str, evt.pid, evt.file_path_hash
        ),
    );

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

/* --------- passive heartbeat for dashboards --------- */

pub fn scan_file_hash_activity() -> Vec<TelemetryOutput> {
    if FILE_HASH_FOUND.load(Ordering::SeqCst) {
        return vec![]; // suppress heartbeat once detections occurred recently
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
    data.insert(
        "soc_note".into(),
        "Heartbeat: file hash monitor active".into(),
    );

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

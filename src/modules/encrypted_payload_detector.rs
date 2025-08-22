use std::{
    collections::HashMap,
    fs,
    mem,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Once,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use aya::{
    include_bytes_aligned,
    maps::perf::PerfEventArray,
    programs::{KProbe, Program, RawTracePoint, TracePoint},
    util::online_cpus,
    Bpf,
};
use bytes::BytesMut;
use chrono::{Local, Utc};
use chrono::Timelike; // for .hour()
use walkdir::WalkDir;

use crate::utils::time::now_ts;
use crate::{
    gnn_hook::{push_metadata_to_gnn_vector_log, push_to_gnn_vector_log},
    modules::replay_writer::store_replay_event,
    telemetry_types::TelemetryOutput,
    telemetry_writer::write_telemetry_record,
    trust_hook::{submit_trust_event, TrustEvent},
};
use crate::logger::log;

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

// ------- small helpers -------

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn get_ppid(pid: u32) -> u32 {
    let path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(path).unwrap_or_default();
    content
        .lines()
        .find(|l| l.starts_with("PPid:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0)
}

fn get_uid(pid: u32) -> u32 {
    let path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(path).unwrap_or_default();
    content
        .lines()
        .find(|l| l.starts_with("Uid:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0)
}

// --- local shims to replace telemetry::{calculate_entropy, get_file_metadata, is_memory_only} ---

fn calculate_entropy(bytes: &[u8]) -> f64 {
    // Shannon entropy (bits per byte)
    let mut freq = [0usize; 256];
    for b in bytes {
        freq[*b as usize] += 1;
    }
    let len = bytes.len() as f64;
    if len == 0.0 {
        return 0.0;
    }
    freq.iter()
        .filter(|&&n| n > 0)
        .map(|&n| {
            let p = (n as f64) / len;
            -p * p.log2()
        })
        .sum()
}

fn get_file_metadata(path: &Path) -> Option<(u64 /*size*/, u64 /*mtime_sec*/)> {
    use std::time::UNIX_EPOCH;
    let m = fs::metadata(path).ok()?;
    let size = m.len();
    let mtime = m.modified().ok()?.duration_since(UNIX_EPOCH).ok()?.as_secs();
    Some((size, mtime))
}

fn is_memory_only(path: &Path) -> bool {
    // crude indicators of anon / memfd / ephemeral paths
    let s = path.to_string_lossy();
    s.starts_with("memfd:") || s.contains(" (deleted)") || s.starts_with("/proc/")
}

/// Attach every program found in the object by section name where possible.
/// Falls back to your specific `trace_execve_entropy` → `syscalls/sys_enter_execve`.
fn attach_programs_or_fallback(bpf: &mut Bpf) -> anyhow::Result<()> {
    let mut attached_any = false;

    // NOTE: programs_mut() iterates as (&str, &mut Program)
    for (sec, prog) in bpf.programs_mut() {
        match prog {
            Program::TracePoint(tp) => {
                tp.load()?;
                // section usually "tracepoint/<cat>/<event>"
                if let Some(rest) = sec.strip_prefix("tracepoint/") {
                    let mut it = rest.splitn(2, '/');
                    let cat = it.next().unwrap_or("");
                    let evt = it.next().unwrap_or("");
                    tp.attach(cat, evt)?;
                    log(&format!("✔️ attached tracepoint {}/{}", cat, evt));
                    attached_any = true;
                }
            }
            Program::RawTracePoint(rtp) => {
                rtp.load()?;
                // section typically "raw_tracepoint/<event>"
                let ev = sec.split('/').nth(1).unwrap_or(sec);
                rtp.attach(ev)?;
                log(&format!("✔️ attached raw_tracepoint {}", ev));
                attached_any = true;
            }
            Program::KProbe(kp) => {
                // Many objects encode the attach point via section (kprobe/<symbol>).
                // Aya represents both entry/return as KProbe. If you need a kretprobe,
                // encode it in the BPF section name. Here we try to derive a symbol:
                kp.load()?;
                if let Some(sym) = sec.split('/').nth(1) {
                    // best-effort attach; if it fails we still consider load success
                    if let Err(e) = kp.attach(sym, 0) {
                        log(&format!("ℹ️ KProbe loaded but attach({sym}) failed: {:?}", e));
                    } else {
                        log(&format!("✔️ attached kprobe {}", sym));
                        attached_any = true;
                    }
                } else {
                    log("ℹ️ KProbe loaded (no symbol in section; skipping attach)");
                }
            }
            // ⬇️ FIX: do not call `other.load()` here; `Program` enum has no unified load().
            _other => {
                log(&format!(
                    "ℹ️ unsupported/unknown program variant; skipping explicit attach [{}]",
                    sec
                ));
            }
        }
    }

    // Fallback path for your original program/section if nothing matched above
    if !attached_any {
        if let Some(p) = bpf.program_mut("trace_execve_entropy") {
            if let Program::TracePoint(tp) = p {
                tp.load()?;
                tp.attach("syscalls", "sys_enter_execve")?;
                log("✔️ fallback attach: trace_execve_entropy → syscalls/sys_enter_execve");
                attached_any = true;
            }
        }
    }

    if !attached_any {
        anyhow::bail!("no programs attached (neither generic sections nor fallback)");
    }
    Ok(())
}

/// Start the eBPF entropy monitor (execve-side).
pub fn start_encrypted_payload_monitor() {
    ENCRYPTED_MONITOR_ONCE.call_once(|| {
        thread::spawn(move || {
            // Prefer same layout as other modules: "../ebpf/..."
            let mut bpf = match Bpf::load(include_bytes_aligned!(
                "../ebpf/encrypted_payload_monitor.bpf.o"
            )) {
                Ok(b) => b,
                Err(e) => {
                    // try legacy relative path (your original)
                    match Bpf::load(include_bytes_aligned!(
                        "../../src/ebpf/encrypted_payload_monitor.bpf.o"
                    )) {
                        Ok(b2) => b2,
                        Err(e2) => {
                            eprintln!("❌ Failed to load eBPF (both paths): {e:?} | {e2:?}");
                            return;
                        }
                    }
                }
            };

            if let Err(e) = attach_programs_or_fallback(&mut bpf) {
                eprintln!("❌ Failed to attach encrypted payload programs: {:?}", e);
                return;
            }

            // Mark started only after successful attach
            ENCRYPTED_MONITOR_STARTED.store(true, Ordering::Release);
            println!("📦 [eBPF] Encrypted Payload Monitor attached.");

            // Map name tolerance: try "events" then "EVENTS"
            let mut perf_array = if let Ok(m) = bpf.map_mut("events") {
                match PerfEventArray::try_from(m) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("❌ PerfEventArray creation failed for 'events': {:?}", e);
                        if let Ok(m2) = bpf.map_mut("EVENTS") {
                            match PerfEventArray::try_from(m2) {
                                Ok(p2) => p2,
                                Err(e2) => {
                                    eprintln!(
                                        "❌ PerfEventArray creation failed for 'EVENTS' as well: {:?}",
                                        e2
                                    );
                                    return;
                                }
                            }
                        } else {
                            eprintln!("❌ Map 'EVENTS' not found either.");
                            return;
                        }
                    }
                }
            } else if let Ok(m2) = bpf.map_mut("EVENTS") {
                match PerfEventArray::try_from(m2) {
                    Ok(p2) => p2,
                    Err(e2) => {
                        eprintln!(
                            "❌ PerfEventArray creation failed for 'EVENTS' (no 'events' map): {:?}",
                            e2
                        );
                        return;
                    }
                }
            } else {
                eprintln!("❌ PerfEventArray not found (tried 'events' and 'EVENTS')");
                return;
            };

            for cpu_id in online_cpus().unwrap_or_default() {
                match perf_array.open(cpu_id, None) {
                    Ok(mut buf) => {
                        thread::spawn(move || {
                            let mut buffers = vec![BytesMut::with_capacity(1024); 32];

                            loop {
                                match buf.read_events(&mut buffers) {
                                    Ok(events) => {
                                        for slice in &buffers[..events.read] {
                                            if slice.len() < mem::size_of::<EncryptedPayloadEvent>() {
                                                continue;
                                            }
                                            let ptr = slice.as_ptr() as *const EncryptedPayloadEvent;
                                            let evt = unsafe { ptr.read_unaligned() };

                                            let fname = String::from_utf8_lossy(&evt.filename)
                                                .trim_matches(char::from(0))
                                                .to_string();

                                            println!(
                                                "[eBPF] Encrypted payload: pid={} ppid={} name={} entropy={}",
                                                evt.pid, evt.ppid, fname, evt.entropy
                                            );

                                            let now = now_secs();
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

                                            // Trust event
                                            let trust_event = TrustEvent {
                                                timestamp: now,
                                                pid: evt.pid as i32,
                                                ppid: evt.ppid as i32,
                                                uid: get_uid(evt.pid),
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

                                            // Telemetry
                                            let mut record = metadata.clone();
                                            record.insert("timestamp".into(), now.to_string());
                                            record.insert("event".into(), "encrypted_payload_realtime".into());
                                            record.insert("category".into(), "memory".into());
                                            record.insert("signal".into(), "encrypted_payload".into());
                                            record.insert("confidence".into(), "0.9".into());
                                            record.insert("gnn_escalate".into(), "true".into());
                                            record.insert("replay_tag".into(), "realtime_entropy".into());
                                            record.insert(
                                                "soc_note".into(),
                                                "eBPF execve trace caught encrypted payload".into(),
                                            );

                                            write_telemetry_record(record.clone());
                                            push_to_gnn_vector_log(record.clone());
                                            push_metadata_to_gnn_vector_log(record.clone());
                                            let _ = store_replay_event(record);
                                        }

                                        if events.lost > 0 {
                                            eprintln!("⚠️ Lost {} encrypted-payload events", events.lost);
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
                    Err(e) => {
                        eprintln!("⚠️ Failed to open perf buffer on CPU {}: {:?}", cpu_id, e);
                    }
                }
            }
        });
    });
}

/// Filesystem sweep (passive) — unchanged logic, with a safety cap on file size.
pub fn detect_encrypted_payloads() -> Vec<HashMap<String, String>> {
    let mut records = Vec::new();

    let suspicious_dirs = vec![
        "/tmp", "/var/tmp", "/dev/shm", "/run/user", "/home", "/opt", "/usr/local/bin",
    ];

    const MAX_SCAN_BYTES: u64 = 8 * 1024 * 1024; // 8 MiB guardrail

    for dir in &suspicious_dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                // Guardrail: skip giant files to avoid RAM spikes
                if let Ok(meta) = fs::metadata(path) {
                    if meta.len() > MAX_SCAN_BYTES {
                        continue;
                    }
                }

                if let Ok(content) = fs::read(&path) {
                    let entropy = calculate_entropy(&content);
                    if entropy > 9.0 {
                        let filename = path.file_name().unwrap_or_default().to_string_lossy();
                        let is_mem_only = is_memory_only(path);
                        let size = get_file_metadata(path).map(|(sz, _)| sz).unwrap_or(0);
                        let hour = Local::now().hour();

                        let is_weird_name =
                            filename.contains(".enc") || filename.contains(".bin") || filename.contains(".dat");
                        let is_obfuscated_name = filename.len() > 30 && !filename.contains('.');
                        let is_off_hours = hour < 6 || hour > 22;

                        let mut trust_score = if entropy > 9.0 && is_mem_only {
                            15.0
                        } else if entropy > 8.0 && is_weird_name {
                            10.0
                        } else {
                            5.0
                        };
                        if is_obfuscated_name {
                            trust_score += 2.0;
                        }
                        if is_off_hours {
                            trust_score += 2.5;
                        }

                        let score_f32 = (100.0_f32 - trust_score).max(0.0_f32);
                        let now = now_secs();

                        // dynamic PID/PPID/UID of THIS process (scanner)
                        let pid = std::process::id();
                        let ppid = get_ppid(pid);
                        let uid = get_uid(pid);

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
                            pid: pid as i32,
                            ppid: ppid as i32,
                            uid,
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
                                path.display(),
                                entropy,
                                is_mem_only,
                                size
                            )),
                        };

                        submit_trust_event(trust_event);

                        let mut record = metadata_map.clone();
                        record.insert("timestamp".into(), now.to_string());
                        record.insert("event".into(), "encrypted_payload".into());
                        record.insert(
                            "severity".into(),
                            if trust_score > 10.0 { "high".into() } else { "medium".into() },
                        );
                        record.insert("replay_tag".into(), "encrypted_file_drop".into());
                        record.insert(
                            "soc_note".into(),
                            "High-entropy file detected in sensitive directory".into(),
                        );
                        record.insert("gnn_escalate".into(), "true".into());
                        record.insert("category".into(), "file".into());
                        record.insert("signal".into(), "encrypted_payload".into());

                        write_telemetry_record(record.clone());
                        push_to_gnn_vector_log(record.clone());
                        push_metadata_to_gnn_vector_log(record.clone());
                        let _ = store_replay_event(record.clone());

                        records.push(record);
                    }
                }
            }
        }
    }

    records
}

/// Heartbeat & ensure thread launched
pub fn scan_encrypted_payload_activity() -> Vec<TelemetryOutput> {
    if !ENCRYPTED_MONITOR_STARTED.load(Ordering::Acquire) {
        start_encrypted_payload_monitor();
        // Note: ENCRYPTED_MONITOR_STARTED is flipped to true only after successful attach
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
        pid: 1, // stable dummy
        ppid: 0,
        uid: 0,
        binary_path: "kernel".into(),
        command_line: "start_encrypted_payload_monitor".into(),
        cwd: "/".into(),
        anomaly_type: "Status".into(),
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

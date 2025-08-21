use std::collections::{HashMap, HashSet};
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Once};
use std::thread;
use std::time::Duration;

use bytes::BytesMut;
use lazy_static::lazy_static;

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::replay_writer::store_replay_event;
use crate::telemetry::TelemetryRecord;
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::TelemetryWriter;
use crate::trust_hook::{generate_feature_vector, generate_trust_payload, submit_trust_event, TrustEvent};
use crate::utils::time::now_ts;

lazy_static! {
    static ref REAL_LOGON_FOUND: AtomicBool = AtomicBool::new(false);
    static ref SEEN_PIDS: Mutex<HashSet<i32>> = Mutex::new(HashSet::new());
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LogonEvent {
    pub pid: i32,
    pub uid: u32,
    pub gid: u32,
    pub ppid: i32,
    pub comm: [u8; 64],
}

const PERF_BUFS_PER_CPU: usize = 32;
const PERF_BUF_CAP: usize = 1024;
const SEEN_PIDS_MAX: usize = 8192;

static START_EBPF: Once = Once::new();

#[cfg(target_os = "linux")]
pub fn start_logon_tracker(writer: Arc<Mutex<TelemetryWriter>>) {
    use aya::maps::perf::PerfEventArray;
    use aya::programs::TracePoint;
    use aya::util::online_cpus;
    use aya::{include_bytes_aligned, Bpf};
    use std::convert::TryInto;

    START_EBPF.call_once(|| {
        thread::spawn(move || {
            let mut bpf = match Bpf::load(include_bytes_aligned!("../ebpf/logon_tracker.bpf.o")) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("❌ Failed to load logon BPF program: {:?}", e);
                    return;
                }
            };

            let program: &mut TracePoint = match bpf.program_mut("trace_execve") {
                Some(p) => match p.try_into() {
                    Ok(tp) => tp,
                    Err(e) => {
                        eprintln!("❌ Failed to cast program to TracePoint: {:?}", e);
                        return;
                    }
                },
                None => {
                    eprintln!("❌ trace_execve program not found in object");
                    return;
                }
            };

            if let Err(e) = program.load() {
                eprintln!("❌ Failed to load logon TracePoint: {:?}", e);
                return;
            }
            if let Err(e) = program.attach("syscalls", "sys_enter_execve") {
                eprintln!("❌ Failed to attach logon TracePoint: {:?}", e);
                return;
            }

            println!("🔓 [eBPF] Logon tracker tracepoint attached.");

            let mut perf_array: PerfEventArray<_> = match bpf.map_mut("events") {
                Ok(map) => match PerfEventArray::try_from(map) {
                    Ok(arr) => arr,
                    Err(e) => {
                        eprintln!("❌ Failed to create PerfEventArray: {:?}", e);
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("❌ Failed to access 'events' map: {:?}", e);
                    return;
                }
            };

            for cpu_id in online_cpus().unwrap_or_default() {
                match perf_array.open(cpu_id, None) {
                    Ok(mut buf) => {
                        let writer = Arc::clone(&writer);
                        thread::spawn(move || loop {
                            let mut buffers = vec![BytesMut::with_capacity(PERF_BUF_CAP); PERF_BUFS_PER_CPU];
                            match buf.read_events(&mut buffers) {
                                Ok(stats) => {
                                    if stats.lost > 0 {
                                        eprintln!("⚠️ Lost {} logon events on CPU {}", stats.lost, cpu_id);
                                    }
                                    for slot in buffers.iter().take(stats.read) {
                                        if slot.len() < mem::size_of::<LogonEvent>() {
                                            // short buffer; skip to avoid UB
                                            continue;
                                        }
                                        // Safe due to size check; event is packed in perf buffer
                                        let evt: LogonEvent = unsafe {
                                            std::ptr::read_unaligned(slot.as_ptr() as *const LogonEvent)
                                        };

                                        let comm = String::from_utf8_lossy(&evt.comm)
                                            .trim_end_matches(char::from(0))
                                            .to_string();

                                        // Dedup per PID (bounded)
                                        {
                                            let mut seen = SEEN_PIDS.lock().unwrap();
                                            if seen.contains(&evt.pid) {
                                                continue;
                                            }
                                            if seen.len() >= SEEN_PIDS_MAX {
                                                // simple bound: clear when over limit
                                                seen.clear();
                                            }
                                            seen.insert(evt.pid);
                                        }

                                        REAL_LOGON_FOUND.store(true, Ordering::SeqCst);

                                        // Heuristic risk
                                        let risk_score = if comm.contains("login") || comm.contains("sshd") {
                                            7.0
                                        } else {
                                            3.0
                                        };

                                        let trust = generate_trust_payload("logon_tracker", 0.3, 85_000, risk_score);
                                        let features = generate_feature_vector(0.3, 85_000, risk_score);

                                        let mut gnn_data = HashMap::new();
                                        gnn_data.insert("vector".into(), format!("{:?}", features));
                                        gnn_data.insert("category".into(), "logon".into());
                                        gnn_data.insert("signal".into(), "execve_logon".into());
                                        // clamp to [0,1]; previous formula could go negative
                                        let conf = (1.0 - (risk_score / 10.0)).clamp(0.0, 1.0);
                                        gnn_data.insert("confidence".into(), format!("{:.2}", conf));
                                        gnn_data.insert("gnn_escalate".into(), "true".into());
                                        gnn_data.insert("summary".into(), format!("eBPF execve login: {}", comm));
                                        gnn_data.insert("replay_tag".into(), "logon_event".into());

                                        push_to_gnn_vector_log(gnn_data.clone());
                                        let _ = store_replay_event(gnn_data.clone());

                                        let timestamp = now_ts();

                                        println!(
                                            "[🔓 Logon Event] PID={} PPID={} COMM='{}' | Trust={} Risk={}",
                                            evt.pid,
                                            evt.ppid,
                                            comm,
                                            trust.get("trust_score").unwrap_or(&"?".to_string()),
                                            risk_score
                                        );

                                        let record = TelemetryRecord {
                                            uid: evt.uid,
                                            pid: evt.pid,
                                            ppid: evt.ppid,
                                            binary_path: comm.clone(),
                                            command_line: format!("eBPF login: {}", comm),
                                            cwd: "/".into(),
                                            env_vars: Some(vec![]),
                                            tags: vec!["logon_event".into()],
                                            timestamp,
                                            risk_score: Some(risk_score as u32),
                                            ..Default::default()
                                        };

                                        if let Ok(mut w) = writer.lock() {
                                            w.append(record);
                                        }

                                        submit_trust_event(TrustEvent {
                                            timestamp,
                                            pid: evt.pid,
                                            ppid: evt.ppid,
                                            uid: evt.uid,
                                            binary_path: comm.clone(),
                                            command_line: format!("eBPF login: {}", comm),
                                            cwd: "/".into(),
                                            anomaly_type: "logon_activity".into(),
                                            component: "logon_tracker".into(),
                                            metadata: trust.clone(),
                                            risk_score: risk_score as f32,
                                            source_module: "logon_tracker".into(),
                                            decay_context: Some("ebpf_logon".into()),
                                            module: Some("logon".into()),
                                            signal: Some("logon_event".into()),
                                            signal_type: Some("logon_event".into()),
                                            score: Some(risk_score as f32),
                                            raw_score: Some(risk_score as f32),
                                            tags: Some(vec!["logon_event".into()]),
                                            description: Some(format!("New logon observed via execve: {}", comm)),
                                        });
                                    }
                                }
                                Err(_) => thread::sleep(Duration::from_millis(10)),
                            }
                        });
                    }
                    Err(e) => eprintln!("❌ Failed to open logon perf buffer on CPU {}: {:?}", cpu_id, e),
                }
            }
        });
    });
}

#[cfg(not(target_os = "linux"))]
pub fn start_logon_tracker(_writer: Arc<Mutex<TelemetryWriter>>) {
    // Non-Linux stub: no-op to keep cross-platform builds happy
}

pub fn scan_logon_activity() -> Vec<TelemetryOutput> {
    if REAL_LOGON_FOUND.load(Ordering::Relaxed) {
        return vec![]; // real logon event already emitted
    }

    let timestamp = now_ts();

    let mut metadata = HashMap::new();
    metadata.insert("fallback".into(), "true".into());
    metadata.insert("reason".into(), "No logon events seen in this session".into());

    submit_trust_event(TrustEvent {
        timestamp,
        pid: -1,
        ppid: -1,
        uid: 0,
        binary_path: "/usr/bin/login".to_string(),
        command_line: "no_logon_activity_detected".to_string(),
        cwd: "/".to_string(),
        anomaly_type: "logon_activity".into(),
        component: "logon_tracker".into(),
        metadata,
        risk_score: 0.0,
        source_module: "logon_tracker".into(),
        decay_context: Some("logon_fallback".into()),
        module: Some("logon".into()),
        signal: Some("no_logon_activity_detected".into()),
        signal_type: Some("logon_event_fallback".into()),
        description: Some("No logon activity detected; passive fallback signal".to_string()),
        tags: Some(vec!["logon_event_fallback".into(), "auth".into(), "passive".into()]),
        score: Some(0.0),
        raw_score: Some(0.0),
    });

    let mut placeholder = HashMap::new();
    placeholder.insert("event_type".into(), "logon_tracker".into());
    placeholder.insert("timestamp".into(), timestamp.to_string());
    placeholder.insert("status".into(), "no_logon_activity_detected".into());

    vec![TelemetryOutput {
        category: "auth".into(),
        signal: "no_logon_activity_detected".into(),
        confidence: 0.0,
        data: placeholder,
    }]
}

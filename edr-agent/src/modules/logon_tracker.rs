use std::collections::{HashSet, HashMap};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::mem;

use aya::maps::perf::PerfEventArray;
use aya::util::online_cpus;
use aya::{include_bytes_aligned, Bpf};
use aya::programs::TracePoint;

use bytes::BytesMut;
use lazy_static::lazy_static;
use crate::modules::replay_writer::store_replay_event;
use crate::telemetry::TelemetryRecord;
use crate::trust_hook::{generate_trust_payload, generate_feature_vector, submit_trust_event};
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::telemetry_writer::TelemetryWriter;
use crate::utils::time::now_ts;
use crate::telemetry_types::TelemetryOutput;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

static START_EBPF: Once =Once::new();
lazy_static! {
    static ref REAL_LOGON_FOUND: AtomicBool = AtomicBool::new(false);
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

lazy_static! {
    static ref SEEN_PIDS: Mutex<HashSet<i32>> = Mutex::new(HashSet::new());
}
use crate::trust_hook::TrustEvent;
pub fn start_logon_tracker(writer: Arc<Mutex<TelemetryWriter>>) {
    START_EBPF.call_once(|| {
        thread::spawn(move || {
            let mut bpf = match Bpf::load(include_bytes_aligned!(
                "../ebpf/logon_tracker.bpf.o"
            )) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Failed to load logon BPF program: {:?}", e);
                    return;
                }
            };

            let program: &mut TracePoint = match bpf.program_mut("trace_execve") {
                Some(p) => p.try_into().expect("Failed to cast to TracePoint"),
                None => {
                    eprintln!("Failed to find trace_execve program");
                    return;
                }
            };

            if let Err(e) = program.load() {
                eprintln!("Failed to load logon BPF: {:?}", e);
                return;
            }

            if let Err(e) = program.attach("syscalls", "sys_enter_execve") {
                eprintln!("Failed to attach BPF logon tracepoint: {:?}", e);
                return;
            }

            println!("🧠 [eBPF] Logon tracker tracepoint active.");

            let mut perf_array: PerfEventArray<_> = match bpf.map_mut("events") {
                Ok(map) => match PerfEventArray::try_from(map) {
                    Ok(array) => array,
                    Err(e) => {
                        eprintln!("Failed to convert map to PerfEventArray: {:?}", e);
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("Failed to access logon perf buffer: {:?}", e);
                    return;
                }
            };

            for cpu_id in online_cpus().unwrap_or_default() {
                if let Ok(mut buf) = perf_array.open(cpu_id, None) {
                    let writer = Arc::clone(&writer);
                    thread::spawn(move || loop {
                        let mut buffers = vec![BytesMut::with_capacity(1024); 32];
                        match buf.read_events(&mut buffers) {
                            Ok(events) => {
                                for buf in buffers.iter().take(events.read) {
                                    let mut data = [0u8; mem::size_of::<LogonEvent>()];
                                    data[..buf.len()].copy_from_slice(&buf[..]);
                                    let evt: LogonEvent = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const _) };

                                    let comm = String::from_utf8_lossy(&evt.comm)
                                        .trim_matches(char::from(0))
                                        .to_string();

                                    let mut seen = SEEN_PIDS.lock().unwrap();
                                    if seen.contains(&evt.pid) {
                                        continue;
                                    }
                                    seen.insert(evt.pid);
                                    REAL_LOGON_FOUND.store(true, Ordering::SeqCst);

                                    let risk_score = if comm.contains("login") || comm.contains("sshd") {
                                        7.0
                                    } else {
                                        3.0
                                    };

                                    let trust = generate_trust_payload("logon_tracker", 0.3, 85000, risk_score);
                                    let features = generate_feature_vector(0.3, 85000, risk_score);

                                    let mut gnn_data = HashMap::new();
                                    gnn_data.insert("vector".into(), format!("{:?}", features));
                                    gnn_data.insert("category".into(), "logon".into());
                                    gnn_data.insert("signal".into(), "execve_logon".into());
                                    gnn_data.insert("confidence".into(), format!("{:.2}", 1.0 - risk_score));
                                    gnn_data.insert("gnn_escalate".into(), "true".into());
                                    gnn_data.insert("summary".into(), format!("eBPF execve login: {}", comm));
                                    gnn_data.insert("replay_tag".into(), "logon_event".into());

                                    push_to_gnn_vector_log(gnn_data.clone());
                                    store_replay_event(gnn_data.clone());

                                    let timestamp = now_ts();

                                    println!(
                                        "[🔓 Logon Event] PID={} CMD={} Trust={} Risk={}",
                                        evt.pid,
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
                                        w.append(record.clone());
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
            }
        });
    });
}
pub fn scan_logon_activity() -> Vec<TelemetryOutput> {
    if REAL_LOGON_FOUND.load(Ordering::Relaxed) {
        return vec![]; // ✅ Real logon event already reported
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

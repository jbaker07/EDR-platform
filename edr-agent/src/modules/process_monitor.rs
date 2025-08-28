use std::collections::HashMap;

use sysinfo::{System, SystemExt, ProcessExt, PidExt};

use crate::modules::user_tracker::{get_logged_in_users, UserSession};
use crate::telemetry_writer::{push_memory_telemetry, write_telemetry_record};
use crate::telemetry_types::{MemoryAnomalyType, TelemetryOutput};
use crate::trust_hook::{submit_trust_event, TrustEvent};
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::utils::time::now_ts;

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: i32,
    pub ppid: i32,
    pub name: String,
    pub exe: String,
    pub cpu_usage: f32,
    pub memory_usage_kb: u64,
    pub status: String,
    pub user: Option<String>,
    pub user_sessions: Option<Vec<UserSession>>,
    pub command_line: String,
}

/// Gathers all running processes and returns their metadata.
pub fn gather_processes() -> Vec<ProcessInfo> {
    let mut system = System::new_all();
    system.refresh_processes();

    let sessions = get_logged_in_users();
    let mut process_list = Vec::new();

    for (pid, process) in system.processes() {
        let pid_i32 = pid.as_u32() as i32;
        let ppid_i32 = process.parent().map_or(0, |ppid| ppid.as_u32() as i32);
        let user_id = process.user_id().map(|uid| uid.to_string());
        let command_line = process.cmd().join(" ");

        let info = ProcessInfo {
            pid: pid_i32,
            ppid: ppid_i32,
            name: process.name().to_string(),
            exe: process.exe().to_string_lossy().to_string(),
            cpu_usage: process.cpu_usage(),
            // Keep naming consistent (KB). If sysinfo already returns KB, the /1024 is harmlessly small.
            memory_usage_kb: process.memory() / 1024,
            status: format!("{:?}", process.status()),
            user: user_id,
            user_sessions: Some(sessions.clone()),
            command_line,
        };

        process_list.push(info);
    }

    process_list
}

pub fn scan_processes() -> Vec<TelemetryOutput> {
    let processes = gather_processes();
    let mut outputs = Vec::new();

    for proc in processes {
        let mut data = HashMap::new();
        data.insert("pid".into(), proc.pid.to_string());
        data.insert("ppid".into(), proc.ppid.to_string());
        data.insert("name".into(), proc.name.clone());
        data.insert("exe".into(), proc.exe.clone());
        data.insert("cpu_usage".into(), proc.cpu_usage.to_string());
        data.insert("memory_usage_kb".into(), proc.memory_usage_kb.to_string());
        data.insert("status".into(), proc.status.clone());
        data.insert("command_line".into(), proc.command_line.clone());
        data.insert("replay_tag".into(), "process_snapshot".into());
        data.insert("gnn_escalate".into(), "false".into()); // snapshots are low-severity by default
        data.insert("timestamp".into(), now_ts().to_string());

        if let Some(user) = &proc.user {
            data.insert("user".into(), user.clone());
        }
        if let Some(sessions) = &proc.user_sessions {
            data.insert("user_sessions_count".into(), sessions.len().to_string());
        }

        let uid_num = proc
            .user
            .as_ref()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);

        let tags = vec!["snapshot".into(), format!("binary::{}", proc.name)];

        let event = TrustEvent {
            timestamp: now_ts(),
            pid: proc.pid,
            ppid: proc.ppid,
            uid: uid_num,
            binary_path: proc.exe.clone(),
            command_line: proc.command_line.clone(),
            cwd: "/proc".into(),
            anomaly_type: "None".into(),
            component: "process".into(),
            metadata: data.clone(),
            risk_score: 3.0,
            source_module: "process_monitor".into(),
            decay_context: Some("process_snapshot".into()),
            module: Some("process_monitor".into()),
            signal: Some("process_snapshot".into()),
            signal_type: Some("process_snapshot".into()),
            description: Some("Process snapshot gathered".into()),
            score: Some(3.0),
            raw_score: Some(3.0),
            tags: Some(tags),
        };

        submit_trust_event(event);
        push_to_gnn_vector_log(data.clone());
        write_telemetry_record(data.clone());
        crate::modules::replay_writer::store_replay_event(data.clone());

        outputs.push(TelemetryOutput {
            category: "process".into(),
            signal: "process_snapshot".into(),
            confidence: 0.60, // normalize to 0..1 range for consistency
            data,
        });
    }

    outputs
}

#[cfg(target_os = "linux")]
use aya::{Bpf, include_bytes_aligned};
#[cfg(target_os = "linux")]
use aya::programs::TracePoint;
#[cfg(target_os = "linux")]
use aya::maps::perf::PerfEventArray;
#[cfg(target_os = "linux")]
use aya::util::online_cpus;
#[cfg(target_os = "linux")]
use bytes::BytesMut;
#[cfg(target_os = "linux")]
use std::{thread, mem, time::Duration};
#[cfg(target_os = "linux")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(target_os = "linux")]
use lazy_static::lazy_static;

#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Debug, Clone)]
struct IpcAbuseEvent {
    pid: u32,
    target_pid: u32,
    syscall_id: u32,
    channel_type: u8,
    timestamp: u64,
}

#[cfg(target_os = "linux")]
lazy_static! {
    pub static ref IPC_ABUSE_DETECTED: AtomicBool = AtomicBool::new(false);
}

#[cfg(target_os = "linux")]
pub fn start_ipc_abuse_monitor() {
    thread::spawn(move || {
        // Load then leak the BPF so perf buffers can live 'static inside spawned threads
        let mut tmp = match Bpf::load(include_bytes_aligned!(
            "../ebpf/ipc_abuse_monitor.bpf.o"
        )) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("❌ Failed to load ipc_abuse bpf: {:?}", e);
                return;
            }
        };
        let bpf: &'static mut Bpf = Box::leak(Box::new(tmp));

        let program = match bpf.program_mut("trace_ipc_abuse") {
            Some(p) => p,
            None => {
                eprintln!("❌ Missing program: trace_ipc_abuse");
                return;
            }
        };

        let tp: &mut TracePoint = match program.try_into() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("❌ Could not cast ipc program: {:?}", e);
                return;
            }
        };

        if let Err(e) = tp.load() {
            eprintln!("❌ Failed to load ipc TP: {:?}", e);
            return;
        }

        if let Err(e) = tp.attach("syscalls", "sys_enter_msgsnd") {
            eprintln!("❌ Failed to attach ipc TP: {:?}", e);
            return;
        }

        println!("🧠 [eBPF] IPC abuse monitor tracepoint active.");

        // map_mut returns Option; handle missing map cleanly
        let map = match bpf.map_mut("EVENTS") {
            Some(m) => m,
            None => {
                eprintln!("❌ Could not locate perf map: EVENTS");
                return;
            }
        };

        // Convert to a PerfEventArray
        let mut perf_array: PerfEventArray<_> = match PerfEventArray::try_from(map) {
            Ok(arr) => arr,
            Err(e) => {
                eprintln!("❌ Could not convert perf map: {:?}", e);
                return;
            }
        };

        // CPU list (handle kernel fetch errors)
        let cpus = online_cpus().unwrap_or_else(|(m, e)| {
            eprintln!("⚠️ online_cpus failed: {}: {:?}", m, e);
            Vec::new()
        });

        for cpu_id in cpus {
            match perf_array.open(cpu_id, None) {
                Ok(mut buf) => {
                    thread::spawn(move || {
                        let mut buffers = vec![BytesMut::with_capacity(1024); 32];
                        loop {
                            match buf.read_events(&mut buffers) {
                                Ok(events) => {
                                    for b in &buffers[..events.read] {
                                        if b.len() < mem::size_of::<IpcAbuseEvent>() {
                                            continue;
                                        }

                                        let ptr = b.as_ptr() as *const IpcAbuseEvent;
                                        let evt = unsafe { ptr.read_unaligned() };

                                        IPC_ABUSE_DETECTED.store(true, Ordering::Relaxed);

                                        let ts = now_ts();
                                        let desc = format!(
                                            "IPC abuse: pid={} → target_pid={} (syscall={} chan_type={})",
                                            evt.pid, evt.target_pid, evt.syscall_id, evt.channel_type
                                        );

                                        let mut meta = HashMap::new();
                                        meta.insert("timestamp".into(), ts.to_string());
                                        meta.insert("pid".into(), evt.pid.to_string());
                                        meta.insert("target_pid".into(), evt.target_pid.to_string());
                                        meta.insert("syscall_id".into(), evt.syscall_id.to_string());
                                        meta.insert("channel_type".into(), evt.channel_type.to_string());
                                        meta.insert("anomaly".into(), "ipc_abuse".into());
                                        meta.insert("replay_tag".into(), "ipc_abuse".into());
                                        meta.insert("gnn_escalate".into(), "true".into());
                                        meta.insert("soc_note".into(), "eBPF IPC abuse anomaly".into());

                                        let trust_event = TrustEvent {
                                            timestamp: ts,
                                            pid: evt.pid as i32,
                                            ppid: evt.target_pid as i32,
                                            uid: 0,
                                            binary_path: "unknown".into(),
                                            command_line: "unknown".into(),
                                            cwd: "unknown".into(),
                                            anomaly_type: "ipc_abuse".into(),
                                            component: "ipc_abuse_monitor".into(),
                                            metadata: meta.clone(),
                                            risk_score: 22.0,
                                            source_module: "ipc_abuse_monitor".into(),
                                            decay_context: Some("channel_misuse".into()),
                                            module: Some("ipc_abuse_monitor".into()),
                                            signal: Some("ebpf_ipc_anomaly".into()),
                                            signal_type: Some("ebpf".into()),
                                            score: Some(22.0),
                                            raw_score: Some(22.0),
                                            tags: Some(vec!["ipc".into(), "anomaly".into(), "ebpf".into()]),
                                            description: Some(desc.clone()),
                                        };

                                        submit_trust_event(trust_event);
                                        push_to_gnn_vector_log(meta.clone());
                                        write_telemetry_record(meta.clone());
                                        crate::modules::replay_writer::store_replay_event(meta.clone());

                                        let _ = push_memory_telemetry(
                                            evt.pid as i32,
                                            evt.target_pid as i32,
                                            0,
                                            "unknown".into(),
                                            "unknown".into(),
                                            "unknown".into(),
                                            MemoryAnomalyType::IPCAbuse,
                                            desc,
                                        ).map_err(|e| eprintln!("⚠️ IPC telemetry failed: {:?}", e));
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed reading IPC event: {:?}", e);
                                    thread::sleep(Duration::from_millis(100));
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
    });
}

#[cfg(target_os = "linux")]
pub fn scan_ipc_abuse_activity() -> Vec<TelemetryOutput> {
    // Fallback is only emitted if no real-time event was observed.
    if IPC_ABUSE_DETECTED.load(Ordering::Relaxed) {
        return vec![];
    }

    let ts = now_ts();
    let mut data = HashMap::new();
    data.insert("timestamp".into(), ts.to_string());
    data.insert("replay_tag".into(), "ipc_abuse".into());
    data.insert("signal".into(), "ipc_abuse_fallback".into());
    data.insert("fallback".into(), "true".into());
    data.insert("method".into(), "no eBPF event observed".into());
    data.insert("note".into(), "Fallback IPC abuse scan triggered".into());
    data.insert("gnn_escalate".into(), "true".into());
    data.insert("soc_note".into(), "Fallback IPC abuse detection used as safety net".into());

    let trust_event = TrustEvent {
        timestamp: ts,
        pid: 0,
        ppid: 0,
        uid: 0,
        binary_path: "unknown".into(),
        command_line: "unknown".into(),
        cwd: "unknown".into(),
        anomaly_type: "ipc_abuse".into(),
        component: "ipc_abuse_monitor".into(),
        metadata: data.clone(),
        risk_score: 17.0,
        source_module: "ipc_abuse_monitor".into(),
        decay_context: Some("fallback_scan".into()),
        module: Some("ipc_abuse_monitor".into()),
        signal: Some("ipc_abuse_fallback".into()),
        signal_type: Some("process".into()),
        score: Some(17.0),
        raw_score: Some(17.0),
        tags: Some(vec!["ipc".into(), "fallback".into(), "anomaly".into()]),
        description: Some("Fallback IPC abuse detection triggered; no eBPF event seen.".into()),
    };

    submit_trust_event(trust_event);
    push_to_gnn_vector_log(data.clone());
    write_telemetry_record(data.clone());
    crate::modules::replay_writer::store_replay_event(data.clone());

    vec![TelemetryOutput {
        category: "process".into(),
        signal: "ipc_abuse_fallback".into(),
        confidence: 0.85,
        data,
    }]
}

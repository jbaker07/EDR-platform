use aya::maps::perf::AsyncPerfEventArray;
use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Ebpf};
use aya::util::online_cpus;
use bytes::BytesMut;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::{task, time};
use anyhow::{Result, anyhow, Context};

use crate::telemetry_writer::{push_memory_telemetry, write_telemetry_record};
use crate::telemetry_types::{MemoryAnomalyType, TelemetryOutput};
use crate::trust_hook::{generate_feature_vector, submit_trust_event, TrustEvent};
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::utils::time::now_ts;
use crate::modules::replay_writer::store_replay_event;

use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

lazy_static! {
    static ref INJECTION_FOUND: AtomicBool = AtomicBool::new(false);
}

#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Clone, Debug)]
pub struct InjectionEvent {
    pub pid: i32,
    pub timestamp: u64,
    pub summary: [u8; 256],
}

#[cfg(target_os = "linux")]
pub async fn start_process_injection_monitor() -> Result<()> {
    // Load and leak BPF so perf buffers (spawned tasks) can outlive this function.
    let tmp = Ebpf::load(include_bytes_aligned!(
        "../ebpf/process_injection.bpf.o"
    ))?;
    let bpf: &'static mut Ebpf = Box::leak(Box::new(tmp));

    // Attach tracepoint program
    let program: &mut TracePoint = bpf
        .program_mut("trace_inject_evt")
        .ok_or_else(|| anyhow!("Missing tracepoint program: trace_inject_evt"))?
        .try_into()?;

    program.load()?;
    program.attach("syscalls", "sys_enter_ptrace")?;

    // Open perf array map
    let mut events = AsyncPerfEventArray::try_from(
        bpf.map_mut("EVENTS")
            .ok_or_else(|| anyhow!("EVENTS map not found"))?,
    )?;

    // Iterate online CPUs and spawn readers
    let cpus = online_cpus()
        .map_err(|(m, e)| anyhow!("online_cpus failed: {m}: {e}"))?;
    for cpu_id in cpus {
        let mut buf = events
            .open(cpu_id, None)
            .with_context(|| format!("open perf buffer on CPU {}", cpu_id))?;

        task::spawn(async move {
            let mut buffers = vec![BytesMut::with_capacity(4096); 16];
            println!("[🧠 InjectionMonitor] Listening on CPU {}", cpu_id);

            loop {
                match buf.read_events(&mut buffers).await {
                    Ok(ev) => {
                        for b in &mut buffers[..ev.read] {
                            if !b.is_empty() {
                                if let Some(evt) = parse_injection_event(&b[..]) {
                                    if let Err(e) = handle_injection_event(evt).await {
                                        eprintln!("[❌ InjectionMonitor] handler error: {e:?}");
                                    }
                                }
                            }
                            b.clear();
                        }
                        if ev.lost > 0 {
                            eprintln!(
                                "[⚠️ InjectionMonitor] Lost {} events on CPU {}",
                                ev.lost, cpu_id
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "[❌ InjectionMonitor] Error reading CPU {}: {:?}",
                            cpu_id, e
                        );
                        time::sleep(Duration::from_secs(2)).await;
                    }
                }
            }
        });
    }

    Ok(())
}

pub async fn handle_injection_event(evt: InjectionEvent) -> Result<()> {
    INJECTION_FOUND.store(true, Ordering::SeqCst);

    let ts = now_ts();
    let risk_score = 88.0_f32;

    // Convert summary from [u8; 256] to String
    let summary = String::from_utf8_lossy(&evt.summary)
        .trim_matches(char::from(0))
        .to_string();

    // Defaults
    let mut ppid = 0;
    let mut uid = 0;
    let mut binary_path = "/proc/[pid]/exe".into();
    let mut command_line = "[unknown]".into();
    let mut cwd = "/".into();

    let mut sys = System::new_all();
    sys.refresh_processes();

    if let Some(proc) = sys.process(Pid::from(evt.pid as usize)) {
        ppid = proc.parent().map(|p| p.as_u32() as i32).unwrap_or(0);
        uid = proc
            .user_id()
            .map(|u| u.to_string().parse::<u32>().unwrap_or(0))
            .unwrap_or(0);
        binary_path = proc.exe().display().to_string();
        let cmd_vec = proc.cmd();
        if !cmd_vec.is_empty() {
            command_line = cmd_vec.join(" ");
        }
        cwd = proc.cwd().display().to_string();
    }

    // Optional feature vector for downstream models
    let features = generate_feature_vector(0.6_f64, 120_000_u64, risk_score as f64);

    let mut metadata = HashMap::new();
    metadata.insert("pid".into(), evt.pid.to_string());
    metadata.insert("ppid".into(), ppid.to_string());
    metadata.insert("uid".into(), uid.to_string());
    metadata.insert("binary_path".into(), binary_path.clone());
    metadata.insert("command_line".into(), command_line.clone());
    metadata.insert("cwd".into(), cwd.clone());
    metadata.insert("summary".into(), summary.clone());
    metadata.insert("timestamp".into(), ts.to_string());
    metadata.insert("features".into(), format!("{:?}", features));
    metadata.insert("replay_tag".into(), "process_injection".into());
    metadata.insert("soc_note".into(), "Process injection via ptrace syscall".into());
    metadata.insert("gnn_escalate".into(), "true".into());

    // Trust Event
    let trust_event = TrustEvent {
        timestamp: ts,
        pid: evt.pid,
        ppid,
        uid,
        binary_path: binary_path.clone(),
        command_line: command_line.clone(),
        cwd: cwd.clone(),
        anomaly_type: "process_injection".into(),
        component: "injection_monitor".into(),
        metadata: metadata.clone(),
        risk_score,
        source_module: "process_injection_monitor".into(),
        decay_context: Some("ebpf_injection".into()),
        module: Some("process".into()),
        signal: Some("ptrace_injection".into()),
        signal_type: Some("ebpf".into()),
        score: Some(risk_score),
        raw_score: Some(risk_score),
        tags: Some(vec![
            "process_injection".into(),
            "memory_forensics".into(),
            "ebpf".into(),
        ]),
        description: Some(summary.clone()),
    };

    submit_trust_event(trust_event);

    // Telemetry + GNN + Replay
    write_telemetry_record(metadata.clone());

    let mut gnn_data = metadata.clone();
    gnn_data.insert("category".into(), "memory".into());
    gnn_data.insert("signal".into(), "ptrace_injection".into());
    gnn_data.insert("confidence".into(), "0.88".into());
    push_to_gnn_vector_log(gnn_data.clone());
    store_replay_event(gnn_data);

    // Memory anomaly (closest existing enum)
    let _ = push_memory_telemetry(
        evt.pid,
        ppid,
        uid,
        &binary_path,
        &command_line,
        &cwd,
        MemoryAnomalyType::ProcHollowing,
        format!("ptrace-based injection suspicion: {}", summary),
    )
    .map_err(|e| eprintln!("⚠️ mem telemetry error (injection): {:?}", e));

    println!(
        "[⚠️ InjectionMonitor] Process injection attempt detected on PID {} → Risk {:.1}",
        evt.pid, risk_score
    );

    Ok(())
}

pub fn scan_injection_fallback() -> Vec<TelemetryOutput> {
    use std::path::PathBuf;

    if INJECTION_FOUND.load(Ordering::SeqCst) {
        return vec![];
    }

    let mut suspicious_found = false;
    let mut pid = 0;
    let mut ppid = 0;
    let mut binary_path = "[unknown]".to_string();
    let mut command_line = "[fallback_scan]".to_string();
    let mut cwd = "/".to_string();

    let entries = match fs::read_dir("/proc") {
        Ok(e) => e,
        Err(_) => return vec![],
    };

    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(pid_num) = entry.file_name().to_string_lossy().parse::<i32>() {
                let maps_path = format!("/proc/{}/maps", pid_num);
                if let Ok(contents) = fs::read_to_string(&maps_path) {
                    for line in contents.lines() {
                        if line.contains("rwx") || (line.contains('x') && !line.contains('/')) {
                            suspicious_found = true;
                            pid = pid_num;

                            // Binary path
                            let exe_path = PathBuf::from(format!("/proc/{}/exe", pid));
                            if let Ok(real) = fs::read_link(&exe_path) {
                                binary_path = real.display().to_string();
                            }

                            // Command line
                            let cmd_path = format!("/proc/{}/cmdline", pid);
                            if let Ok(data) = fs::read(&cmd_path) {
                                if !data.is_empty() {
                                    command_line = String::from_utf8_lossy(&data)
                                        .replace('\0', " ")
                                        .trim()
                                        .to_string();
                                }
                            }

                            // CWD
                            let cwd_path = PathBuf::from(format!("/proc/{}/cwd", pid));
                            if let Ok(real) = fs::read_link(&cwd_path) {
                                cwd = real.display().to_string();
                            }

                            // PPID
                            let stat_path = format!("/proc/{}/stat", pid);
                            if let Ok(stat) = fs::read_to_string(&stat_path) {
                                let fields: Vec<&str> = stat.split_whitespace().collect();
                                if fields.len() > 3 {
                                    ppid = fields[3].parse().unwrap_or(0);
                                }
                            }

                            break;
                        }
                    }
                }
            }
        }

        if suspicious_found {
            break;
        }
    }

    if !suspicious_found {
        return vec![];
    }

    let ts = now_ts();
    let mut data = HashMap::new();
    data.insert("timestamp".into(), ts.to_string());
    data.insert("signal".into(), "process_injection_fallback".into());
    data.insert("replay_tag".into(), "process_injection".into());
    data.insert("soc_note".into(), "Fallback scan detected possible injection".into());
    data.insert("gnn_escalate".into(), "true".into());
    data.insert("pid".into(), pid.to_string());
    data.insert("ppid".into(), ppid.to_string());
    data.insert("binary_path".into(), binary_path.clone());
    data.insert("command_line".into(), command_line.clone());
    data.insert("cwd".into(), cwd.clone());

    let trust_event = TrustEvent {
        timestamp: ts,
        pid,
        ppid,
        uid: 0,
        binary_path,
        command_line,
        cwd,
        anomaly_type: "process_injection".into(),
        component: "process_injection".into(),
        metadata: data.clone(),
        risk_score: 18.0,
        source_module: "process_injection_monitor".into(),
        decay_context: Some("injection_fallback".into()),
        module: Some("memory".into()),
        signal: Some("injection_scan_fallback".into()),
        signal_type: Some("passive".into()),
        score: Some(18.0),
        raw_score: Some(18.0),
        tags: Some(vec![
            "process_injection".into(),
            "fallback_scan".into(),
            "ebpf_missing".into(),
        ]),
        description: Some("Passive scan triggered possible process injection detection.".into()),
    };

    submit_trust_event(trust_event);
    write_telemetry_record(data.clone());
    push_to_gnn_vector_log(data.clone());
    store_replay_event(data.clone());

    vec![TelemetryOutput {
        category: "memory".into(),
        signal: "injection_scan_fallback".into(),
        confidence: 0.3,
        data,
    }]
}

#[cfg(target_os = "linux")]
fn parse_injection_event(buf: &[u8]) -> Option<InjectionEvent> {
    use std::ptr::read_unaligned;
    if buf.len() < std::mem::size_of::<InjectionEvent>() {
        return None;
    }
    let ptr = buf.as_ptr() as *const InjectionEvent;
    Some(unsafe { read_unaligned(ptr) })
}

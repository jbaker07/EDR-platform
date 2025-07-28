use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek};
use std::path::Path;
use chrono::Utc;
use tokio::task;
use aya::{Bpf, include_bytes_aligned};
use aya::programs::TracePoint;
use aya::util::online_cpus;
use aya::maps::perf::AsyncPerfEventArray;
use futures::StreamExt;
use anyhow::{Result, anyhow};
use std::{mem, ptr};
use tokio_stream::wrappers::ReceiverStream;
use bytes::BytesMut;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::trust_hook::{TrustEvent, submit_trust_event};
use crate::utils::time::now_ts;
use crate::telemetry::mark_memory_anomaly_detected;
use crate::telemetry_writer::{push_memory_telemetry, TelemetryWriter};
use crate::logger::log;
use crate::telemetry_types::{TelemetryEvent, MemoryAnomalyType, TelemetryOutput};
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::replay_writer::store_replay_event;


lazy_static::lazy_static! {
    static ref MEMORY_ANOMALY_FOUND: AtomicBool = AtomicBool::new(false);
}

#[repr(C)]
#[derive(Debug, Clone)]
struct MemEvent {
    pid: u32,
    uid: u32,
    ppid: u32,
    comm: [u8; 64],
    details: [u8; 128],
}


#[cfg(target_os = "linux")]
pub async fn start_ebpf_mem_scan() -> Result<()> {
    let mut bpf = Bpf::load(include_bytes_aligned!("../ebpf/mem_scan.bpf.o"))?;

    let mmap_prog: &mut TracePoint = bpf
        .program_mut("trace_mmap")
        .ok_or_else(|| anyhow!("Missing program: trace_mmap"))?
        .try_into()?;
    mmap_prog.load()?;
    mmap_prog.attach("syscalls", "sys_enter_mmap")?;

    let execve_prog: &mut TracePoint = bpf
        .program_mut("trace_execve_mem")
        .ok_or_else(|| anyhow!("Missing program: trace_execve_mem"))?
        .try_into()?;
    execve_prog.load()?;
    execve_prog.attach("syscalls", "sys_enter_execve")?;

    let mut perf_array = AsyncPerfEventArray::try_from(bpf.map_mut("events")?)?;

    for cpu_id in online_cpus()? {
        let mut buf = perf_array.open(cpu_id, None)?;
        task::spawn(async move {
            let mut event_buf = Vec::new();

            loop {
                match buf.read_events(&mut event_buf).await {
                    Ok(events) if events.read > 0 => {
                        for i in 0..events.read {
                            let ptr = event_buf[i].as_ptr() as *const MemEvent;
                            let event = unsafe { ptr.read_unaligned() };

                            let comm = String::from_utf8_lossy(&event.comm)
                                .trim_end_matches('\0')
                                .to_string();
                            let desc = String::from_utf8_lossy(&event.details)
                                .trim_end_matches('\0')
                                .to_string();

                            // Push TelemetryOutput to memory channel
                            let _ = push_memory_telemetry(
                                event.pid as i32,
                                event.ppid as i32,
                                event.uid,
                                &comm,
                                "ebpf_mem_trace".into(),
                                "/proc".into(),
                                MemoryAnomalyType::RWXMapping,
                                desc.clone(),
                            )
                            .map_err(|e| eprintln!("Telemetry push failed: {:?}", e));

                            // Submit structured TrustEvent
                            let trust_event = TrustEvent::new_full(
                                now_ts(),
                                event.pid as i32,
                                event.ppid as i32,
                                event.uid,
                                comm.clone(),
                                "ebpf memory trace".into(),
                                "/proc".into(),
                                "RWXMapping".into(),
                                "memory".into(),
                                "mem_scan".into(),
                                Some(desc.clone()),
                                Some("memory::ebpf_mem_trace".into()),
                                Some(vec!["rwx_mapping".into(), "ebpf_memory".into()]),
                                Some(6.0),
                            );
                            submit_trust_event(trust_event);

                            // Push to GNN vector log
                            let mut gnn_data = HashMap::new();
                            gnn_data.insert(
                                "vector".into(),
                                format!(
                                    "{{\"pid\":{},\"ppid\":{},\"uid\":{},\"desc\":\"{}\"}}",
                                    event.pid, event.ppid, event.uid, desc
                                ),
                            );
                            gnn_data.insert("category".into(), "memory".into());
                            gnn_data.insert("signal".into(), "ebpf_mem_trace".into());
                            gnn_data.insert("confidence".into(), "0.45".into());
                            gnn_data.insert("gnn_escalate".into(), "true".into());
                            gnn_data.insert("summary".into(), format!("RWX mapping via mmap/execve: {}", comm));
                            gnn_data.insert("replay_tag".into(), "mem_rwx".into());

                            push_to_gnn_vector_log(gnn_data.clone());
                            store_replay_event(gnn_data.clone());

                            // Final structured telemetry record
                            let mut map = HashMap::new();
                            map.insert("binary".into(), comm.clone());
                            map.insert("desc".into(), desc.clone());

                            let record = TelemetryWriter::build(
                                map,
                                Some(6),
                                vec!["memory::ebpf".into()],
                            );
                            TelemetryWriter::new().send(record);

                            mark_memory_anomaly_detected();
                            log(&format!(
                                "[📦 Memory eBPF] PID={} UID={} DESC={}",
                                event.pid, event.uid, desc
                            ));
                        }
                    }
                    Ok(_) => continue,
                    Err(e) => {
                        eprintln!("Error reading memory event: {:?}", e);
                        break;
                    }
                }
            }
        });
    }

    Ok(())
}




fn detect_memory_maps(pid: u32) -> Vec<TelemetryEvent> {
    let mut events = Vec::new();
    let maps_path = format!("/proc/{}/maps", pid);

    if !Path::new(&maps_path).exists() {
        return events;
    }

    if let Ok(file) = File::open(&maps_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            let ts = Utc::now().timestamp();
            let desc = line.clone();

            for (atype, pat) in &[
                (MemoryAnomalyType::RWXMapping, " rwx"),
                (MemoryAnomalyType::AnonymousExec, "[anon]"),
                (MemoryAnomalyType::NullBaseExec, "00000000"),
            ] {
                if line.contains(pat) && line.contains("r-x") {
                    events.push(TelemetryEvent::MemoryAnomaly {
                        pid,
                        ppid: 0,
                        uid: 0,
                        binary_path: "unknown".into(),
                        command_line: "unknown".into(),
                        description: format!("{}: {}", pat, desc),
                        anomaly_type: atype.clone(),
                        timestamp: ts,
                    });
                }
            }
        }
    }

    events
}

fn detect_dirty_rss(pid: u32) -> Vec<TelemetryEvent> {
    let smaps_path = format!("/proc/{}/smaps", pid);
    let mut dirty_kb = 0;

    if !Path::new(&smaps_path).exists() {
        return vec![];
    }

    if let Ok(file) = File::open(&smaps_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if line.starts_with("Private_Dirty:") {
                if let Some(kb) = line.split_whitespace().nth(1) {
                    if let Ok(val) = kb.parse::<u32>() {
                        dirty_kb += val;
                    }
                }
            }
        }
    }

    if dirty_kb > 100_000 {
        vec![TelemetryEvent::MemoryAnomaly {
            pid,
            ppid: 0,
            uid: 0,
            binary_path: "unknown".into(),
            command_line: "unknown".into(),
            description: format!("High Private_Dirty RSS: {} KB", dirty_kb),
            anomaly_type: MemoryAnomalyType::HighDirtyRSS,
            timestamp: Utc::now().timestamp(),
        }]
    } else {
        vec![]
    }
}


pub fn detect_high_entropy(pid: u32) -> Vec<TelemetryEvent> {
    let mut events = Vec::new();
    let maps_path = format!("/proc/{}/maps", pid);
    let mem_path = format!("/proc/{}/mem", pid);

    if !(Path::new(&maps_path).exists() && Path::new(&mem_path).exists()) {
        return events;
    }

    let mem_file = match OpenOptions::new().read(true).open(&mem_path) {
        Ok(f) => f,
        Err(_) => return events,
    };

    let file = match File::open(&maps_path) {
        Ok(f) => f,
        Err(_) => return events,
    };

    let reader = BufReader::new(file);
    let mut mem_file = mem_file;

    for line in reader.lines().flatten() {
        if let Some(range) = line.split_whitespace().next() {
            let mut parts = range.split('-');
            if let (Some(start), Some(end)) = (parts.next(), parts.next()) {
                if let (Ok(start_addr), Ok(end_addr)) =
                    (usize::from_str_radix(start, 16), usize::from_str_radix(end, 16))
                {
                    let mut buffer = vec![0u8; 256]; // Larger region to detect better entropy
                    if mem_file
                        .seek(std::io::SeekFrom::Start(start_addr as u64))
                        .is_ok()
                        && mem_file.read_exact(&mut buffer).is_ok()
                    {
                        // Calculate byte frequency
                        let mut counts = [0usize; 256];
                        for byte in &buffer {
                            counts[*byte as usize] += 1;
                        }

                        let total = buffer.len() as f64;
                        let entropy = counts
                            .iter()
                            .filter(|&&c| c > 0)
                            .map(|&c| {
                                let p = c as f64 / total;
                                -p * p.log2()
                            })
                            .sum::<f64>();

                        if entropy > 7.5 {
                            events.push(TelemetryEvent::MemoryAnomaly {
                                pid,
                                ppid: 0,
                                uid: 0,
                                binary_path: "unknown".into(),
                                command_line: "unknown".into(),
                                description: format!(
                                    "High entropy memory region ({} - {}): {:.2} bits/byte",
                                    start, end, entropy
                                ),
                                anomaly_type: MemoryAnomalyType::HighEntropy,
                                timestamp: Utc::now().timestamp(),
                            });
                        }
                    }
                }
            }
        }
    }

    events
}
pub fn monitor_memory_usage(pid: u32) -> Vec<TelemetryEvent> {
    let mut all = Vec::new();
    all.extend(detect_memory_maps(pid));
    all.extend(detect_dirty_rss(pid));
    all.extend(detect_high_entropy(pid));
    all
}

fn get_binary_path(pid: i32) -> Option<String> {
    std::fs::read_link(format!("/proc/{}/exe", pid)).ok().map(|p| p.to_string_lossy().to_string())
}

fn get_cmdline(pid: i32) -> Option<String> {
    std::fs::read_to_string(format!("/proc/{}/cmdline", pid)).ok().map(|s| s.replace('\0', " "))
}

fn get_cwd(pid: i32) -> Option<String> {
    std::fs::read_link(format!("/proc/{}/cwd", pid)).ok().map(|p| p.to_string_lossy().to_string())
}

pub async fn push_memory_anomalies(pid: u32) {
    for evt in monitor_memory_usage(pid) {
        if let TelemetryEvent::MemoryAnomaly {
            pid,
            anomaly_type,
            description,
            timestamp,
            ..
        } = evt
        {
            let ts = if timestamp == 0 {
                now_ts()
            } else {
                timestamp.try_into().unwrap_or_else(|_| now_ts())
            };

            // Resolve context (safe fallbacks)
            let binary_path = get_binary_path(pid as i32).unwrap_or_else(|| "unknown".into());
            let command_line = get_cmdline(pid as i32).unwrap_or_else(|| "unknown".into());
            let cwd = get_cwd(pid as i32).unwrap_or_else(|| "unknown".into());

            let _ = push_memory_telemetry(
                pid as i32,
                0,
                0,
                &binary_path,
                &command_line,
                &cwd,
                anomaly_type.clone(),
                description.clone(),
            )
            .map_err(|e| eprintln!("Telemetry push failed: {:?}", e));

            let mut metadata = HashMap::new();
            metadata.insert("anomaly_type".into(), anomaly_type.to_string());
            metadata.insert("desc".into(), description.clone());

            let trust_event = TrustEvent {
                timestamp: ts,
                pid: pid as i32,
                ppid: 0,
                uid: 0,
                binary_path: binary_path.clone(),
                command_line: command_line.clone(),
                cwd: cwd.clone(),
                anomaly_type: anomaly_type.to_string(),
                component: "memory".into(),
                metadata,
                risk_score: 7.0,
                source_module: "mem_scan".into(),
                decay_context: Some("memory_behavior".into()),
                module: Some("mem_scan".into()),
                signal: Some(anomaly_type.to_string()),
                signal_type: Some("memory::anomaly".into()),
                score: Some(7.0),
                raw_score: Some(7.0),
                tags: Some(vec!["memory".into(), anomaly_type.to_string()]),
                description: Some(description.clone()),
            };

            submit_trust_event(trust_event);

            let mut gnn_data = HashMap::new();
            gnn_data.insert("vector".into(), format!("{{\"pid\":{}}}", pid));
            gnn_data.insert("category".into(), "memory".into());
            gnn_data.insert("signal".into(), anomaly_type.to_string());
            gnn_data.insert("confidence".into(), "0.45".into()); // Tunable
            gnn_data.insert("gnn_escalate".into(), "true".into());
            gnn_data.insert("summary".into(), description.clone());
            gnn_data.insert("replay_tag".into(), "memory_anomaly".into());

            push_to_gnn_vector_log(gnn_data.clone());
            store_replay_event(gnn_data);

            mark_memory_anomaly_detected();
        }
    }
}



#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Debug, Clone)]
struct HollowEvent {
    pid: u32,
    target_pid: u32,
    flags: u32,
    timestamp: u64,
}
 #[cfg(target_os = "linux")]
pub async fn start_ebpf_proc_hollow_scan() -> Result<()> {
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../ebpf/proc_hollow_monitor.bpf.o"
    ))?;

    let prog: &mut TracePoint = bpf
        .program_mut("trace_proc_hollow")
        .ok_or_else(|| anyhow!("Missing program: trace_proc_hollow"))?
        .try_into()?;
    prog.load()?;
    prog.attach("syscalls", "sys_enter_mmap")?;

    let mut perf_array = AsyncPerfEventArray::try_from(bpf.map_mut("EVENTS")?)?;

    for cpu_id in online_cpus()? {
        let mut buf = perf_array.open(cpu_id, None)?;
        task::spawn(async move {
            let mut event_buf = Vec::new();

            loop {
                match buf.read_events(&mut event_buf).await {
                    Ok(events) if events.read > 0 => {
                        for i in 0..events.read {
                            let ptr = event_buf[i].as_ptr() as *const HollowEvent;
                            let event = unsafe { ptr.read_unaligned() };

                            let binary_path = get_binary_path(event.pid as i32)
                                .unwrap_or_else(|| "unknown".into());
                            let command_line = get_cmdline(event.pid as i32)
                                .unwrap_or_else(|| "unknown".into());
                            let cwd = get_cwd(event.pid as i32)
                                .unwrap_or_else(|| "unknown".into());

                            let desc = format!(
                                "Process hollowing attempt on PID {} (flags=0x{:x})",
                                event.target_pid, event.flags
                            );

                            let _ = push_memory_telemetry(
                                event.pid as i32,
                                0,
                                0,
                                &binary_path,
                                &command_line,
                                &cwd,
                                MemoryAnomalyType::ProcHollowing,
                                desc.clone(),
                            )
                            .map_err(|e| eprintln!("Failed to push hollow event: {:?}", e));

                            let mut metadata = HashMap::new();
                            metadata.insert("target_pid".into(), event.target_pid.to_string());
                            metadata.insert("flags".into(), format!("0x{:x}", event.flags));
                            metadata.insert("risk_reason".into(), "Detected mmap-based hollowing of target PID".into());

                            let trust_event = TrustEvent {
                                timestamp: now_ts(),
                                pid: event.pid as i32,
                                ppid: 0,
                                uid: 0,
                                binary_path,
                                command_line,
                                cwd,
                                anomaly_type: "ProcHollowing".into(),
                                component: "memory".into(),
                                metadata,
                                risk_score: 7.5,
                                source_module: "mem_scan".into(),
                                decay_context: Some("memory_behavior".into()),
                                module: Some("mem_scan".into()),
                                signal: Some("ProcHollowing".into()),
                                signal_type: Some("memory::proc_hollowing".into()),
                                score: Some(7.5),
                                raw_score: Some(7.5),
                                tags: Some(vec!["process_hollowing".into(), "memory".into()]),
                                description: Some(desc.clone()),
                            };

                            submit_trust_event(trust_event);

                            let mut gnn_data = HashMap::new();
                            gnn_data.insert(
                                "vector".into(),
                                format!(
                                    "{{\"pid\":{},\"target_pid\":{},\"flags\":\"0x{:x}\"}}",
                                    event.pid, event.target_pid, event.flags
                                ),
                            );
                            gnn_data.insert("category".into(), "memory".into());
                            gnn_data.insert("signal".into(), "proc_hollowing".into());
                            gnn_data.insert("confidence".into(), "0.4".into());
                            gnn_data.insert("gnn_escalate".into(), "true".into());
                            gnn_data.insert("summary".into(), desc.clone());
                            gnn_data.insert("replay_tag".into(), "proc_hollow".into());
                            gnn_data.insert("source_pid".into(), event.pid.to_string());
                            gnn_data.insert("target_pid".into(), event.target_pid.to_string());

                            push_to_gnn_vector_log(gnn_data.clone());
                            store_replay_event(gnn_data.clone());

                            mark_memory_anomaly_detected();

                            log(&format!(
                                "[🕳️ Hollowing] PID={} ➝ Target={} FLAGS=0x{:x}",
                                event.pid, event.target_pid, event.flags
                            ));
                        }
                    }
                    Ok(_) => continue,
                    Err(e) => {
                        eprintln!("Error reading proc hollow event: {:?}", e);
                        break;
                    }
                }
            }
        });
    }

    Ok(())
}



pub fn scan_memory_health() -> Vec<TelemetryOutput> {
    use crate::trust_hook::submit_trust_event;
    use crate::telemetry_types::TelemetryOutput;
    use crate::utils::time::now_ts;
    use std::sync::atomic::Ordering;

    if MEMORY_ANOMALY_FOUND.load(Ordering::Relaxed) {
        return vec![];
    }

    let ts = now_ts();

    let mut metadata = HashMap::new();
    metadata.insert("event_type".into(), "memory_scan".into());
    metadata.insert("status".into(), "healthy".into());

    let trust_event = TrustEvent {
        timestamp: ts,
        pid: 0,
        ppid: 0,
        uid: 0,
        binary_path: "kernel".into(),
        command_line: "memory_health_scan".into(),
        cwd: "/proc".into(),
        anomaly_type: "None".into(),
        component: "memory".into(),
        metadata,
        risk_score: 0.0,
        source_module: "mem_scan".into(),

        decay_context: Some("memory_health".into()),
        module: Some("mem_scan".into()),
        signal: Some("memory_health".into()),
        signal_type: Some("memory::health_check".into()),
        score: Some(0.0),
        raw_score: Some(0.0),
        tags: Some(vec!["memory".into(), "healthy".into()]),
        description: Some("Memory scan completed: no anomalies found.".into()),
    };

    submit_trust_event(trust_event);

    vec![TelemetryOutput {
        category: "memory".to_string(),
        signal: "heap_growth".to_string(),
        confidence: 0.95,
        data: HashMap::new(),
    }]
}

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
use crate::utils::{parse_proc_maps, parse_proc_maps_for_addr};
use crate::modules::telemetry_fingerprint::FingerprintEntry::MemoryRegion;


lazy_static::lazy_static! {
    static ref MEMORY_ANOMALY_FOUND: AtomicBool = AtomicBool::new(false);
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MemEvent {
    pub pid: u32,
    pub uid: u32,
    pub ppid: u32,
    pub addr: u64,
    pub comm: [u8; 64],
    pub details: [u8; 128],
}


#[cfg(target_os = "linux")]
pub async fn start_ebpf_mem_scan() -> Vec<TelemetryOutput> {
    let mut outputs = Vec::new();

    let mut bpf = match Bpf::load(include_bytes_aligned!("../ebpf/mem_scan.bpf.o")) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to load eBPF program: {:?}", e);
            return outputs;
        }
    };

    let mmap_prog: &mut TracePoint = match bpf.program_mut("trace_mmap") {
        Some(p) => p.try_into().unwrap(),
        None => return outputs,
    };
    mmap_prog.load().unwrap();
    mmap_prog.attach("syscalls", "sys_enter_mmap").unwrap();

    let execve_prog: &mut TracePoint = match bpf.program_mut("trace_execve_mem") {
        Some(p) => p.try_into().unwrap(),
        None => return outputs,
    };
    execve_prog.load().unwrap();
    execve_prog.attach("syscalls", "sys_enter_execve").unwrap();

    let mut perf_array = AsyncPerfEventArray::try_from(bpf.map_mut("events").unwrap()).unwrap();

    for cpu_id in match aya::util::online_cpus() {
        Ok(v) => v,
        Err(_) => return outputs,
    } {
        let mut buf = perf_array.open(cpu_id, None).unwrap();
        let fingerprints = load_fingerprints_from_disk("src/modules/telemetry_fingerprint.json");

        task::spawn(async move {
            let mut event_buf = Vec::new();

            loop {
                match buf.read_events(&mut event_buf).await {
                    Ok(events) if events.read > 0 => {
                        for i in 0..events.read {
                            let ptr = event_buf[i].as_ptr() as *const MemEvent;
                            let event = unsafe { ptr.read_unaligned() };

                            let comm = String::from_utf8_lossy(&event.comm).trim_end_matches('\0').to_string();
                            let desc = String::from_utf8_lossy(&event.details).trim_end_matches('\0').to_string();
                            let pid = event.pid as i32;
                            let addr = event.addr;

                            let mut region_map = HashMap::new();
                            region_map.insert("path".to_string(), region.path.clone());
                            region_map.insert("pid".to_string(), pid.to_string());

                            if is_known_good(&region_map, &fingerprints) {
                                log(&format!(
                                    "[🧠 mem_scan] Suppressed known-good memory region: {} (PID {})",
                                    region.path, pid
                                ));
                                continue;
                            }

                            let mut tags = vec!["memory::ebpf".into()];
                            if region.exec_capable {
                                tags.push("exec_mem".into());
                            }

                            let mut record = HashMap::new();
                            record.insert("pid".into(), pid.to_string());
                            record.insert("uid".into(), event.uid.to_string());
                            record.insert("ppid".into(), event.ppid.to_string());
                            record.insert("path".into(), region.path.clone());
                            record.insert("perms".into(), region.perms.clone());
                            record.insert("offset".into(), region.offset.to_string());
                            record.insert("size".into(), region.size.to_string());
                            record.insert("entropy".into(), format!("{:.2}", region.entropy));
                            record.insert("exec_capable".into(), region.exec_capable.to_string());
                            record.insert("trusted_uid".into(), region.trusted_uid.to_string());
                            record.insert("category".into(), region.category.clone());
                            record.insert("description".into(), desc.clone());
                            record.insert("source_module".into(), "mem_scan".into());
                            record.insert("created_at".into(), now_ts().to_string());
                            record.insert("tags".into(), tags.join(","));

                            let telemetry = TelemetryWriter::build(record.clone(), Some(6), tags.clone());
                            TelemetryWriter::new().send(telemetry.clone());
                            outputs.push(telemetry.clone());

                            let trust_event = TrustEvent::new_full(
                                now_ts(),
                                pid,
                                event.ppid as i32,
                                event.uid,
                                comm.clone(),
                                "ebpf memory trace".into(),
                                region.path.clone(),
                                "RWXMapping".into(),
                                "memory".into(),
                                "mem_scan".into(),
                                Some(desc.clone()),
                                Some("memory::ebpf_mem_trace".into()),
                                Some(vec!["rwx_mapping".into(), "ebpf_memory".into()]),
                                Some(6.0),
                            );
                            submit_trust_event(trust_event);

                            let mut gnn_data = HashMap::new();
                            gnn_data.insert("vector".into(), format!(
                                "{{\"pid\":{},\"ppid\":{},\"uid\":{},\"desc\":\"{}\"}}",
                                pid, event.ppid, event.uid, desc
                            ));
                            gnn_data.insert("category".into(), "memory".into());
                            gnn_data.insert("signal".into(), "ebpf_mem_trace".into());
                            gnn_data.insert("confidence".into(), "0.45".into());
                            gnn_data.insert("gnn_escalate".into(), "true".into());
                            gnn_data.insert("summary".into(), format!("RWX mapping via mmap/execve: {}", comm));
                            gnn_data.insert("replay_tag".into(), "mem_rwx".into());

                            push_to_gnn_vector_log(gnn_data.clone());
                            store_replay_event(gnn_data.clone());

                            log(&format!(
                                "[📦 Memory eBPF] PID={} UID={} DESC={}",
                                pid, event.uid, desc
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

    outputs
}






fn get_uid_from_proc(pid: i32) -> Option<u32> {
    let status_path = format!("/proc/{}/status", pid);
    if let Ok(content) = std::fs::read_to_string(status_path) {
        for line in content.lines() {
            if line.starts_with("Uid:") {
                return line.split_whitespace().nth(1).and_then(|s| s.parse().ok());
            }
        }
    }
    None
}



pub fn detect_memory_maps(pid: u32) -> Vec<TelemetryEvent> {
    let mut events = Vec::new();
    let ts = Utc::now().timestamp();
    let fingerprints = load_fingerprints_from_disk("src/modules/telemetry_fingerprint.json");

    if let Ok(regions) = parse_proc_maps(pid as i32) {
        for region in regions {
            let is_rwx = region.perms.contains("r") && region.perms.contains("x") && region.perms.contains("w");
            let is_anon = region.path.contains("[anon]");
            let is_null_base = region.path.starts_with("00000000");

            let anomaly = if is_rwx {
                Some(MemoryAnomalyType::RWXMapping)
            } else if is_anon {
                Some(MemoryAnomalyType::AnonymousExec)
            } else if is_null_base {
                Some(MemoryAnomalyType::NullBaseExec)
            } else {
                None
            };

            if let Some(atype) = anomaly {
                let mut data = HashMap::new();
                data.insert("path".into(), region.path.clone());
                data.insert("perms".into(), region.perms.clone());
                data.insert("offset".into(), region.offset.to_string());
                data.insert("size".into(), region.size.to_string());
                data.insert("entropy".into(), format!("{:.2}", region.entropy));
                data.insert("exec_capable".into(), region.exec_capable.to_string());
                data.insert("trusted_uid".into(), region.trusted_uid.to_string());
                data.insert("category".into(), region.category.clone());
                data.insert("pid".into(), pid.to_string());
                data.insert("type".into(), "memory_region".into());

                if is_known_good(&data, &fingerprints) {
                    log(&format!(
                        "[🧠 mem_scan] Suppressed fingerprinted memory anomaly: {} [{}]",
                        region.path, pid
                    ));
                    continue;
                }

                events.push(TelemetryEvent::MemoryAnomaly {
                    pid,
                    ppid: 0,
                    uid: region.trusted_uid,
                    binary_path: region.path.clone(),
                    command_line: "".into(),
                    description: format!(
                        "{} region at {} with perms {} (entropy: {:.2})",
                        atype.to_string(),
                        region.path,
                        region.perms,
                        region.entropy
                    ),
                    anomaly_type: atype,
                    timestamp: ts,
                });
            }
        }
    }

    events
}

pub fn detect_dirty_rss(pid: u32) -> Vec<TelemetryEvent> {
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
        let size = dirty_kb * 1024; // Convert KB to bytes
        let perms = "---";
        let offset = 0;
        let entropy = 0.0;
        let trusted_uid = 0;
        let category = "proc_smaps";

        let mut data = HashMap::new();
        data.insert("path".into(), smaps_path.clone());
        data.insert("perms".into(), perms.to_string());
        data.insert("offset".into(), offset.to_string());
        data.insert("size".into(), size.to_string());
        data.insert("entropy".into(), format!("{:.2}", entropy));
        data.insert("exec_capable".into(), "false".into());
        data.insert("trusted_uid".into(), trusted_uid.to_string());
        data.insert("category".into(), category.into());
        data.insert("pid".into(), pid.to_string());
        data.insert("type".into(), "memory_region".into());

        let fingerprints = load_fingerprints_from_disk("src/modules/telemetry_fingerprint.json");
        if is_known_good(&data, &fingerprints) {
            log(&format!(
                "[🧠 mem_scan] Suppressed known-good dirty RSS alert for PID {} ({} KB)",
                pid, dirty_kb
            ));
            return vec![];
        }

        vec![TelemetryEvent::MemoryAnomaly {
            pid,
            ppid: 0,
            uid: 0,
            binary_path: smaps_path,
            command_line: "rss_dirty_check".into(),
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
    let fingerprints = load_fingerprints_from_disk("src/modules/telemetry_fingerprint.json");

    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            continue;
        }

        let range = parts[0];
        let perms = parts[1].to_string();
        let offset = usize::from_str_radix(parts[2], 16).unwrap_or(0);
        let path = parts.get(5).cloned().unwrap_or("").to_string();

        let mut addr_parts = range.split('-');
        if let (Some(start), Some(end)) = (addr_parts.next(), addr_parts.next()) {
            if let (Ok(start_addr), Ok(end_addr)) =
                (usize::from_str_radix(start, 16), usize::from_str_radix(end, 16))
            {
                let size = end_addr.saturating_sub(start_addr);
                let mut buffer = vec![0u8; 256];

                if mem_file
                    .seek(std::io::SeekFrom::Start(start_addr as u64))
                    .is_ok()
                    && mem_file.read_exact(&mut buffer).is_ok()
                {
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
                        // === 🧬 Convert region to suppression map ===
                        let mut data = HashMap::new();
                        data.insert("path".into(), path.clone());
                        data.insert("perms".into(), perms.clone());
                        data.insert("offset".into(), offset.to_string());
                        data.insert("size".into(), size.to_string());
                        data.insert("entropy".into(), format!("{:.2}", entropy));
                        data.insert("exec_capable".into(), perms.contains('x').to_string());
                        data.insert("trusted_uid".into(), "0".into());
                        data.insert("category".into(), "high_entropy".into());
                        data.insert("pid".into(), pid.to_string());
                        data.insert("type".into(), "memory_region".into());

                        if is_known_good(&data, &fingerprints) {
                            log(&format!(
                                "[🧠 mem_scan] Suppressed known-good high entropy memory: {} (PID {})",
                                path, pid
                            ));
                            continue;
                        }

                        events.push(TelemetryEvent::MemoryAnomaly {
                            pid,
                            ppid: 0,
                            uid: 0,
                            binary_path: path.clone(),
                            command_line: format!("entropy={:.2}", entropy),
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

    events
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

                            let pid = event.pid as i32;
                            let binary_path = get_binary_path(pid).unwrap_or_else(|| "unknown".into());
                            let command_line = get_cmdline(pid).unwrap_or_else(|| "unknown".into());
                            let cwd = get_cwd(pid).unwrap_or_else(|| "unknown".into());

                            // === 🔒 Fingerprint Suppression for Hollowing Behavior ===
                            let fp_entry = FingerprintEntry::MemoryRegion {
                                path: binary_path.clone(),
                                pid,
                                perms: "rwx".into(),  // inferred intent of hollowing
                                offset: 0,
                                size: 0,
                                entropy: None,
                                exec_capable: true,
                                trusted_uid: 0,
                                category: "proc_hollowing".into(),
                                tags: vec!["memory".into(), "proc_hollowing".into()],
                                source_module: "mem_scan".into(),
                                created_at: now_ts(),
                                description: format!(
                                    "Proc hollow attempt (pid {} ➝ target {}) with flags 0x{:x}",
                                    pid, event.target_pid, event.flags
                                ),
                            };

                            let fingerprints = load_fingerprints();
                            if is_known_good_fingerprint(&fp_entry, &fingerprints) {
                                log(&format!(
                                    "[💠 fingerprint] Suppressed hollowing event for known binary: {}",
                                    binary_path
                                ));
                                continue;
                            }

                            // === 🧠 TELEMETRY + TRUST EVENT ===
                            let desc = format!(
                                "Process hollowing attempt on PID {} (flags=0x{:x})",
                                event.target_pid, event.flags
                            );

                            let _ = push_memory_telemetry(
                                pid,
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
                                pid,
                                ppid: 0,
                                uid: 0,
                                binary_path: binary_path.clone(),
                                command_line: command_line.clone(),
                                cwd: cwd.clone(),
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
                            gnn_data.insert("vector".into(), format!(
                                "{{\"pid\":{},\"target_pid\":{},\"flags\":\"0x{:x}\"}}",
                                event.pid, event.target_pid, event.flags
                            ));
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

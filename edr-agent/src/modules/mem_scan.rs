// src/modules/mem_scan.rs
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek};
use std::path::Path;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Result, anyhow, Context};
use bytes::BytesMut;
use chrono::Utc;

use aya::{Ebpf, include_bytes_aligned};
use aya::programs::TracePoint;
use aya::util::online_cpus;
use aya::maps::perf::AsyncPerfEventArray;

use tokio::task;

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::logger::log;
use crate::modules::replay_writer::store_replay_event;
use crate::telemetry::mark_memory_anomaly_detected;
use crate::telemetry_types::{TelemetryEvent, MemoryAnomalyType, TelemetryOutput};
use crate::trust_hook::{TrustEvent, submit_trust_event};
use crate::utils::utils::{parse_proc_maps, parse_proc_maps_for_addr};
use crate::utils::time::now_ts;
use crate::telemetry_writer::write_telemetry_record;
use crate::modules::telemetry_fingerprint::{load_fingerprints_from_disk, is_known_good, FingerprintEntry};

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
pub async fn start_ebpf_mem_scan() -> Result<()> {
    // Load and leak to satisfy 'static in spawned tasks (perf readers borrow from map/bpf)
    let mut tmp = Ebpf::load(include_bytes_aligned!("../ebpf/mem_scan.bpf.o"))
        .map_err(|e| anyhow!("load mem_scan.bpf.o: {e:?}"))?;
    let bpf: &'static mut Ebpf = Box::leak(Box::new(tmp));

    // mmap
    {
        let prog: &mut TracePoint = bpf
            .program_mut("trace_mmap")
            .ok_or_else(|| anyhow!("program not found: trace_mmap"))?
            .try_into()?;
        prog.load()?;
        prog.attach("syscalls", "sys_enter_mmap")?;
    }

    // execve
    {
        let prog: &mut TracePoint = bpf
            .program_mut("trace_execve_mem")
            .ok_or_else(|| anyhow!("program not found: trace_execve_mem"))?
            .try_into()?;
        prog.load()?;
        prog.attach("syscalls", "sys_enter_execve")?;
    }

    // Open perf array used by the eBPF program
    let map = bpf
        .map_mut("events")
        .ok_or_else(|| anyhow!("map 'events' not found"))?;
    let mut perf_array = AsyncPerfEventArray::try_from(map)
        .context("AsyncPerfEventArray init for 'events'")?;

    // Preload fingerprints once (shared by tasks)
    let fingerprints = load_fingerprints_from_disk("src/modules/telemetry_fingerprint.json");

    let cpus = online_cpus()
        .map_err(|(m, e)| anyhow!("online_cpus failed: {m}: {e}"))?;
    for cpu_id in cpus {
        let mut buf = perf_array.open(cpu_id, None)?;
        let fingerprints = fingerprints.clone();

        task::spawn(async move {
            // prealloc a few slots for perf batches
            let mut event_buf = vec![BytesMut::with_capacity(1024); 8];

            loop {
                match buf.read_events(&mut event_buf).await {
                    Ok(events) if events.read > 0 => {
                        for i in 0..events.read {
                            let slot = &event_buf[i];
                            if slot.len() < std::mem::size_of::<MemEvent>() {
                                continue;
                            }
                            // Safe due to size check; kernel writes packed struct
                            let evt = unsafe {
                                std::ptr::read_unaligned(slot.as_ptr() as *const MemEvent)
                            };

                            let comm = String::from_utf8_lossy(&evt.comm)
                                .trim_end_matches('\0')
                                .to_string();
                            let details = String::from_utf8_lossy(&evt.details)
                                .trim_end_matches('\0')
                                .to_string();

                            let pid = evt.pid as i32;
                            let addr = evt.addr;

                            // Enrich memory region by address (best effort)
                            let region = parse_proc_maps_for_addr(pid, addr as u64);

                            // Build a suppression probe map using matched fields
                            let mut fp_map = HashMap::new();
                            if let Some(FingerprintEntry::MemoryRegion { path, perms, .. }) = region.as_ref() {
                                fp_map.insert("path".into(), path.clone());
                                fp_map.insert("perms".into(), perms.clone());
                            } else {
                                fp_map.insert("path".into(), "[anon]".into());
                                fp_map.insert("perms".into(), "---".into());
                            }
                            fp_map.insert("pid".into(), pid.to_string());
                            fp_map.insert("category".into(), "mem_scan".into());

                            if is_known_good(&fp_map, &fingerprints) {
                                log(&format!(
                                    "[🧠 mem_scan] Suppressed fingerprinted region for PID {pid}: {}",
                                    fp_map.get("path").cloned().unwrap_or_default()
                                ));
                                continue;
                            }

                            // Telemetry record
                            let mut data = HashMap::new();
                            data.insert("pid".into(), pid.to_string());
                            data.insert("uid".into(), evt.uid.to_string());
                            data.insert("ppid".into(), evt.ppid.to_string());
                            data.insert("addr".into(), format!("0x{:x}", addr));
                            data.insert("comm".into(), comm.clone());
                            data.insert("description".into(), details.clone());

                            if let Some(FingerprintEntry::MemoryRegion {
                                path,
                                perms,
                                offset,
                                size,
                                entropy,
                                exec_capable,
                                trusted_uid,
                                category,
                                ..
                            }) = &region
                            {
                                data.insert("path".into(), path.clone());
                                data.insert("perms".into(), perms.clone());
                                // NOTE: these are &Option<T>; deref first, then unwrap_or(...)
                                data.insert("offset".into(), (*offset).unwrap_or(0).to_string());
                                data.insert("size".into(), (*size).unwrap_or(0).to_string());
                                data.insert("entropy".into(), format!("{:.2}", (*entropy).unwrap_or(0.0)));
                                data.insert("exec_capable".into(), (*exec_capable).unwrap_or(false).to_string());
                                // trusted_uid is a plain u32 in your fingerprint struct
                                data.insert("trusted_uid".into(), (*trusted_uid).to_string());
                                data.insert("category".into(), category.clone());
                            } else {
                                data.insert("path".into(), "[unknown]".into());
                                data.insert("perms".into(), "---".into());
                                data.insert("category".into(), "memory".into());
                            }

                            // Side effects
                            let mut gnn = data.clone();
                            gnn.insert("signal".into(), "ebpf_mem_trace".into());
                            gnn.insert("gnn_escalate".into(), "true".into());
                            gnn.insert("replay_tag".into(), "mem_rwx".into());
                            gnn.insert("confidence".into(), "0.45".into());
                            push_to_gnn_vector_log(gnn.clone());
                            let _ = store_replay_event(gnn);

                            let mut wtr = data.clone();
                            wtr.insert("timestamp".into(), now_ts().to_string());
                            wtr.insert("event_type".into(), "ebpf_mem_trace".into());
                            write_telemetry_record(wtr);

                            // Choose anomaly based on matched fields
                            let anomaly = if let Some(FingerprintEntry::MemoryRegion { exec_capable, perms, path, .. }) = region.as_ref() {
                                if (*exec_capable).unwrap_or(false) && perms.contains('w') {
                                    MemoryAnomalyType::RWXMapping
                                } else if path.contains("[anon]") {
                                    MemoryAnomalyType::AnonymousExec
                                } else {
                                    MemoryAnomalyType::HighEntropy
                                }
                            } else {
                                MemoryAnomalyType::HighEntropy
                            };

                            let binary_for_event = region
                                .as_ref()
                                .and_then(|r| match r {
                                    FingerprintEntry::MemoryRegion { path, .. } => Some(path.clone()),
                                    _ => None,
                                })
                                .unwrap_or_else(|| "[unknown]".into());

                            let trust_event = TrustEvent {
                                timestamp: now_ts(),
                                pid,
                                ppid: evt.ppid as i32,
                                uid: evt.uid,
                                binary_path: binary_for_event,
                                command_line: comm.clone(),
                                cwd: "/".into(),
                                anomaly_type: format!("{anomaly:?}"),
                                component: "memory".into(),
                                metadata: data.clone(),
                                risk_score: 6.0,
                                source_module: "mem_scan".into(),
                                decay_context: Some("memory_behavior".into()),
                                module: Some("mem_scan".into()),
                                signal: Some("ebpf_mem_trace".into()),
                                signal_type: Some("memory::ebpf".into()),
                                score: Some(6.0),
                                raw_score: Some(6.0),
                                tags: Some(vec!["memory".into(), "ebpf".into(), "rwx_mapping".into()]),
                                description: Some(details.clone()),
                            };
                            submit_trust_event(trust_event);

                            MEMORY_ANOMALY_FOUND.store(true, Ordering::Relaxed);
                            mark_memory_anomaly_detected();

                            log(&format!(
                                "[📦 Memory eBPF] PID={} UID={} ADDR=0x{:x} DESC='{}'",
                                pid, evt.uid, addr, details
                            ));
                        }
                    }
                    Ok(_) => continue,
                    Err(e) => {
                        eprintln!("Error reading memory perf events (CPU {cpu_id}): {:?}", e);
                        break;
                    }
                }
            }
        });
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub async fn start_ebpf_mem_scan() -> Result<()> {
    // Non-Linux: no-op
    Ok(())
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

    // parse_proc_maps returns Vec<HashMap<String,String>>
    let regions = parse_proc_maps(pid as i32);

    for region in regions {
        let perms = region.get("perms").cloned().unwrap_or_else(|| "---".into());
        let path  = region.get("path").cloned().unwrap_or_default();

        let is_rwx = perms.contains('r') && perms.contains('w') && perms.contains('x');
        let is_anon = path.contains("[anon]");
        let is_null_base = path.starts_with("00000000");

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
            // build data map for suppression + emission
            let mut data = HashMap::new();
            data.insert("path".into(), path.clone());
            data.insert("perms".into(), perms.clone());
            data.insert("offset".into(), region.get("offset").cloned().unwrap_or_else(|| "0".into()));
            data.insert("size".into(), region.get("size").cloned().unwrap_or_else(|| "0".into()));
            data.insert("entropy".into(), region.get("entropy").cloned().unwrap_or_else(|| "0.0".into()));
            data.insert("exec_capable".into(), (perms.contains('x')).to_string());
            data.insert("trusted_uid".into(), region.get("trusted_uid").cloned().unwrap_or_else(|| "0".into()));
            data.insert("category".into(), region.get("category").cloned().unwrap_or_else(|| "memory".into()));
            data.insert("pid".into(), pid.to_string());
            data.insert("type".into(), "memory_region".into());

            if is_known_good(&data, &fingerprints) {
                log(&format!(
                    "[🧠 mem_scan] Suppressed fingerprinted memory anomaly: {} [{}]",
                    path, pid
                ));
                continue;
            }

            // parse numeric pieces if present
            let uid_parsed = region.get("trusted_uid").and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
            let entropy_parsed = region.get("entropy").and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);

            events.push(TelemetryEvent::MemoryAnomaly {
                pid,
                ppid: 0,
                uid: uid_parsed,
                binary_path: path.clone(),
                command_line: "".into(),
                description: format!(
                    "{} region {} (perms {}) entropy {:.2}",
                    atype.to_string(),
                    path,
                    perms,
                    entropy_parsed
                ),
                anomaly_type: atype,
                timestamp: ts,
            });
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
        let size = dirty_kb * 1024; // bytes

        let mut data = HashMap::new();
        data.insert("path".into(), smaps_path.clone());
        data.insert("perms".into(), "---".to_string());
        data.insert("offset".into(), "0".to_string());
        data.insert("size".into(), size.to_string());
        data.insert("entropy".into(), format!("{:.2}", 0.0));
        data.insert("exec_capable".into(), "false".into());
        data.insert("trusted_uid".into(), "0".into());
        data.insert("category".into(), "proc_smaps".into());
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

    let mut mem_file = match OpenOptions::new().read(true).open(&mem_path) {
        Ok(f) => f,
        Err(_) => return events,
    };

    let file = match File::open(&maps_path) {
        Ok(f) => f,
        Err(_) => return events,
    };

    let reader = BufReader::new(file);
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
    // Load and leak as above to satisfy spawned task lifetimes
    let mut tmp = Ebpf::load(include_bytes_aligned!("../ebpf/proc_hollow_monitor.bpf.o"))
        .map_err(|e| anyhow!("load proc_hollow_monitor.bpf.o: {e:?}"))?;
    let bpf: &'static mut Ebpf = Box::leak(Box::new(tmp));

    let prog: &mut TracePoint = bpf
        .program_mut("trace_proc_hollow")
        .ok_or_else(|| anyhow!("Missing program: trace_proc_hollow"))?
        .try_into()?;
    prog.load()?;
    prog.attach("syscalls", "sys_enter_mmap")?;

    let map = bpf
        .map_mut("EVENTS")
        .ok_or_else(|| anyhow!("map 'EVENTS' not found"))?;
    let mut perf_array = AsyncPerfEventArray::try_from(map)
        .context("AsyncPerfEventArray init for 'EVENTS'")?;

    let cpus = online_cpus()
        .map_err(|(m, e)| anyhow!("online_cpus failed: {m}: {e}"))?;
    for cpu_id in cpus {
        let mut buf = perf_array.open(cpu_id, None)?;
        task::spawn(async move {
            let mut event_buf = vec![BytesMut::with_capacity(1024); 8];

            loop {
                match buf.read_events(&mut event_buf).await {
                    Ok(events) if events.read > 0 => {
                        for i in 0..events.read {
                            let slot = &event_buf[i];
                            if slot.len() < std::mem::size_of::<HollowEvent>() {
                                continue;
                            }
                            let event = unsafe {
                                std::ptr::read_unaligned(slot.as_ptr() as *const HollowEvent)
                            };

                            let pid = event.pid as i32;
                            let binary_path = get_binary_path(pid).unwrap_or_else(|| "unknown".into());
                            let command_line = get_cmdline(pid).unwrap_or_else(|| "unknown".into());
                            let cwd = get_cwd(pid).unwrap_or_else(|| "unknown".into());

                            // fingerprint suppression (map form)
                            let fingerprints = load_fingerprints_from_disk("src/modules/telemetry_fingerprint.json");
                            let mut fp_map = HashMap::new();
                            fp_map.insert("path".into(), binary_path.clone());
                            fp_map.insert("category".into(), "proc_hollowing".into());
                            fp_map.insert("pid".into(), pid.to_string());
                            if is_known_good(&fp_map, &fingerprints) {
                                log(&format!(
                                    "[💠 fingerprint] Suppressed hollowing event for known binary: {}",
                                    binary_path
                                ));
                                continue;
                            }

                            let desc = format!(
                                "Process hollowing attempt on PID {} (flags=0x{:x})",
                                event.target_pid, event.flags
                            );

                            // trust + telemetry
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
                                metadata: metadata.clone(),
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
                                format!("{{\"pid\":{},\"target_pid\":{},\"flags\":\"0x{:x}\"}}",
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
                            let _ = store_replay_event(gnn_data);

                            MEMORY_ANOMALY_FOUND.store(true, Ordering::Relaxed);
                            mark_memory_anomaly_detected();

                            log(&format!(
                                "[🕳️ Hollowing] PID={} ➝ Target={} FLAGS=0x{:x}",
                                event.pid, event.target_pid, event.flags
                            ));
                        }
                    }
                    Ok(_) => continue,
                    Err(e) => {
                        eprintln!("Error reading proc hollow event (CPU {cpu_id}): {:?}", e);
                        break;
                    }
                }
            }
        });
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub async fn start_ebpf_proc_hollow_scan() -> Result<()> {
    // Non-Linux: no-op
    Ok(())
}

pub fn scan_memory_health() -> Vec<TelemetryOutput> {
    if MEMORY_ANOMALY_FOUND.load(Ordering::Relaxed) {
        // a real anomaly was seen recently; skip heartbeat this cycle
        MEMORY_ANOMALY_FOUND.store(false, Ordering::Relaxed);
        return vec![];
    }

    let ts = now_ts();
    let mut data = HashMap::new();
    data.insert("event_type".into(), "memory_health".into());
    data.insert("status".into(), "healthy".into());
    data.insert("timestamp".into(), ts.to_string());

    // (Optional) trust heartbeat
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
        metadata: data.clone(),
        risk_score: 0.0,
        source_module: "mem_scan".into(),
        decay_context: Some("memory_health".into()),
        module: Some("mem_scan".into()),
        signal: Some("memory_health".into()),
        signal_type: Some("memory::health_check".into()),
        score: Some(0.0),
        raw_score: Some(0.0),
        tags: Some(vec!["memory".into(), "healthy".into()]),
        description: Some("Memory scan heartbeat: no anomalies reported.".into()),
    };
    submit_trust_event(trust_event);

    vec![TelemetryOutput {
        category: "memory".into(),
        signal: "memory_health".into(),
        confidence: 0.0,
        data,
    }]
}

/* -------------------- small /proc helpers -------------------- */

fn get_binary_path(pid: i32) -> Option<String> {
    std::fs::read_link(format!("/proc/{}/exe", pid)).ok().map(|p| p.display().to_string())
}

fn get_cmdline(pid: i32) -> Option<String> {
    std::fs::read(format!("/proc/{}/cmdline", pid))
        .ok()
        .map(|b| String::from_utf8_lossy(&b).replace('\0', " ").trim().to_string())
}

fn get_cwd(pid: i32) -> Option<String> {
    std::fs::read_link(format!("/proc/{}/cwd", pid)).ok().map(|p| p.display().to_string())
}

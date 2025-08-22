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
    time::{SystemTime, UNIX_EPOCH},
};

use aya::{
    include_bytes_aligned,
    maps::perf::AsyncPerfEventArray,
    programs::{KProbe, Program, RawTracePoint, TracePoint}, // No KRetProbe (use KProbe for both)
    util::online_cpus,
    Bpf,
};
use bytes::BytesMut;
use tokio::{runtime::Runtime, task};

use crate::{
    gnn_hook::push_to_gnn_vector_log,
    logger::{log_scoped},
    modules::replay_writer::store_replay_event,
    telemetry_types::{ContainerExecEvent, TelemetryOutput},
    telemetry_writer::write_telemetry_record,
    trust_hook::{submit_trust_event, TrustEvent},
};

static CONTAINER_MONITOR_STARTED: AtomicBool = AtomicBool::new(false);
static CONTAINER_MONITOR_ONCE: Once = Once::new();

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ContainerExecEventRaw {
    pub pid: u32,
    pub uid: u32,
    pub gid: u32,
    pub timestamp: u64,
    pub comm: [u8; 64],
    pub filename: [u8; 256],
}

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Attach all programs in the object based on their section names.
/// Covers `tracepoint/<cat>/<name>`, `raw_tracepoint/<name>`,
/// and kprobes / kretprobes (both via KProbe in Aya).
fn attach_all_programs(bpf: &mut Bpf) -> anyhow::Result<()> {
    for (sec, prog) in bpf.programs_mut() {
        // Derive a reasonable program symbol/name from section as a fallback
        // e.g., "tracepoint/syscalls/sys_enter_execve" -> "sys_enter_execve"
        let fallback_name = sec.rsplit('/').next().unwrap_or("unknown");

        match prog {
            Program::TracePoint(tp) => {
                tp.load()?;
                if let Some(rest) = sec.strip_prefix("tracepoint/") {
                    let mut it = rest.splitn(2, '/');
                    let cat = it.next().unwrap_or("");
                    let evt = it.next().unwrap_or(fallback_name);
                    tp.attach(cat, evt)?;
                    log_scoped(
                        "container_monitor",
                        &format!("✔️ attached tracepoint {}/{}", cat, evt),
                    );
                } else {
                    // Fallback: common exec TP
                    tp.attach("sched", "sched_process_exec")?;
                    log_scoped(
                        "container_monitor",
                        "✔️ attached tracepoint sched/sched_process_exec (fallback)",
                    );
                }
            }
            Program::RawTracePoint(rtp) => {
                rtp.load()?;
                let evt = sec.split('/').nth(1).unwrap_or(fallback_name);
                rtp.attach(evt)?;
                log_scoped(
                    "container_monitor",
                    &format!("✔️ attached raw_tracepoint {}", evt),
                );
            }
            Program::KProbe(kp) => {
                kp.load()?;
                // Sections like "kprobe/<symbol>" or "kretprobe/<symbol>" both handled by KProbe
                let sym = sec.split('/').nth(1).unwrap_or(fallback_name);
                if let Err(e) = kp.attach(sym, 0) {
                    log_scoped(
                        "container_monitor",
                        &format!("ℹ️ KProbe load ok but attach({sym}) failed: {:?}", e),
                    );
                } else {
                    log_scoped(
                        "container_monitor",
                        &format!("✔️ attached kprobe {}", sym),
                    );
                }
            }
            // For other program kinds, don't try to call `.load()` on the enum (not available);
            // just skip explicit attach and move on.
            _ => {
                log_scoped(
                    "container_monitor",
                    &format!("ℹ️ skipping unsupported program kind for section [{}]", sec),
                );
            }
        }
    }
    Ok(())
}

pub async fn start_ebpf_container_exec_monitor() -> anyhow::Result<()> {
    // Path is relative to this file: src/modules/ -> ../ebpf/...
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../ebpf/container_exec_monitor.bpf.o"
    ))?;

    // Attach all programs present in the object
    attach_all_programs(&mut bpf)?;

    // Open perf array used by the eBPF program
    let mut perf_array = AsyncPerfEventArray::try_from(bpf.map_mut("CONTAINER_EXEC_EVENTS")?)?;

    for cpu_id in online_cpus()? {
        let mut buf = perf_array.open(cpu_id, None)?;

        task::spawn(async move {
            // One buffer per task is fine; aya will fill it
            let mut bufs = vec![BytesMut::with_capacity(4096); 1];

            loop {
                match buf.read_events(&mut bufs).await {
                    Ok(events) => {
                        for i in 0..events.read {
                            let b = &bufs[i];
                            if b.len() < mem::size_of::<ContainerExecEventRaw>() {
                                continue;
                            }

                            // SAFETY: read_unaligned allows unaligned perf payload
                            let raw: ContainerExecEventRaw =
                                unsafe { std::ptr::read_unaligned(b.as_ptr() as *const _) };

                            let comm = String::from_utf8_lossy(&raw.comm)
                                .trim_end_matches('\0')
                                .to_string();

                            let filename = String::from_utf8_lossy(&raw.filename)
                                .trim_end_matches('\0')
                                .to_string();

                            let container_event = ContainerExecEvent {
                                pid: raw.pid,
                                ppid: 0,
                                uid: raw.uid,
                                gid: raw.gid,
                                comm,
                                cmdline: filename,
                                timestamp: raw.timestamp,
                                container_type: "unknown".to_string(),
                            };

                            // Build telemetry record map
                            let mut record = HashMap::new();
                            record.insert("pid".into(), container_event.pid.to_string());
                            record.insert("ppid".into(), container_event.ppid.to_string());
                            record.insert("uid".into(), container_event.uid.to_string());
                            record.insert("gid".into(), container_event.gid.to_string());
                            record.insert("comm".into(), container_event.comm.clone());
                            record.insert("cmdline".into(), container_event.cmdline.clone());
                            record.insert("timestamp".into(), container_event.timestamp.to_string());
                            record.insert(
                                "container_type".into(),
                                container_event.container_type.clone(),
                            );
                            record.insert("event_type".into(), "container_exec".into());
                            record.insert("signal".into(), "container_exec".into());
                            record.insert("category".into(), "container".into());
                            record.insert("confidence".into(), "0.9".into());
                            record.insert("gnn_escalate".into(), "true".into());
                            record.insert("replay_tag".into(), "container_exec_detected".into());

                            // Trust + sinks
                            let trust_event = TrustEvent::new_full(
                                container_event.timestamp,
                                container_event.pid as i32,
                                container_event.ppid as i32,
                                container_event.uid,
                                container_event.comm.clone(),
                                container_event.cmdline.clone(),
                                "/proc".into(),
                                "container_exec".into(),
                                "container::exec_detect".into(),
                                "container_monitor".into(),
                                Some("Containerized process execution detected".into()),
                                Some("container::exec".into()),
                                Some(vec![
                                    "container".into(),
                                    "exec".into(),
                                    container_event.comm.clone(),
                                ]),
                                Some(7.0),
                            );
                            submit_trust_event(trust_event);

                            let telemetry_output = TelemetryOutput {
                                category: "container".into(),
                                signal: "container_exec".into(),
                                confidence: 0.9,
                                data: record.clone(),
                            };

                            write_telemetry_record(record.clone());
                            push_to_gnn_vector_log(record.clone());
                            store_replay_event(record);
                        }

                        if events.lost > 0 {
                            log_scoped(
                                "container_monitor",
                                &format!("⚠️ Lost {} container exec events", events.lost),
                            );
                        }
                    }
                    Err(e) => {
                        log_scoped(
                            "container_monitor",
                            &format!("❌ Error reading container perf events: {:?}", e),
                        );
                        // small backoff to avoid tight error loops
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    }
                }
            }
        });
    }

    Ok(())
}

pub fn scan_container_activity() -> Vec<TelemetryOutput> {
    let mut outputs = Vec::new();
    let now = now_ts();

    CONTAINER_MONITOR_ONCE.call_once(|| {
        if !CONTAINER_MONITOR_STARTED.load(Ordering::Relaxed) {
            // Spawn a dedicated thread with its own Tokio runtime
            let _ = thread::spawn(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    match start_ebpf_container_exec_monitor().await {
                        Ok(_) => {
                            CONTAINER_MONITOR_STARTED.store(true, Ordering::Relaxed);
                            log_scoped("container_monitor", "✅ container_exec_monitor attached.");

                            // Heartbeat — only after successful startup
                            let mut data = HashMap::new();
                            data.insert("event_type".into(), "container_monitor_active".into());
                            data.insert("status".into(), "ebpf_monitor_spawned".into());
                            data.insert("timestamp".into(), now.to_string());
                            data.insert("category".into(), "container".into());
                            data.insert("signal".into(), "container_monitor_active".into());
                            data.insert("replay_tag".into(), "container_monitor_heartbeat".into());

                            let trust_event = TrustEvent {
                                timestamp: now,
                                pid: 0,
                                ppid: 0,
                                uid: 0,
                                binary_path: "kernel".into(),
                                command_line: "start_ebpf_container_exec_monitor".into(),
                                cwd: "/".into(),
                                anomaly_type: "Status".into(),
                                component: "container_monitor".into(),
                                metadata: {
                                    let mut m = HashMap::new();
                                    m.insert("event".into(), "container_monitor_heartbeat".into());
                                    m.insert("status".into(), "active".into());
                                    m
                                },
                                risk_score: 0.0,
                                source_module: "container_monitor".into(),
                                decay_context: Some("container_monitoring".into()),
                                module: Some("container_monitor".into()),
                                signal: Some("container_monitor_active".into()),
                                signal_type: Some("monitor_heartbeat".into()),
                                score: Some(100.0),
                                raw_score: Some(0.0),
                                tags: Some(vec!["monitor_alive".into(), "ebpf_monitor_ready".into()]),
                                description: Some(
                                    "Container eBPF monitor heartbeat confirmation".into(),
                                ),
                            };
                            submit_trust_event(trust_event);

                            write_telemetry_record(data.clone());
                            push_to_gnn_vector_log(data.clone());
                            store_replay_event(data);
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to launch container monitor: {:?}", e);
                        }
                    }
                });
            });
        }
    });

    // Return empty (or you could emit a lightweight “starting” signal here).
    outputs
}

// ---- helpers for future local enrichment (kept from your file) ----

fn extract_comm(pid: u32) -> String {
    let comm_path = format!("/proc/{}/comm", pid);
    fs::read_to_string(comm_path)
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn extract_cmdline(pid: u32) -> String {
    let cmdline_path = format!("/proc/{}/cmdline", pid);
    fs::read(cmdline_path)
        .map(|bytes| {
            String::from_utf8_lossy(&bytes)
                .replace('\0', " ")
                .trim()
                .to_string()
        })
        .unwrap_or_default()
}

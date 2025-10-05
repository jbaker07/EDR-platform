// src/modules/container_monitor.rs
use std::{
    collections::HashMap,
    fs, mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Once,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{anyhow, Context};
use aya::{
    include_bytes_aligned,
    maps::{perf::AsyncPerfEventArray, ring_buf::RingBuf, Map},
    programs::{KProbe, Program, RawTracePoint, TracePoint},
    util::online_cpus,
    Ebpf,
};
use bytes::BytesMut;
use tokio::{runtime::Runtime, task};

use crate::{
    gnn_hook::push_to_gnn_vector_log,
    logger::log_scoped,
    modules::replay_writer::store_replay_event,
    telemetry_types::{ContainerExecEvent, TelemetryOutput},
    telemetry_writer::write_telemetry_record,
    trust_hook::{submit_trust_event, TrustEvent},
    utils::strnorm::{cbytes_to_string_lossy, sanitize_tag_value},
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

// Matches the CO-RE ringbuf event in container_exec_monitor.c
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ContainerExecEventRingRaw {
    pub ts: u64,  // ns
    pub pid: u32, // tgid
    pub uid: u32,
    pub cgroup_id: u64,
    pub mnt_ns: u32,
    pub pid_ns: u32,
    pub flags: u32,     // bit0: execveat
    pub comm: [u8; 16], // TASK_COMM_LEN
    pub filename: [u8; 256],
}

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Attach all programs in the object based on their section names.
fn attach_all_programs(bpf: &mut Ebpf) -> anyhow::Result<()> {
    for (sec, prog) in bpf.programs_mut() {
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
                        &format!("✔️ tracepoint {}/{}", cat, evt),
                    );
                } else {
                    tp.attach("sched", "sched_process_exec")?;
                    log_scoped(
                        "container_monitor",
                        "✔️ tracepoint sched/sched_process_exec (fallback)",
                    );
                }
            }
            Program::RawTracePoint(rtp) => {
                rtp.load()?;
                let evt = sec.split('/').nth(1).unwrap_or(fallback_name);
                rtp.attach(evt)?;
                log_scoped("container_monitor", &format!("✔️ raw_tracepoint {}", evt));
            }
            Program::KProbe(kp) => {
                kp.load()?;
                let sym = sec.split('/').nth(1).unwrap_or(fallback_name);
                if let Err(e) = kp.attach(sym, 0) {
                    log_scoped(
                        "container_monitor",
                        &format!("ℹ️ kprobe attach({sym}) failed: {:?}", e),
                    );
                } else {
                    log_scoped("container_monitor", &format!("✔️ kprobe {}", sym));
                }
            }
            _ => {
                log_scoped(
                    "container_monitor",
                    &format!("ℹ️ skip unsupported program section [{}]", sec),
                );
            }
        }
    }
    Ok(())
}

pub async fn start_ebpf_container_exec_monitor() -> anyhow::Result<()> {
    #[cfg(not(feature = "embed_bpf"))]
    {
        log_scoped(
            "container_monitor",
            "⚠️ embed_bpf disabled or container_exec_monitor.bpf.o missing; skipping attach",
        );
        return Ok(());
    }

    #[cfg(feature = "embed_bpf")]
    {
        // src/modules/ -> ../ebpf/...
        let mut tmp = Ebpf::load(include_bytes_aligned!(
            "../ebpf/container_exec_monitor.bpf.o"
        ))?;
        let bpf: &'static mut Ebpf = Box::leak(Box::new(tmp));
        attach_all_programs(bpf)?;

        /// Prefer PERF first (if present)
        if let Some(map) = bpf.take_map("CONTAINER_EXEC_EVENTS") {
            attach_perf_owned(map)?;
            return Ok(());
        }

        // Ringbuf options used by our CO-RE obj
        if let Some(map) = bpf.take_map("container_exec_") {
            attach_ringbuf_owned(map)?;
            return Ok(());
        }

        // Very generic fallback some objs use
        if let Some(map) = bpf.take_map("events") {
            attach_ringbuf_owned(map)?;
            return Ok(());
        }

        return Err(anyhow!(
            "no container-exec map found (tried CONTAINER_EXEC_EVENTS, container_exec_, events)"
        ));
    }
}

// ---------------------- PERF readers (owned + async) ----------------------

fn attach_perf_owned(map: Map) -> anyhow::Result<()> {
    // Build event array from OWNED map (no borrowing of `bpf`).
    let mut perf_array =
        AsyncPerfEventArray::try_from(map).context("open perf map 'CONTAINER_EXEC_EVENTS'")?;

    let cpus = online_cpus().map_err(|(m, e)| anyhow!("online_cpus failed: {m}: {e}"))?;
    for cpu_id in cpus {
        let mut buf = perf_array.open(cpu_id, None)?;
        task::spawn(async move {
            let mut bufs = vec![BytesMut::with_capacity(4096); 32];

            loop {
                match buf.read_events(&mut bufs).await {
                    Ok(events) => {
                        for i in 0..events.read {
                            let b = &bufs[i];
                            if b.len() < mem::size_of::<ContainerExecEventRaw>() {
                                continue;
                            }

                            // Unaligned read from perf
                            let raw: ContainerExecEventRaw =
                                unsafe { std::ptr::read_unaligned(b.as_ptr() as *const _) };

                            let comm = cbytes_to_string_lossy(&raw.comm);
                            let filename = cbytes_to_string_lossy(&raw.filename);

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

                            let mut record = HashMap::new();
                            record.insert("pid".into(), container_event.pid.to_string());
                            record.insert("ppid".into(), container_event.ppid.to_string());
                            record.insert("uid".into(), container_event.uid.to_string());
                            record.insert("gid".into(), container_event.gid.to_string());
                            record.insert("comm".into(), container_event.comm.clone());
                            record.insert("cmdline".into(), container_event.cmdline.clone());
                            record
                                .insert("timestamp".into(), container_event.timestamp.to_string());
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

                            let comm_tag_value = sanitize_tag_value(&container_event.comm);

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
                                    comm_tag_value.clone(),
                                ]),
                                Some(7.0),
                            );
                            submit_trust_event(trust_event);

                            let _telemetry_output = TelemetryOutput {
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
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    }
                }
            }
        });
    }

    Ok(())
}

// ---------------------- Ringbuf readers (owned) ----------------------

fn attach_ringbuf_owned(map: Map) -> anyhow::Result<()> {
    // Owned ring buffer from owned map
    let mut rb = RingBuf::try_from(map).context("open ringbuf 'events'")?;

    task::spawn_blocking(move || loop {
        while let Some(rec) = rb.next() {
            let data: &[u8] = rec.as_ref();
            if data.len() < std::mem::size_of::<ContainerExecEventRingRaw>() {
                continue;
            }

            // Unaligned read from ringbuf
            let raw: ContainerExecEventRingRaw =
                unsafe { std::ptr::read_unaligned(data.as_ptr() as *const _) };

            let comm = cbytes_to_string_lossy(&raw.comm);
            let filename = cbytes_to_string_lossy(&raw.filename);
            let ts_sec = raw.ts / 1_000_000_000; // ns → s

            let container_event = ContainerExecEvent {
                pid: raw.pid,
                ppid: 0,
                uid: raw.uid,
                gid: 0,
                comm: comm.clone(),
                cmdline: filename.clone(),
                timestamp: ts_sec,
                container_type: "unknown".to_string(),
            };

            let mut record = HashMap::new();
            record.insert("pid".into(), container_event.pid.to_string());
            record.insert("ppid".into(), container_event.ppid.to_string());
            record.insert("uid".into(), container_event.uid.to_string());
            record.insert("gid".into(), container_event.gid.to_string());
            record.insert("comm".into(), container_event.comm.clone());
            record.insert("cmdline".into(), container_event.cmdline.clone());
            record.insert("timestamp".into(), container_event.timestamp.to_string());
            record.insert("mnt_ns".into(), raw.mnt_ns.to_string());
            record.insert("pid_ns".into(), raw.pid_ns.to_string());
            record.insert("cgroup_id".into(), raw.cgroup_id.to_string());
            record.insert("flags".into(), raw.flags.to_string());
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

            let comm_tag_value = sanitize_tag_value(&container_event.comm);

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
                Some(vec!["container".into(), "exec".into(), comm_tag_value]),
                Some(7.0),
            );
            submit_trust_event(trust_event);

            let _telemetry_output = TelemetryOutput {
                category: "container".into(),
                signal: "container_exec".into(),
                confidence: 0.9,
                data: record.clone(),
            };

            write_telemetry_record(record.clone());
            push_to_gnn_vector_log(record.clone());
            store_replay_event(record);
        }

        // small backoff so we don't spin if the buffer is empty
        std::thread::sleep(Duration::from_millis(50));
    });

    Ok(())
}

pub fn scan_container_activity() -> Vec<TelemetryOutput> {
    let mut outputs = Vec::new();
    let now = now_ts();

    CONTAINER_MONITOR_ONCE.call_once(|| {
        if !CONTAINER_MONITOR_STARTED.load(Ordering::Relaxed) {
            let _ = thread::spawn(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    match start_ebpf_container_exec_monitor().await {
                        Ok(_) => {
                            CONTAINER_MONITOR_STARTED.store(true, Ordering::Relaxed);
                            log_scoped("container_monitor", "✅ container_exec_monitor attached.");

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
                                tags: Some(vec![
                                    "monitor_alive".into(),
                                    "ebpf_monitor_ready".into(),
                                ]),
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

    outputs
}

// ---- helpers for future local enrichment ----

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

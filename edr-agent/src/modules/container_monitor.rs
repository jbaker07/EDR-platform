use std::{
    collections::HashMap,
    fs, io::Read,
    mem,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Once,
    },
    thread,
    time::{SystemTime, UNIX_EPOCH},
};
use std::convert::TryInto;
use std::panic::{AssertUnwindSafe, catch_unwind};

use aya::{
    include_bytes_aligned,
    maps::perf::AsyncPerfEventArray,
    util::online_cpus,
    Bpf,
};
use bytes::BytesMut;
use chrono::Utc;
use tokio::{runtime::Runtime, task};

use crate::{
    forensic::utils::{extract_status_fields, read_cgroup_type},
    telemetry_types::{ContainerExecEvent, TelemetryOutput},
    telemetry_writer::write_telemetry_record,
    trust_hook::{TrustEvent, submit_trust_event},
    gnn_hook::push_to_gnn_vector_log,
    logger::log,
};

use crate::modules::replay_writer::store_replay_event;

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


pub async fn start_ebpf_container_exec_monitor() -> anyhow::Result<()> {
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../ebpf/container_exec_monitor.bpf.o"
    ))?;

    let mut perf_array = AsyncPerfEventArray::try_from(
        bpf.map_mut("CONTAINER_EXEC_EVENTS")?
    )?;

    for cpu_id in online_cpus()? {
        let mut buf = perf_array.open(cpu_id, None)?;

        task::spawn(async move {
            let mut event_buf = vec![BytesMut::with_capacity(4096); 1];

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                tokio::runtime::Handle::current().block_on(async {
                    loop {
                        match buf.read_events(&mut event_buf).await {
                            Ok(events) => {
                                for _ in 0..events.read {
                                    if event_buf[0].len() < mem::size_of::<ContainerExecEventRaw>() {
                                        continue;
                                    }

                                    let raw: ContainerExecEventRaw = unsafe {
                                        std::ptr::read_unaligned(event_buf[0].as_ptr() as *const _)
                                    };

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

                                    let mut record = HashMap::new();
                                    record.insert("pid".into(), container_event.pid.to_string());
                                    record.insert("ppid".into(), container_event.ppid.to_string());
                                    record.insert("uid".into(), container_event.uid.to_string());
                                    record.insert("gid".into(), container_event.gid.to_string());
                                    record.insert("comm".into(), container_event.comm.clone());
                                    record.insert("cmdline".into(), container_event.cmdline.clone());
                                    record.insert("timestamp".into(), container_event.timestamp.to_string());
                                    record.insert("container_type".into(), container_event.container_type.clone());
                                    record.insert("event_type".into(), "container_exec".into());
                                    record.insert("signal".into(), "container_exec".into());
                                    record.insert("category".into(), "container".into());
                                    record.insert("confidence".into(), "0.9".into());
                                    record.insert("gnn_escalate".into(), "true".into());
                                    record.insert("replay_tag".into(), "container_exec_detected".into());

                                    let trust_event = TrustEvent::new_full(
                                        container_event.timestamp,
                                        container_event.pid as i32,
                                        container_event.ppid.try_into().unwrap_or(-1),
                                        container_event.uid,
                                        container_event.comm.clone(),
                                        container_event.cmdline.clone(),
                                        "/proc".into(),
                                        "container_exec".into(),
                                        "container::exec_detect".into(),
                                        "container_monitor".into(),
                                        Some("Containerized process execution detected".into()),
                                        Some("container::exec".into()),
                                        Some(vec!["container".into(), "exec".into(), container_event.comm.clone()]),
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
                                    log(&format!("⚠️ Lost {} container exec events", events.lost));
                                }
                            }
                            Err(e) => {
                                log(&format!("❌ Error reading container perf events: {:?}", e));
                            }
                        }
                    }
                });
            }));

            if result.is_err() {
                log("❌ container_exec_monitor crashed on one core but other CPUs may still be active.");
            }
        });
    }

    Ok(())
}


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

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

use std::cell::RefCell;
use std::rc::Rc;
use std::{sync::{Arc, Mutex}};

pub fn scan_container_activity() -> Vec<TelemetryOutput> {
    let outputs = Arc::new(Mutex::new(Vec::new()));
    let outputs_clone = Arc::clone(&outputs);
    let now = now_ts();

    CONTAINER_MONITOR_ONCE.call_once(|| {
        if !CONTAINER_MONITOR_STARTED.load(Ordering::Relaxed) {
            let _ = thread::spawn(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    match start_ebpf_container_exec_monitor().await {
                        Ok(_) => {
                            CONTAINER_MONITOR_STARTED.store(true, Ordering::Relaxed);
                            println!("✅ container_exec_monitor launched.");

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
                                description: Some("Container eBPF monitor heartbeat confirmation".into()),
                            };
                            submit_trust_event(trust_event);

                            let mut data = HashMap::new();
                            data.insert("event_type".into(), "container_monitor_active".into());
                            data.insert("status".into(), "ebpf_monitor_spawned".into());
                            data.insert("timestamp".into(), now.to_string());
                            data.insert("category".into(), "container".into());
                            data.insert("signal".into(), "container_monitor_active".into());
                            data.insert("replay_tag".into(), "container_monitor_heartbeat".into());

                            let output = TelemetryOutput {
                                category: "container".into(),
                                signal: "container_monitor_active".into(),
                                confidence: 0.01,
                                data,
                            };

                            if let Ok(mut guard) = outputs_clone.lock() {
                                guard.push(output);
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to launch container monitor: {:?}", e);
                        }
                    }
                });
            });
        }
    });
    let result = outputs.lock().unwrap().clone();
    result
}
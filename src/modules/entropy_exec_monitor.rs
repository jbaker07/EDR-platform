use aya::{
    include_bytes_aligned,
    maps::perf::PerfEventArray,
    programs::{Program, TracePoint},
    util::online_cpus,
    Bpf,
};
use bytes::BytesMut;
use chrono::Utc;
use std::{
    collections::HashMap,
    convert::TryInto,
    fs,
    mem,
    ptr::read_unaligned,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};
use crate::telemetry::estimate_entropy;  // not utils::estimate_entropy

use crate::{
    gnn_hook::push_to_gnn_vector_log,
    logger::log,
    telemetry_writer::write_telemetry_record,
    trust_hook::{submit_trust_event, TrustEvent},
};

static MONITOR_STARTED: AtomicBool = AtomicBool::new(false);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EntropyEvent {
    pub pid: u32,
    pub ppid: u32,
    pub filename: [u8; 256],
    pub syscall: [u8; 16],
}

fn cstr_trim(bytes: &[u8]) -> String {
    let n = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..n]).to_string()
}

/// Attach a tracepoint program using a stable fallback mapping derived from its name.
/// (Modern `aya` doesn't expose `Program::section()`; we skip section-based attach.)
fn attach_by_fallback(bpf: &mut Bpf, prog_name: &str) -> anyhow::Result<()> {
    let prog = bpf
        .program_mut(prog_name)
        .ok_or_else(|| anyhow::anyhow!("program '{}' not found in object", prog_name))?;

    let tp: &mut TracePoint = prog.try_into()?;
    tp.load()?;

    let (cat, ev) = match prog_name {
        "trace_execve" => ("syscalls", "sys_enter_execve"),
        "trace_openat" => ("syscalls", "sys_enter_openat"),
        "trace_mmap" => ("syscalls", "sys_enter_mmap"),
        _ => ("syscalls", "sys_enter_execve"),
    };
    tp.attach(cat, ev)?;
    log(&format!(
        "✔️ attached fallback tracepoint {}/{} for {}",
        cat, ev, prog_name
    ));
    Ok(())
}

pub fn start_entropy_exec_monitor() {
    if MONITOR_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    thread::spawn(|| {
        let mut bpf = match Bpf::load(include_bytes_aligned!(
            "../ebpf/entropy_exec_monitor.bpf.o"
        )) {
            Ok(bpf) => bpf,
            Err(e) => {
                log(&format!(
                    "❌ Failed to load entropy_exec_monitor.bpf.o: {:?}",
                    e
                ));
                return;
            }
        };

        // Attach 3 programs (best-effort)
        for name in ["trace_execve", "trace_openat", "trace_mmap"] {
            if let Err(e) = attach_by_fallback(&mut bpf, name) {
                log(&format!("⚠️ Attach failed for {}: {:?}", name, e));
            }
        }

        // Try common map names
        let mut perf_array = match bpf
            .map_mut("events")
            .ok()
            .and_then(|m| PerfEventArray::try_from(m).ok())
            .or_else(|| {
                bpf.map_mut("EVENTS")
                    .ok()
                    .and_then(|m| PerfEventArray::try_from(m).ok())
            }) {
            Some(p) => p,
            None => {
                log("❌ PerfEventArray not found (tried 'events' and 'EVENTS')");
                return;
            }
        };

        // Open on all CPUs
        let cpus = online_cpus().unwrap_or_default();
        for cpu_id in cpus {
            match perf_array.open(cpu_id, None) {
                Ok(mut buf) => {
                    thread::spawn(move || {
                        let mut slots = (0..16)
                            .map(|_| BytesMut::with_capacity(4096))
                            .collect::<Vec<_>>();

                        loop {
                            match buf.read_events(&mut slots) {
                                Ok(events) => {
                                    if events.lost > 0 {
                                        log(&format!(
                                            "⚠️ Lost {} entropy events (CPU {})",
                                            events.lost, cpu_id
                                        ));
                                    }

                                    for slice in &slots[..events.read] {
                                        if slice.len()
                                            < mem::size_of::<EntropyEvent>()
                                        {
                                            continue;
                                        }

                                        let ev: EntropyEvent = unsafe {
                                            read_unaligned(
                                                slice.as_ptr()
                                                    as *const EntropyEvent,
                                            )
                                        };

                                        let filename =
                                            cstr_trim(&ev.filename);
                                        if filename.is_empty() {
                                            continue;
                                        }

                                        let syscall =
                                            cstr_trim(&ev.syscall);

                                        // Guardrail: avoid massive reads
                                        const MAX_BYTES: u64 =
                                            4 * 1024 * 1024;
                                        let entropy_opt = (|| {
                                            let meta = fs::metadata(
                                                &filename,
                                            )
                                            .ok()?;
                                            if !meta.is_file()
                                                || meta.len() > MAX_BYTES
                                            {
                                                return None;
                                            }
                                            let contents =
                                                fs::read(&filename).ok()?;
                                            Some(estimate_entropy(
                                                &contents,
                                            ))
                                        })();

                                        let entropy =
                                            match entropy_opt {
                                                Some(v) => v,
                                                None => {
                                                    // Skip if unreadable/too big/not a file
                                                    continue;
                                                }
                                            };

                                        let severity = if entropy >= 7.9 {
                                            0.95
                                        } else {
                                            0.6
                                        };

                                        let mut data: HashMap<
                                            String,
                                            String,
                                        > = HashMap::new();
                                        data.insert(
                                            "entropy".into(),
                                            format!("{:.4}", entropy),
                                        );
                                        data.insert(
                                            "syscall".into(),
                                            syscall.clone(),
                                        );
                                        data.insert(
                                            "path".into(),
                                            filename.clone(),
                                        );
                                        data.insert(
                                            "timestamp".into(),
                                            Utc::now()
                                                .timestamp()
                                                .to_string(),
                                        );
                                        data.insert(
                                            "pid".into(),
                                            ev.pid.to_string(),
                                        );
                                        data.insert(
                                            "ppid".into(),
                                            ev.ppid.to_string(),
                                        );
                                        data.insert(
                                            "category".into(),
                                            "file".to_string(),
                                        );
                                        data.insert(
                                            "signal".into(),
                                            "entropy_exec_trigger"
                                                .to_string(),
                                        );
                                        data.insert(
                                            "confidence".into(),
                                            format!("{:.2}", severity),
                                        );
                                        data.insert(
                                            "command_line".into(),
                                            format!("[{}]", syscall),
                                        );
                                        data.insert(
                                            "cwd".into(),
                                            "n/a".to_string(),
                                        );

                                        let mut tags = vec![
                                            "high_entropy".to_string(),
                                            format!(
                                                "triggered_by_{}",
                                                syscall
                                            ),
                                            "runtime_risk".to_string(),
                                        ];
                                        if filename.contains("/tmp") {
                                            tags.push(
                                                "temp_path".to_string(),
                                            );
                                        }

                                        let trust_event =
                                            TrustEvent::from_parts(
                                                Utc::now().timestamp()
                                                    as u64,
                                                ev.pid as i32,
                                                ev.ppid as i32,
                                                0, // uid unknown here
                                                filename.clone(),
                                                "EncryptedPayload",
                                                "file",
                                                Some(data.clone()),
                                                severity,
                                                "entropy_exec_monitor",
                                                Some(format!(
                                                    "File with entropy {:.2} observed via {}",
                                                    entropy, syscall
                                                )),
                                                Some(tags),
                                                Some(
                                                    "entropy_exec_trigger"
                                                        .into(),
                                                ),
                                                Some(syscall.clone()),
                                                Some(
                                                    "file_behavior"
                                                        .into(),
                                                ),
                                                Some(
                                                    "entropy_exec_monitor"
                                                        .into(),
                                                ),
                                            );

                                        write_telemetry_record(
                                            data.clone(),
                                        );
                                        push_to_gnn_vector_log(
                                            data.clone(),
                                        );
                                        submit_trust_event(trust_event);
                                    }
                                }
                                Err(e) => {
                                    log(&format!(
                                        "⚠️ perf read error (CPU {}): {:?}",
                                        cpu_id, e
                                    ));
                                    thread::sleep(
                                        Duration::from_millis(50),
                                    );
                                }
                            }
                        }
                    });
                }
                Err(e) => log(&format!(
                    "⚠️ Failed opening perf buffer on CPU {}: {:?}",
                    cpu_id, e
                )),
            }
        }
    });
}

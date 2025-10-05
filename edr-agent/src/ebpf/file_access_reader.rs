// src/ebpf/file_access_reader.rs
#![cfg(target_os = "linux")]

use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use aya::{programs::TracePoint, Bpf};

use crate::events_reader::{self, MapKind};
use crate::pb_adapter;
use forensic_hooks::utils::strnorm::cbytes_to_string_lossy;

use anyhow::{anyhow, Context, Result as AnyResult};

use forensic_hooks::forensic::utils::read_proc_value;
use forensic_hooks::gnn_hook::push_to_gnn_vector_log;
use forensic_hooks::modules::replay_writer::store_replay_event;
use forensic_hooks::telemetry_types::TelemetryOutput;
use forensic_hooks::telemetry_writer::{self, TelemetryWriter};
use log::{error, info};

#[repr(C)]
#[derive(Clone, Copy)]
struct FileAccessEvent {
    pid: u32,
    uid: u32,
    comm: [u8; 64],
    filename: [u8; 256],
}

fn parse_and_emit(buf: &[u8]) -> bool {
    use std::ptr::copy_nonoverlapping;
    if buf.len() < std::mem::size_of::<FileAccessEvent>() {
        return false;
    }

    let mut e = FileAccessEvent {
        pid: 0,
        uid: 0,
        comm: [0; 64],
        filename: [0; 256],
    };
    unsafe {
        copy_nonoverlapping(
            buf.as_ptr(),
            &mut e as *mut _ as *mut u8,
            std::mem::size_of::<FileAccessEvent>(),
        );
    }

    let comm = cbytes_to_string_lossy(&e.comm);
    let path = cbytes_to_string_lossy(&e.filename);

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("pid".into(), e.pid.to_string());
    data.insert("uid".into(), e.uid.to_string());
    data.insert("comm".into(), comm.clone());
    data.insert("file_path".into(), path.clone());

    // Best-effort enrichment
    if let Ok(bin) = read_proc_value(e.pid, "exe") {
        data.insert("binary_path".into(), bin);
    }
    if let Ok(cmd) = read_proc_value(e.pid, "cmdline") {
        data.insert("command_line".into(), cmd);
    }
    if let Ok(cwd) = read_proc_value(e.pid, "cwd") {
        data.insert("cwd".into(), cwd);
    }

    let out = TelemetryOutput {
        category: "file".into(),
        signal: "file_access".into(), // open/openat activity
        confidence: 0.50,
        data,
    };

    pb_adapter::emit_adapter_facts(&out);

    // fan-out
    let mut m = out.data.clone();
    m.insert("category".into(), out.category.clone());
    m.insert("signal".into(), out.signal.clone());
    m.insert("confidence".into(), format!("{:.2}", out.confidence));
    telemetry_writer::write_telemetry_record(m.clone());
    push_to_gnn_vector_log(m.clone());
    store_replay_event(m);

    true
}

/// Attach the BPF programs (Aya path) — called by the PERF reader.
fn attach(bpf: &mut Bpf) -> Result<(), String> {
    // Try full name, then common truncation for kernels that shorten symbols
    let prog_name = if bpf.program("trace_file_access").is_some() {
        "trace_file_access"
    } else if bpf.program("trace_file_acce").is_some() {
        "trace_file_acce"
    } else {
        return Err("missing prog trace_file_access/trace_file_acce".into());
    };

    let tp: &mut TracePoint = bpf
        .program_mut(prog_name)
        .ok_or_else(|| "program lookup failed".to_string())?
        .try_into()
        .map_err(|e| format!("program type cast failed: {e:?}"))?;

    tp.load().map_err(|e| format!("load failed: {e:?}"))?;

    // Attach to the intended tracepoint
    tp.attach("syscalls", "sys_enter_openat")
        .map(|_| ())
        .map_err(|e| format!("attach failed: {e:?}"))
}

/// Entry point used by main.rs (no writer needed; keep signature for compatibility).
pub fn start_file_access_reader(_writer: Arc<Mutex<TelemetryWriter>>) {
    match start() {
        Ok(_) => info!("✅ file_access_reader started (PERF)"),
        Err(e) => error!("⛔ file_access_reader disabled: {e:?}"),
    }
}

/// Actual starter (returns Result so callers can handle boot failures).
pub fn start() -> AnyResult<()> {
    let prefix = std::env::var("EDR_PIN_PREFIX").unwrap_or_else(|_| "/sys/fs/bpf/edr".to_string());
    let force = std::env::var("EDR_FORCE_PINNED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let pin = format!("{}/events", prefix.trim_end_matches('/'));

    if force || super::pins_available(&prefix) {
        if !Path::new(&pin).exists() {
            log::error!("pinned subscribe failed: missing {pin}");
            log::error!("hint: run edr_attach_any -p {prefix}");
            return Err(anyhow!("missing pin: {pin}"));
        }
        log::info!("subscribe: file_access_reader source=perf path={pin}");
        events_reader::start_perf_reader_from_pinned(&pin, |buf| {
            crate::metrics::events_in().fetch_add(1, Ordering::Relaxed);
            if parse_and_emit(buf) {
                crate::metrics::events_accepted().fetch_add(1, Ordering::Relaxed);
            }
        })
        .map_err(|e| anyhow!(e))?;
        return Ok(());
    }

    #[cfg(all(target_os = "linux", feature = "with-ebpf-load"))]
    {
        log::warn!("loader: file_access_reader using .bpf.o (feature explicitly enabled)");

        // Raise RLIMIT_MEMLOCK (best effort)
        unsafe {
            let rlim = libc::rlimit {
                rlim_cur: u64::MAX,
                rlim_max: u64::MAX,
            };
            let _ = libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim as *const _);
        }

        let obj_path = events_reader::resolve_bpf("EDR_BPF_FILE_OBJ", "file_access_monitor.bpf.o");

        match events_reader::detect_events_map_kind(&obj_path).map_err(|e| anyhow!(e))? {
            MapKind::Perf => {}
            other => {
                return Err(anyhow!(
                    "file_access_monitor expects PERF_EVENT_ARRAY, got {:?}",
                    other
                ))
            }
        }

        let bytes =
            std::fs::read(&obj_path).with_context(|| format!("read {}", obj_path.clone()))?;

        events_reader::start_perf_reader_with_attach(&bytes, attach, |buf| {
            crate::metrics::events_in().fetch_add(1, Ordering::Relaxed);
            if parse_and_emit(buf) {
                crate::metrics::events_accepted().fetch_add(1, Ordering::Relaxed);
            }
        })
        .map_err(|e| anyhow!(e))?;
        return Ok(());
    }

    #[cfg(not(all(target_os = "linux", feature = "with-ebpf-load")))]
    {
        log::info!("loader path disabled; build with `--features with-ebpf-load` to enable");
        Ok(())
    }
}

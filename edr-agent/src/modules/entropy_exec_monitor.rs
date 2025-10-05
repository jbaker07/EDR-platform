use crate::telemetry::estimate_entropy;
use crate::{
    gnn_hook::push_to_gnn_vector_log,
    logger::log,
    telemetry_writer::write_telemetry_record,
    trust_hook::{submit_trust_event, TrustEvent},
    utils::strnorm::cbytes_to_string_lossy,
};
use aya::{
    include_bytes_aligned,
    maps::{perf::PerfEventArray, Array, MapData},
    programs::TracePoint,
    util::online_cpus,
    Bpf,
};
use bytes::BytesMut;
use chrono::Utc;
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    convert::TryInto,
    fs, mem,
    ptr::read_unaligned,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
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
    cbytes_to_string_lossy(bytes)
}

/* Return (category, event) for attach */
fn derive_cat_ev(name: &str) -> Option<(&'static str, &'static str)> {
    if name == "raw_syscalls/sys_enter" || name.contains("raw_syscalls/sys_enter") {
        return Some(("raw_syscalls", "sys_enter"));
    }
    if name.contains("syscalls/sys_enter_execve") {
        return Some(("syscalls", "sys_enter_execve"));
    }
    if name.contains("syscalls/sys_enter_openat") {
        return Some(("syscalls", "sys_enter_openat"));
    }
    if name.contains("syscalls/sys_enter_mmap") {
        return Some(("syscalls", "sys_enter_mmap"));
    }
    None
}

fn attach_tracepoint(bpf: &mut Bpf, prog_key: &str, cat: &str, ev: &str) -> Result<(), String> {
    let prog = bpf
        .program_mut(prog_key)
        .ok_or_else(|| format!("program '{}' not found in object", prog_key))?;
    let tp: &mut TracePoint = prog
        .try_into()
        .map_err(|e| format!("program '{}' wrong type: {:?}", prog_key, e))?;
    tp.load()
        .map_err(|e| format!("load '{}' failed: {:?}", prog_key, e))?;
    tp.attach(cat, ev)
        .map_err(|e| format!("attach {}/{} via '{}' failed: {:?}", cat, ev, prog_key, e))?;
    log(&format!(
        "✔️ attached tracepoint {}/{} using key '{}'",
        cat, ev, prog_key
    ));
    Ok(())
}

pub fn start_entropy_exec_monitor() {
    if MONITOR_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    #[cfg(not(feature = "embed_bpf"))]
    {
        log("⚠️ embed_bpf disabled or entropy_exec_monitor.bpf.o missing; skipping attach");
        return;
    }

    #[cfg(feature = "embed_bpf")]
    thread::spawn(|| {
        // Leak BPF so perf-buffer readers can outlive this stack frame.
        let mut tmp = match Bpf::load(include_bytes_aligned!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/ebpf/entropy_exec_monitor.bpf.o"
        ))) {
            Ok(b) => b,
            Err(e) => {
                log(&format!(
                    "❌ Failed to load entropy_exec_monitor.bpf.o: {:?}",
                    e
                ));
                return;
            }
        };
        let bpf: &'static mut Bpf = Box::leak(Box::new(tmp));

        // list programs for visibility
        let prog_names: Vec<String> = bpf.programs().map(|(n, _)| n.to_string()).collect();
        for (i, n) in prog_names.iter().enumerate() {
            log(&format!("entropy_exec_monitor: program[{}]='{}'", i, n));
        }

        // attach only the sections we actually compiled (raw_syscalls/sys_enter)
        let mut attached_any = false;
        for name in &prog_names {
            if let Some((cat, ev)) = derive_cat_ev(name) {
                match attach_tracepoint(bpf, name, cat, ev) {
                    Ok(()) => attached_any = true,
                    Err(e) => log(&format!("⚠️ attach failed for '{}': {}", name, e)),
                }
            }
        }
        if !attached_any {
            log("❌ No tracepoints attached (no matching program names found)");
            return;
        }

        // set aarch64 syscall numbers for execve/openat/mmap into the sysnrs map
        if let Some(m) = bpf.map_mut("sysnrs") {
            if let Ok(mut sysnrs) = Array::<_, u64>::try_from(m) {
                let _ = sysnrs.set(0, 221, 0);
                let _ = sysnrs.set(1, 56, 0);
                let _ = sysnrs.set(2, 222, 0);
                log("entropy_exec_monitor: sysnrs set to execve=221 openat=56 mmap=222");
            } else {
                log("⚠️ sysnrs map present but Array::<_,u64>::try_from failed");
            }
        } else {
            log("⚠️ no sysnrs map in object (will still emit events for execs without filtering)");
        }

        // ---- helper: spawn per-CPU readers; Aya 0.13.1 requires T: BorrowMut<MapData> + Send + Sync + 'static
        fn spawn_readers<T>(mut perf_array: PerfEventArray<T>)
        where
            T: BorrowMut<MapData> + Send + Sync + 'static,
        {
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
                                            if slice.len() < mem::size_of::<EntropyEvent>() {
                                                continue;
                                            }
                                            let ev: EntropyEvent = unsafe {
                                                read_unaligned(slice.as_ptr() as *const EntropyEvent)
                                            };
                                            let filename = cstr_trim(&ev.filename);
                                            if filename.is_empty() {
                                                continue;
                                            }
                                            let syscall = cstr_trim(&ev.syscall);

                                            const MAX_BYTES: u64 = 4 * 1024 * 1024;
                                            let entropy_opt = (|| {
                                                let meta = fs::metadata(&filename).ok()?;
                                                if !meta.is_file() || meta.len() > MAX_BYTES {
                                                    return None;
                                                }
                                                let contents = fs::read(&filename).ok()?;
                                                Some(estimate_entropy(&contents))
                                            })(
                                            );
                                            let entropy = match entropy_opt {
                                                Some(v) => v,
                                                None => continue,
                                            };
                                            let severity: f32 =
                                                if entropy >= 7.9 { 0.95 } else { 0.6 };

                                            let mut data: HashMap<String, String> = HashMap::new();
                                            data.insert(
                                                "entropy".into(),
                                                format!("{:.4}", entropy),
                                            );
                                            data.insert("syscall".into(), syscall.clone());
                                            data.insert("path".into(), filename.clone());
                                            data.insert(
                                                "timestamp".into(),
                                                Utc::now().timestamp().to_string(),
                                            );
                                            data.insert("pid".into(), ev.pid.to_string());
                                            data.insert("ppid".into(), ev.ppid.to_string());
                                            data.insert("category".into(), "file".to_string());
                                            data.insert(
                                                "signal".into(),
                                                "entropy_exec_trigger".to_string(),
                                            );
                                            data.insert(
                                                "confidence".into(),
                                                format!("{:.2}", severity),
                                            );
                                            data.insert(
                                                "command_line".into(),
                                                format!("[{}]", syscall),
                                            );
                                            data.insert("cwd".into(), "n/a".to_string());

                                            let mut tags = vec![
                                                "high_entropy".to_string(),
                                                format!("triggered_by_{}", syscall),
                                                "runtime_risk".to_string(),
                                            ];
                                            if filename.contains("/tmp") {
                                                tags.push("temp_path".to_string());
                                            }

                                            let trust_event = TrustEvent::from_parts(
                                                Utc::now().timestamp() as u64,
                                                ev.pid as i32,
                                                ev.ppid as i32,
                                                0,
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
                                                Some("entropy_exec_trigger".into()),
                                                Some(syscall.clone()),
                                                Some("file_behavior".into()),
                                                Some("entropy_exec_monitor".into()),
                                            );

                                            write_telemetry_record(data.clone());
                                            push_to_gnn_vector_log(data.clone());
                                            submit_trust_event(trust_event);
                                        }
                                    }
                                    Err(e) => {
                                        log(&format!(
                                            "⚠️ perf read error (CPU {}): {:?}",
                                            cpu_id, e
                                        ));
                                        thread::sleep(Duration::from_millis(50));
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
        }

        // ---------- pick ONE map name without taking a mutable borrow twice ----------
        // Prefer immutable inspection to choose the name first.
        let chosen_name = if bpf.map("events").is_some() {
            Some("events")
        } else if bpf.map("EVENTS").is_some() {
            Some("EVENTS")
        } else {
            None
        };

        let Some(map_name) = chosen_name else {
            log("❌ PerfEventArray not found (tried 'events' and 'EVENTS')");
            return;
        };

        // Now take a single mutable borrow and spawn readers.
        if let Some(map) = bpf.map_mut(map_name) {
            match PerfEventArray::try_from(map) {
                Ok(perf_array) => {
                    spawn_readers(perf_array);
                }
                Err(e) => {
                    log(&format!(
                        "❌ Failed to create PerfEventArray from '{}': {:?}",
                        map_name, e
                    ));
                    return;
                }
            }
        } else {
            log(&format!(
                "❌ map '{}' disappeared after selection",
                map_name
            ));
            return;
        }
        // ---------------------------------------------------------------------------
    });
}

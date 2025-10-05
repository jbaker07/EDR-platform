// src/ebpf/proc_lifecycle_reader.rs
//
// CO-RE ringbuf reader for proc_lifecycle.bpf.o (process fork/exec/exit)
// Pattern parity with wx_exec_reader/syscall_reader/file_access_reader:
// - resolve via events_reader::resolve_bpf
// - attach sched_process_{fork,exec,exit} (allow truncated names)
// - select a ringbuf map (prefer edr_events_rb, then events/EVENTS)
// - forward raw bytes to super::ebpf_ingest (no blocking work)
// - leak Link handles + mem::forget(obj); poll at 1ms

#![allow(clippy::needless_return)]
#![cfg(target_os = "linux")]

use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::Duration; // <-- for EVENTS_IN.fetch_add

use forensic_hooks::telemetry_writer::TelemetryWriter;
use libbpf_rs::{Link, ObjectBuilder, RingBufferBuilder};

use crate::ebpf::events_reader;

pub fn start_proc_lifecycle_reader(writer: Arc<Mutex<TelemetryWriter>>) {
    // Raise RLIMIT_MEMLOCK (best effort)
    unsafe {
        let rlim = libc::rlimit {
            rlim_cur: u64::MAX,
            rlim_max: u64::MAX,
        };
        let _ = libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim as *const _);
    }

    let prefix = std::env::var("EDR_PIN_PREFIX").unwrap_or_else(|_| "/sys/fs/bpf/edr".to_string());
    let force = std::env::var("EDR_FORCE_PINNED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let base = prefix.trim_end_matches('/');

    let handler: Arc<dyn Fn(&[u8]) + Send + Sync> = {
        let w = writer.clone();
        Arc::new(move |data: &[u8]| {
            crate::metrics::events_in().fetch_add(1, Ordering::Relaxed);
            if let Some(rec) = super::ebpf_ingest::on_edr_event(data, &w) {
                if let Ok(mut ww) = w.lock() {
                    ww.append(rec);
                }
            }
        })
    };

    if force || super::pins_available(&prefix) {
        let path = format!("{base}/edr_events_rb");
        if !Path::new(&path).exists() {
            log::error!("pinned subscribe failed: missing {path}");
            log::error!("hint: run edr_attach_any -p {prefix}");
            return;
        }
        log::info!("subscribe: proc_lifecycle_reader source=ringbuf path={path}");
        let handler_pinned = handler.clone();
        if let Err(e) = events_reader::start_ringbuf_reader_from_pinned(&path, move |data| {
            handler_pinned.as_ref()(data);
        }) {
            log::error!("pinned subscribe failed for proc_lifecycle_reader: {e}");
        }
        return;
    }

    #[cfg(not(all(target_os = "linux", feature = "with-ebpf-load")))]
    {
        log::info!("loader path disabled; use pins");
        return;
    }

    #[cfg(all(target_os = "linux", feature = "with-ebpf-load"))]
    {
        log::warn!("loader: proc_lifecycle_reader using .bpf.o (feature explicitly enabled)");

        // Honors EDR_BPF_PROC_OBJ and EDR_BPF_DIR
        let obj_path = events_reader::resolve_bpf("EDR_BPF_PROC_OBJ", "proc_lifecycle.bpf.o");

        // Open & load the BPF object
        let open_obj = match ObjectBuilder::default().open_file(&obj_path) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("[ebpf/proc] open {obj_path} failed: {e}");
                return;
            }
        };
        let mut obj = match open_obj.load() {
            Ok(o) => o,
            Err(e) => {
                eprintln!("[ebpf/proc] load failed: {e}");
                return;
            }
        };

        // Attach tracepoints; try common truncated function-name variants.
        let mut links: Vec<Link> = Vec::new();
        let mut attach_tp = |candidates: &[&str], cat: &str, name: &str| {
            for func in candidates {
                if let Some(p) = obj.prog_mut(func) {
                    match p.attach_tracepoint(cat, name) {
                        Ok(l) => {
                            links.push(l);
                            return;
                        }
                        Err(e) => eprintln!("[ebpf/proc] attach {func}@{cat}/{name} failed: {e}"),
                    }
                }
            }
        };

        attach_tp(
            &[
                "tp_sched_process_exec",
                "tp_sched_process_exe",
                "tp_sched_proc_exec",
            ],
            "sched",
            "sched_process_exec",
        );
        attach_tp(
            &[
                "tp_sched_process_fork",
                "tp_sched_process_for",
                "tp_sched_proc_fork",
            ],
            "sched",
            "sched_process_fork",
        );
        attach_tp(
            &[
                "tp_sched_process_exit",
                "tp_sched_process_exi",
                "tp_sched_proc_exit",
            ],
            "sched",
            "sched_process_exit",
        );

        let map_name = match events_reader::choose_single_ringbuf_name(&obj) {
            Some(name) => name,
            None => {
                eprintln!("[ebpf/proc] ❌ ringbuf map not found (edr_events_rb/events/EVENTS) in {obj_path}");
                return;
            }
        };
        eprintln!("[ebpf/proc] subscribing ringbuf map='{map_name}' obj='{obj_path}'");

        let mut map = match obj.map_mut(&map_name) {
            Some(m) => m,
            None => {
                eprintln!("[ebpf/proc] map '{map_name}' vanished after lookup");
                return;
            }
        };

        let mut rb_builder = RingBufferBuilder::new();
        let handler_rb = handler.clone();
        if let Err(e) = rb_builder.add(&mut map, move |data: &[u8]| {
            handler_rb.as_ref()(data);
            0
        }) {
            eprintln!("[ebpf/proc] ❌ ringbuf add failed for map='{map_name}': {e}");
            return;
        }

        let rb = match rb_builder.build() {
            Ok(rb) => rb,
            Err(e) => {
                eprintln!("[ebpf/proc] ❌ ringbuf build failed for map='{map_name}': {e}");
                return;
            }
        };

        let _leaked_links: &'static mut Vec<Link> = Box::leak(Box::new(links));
        std::mem::forget(obj);

        std::thread::spawn(move || loop {
            let _ = rb.poll(Duration::from_millis(1));
        });
    }
}

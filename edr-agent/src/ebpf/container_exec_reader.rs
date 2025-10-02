// src/ebpf/container_exec_reader.rs

#![allow(clippy::needless_return)]
#![cfg(target_os = "linux")]

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::sync::atomic::Ordering;

use libbpf_rs::{Link, MapType, ObjectBuilder, RingBufferBuilder};
use forensic_hooks::telemetry_writer::TelemetryWriter;

use crate::ebpf::events_reader; // only need the module for the PERF helper

pub fn start_container_exec_reader(writer: Arc<Mutex<TelemetryWriter>>) {
    // Best-effort raise memlock
    unsafe {
        let rlim = libc::rlimit { rlim_cur: u64::MAX, rlim_max: u64::MAX };
        let _ = libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim as *const _);
    }

    // Resolve object path (env override → default filename)
    let obj_path = events_reader::resolve_bpf(
        "EDR_BPF_CONTAINER_OBJ",
        "container_exec_monitor.bpf.o",
    );

    // Try RingBuf path by enumerating all ringbuf maps in the object.
    // If none are found, fall back to PERF.
    if !start_ringbuf_all(&obj_path, writer.clone()) {
        start_perf(&obj_path, writer);
    }
}

fn start_ringbuf_all(obj_path: &str, writer: Arc<Mutex<TelemetryWriter>>) -> bool {
    // Open & load
    let open_obj = match ObjectBuilder::default().open_file(obj_path) {
        Ok(o) => o,
        Err(e) => { eprintln!("[ebpf/container_exec] open {obj_path} failed: {e}"); return false; }
    };
    let mut obj = match open_obj.load() {
        Ok(o) => o,
        Err(e) => { eprintln!("[ebpf/container_exec] load failed: {e}"); return false; }
    };

    // Attach whatever exec-related tracepoints exist (tolerate symbol truncation)
    let mut links: Vec<Link> = Vec::new();
    let mut attach_tp = |candidates: &[&str], cat: &str, name: &str| {
        for func in candidates {
            if let Some(p) = obj.prog_mut(func) {
                match p.attach_tracepoint(cat, name) {
                    Ok(l) => { links.push(l); return; }
                    Err(e) => eprintln!("[ebpf/container_exec] attach {func}@{cat}/{name} failed: {e}"),
                }
            }
        }
    };

    attach_tp(&["tp_sched_process_exec", "sched_process_exec", "on_exec"], "sched", "sched_process_exec");
    attach_tp(&["tp_enter_execve", "sys_enter_execve"], "syscalls", "sys_enter_execve");
    attach_tp(&["tp_enter_execveat", "sys_enter_execveat"], "syscalls", "sys_enter_execveat");

    // Enumerate ALL ringbuf maps and subscribe each with its own RingBufferBuilder
    let mut subscribed = 0usize;

    // Collect map names first to avoid mutable borrow issues during iteration
    let ringbuf_names: Vec<String> = obj
        .maps_iter()
        .filter(|m| m.map_type() == MapType::RingBuf)
        .map(|m| m.name().to_string())
        .collect();

    if ringbuf_names.is_empty() {
        eprintln!("[ebpf/container_exec] no ringbuf maps in {obj_path}; will try PERF fallback");
    } else {
        for name in ringbuf_names {
            let mut map = match obj.map_mut(&name) {
                Some(m) => m,
                None => {
                    eprintln!("[ebpf/container_exec] map '{name}' vanished after lookup");
                    continue;
                }
            };

            let mut rb_builder = RingBufferBuilder::new();
            let w = writer.clone();

            rb_builder
                .add(&mut map, move |data: &[u8]| {
                    // Count every raw frame delivered to userspace (pre-filter).
                    crate::EVENTS_IN.fetch_add(1, Ordering::Relaxed);

                    if let Some(rec) = crate::ebpf::ebpf_ingest::on_edr_event(data, &w) {
                        if let Ok(mut ww) = w.lock() { ww.append(rec); }
                    }
                    0 // libbpf-rs expects i32; 0 = OK
                })
                .unwrap_or_else(|e| panic!("[container_exec] ringbuf add({name}) failed: {e}"));
            let rb = rb_builder
                .build()
                .unwrap_or_else(|e| panic!("[container_exec] ringbuf build({name}) failed: {e}"));

            eprintln!("[container_exec] ringbuf subscriber active → {name}");

            // Poll tight in a background thread
            std::thread::Builder::new()
                .name(format!("rb:container_exec:{name}"))
                .spawn(move || loop {
                    let _ = rb.poll(Duration::from_millis(1));
                })
                .expect("spawn ringbuf poll thread");

            subscribed += 1;
        }
    }

    // Keep attachments alive & object pinned if we subscribed anything
    if subscribed > 0 {
        let _leaked_links: &'static mut Vec<Link> = Box::leak(Box::new(links));
        std::mem::forget(obj);
        return true;
    }

    false
}

fn start_perf(obj_path: &str, writer: Arc<Mutex<TelemetryWriter>>) {
    // PERF path expects bytes (Aya loader). Read the file now.
    let bytes = match std::fs::read(obj_path) {
        Ok(b) => b,
        Err(e) => { eprintln!("[ebpf/container_exec] read {obj_path} failed: {e}"); return; }
    };

    // Attach callback (Aya::Bpf)
    fn attach(bpf: &mut aya::Bpf) -> Result<(), String> {
        // Try common names (kernels can truncate)
        let name = if bpf.program_mut("tp_sched_process_exec").is_some() {
            "tp_sched_process_exec"
        } else if bpf.program_mut("tp_enter_execve").is_some() {
            "tp_enter_execve"
        } else if bpf.program_mut("tp_enter_execveat").is_some() {
            "tp_enter_execveat"
        } else {
            return Err("no exec tracepoint program found".into());
        };

        let p = bpf.program_mut(name).ok_or("program vanished")?;
        let tp: &mut aya::programs::TracePoint = p
            .try_into()
            .map_err(|e| format!("program type cast failed: {e:?}"))?;
        tp.load().map_err(|e| format!("load failed: {e:?}"))?;

        // Attach to the corresponding tracepoint
        if name == "tp_sched_process_exec" {
            tp.attach("sched", "sched_process_exec").map_err(|e| format!("attach failed: {e:?}"))?;
        } else if name == "tp_enter_execve" {
            tp.attach("syscalls", "sys_enter_execve").map_err(|e| format!("attach failed: {e:?}"))?;
        } else {
            tp.attach("syscalls", "sys_enter_execveat").map_err(|e| format!("attach failed: {e:?}"))?;
        }
        Ok(())
    }

    // Parser: forward to ingest and APPEND accepted records
    let w = writer.clone();
    let on_evt = move |data: &[u8]| {
        // Count every raw frame delivered to userspace (pre-filter).
        crate::EVENTS_IN.fetch_add(1, Ordering::Relaxed);

        if let Some(rec) = crate::ebpf::ebpf_ingest::on_edr_event(data, &w) {
            if let Ok(mut ww) = w.lock() { ww.append(rec); }
        }
    };

    // Start PERF reader via the unified helper you already use elsewhere
    if let Err(e) = events_reader::start_perf_reader_with_attach(&bytes, attach, on_evt) {
        eprintln!("[ebpf/container_exec] PERF reader failed: {e}");
    } else {
        eprintln!("✅ container_exec_reader started (PERF)");
    }
}

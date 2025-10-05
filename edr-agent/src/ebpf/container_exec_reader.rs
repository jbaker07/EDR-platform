// src/ebpf/container_exec_reader.rs

#![allow(clippy::needless_return)]
#![cfg(target_os = "linux")]

use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use forensic_hooks::telemetry_writer::TelemetryWriter;
use libbpf_rs::{Link, MapType, ObjectBuilder, RingBufferBuilder};

use crate::ebpf::events_reader; // only need the module for the PERF helper

pub fn start_container_exec_reader(writer: Arc<Mutex<TelemetryWriter>>) {
    // Best-effort raise memlock
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
            if let Some(rec) = crate::ebpf::ebpf_ingest::on_edr_event(data, &w) {
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
        log::info!("subscribe: container_exec_reader source=ringbuf path={path}");
        let handler_pinned = handler.clone();
        if let Err(e) = events_reader::start_ringbuf_reader_from_pinned(&path, move |data| {
            handler_pinned.as_ref()(data);
        }) {
            log::error!("pinned subscribe failed for container_exec_reader: {e}");
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
        log::warn!("loader: container_exec_reader using .bpf.o (feature explicitly enabled)");

        let obj_path =
            events_reader::resolve_bpf("EDR_BPF_CONTAINER_OBJ", "container_exec_monitor.bpf.o");

        if !start_ringbuf_all(&obj_path, handler.clone()) {
            start_perf(&obj_path, handler);
        }
    }
}

fn start_ringbuf_all(obj_path: &str, handler: Arc<dyn Fn(&[u8]) + Send + Sync>) -> bool {
    let open_obj = match ObjectBuilder::default().open_file(obj_path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("[ebpf/container_exec] open {obj_path} failed: {e}");
            return false;
        }
    };
    let mut obj = match open_obj.load() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("[ebpf/container_exec] load failed: {e}");
            return false;
        }
    };

    let mut links: Vec<Link> = Vec::new();
    let mut attach_tp = |candidates: &[&str], cat: &str, name: &str| {
        for func in candidates {
            if let Some(p) = obj.prog_mut(func) {
                match p.attach_tracepoint(cat, name) {
                    Ok(l) => {
                        links.push(l);
                        return;
                    }
                    Err(e) => {
                        eprintln!("[ebpf/container_exec] attach {func}@{cat}/{name} failed: {e}")
                    }
                }
            }
        }
    };

    attach_tp(
        &["tp_sched_process_exec", "sched_process_exec", "on_exec"],
        "sched",
        "sched_process_exec",
    );
    attach_tp(
        &["tp_enter_execve", "sys_enter_execve"],
        "syscalls",
        "sys_enter_execve",
    );
    attach_tp(
        &["tp_enter_execveat", "sys_enter_execveat"],
        "syscalls",
        "sys_enter_execveat",
    );

    let ringbuf_names: Vec<String> = obj
        .maps_iter()
        .filter(|m| m.map_type() == MapType::RingBuf)
        .map(|m| m.name().to_string())
        .collect();

    if ringbuf_names.is_empty() {
        eprintln!("[ebpf/container_exec] no ringbuf maps in {obj_path}; will try PERF fallback");
        return false;
    }

    let mut subscribed = 0usize;
    for name in ringbuf_names {
        let mut map = match obj.map_mut(&name) {
            Some(m) => m,
            None => {
                eprintln!("[ebpf/container_exec] map '{name}' vanished after lookup");
                continue;
            }
        };

        let mut rb_builder = RingBufferBuilder::new();
        let handler_rb = handler.clone();

        if let Err(e) = rb_builder.add(&mut map, move |data: &[u8]| {
            handler_rb.as_ref()(data);
            0
        }) {
            eprintln!("[container_exec] ringbuf add({name}) failed: {e}");
            continue;
        }
        match rb_builder.build() {
            Ok(rb) => {
                eprintln!("[container_exec] ringbuf subscriber active → {name}");
                std::thread::Builder::new()
                    .name(format!("rb:container_exec:{name}"))
                    .spawn(move || loop {
                        let _ = rb.poll(Duration::from_millis(1));
                    })
                    .expect("spawn ringbuf poll thread");
                subscribed += 1;
            }
            Err(e) => {
                eprintln!("[container_exec] ringbuf build({name}) failed: {e}");
            }
        }
    }

    if subscribed > 0 {
        let _leaked_links: &'static mut Vec<Link> = Box::leak(Box::new(links));
        std::mem::forget(obj);
        return true;
    }

    false
}

fn start_perf(obj_path: &str, handler: Arc<dyn Fn(&[u8]) + Send + Sync>) {
    let bytes = match std::fs::read(obj_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[ebpf/container_exec] read {obj_path} failed: {e}");
            return;
        }
    };

    fn attach(bpf: &mut aya::Bpf) -> Result<(), String> {
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

        if name == "tp_sched_process_exec" {
            tp.attach("sched", "sched_process_exec")
                .map_err(|e| format!("attach failed: {e:?}"))?;
        } else if name == "tp_enter_execve" {
            tp.attach("syscalls", "sys_enter_execve")
                .map_err(|e| format!("attach failed: {e:?}"))?;
        } else {
            tp.attach("syscalls", "sys_enter_execveat")
                .map_err(|e| format!("attach failed: {e:?}"))?;
        }
        Ok(())
    }

    let handler_perf = handler.clone();
    let on_evt = move |data: &[u8]| {
        handler_perf.as_ref()(data);
    };

    if let Err(e) = events_reader::start_perf_reader_with_attach(&bytes, attach, on_evt) {
        eprintln!("[ebpf/container_exec] PERF reader failed: {e}");
    }
}

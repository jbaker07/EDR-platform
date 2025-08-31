// src/ebpf/events_reader.rs
// Unified helpers to read either a ringbuf ("EVENTS"/"events"/"net_events") via libbpf-rs
// or a perf event array ("EVENTS"/"events") via Aya.
//
// Works with aya 0.13.1 and libbpf-rs 0.21.

#![cfg(target_os = "linux")]

use std::{thread, time::Duration, sync::Arc};
use bytes::BytesMut;

use libbpf_rs::{MapType, ObjectBuilder, RingBufferBuilder};
use aya::{Bpf, maps::perf::PerfEventArray, util::online_cpus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapKind { RingBuf, Perf }

/// Detect whether the object file exposes a ringbuf or perf array named
/// "EVENTS"/"events"/"net_events" (libbpf-rs path).
pub fn detect_events_map_kind(obj_path: &str) -> Result<MapKind, String> {
    let open = ObjectBuilder::default()
        .open_file(obj_path)
        .map_err(|e| e.to_string())?;

    // Try common map names in order (immutable checks)
    for name in ["EVENTS", "events", "net_events"] {
        if let Some(m) = open.map(name) {
            return Ok(match m.map_type() {
                MapType::RingBuf        => MapKind::RingBuf,
                MapType::PerfEventArray => MapKind::Perf,
                other => return Err(format!("Unsupported EVENTS map type: {:?}", other)),
            });
        }
    }
    Err("No EVENTS/events/net_events map found".into())
}

pub fn resolve_bpf(env_single: &str, file: &str) -> String {
    if let Ok(p) = std::env::var(env_single) { return p; }
    if let Ok(dir) = std::env::var("EDR_BPF_DIR") {
        return format!("{}/{}", dir.trim_end_matches('/'), file);
    }
    format!("src/ebpf/{}", file)
}

/* ------------------------- RINGBUF readers ------------------------- */

/// Ringbuf path with **no attaches** (use if your BPF programs are attached elsewhere).
pub fn start_ringbuf_reader_simple(
    obj_path: &str,
    on_evt: impl Fn(&[u8]) + Send + 'static,
) -> Result<(), String> {
    let open = ObjectBuilder::default().open_file(obj_path).map_err(|e| e.to_string())?;
    let mut obj = open.load().map_err(|e| e.to_string())?;

    // Decide map name using immutable queries first
    let map_name = if obj.map("EVENTS").is_some() {
        "EVENTS"
    } else if obj.map("events").is_some() {
        "events"
    } else if obj.map("net_events").is_some() {
        "net_events"
    } else {
        return Err("ringbuf map 'EVENTS'/'events'/'net_events' not found".to_string());
    };

    // Now borrow mutably exactly once
    let mut events_map = obj.map_mut(map_name)
        .ok_or_else(|| format!("ringbuf map '{map_name}' vanished"))?;

    let mut rb = RingBufferBuilder::new();
    rb.add(&mut events_map, move |data: &[u8]| { on_evt(data); 0 })
        .map_err(|e| e.to_string())?;
    let rb = rb.build().map_err(|e| e.to_string())?;

    thread::spawn(move || loop {
        let _ = rb.poll(Duration::from_millis(100));
        thread::sleep(Duration::from_millis(1));
    });
    Ok(())
}

/// Ringbuf path where **you** attach programs via the provided closure.
/// Inside the closure, use `obj.prog_mut("<fn_name>").unwrap().attach_*`.
pub fn start_ringbuf_reader_with_attach(
    obj_path: &str,
    attach: impl Fn(&mut libbpf_rs::Object) -> Result<(), String>,
    on_evt: impl Fn(&[u8]) + Send + 'static,
) -> Result<(), String> {
    let open = ObjectBuilder::default().open_file(obj_path).map_err(|e| e.to_string())?;
    let mut obj = open.load().map_err(|e| e.to_string())?;

    // caller attaches programs explicitly
    attach(&mut obj)?;

    // Decide map name immutably, then mutably borrow once
    let map_name = if obj.map("EVENTS").is_some() {
        "EVENTS"
    } else if obj.map("events").is_some() {
        "events"
    } else if obj.map("net_events").is_some() {
        "net_events"
    } else {
        return Err("ringbuf map 'EVENTS'/'events'/'net_events' not found".to_string());
    };
    let mut events_map = obj.map_mut(map_name)
        .ok_or_else(|| format!("ringbuf map '{map_name}' vanished"))?;

    let mut rb = RingBufferBuilder::new();
    rb.add(&mut events_map, move |data: &[u8]| { on_evt(data); 0 })
        .map_err(|e| e.to_string())?;
    let rb = rb.build().map_err(|e| e.to_string())?;

    thread::spawn(move || loop {
        let _ = rb.poll(Duration::from_millis(100));
        thread::sleep(Duration::from_millis(1));
    });
    Ok(())
}

/* --------------------------- PERF reader --------------------------- */

/// PERF path (Aya 0.13.1): load + attach (caller supplies the attach), then read
/// PERF_EVENT_ARRAY map named "EVENTS" or "events".
/// We leak the Bpf so reader threads can outlive this function frame.
pub fn start_perf_reader_with_attach(
    obj_bytes: &[u8],
    attach: impl Fn(&mut Bpf) -> Result<(), String>,
    on_evt: impl Fn(&[u8]) + Send + Sync + 'static,
) -> Result<(), String> {
    // Load & attach first
    let mut tmp = Bpf::load(obj_bytes).map_err(|e| format!("Bpf::load: {e:?}"))?;
    attach(&mut tmp)?;

    // Leak so perf reader threads can outlive this frame
    let bpf: &'static mut Bpf = Box::leak(Box::new(tmp));

    // Decide the map name immutably (no mutable borrow yet)
    let map_name = if bpf.map("EVENTS").is_some() {
        "EVENTS"
    } else if bpf.map("events").is_some() {
        "events"
    } else {
        return Err("EVENTS/events map not found".to_string());
    };

    // Now do exactly one mutable borrow and convert to PerfEventArray<_>
    let mut perf = {
        let m = bpf
            .map_mut(map_name)
            .ok_or_else(|| format!("map '{map_name}' vanished"))?;
        PerfEventArray::try_from(m)
            .map_err(|_| "map is not a PERF_EVENT_ARRAY".to_string())?
    };

    let cb = Arc::new(on_evt);

    for cpu in online_cpus().unwrap_or_default() {
        match perf.open(cpu, None) {
            Ok(mut buf) => {
                let cb = cb.clone();
                std::thread::spawn(move || {
                    let mut bufs = vec![BytesMut::with_capacity(1024); 32];
                    loop {
                        match buf.read_events(&mut bufs) {
                            Ok(ev) => {
                                for b in &bufs[..ev.read] { cb(b); }
                                if ev.lost > 0 {
                                    eprintln!("⚠️ lost {} events (cpu {})", ev.lost, cpu);
                                }
                            }
                            Err(e) => {
                                eprintln!("perf read error (cpu {}): {:?}", cpu, e);
                                std::thread::sleep(Duration::from_millis(100));
                            }
                        }
                    }
                });
            }
            Err(e) => eprintln!("open perf buffer cpu {} failed: {:?}", cpu, e),
        }
    }
    Ok(())
}

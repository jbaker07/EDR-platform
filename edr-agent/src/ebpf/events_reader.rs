// src/ebpf/events_reader.rs
// Unified helpers to read either a ringbuf ("EVENTS"/"events"/"net_events"/"wx_events") via libbpf-rs
// or a perf event array ("EVENTS"/"events") via Aya.
// Also provides DIRECT readers for pinned maps (no object reloads).
// Works with aya 0.13.1 and libbpf-rs 0.21.

#![cfg(target_os = "linux")]

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use bytes::BytesMut;
use log::{info, trace, warn};

use libbpf_rs::{MapType, ObjectBuilder, RingBufferBuilder};
use aya::{maps::perf::PerfEventArray, util::online_cpus, Bpf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapKind {
    RingBuf,
    Perf,
}

/// A label for the source stream so callers can demux decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    EdrSys,    // edr_events_rb
    Net,       // net_events
    Wx,        // wx_events
    FilePerf,  // events (PERF_EVENT_ARRAY)
}

/* ---------------------------- path helpers ---------------------------- */

fn pin_prefix() -> String {
    // default to canonical; caller can still point EDR_PIN_PREFIX elsewhere
    std::env::var("EDR_PIN_PREFIX").unwrap_or_else(|_| "/sys/fs/bpf/edr".into())
}

fn canonical_and_fallback_paths(map_name: &str) -> [String; 2] {
    let base = pin_prefix();
    [
        format!("{}/{}", base.trim_end_matches('/'), map_name),
        format!("{}/maps/{}", base.trim_end_matches('/'), map_name),
    ]
}

fn bpf_fd_for_path(path: &str) -> Result<std::os::fd::OwnedFd, String> {
    use libbpf_rs::libbpf_sys as sys;
    use std::ffi::CString;
    use std::os::fd::FromRawFd;

    let cpath = CString::new(path).map_err(|e| format!("CString({path}): {e}"))?;
    let fd = unsafe { sys::bpf_obj_get(cpath.as_ptr()) };
    if fd < 0 {
        return Err(format!(
            "bpf_obj_get({path}) failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(unsafe { std::os::fd::OwnedFd::from_raw_fd(fd) })
}

/// Choose a pinned map path, preferring canonical '/sys/fs/bpf/edr/<name>' and
/// falling back to '/sys/fs/bpf/edr/maps/<name>' if the canonical one isn't found.
fn choose_pinned_map_path(map_name: &str) -> Result<(String, std::os::fd::OwnedFd), String> {
    for p in canonical_and_fallback_paths(map_name) {
        if let Ok(fd) = bpf_fd_for_path(&p) {
            return Ok((p, fd));
        }
    }
    Err(format!(
        "pinned map '{}' not found under {}/{{{},maps/{}}}",
        map_name,
        pin_prefix(),
        map_name,
        map_name
    ))
}

/* ------------------ map-kind detection from object file ------------------ */

/// Detect whether the object file exposes a ringbuf or perf array.
/// Checks ENV override first (EDR_NET_RINGBUF), then common map names.
pub fn detect_events_map_kind(obj_path: &str) -> Result<MapKind, String> {
    let open = ObjectBuilder::default()
        .open_file(obj_path)
        .map_err(|e| format!("ObjectBuilder::open_file({obj_path}): {e}"))?;

    // ENV override first (useful for net/wx variants)
    if let Ok(over) = std::env::var("EDR_NET_RINGBUF") {
        trace!("events_reader: EDR_NET_RINGBUF override='{}'", over);
        if let Some(m) = open.map(&over) {
            return Ok(match m.map_type() {
                MapType::RingBuf => MapKind::RingBuf,
                MapType::PerfEventArray => MapKind::Perf,
                other => {
                    return Err(format!(
                        "Unsupported EVENTS map type in {obj_path}:{over}: {:?}",
                        other
                    ))
                }
            });
        } else {
            warn!("events_reader: override map '{}' not found in {}", over, obj_path);
        }
    }

    // Try common map names in order (immutable checks)
    for name in ["EVENTS", "events", "net_events", "wx_events"] {
        if let Some(m) = open.map(name) {
            return Ok(match m.map_type() {
                MapType::RingBuf => MapKind::RingBuf,
                MapType::PerfEventArray => MapKind::Perf,
                other => {
                    return Err(format!(
                        "Unsupported EVENTS map type in {obj_path}:{name}: {:?}",
                        other
                    ))
                }
            });
        }
    }
    Err(format!(
        "No EVENTS/events/net_events/wx_events map found in {}",
        obj_path
    ))
}

pub fn resolve_bpf(env_single: &str, file: &str) -> String {
    if let Ok(p) = std::env::var(env_single) {
        return p;
    }
    if let Ok(dir) = std::env::var("EDR_BPF_DIR") {
        return format!("{}/{}", dir.trim_end_matches('/'), file);
    }
    format!("src/ebpf/{}", file)
}

/* ------------------------- RINGBUF readers (object) ------------------------- */

pub fn choose_single_ringbuf_name(obj: &libbpf_rs::Object) -> Option<String> {
    for env in ["EDR_SYS_RINGBUF", "EDR_NET_RINGBUF", "EDR_WX_RINGBUF"] {
        if let Ok(over) = std::env::var(env) {
            if obj.map(&over).is_some() {
                return Some(over);
            }
        }
    }
    for n in ["wx_events", "net_events", "edr_events_rb", "EVENTS", "events"] {
        if obj.map(n).is_some() {
            return Some(n.to_string());
        }
    }
    None
}

/// Ringbuf path with **no attaches** (use if your BPF programs are attached elsewhere).
pub fn start_ringbuf_reader_simple(
    obj_path: &str,
    on_evt: impl Fn(&[u8]) + Send + 'static,
) -> Result<(), String> {
    trace!("events_reader: opening ringbuf object {}", obj_path);
    let open = ObjectBuilder::default()
        .open_file(obj_path)
        .map_err(|e| format!("ObjectBuilder::open_file({obj_path}): {e}"))?;
    let mut obj = open
        .load()
        .map_err(|e| format!("open.load() {}: {e}", obj_path))?;

    // Decide map name using immutable queries first
    let map_name = choose_single_ringbuf_name(&obj)
        .ok_or_else(|| format!("ringbuf map 'wx_events'/'net_events'/'edr_events_rb'/'EVENTS'/'events' not found in {}", obj_path))?;

    // Now borrow mutably exactly once
    let mut events_map = obj
        .map_mut(&map_name)
        .ok_or_else(|| format!("ringbuf map '{}' vanished in {}", map_name, obj_path))?;

    let mut rb = RingBufferBuilder::new();
    rb.add(&mut events_map, move |data: &[u8]| {
        on_evt(data);
        0
    })
    .map_err(|e| format!("RingBufferBuilder::add {}:{}: {}", obj_path, map_name, e))?;
    let rb = rb
        .build()
        .map_err(|e| format!("RingBufferBuilder::build {}:{}: {}", obj_path, map_name, e))?;
    info!(
        "events_reader: subscribed RINGBUF map='{}' obj='{}'",
        map_name, obj_path
    );

    thread::spawn(move || loop {
        // poll fast to avoid overruns
        let _ = rb.poll(Duration::from_millis(10));
    });
    Ok(())
}

/// Ringbuf path where **you** attach programs via the provided closure.
pub fn start_ringbuf_reader_with_attach(
    obj_path: &str,
    attach: impl Fn(&mut libbpf_rs::Object) -> Result<(), String>,
    on_evt: impl Fn(&[u8]) + Send + 'static,
) -> Result<(), String> {
    trace!("events_reader: opening ringbuf (with attach) object {}", obj_path);
    let open = ObjectBuilder::default()
        .open_file(obj_path)
        .map_err(|e| format!("ObjectBuilder::open_file({obj_path}): {e}"))?;
    let mut obj = open
        .load()
        .map_err(|e| format!("open.load() {}: {e}", obj_path))?;

    attach(&mut obj)?;

    let map_name = choose_single_ringbuf_name(&obj)
        .ok_or_else(|| format!("ringbuf map 'wx_events'/'net_events'/'edr_events_rb'/'EVENTS'/'events' not found in {}", obj_path))?;
    let mut events_map = obj
        .map_mut(&map_name)
        .ok_or_else(|| format!("ringbuf map '{}' vanished in {}", map_name, obj_path))?;

    let mut rb = RingBufferBuilder::new();
    rb.add(&mut events_map, move |data: &[u8]| {
        on_evt(data);
        0
    })
    .map_err(|e| format!("RingBufferBuilder::add {}:{}: {}", obj_path, map_name, e))?;
    let rb = rb
        .build()
        .map_err(|e| format!("RingBufferBuilder::build {}:{}: {}", obj_path, map_name, e))?;
    info!(
        "events_reader: subscribed RINGBUF map='{}' obj='{}'",
        map_name, obj_path
    );

    thread::spawn(move || loop {
        let _ = rb.poll(Duration::from_millis(10));
    });
    Ok(())
}

/* --------------------------- PERF reader (object) --------------------------- */

pub fn start_perf_reader_with_attach(
    obj_bytes: &[u8],
    attach: impl Fn(&mut Bpf) -> Result<(), String>,
    on_evt: impl Fn(&[u8]) + Send + Sync + 'static,
) -> Result<(), String> {
    trace!("events_reader: opening PERF object (Aya)");
    let mut tmp = Bpf::load(obj_bytes).map_err(|e| format!("Bpf::load (PERF): {e:?}"))?;
    attach(&mut tmp)?;
    let bpf: &'static mut Bpf = Box::leak(Box::new(tmp));

    let map_name = if bpf.map("EVENTS").is_some() {
        "EVENTS"
    } else if bpf.map("events").is_some() {
        "events"
    } else {
        return Err("EVENTS/events map not found (PERF path)".to_string());
    };

    let mut perf = {
        let m = bpf
            .map_mut(map_name)
            .ok_or_else(|| format!("map '{map_name}' vanished"))?;
        PerfEventArray::try_from(m).map_err(|_| "map is not a PERF_EVENT_ARRAY".to_string())?
    };

    let cb = Arc::new(on_evt);
    let page_count: usize = std::env::var("EDR_PERF_PAGES")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(256);

    for cpu in online_cpus().unwrap_or_default() {
        match perf.open(cpu, Some(page_count)) {
            Ok(mut buf) => {
                let cb = cb.clone();
                thread::spawn(move || {
                    let mut bufs = vec![BytesMut::with_capacity(4096); 256];
                    loop {
                        match buf.read_events(&mut bufs) {
                            Ok(ev) => {
                                for b in &bufs[..ev.read] {
                                    cb(b.as_ref());
                                }
                                if ev.lost > 0 {
                                    // bump global drop counter
                                    crate::BPF_DROPS.fetch_add(ev.lost as u64, std::sync::atomic::Ordering::Relaxed);
                                    eprintln!("⚠️ lost {} events (cpu {})", ev.lost, cpu);
                                }
                            }
                            Err(e) => {
                                eprintln!("perf read error (cpu {}): {:?}", cpu, e);
                                std::thread::sleep(Duration::from_millis(1));
                            }
                        }

                    }
                });
            }
            Err(e) => eprintln!("open perf buffer cpu {} failed: {:?}", cpu, e),
        }
    }
    info!("events_reader: subscribed PERF_EVENT_ARRAY map='{}'", map_name);
    Ok(())
}

/* -------------------- DIRECT readers (pinned maps) -------------------- */

use std::ffi::{c_void};
use std::os::fd::{AsRawFd, OwnedFd};

/// Subscribe to a **pinned ringbuf map** (by *path*) without re-opening the BPF object.
///
/// Example canonical path candidates:
///   - /sys/fs/bpf/edr/edr_events_rb
///   - /sys/fs/bpf/edr/net_events
///   - /sys/fs/bpf/edr/wx_events
///
/// (We also provide a *name*-based helper that chooses the right path below.)
pub fn start_ringbuf_reader_from_pinned(
    map_path: &str,
    on_evt: impl Fn(&[u8]) + Send + 'static,
) -> Result<(), String> {
    use libbpf_rs::libbpf_sys as sys;

    // libbpf callback expects size: u64
    unsafe extern "C" fn rb_cb(ctx: *mut c_void, data: *mut c_void, size: u64) -> i32 {
        let cb = &mut *(ctx as *mut Box<dyn FnMut(&[u8]) + Send>);
        let slice = std::slice::from_raw_parts(data as *const u8, size as usize);
        cb(slice);
        0
    }

    let fd = bpf_fd_for_path(map_path)?;
    let owned_fd = fd;

    // Box the closure and pass it to C
    let mut cb: Box<dyn FnMut(&[u8]) + Send> = Box::new(on_evt);
    let ctx_ptr = Box::into_raw(Box::new(cb)) as *mut c_void;

    let rb = unsafe {
        sys::ring_buffer__new(
            owned_fd.as_raw_fd(),
            Some(rb_cb),
            ctx_ptr,
            std::ptr::null_mut(),
        )
    };
    if rb.is_null() {
        unsafe { drop(Box::from_raw(ctx_ptr as *mut Box<dyn FnMut(&[u8]) + Send>)); }
        return Err(format!(
            "ring_buffer__new({map_path}) failed: {}",
            std::io::Error::last_os_error()
        ));
    }

    // Keep the FD alive
    let _leak_fd: &'static mut OwnedFd = Box::leak(Box::new(owned_fd));

    // Make the pointer Send by passing its integer value
    let rb_usize = rb as usize;

    // OWN the path string so the thread has 'static data
    let path_owned = map_path.to_string();

    thread::spawn(move || loop {
        let rb_ptr = rb_usize as *mut sys::ring_buffer;
        let rc = unsafe { sys::ring_buffer__poll(rb_ptr, 10) }; // poll faster to avoid overruns
        if rc < 0 {
            eprintln!(
                "❌ ringbuf poll error on {}: {}",
                path_owned,
                std::io::Error::last_os_error()
            );
            thread::sleep(Duration::from_millis(50));
        }
    });

    eprintln!("🧲 ringbuf subscriber active → {map_path}");
    Ok(())
}

/// Name-based convenience: picks canonical path (with fallback) by map name, then subscribes.
pub fn start_ringbuf_reader_from_pinname(
    map_name: &str,
    on_evt: impl Fn(&[u8]) + Send + 'static,
) -> Result<(), String> {
    let (path, _fd) = choose_pinned_map_path(map_name)?;
    // Re-open via path to keep API shape consistent and ensure FD lifetime is held.
    start_ringbuf_reader_from_pinned(&path, on_evt)
}

/* ---------- Shared libbpf PERF callbacks (pinned perf path) ---------- */

unsafe extern "C" fn perf_sample_cb(
    ctx: *mut c_void,
    _cpu: i32,
    data: *mut c_void,
    size: u32,
) {
    let cb = &mut *(ctx as *mut Box<dyn FnMut(&[u8]) + Send>);
    let slice = std::slice::from_raw_parts(data as *const u8, size as usize);
    cb(slice);
}

unsafe extern "C" fn perf_lost_cb(_ctx: *mut c_void, cpu: i32, lost_cnt: u64) {
    // count perf-buffer drops globally if the crate exposes the counter
    crate::BPF_DROPS.fetch_add(lost_cnt, std::sync::atomic::Ordering::Relaxed);
    eprintln!("⚠️ perf lost events on cpu {cpu}: {lost_cnt}");
}

/// Subscribe to a **pinned PERF_EVENT_ARRAY map** (by *path*) without re-opening the BPF object.
///
/// Example canonical path candidates:
///   - /sys/fs/bpf/edr/events
pub fn start_perf_reader_from_pinned(
    map_path: &str,
    on_evt: impl Fn(&[u8]) + Send + 'static,
) -> Result<(), String> {
    use libbpf_rs::libbpf_sys as sys;
    use std::ptr;

    let owned_fd = bpf_fd_for_path(map_path)?;

    let mut cb: Box<dyn FnMut(&[u8]) + Send> = Box::new(on_evt);
    let ctx_ptr = Box::into_raw(Box::new(cb)) as *mut c_void;

    let pages_env: u64 = std::env::var("EDR_PERF_PAGES")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(256);

    let pb = unsafe {
        sys::perf_buffer__new(
            owned_fd.as_raw_fd(),
            pages_env, // pages per CPU (bigger = fewer drops)
            Some(perf_sample_cb),
            Some(perf_lost_cb),
            ctx_ptr,
            ptr::null_mut(),
        )
    };
    if pb.is_null() {
        unsafe { drop(Box::from_raw(ctx_ptr as *mut Box<dyn FnMut(&[u8]) + Send>)); }
        return Err(format!(
            "perf_buffer__new({map_path}) failed: {}",
            std::io::Error::last_os_error()
        ));
    }

    let _leak_fd: &'static mut OwnedFd = Box::leak(Box::new(owned_fd));
    let pb_usize = pb as usize;
    let path_owned = map_path.to_string();

    thread::spawn(move || loop {
        let pb_ptr = pb_usize as *mut sys::perf_buffer;
        let rc = unsafe { sys::perf_buffer__poll(pb_ptr, 10) }; // poll faster
        if rc < 0 {
            eprintln!(
                "❌ perf poll error on {}: {}",
                path_owned,
                std::io::Error::last_os_error()
            );
            thread::sleep(Duration::from_millis(50));
        }
    });

    eprintln!("🧲 perf subscriber active → {map_path}");
    Ok(())
}

/// Name-based convenience: picks canonical path (with fallback) by map name, then subscribes.
pub fn start_perf_reader_from_pinname(
    map_name: &str,
    on_evt: impl Fn(&[u8]) + Send + 'static,
) -> Result<(), String> {
    let (path, _fd) = choose_pinned_map_path(map_name)?;
    start_perf_reader_from_pinned(&path, on_evt)
}

/* -------------------- convenience: start all pinned readers -------------------- */

/// Start all four pinned readers using the pin prefix (EDR_PIN_PREFIX or default).
/// You can either pass *per-map* callbacks, or use `start_all_pinned_readers_demux` below.
pub fn start_all_pinned_readers(
    on_edr_sys: impl Fn(&[u8]) + Send + 'static,
    on_net: impl Fn(&[u8]) + Send + 'static,
    on_wx: impl Fn(&[u8]) + Send + 'static,
    on_file_perf: impl Fn(&[u8]) + Send + 'static,
) -> Result<(), String> {
    start_ringbuf_reader_from_pinname("edr_events_rb", on_edr_sys)?;
    start_ringbuf_reader_from_pinname("net_events", on_net)?;
    start_ringbuf_reader_from_pinname("wx_events", on_wx)?;
    start_perf_reader_from_pinname("events", on_file_perf)?;
    Ok(())
}

/// Same as above, but gives you a single demuxed callback with a `Source` tag.
pub fn start_all_pinned_readers_demux(
    on: impl Fn(Source, &[u8]) + Send + Sync + 'static + Clone,
) -> Result<(), String> {
    let on1 = on.clone();
    start_ringbuf_reader_from_pinname("edr_events_rb", move |b| on1(Source::EdrSys, b))?;
    let on2 = on.clone();
    start_ringbuf_reader_from_pinname("net_events", move |b| on2(Source::Net, b))?;
    let on3 = on.clone();
    start_ringbuf_reader_from_pinname("wx_events", move |b| on3(Source::Wx, b))?;
    start_perf_reader_from_pinname("events", move |b| on(Source::FilePerf, b))?;
    Ok(())
}

/* -------------------- fanout: start object-attach readers -------------------- */

/// Called by main.rs when `EDR_USE_LEGACY_REALTIME=1`.
/// Spins up the four object-attached readers on background threads.
pub fn start_realtime_monitors(
    writer: Arc<Mutex<forensic_hooks::telemetry_writer::TelemetryWriter>>,
    include_file_access: bool,
) {
    use crate::ebpf::container_exec_reader::start_container_exec_reader;
    use crate::file_access_reader::start_file_access_reader;
    use crate::ebpf::proc_lifecycle_reader::start_proc_lifecycle_reader;
    use crate::ebpf::syscall_reader::start_syscall_reader;

    eprintln!("[events_reader] starting realtime monitors (object attach)");

    let w = writer.clone();
    thread::spawn(move || start_syscall_reader(w));

    let w = writer.clone();
    thread::spawn(move || start_proc_lifecycle_reader(w));

    let w = writer.clone();
    thread::spawn(move || start_container_exec_reader(w));

    if include_file_access {
        let w = writer.clone();
        thread::spawn(move || start_file_access_reader(w));
    } else {
        info!("[events_reader] skipping file_access_reader in realtime attach (EDR_USE_LEGACY_REALTIME=0)");
    }
}

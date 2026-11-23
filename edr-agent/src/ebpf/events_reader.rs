// src/ebpf/events_reader.rs
// Readers for the maps edr_attach_any pins: the ring buffers edr_events_rb, net_events and
// wx_events, and the perf array events. They open the pinned maps through libbpf and never
// load a BPF object.

#![cfg(target_os = "linux")]

use std::{thread, time::Duration};

use anyhow::{anyhow, Result as AnyResult};
use std::path::Path;

/// A label for the source stream so callers can demux decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    EdrSys,   // edr_events_rb
    Net,      // net_events
    Wx,       // wx_events
    FilePerf, // events (PERF_EVENT_ARRAY)
}

/* ---------------------------- path helpers ---------------------------- */

pub(crate) fn bpf_fd_for_path(path: &str) -> Result<std::os::fd::OwnedFd, String> {
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

/* -------------------- DIRECT readers (pinned maps) -------------------- */

use std::ffi::c_void;
use std::os::fd::{AsRawFd, OwnedFd};

/// Subscribe to a **pinned ringbuf map** (by *path*) without re-opening the BPF object.
///
/// Example canonical path candidates:
///   - /sys/fs/bpf/edr/edr_events_rb
///   - /sys/fs/bpf/edr/net_events
///   - /sys/fs/bpf/edr/wx_events
pub fn start_ringbuf_reader_from_pinned(
    map_path: &str,
    on_evt: impl Fn(&[u8]) + Send + 'static,
) -> Result<(), String> {
    use libbpf_rs::libbpf_sys as sys;

    // libbpf callback expects size: u64
    unsafe extern "C" fn rb_cb(ctx: *mut c_void, data: *mut c_void, size: u64) -> i32 {
        let cb = &mut *(ctx as *mut Box<dyn FnMut(&[u8]) + Send>);
        let slice = std::slice::from_raw_parts(data as *const u8, size as usize);
        crate::metrics::events_in().fetch_add(1, std::sync::atomic::Ordering::Relaxed);
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
        unsafe {
            drop(Box::from_raw(ctx_ptr as *mut Box<dyn FnMut(&[u8]) + Send>));
        }
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

/* ---------- Shared libbpf PERF callbacks (pinned perf path) ---------- */

unsafe extern "C" fn perf_sample_cb(ctx: *mut c_void, _cpu: i32, data: *mut c_void, size: u32) {
    let cb = &mut *(ctx as *mut Box<dyn FnMut(&[u8]) + Send>);
    let slice = std::slice::from_raw_parts(data as *const u8, size as usize);
    crate::metrics::events_in().fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    cb(slice);
}

unsafe extern "C" fn perf_lost_cb(_ctx: *mut c_void, cpu: i32, lost_cnt: u64) {
    // count perf-buffer drops globally if the crate exposes the counter
    crate::metrics::bpf_drops_total().fetch_add(lost_cnt, std::sync::atomic::Ordering::Relaxed);
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
        unsafe {
            drop(Box::from_raw(ctx_ptr as *mut Box<dyn FnMut(&[u8]) + Send>));
        }
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

/* -------------------- convenience: start all pinned readers -------------------- */

/// Start all four pinned readers using the provided pin prefix.
/// Takes *per-map* callbacks. For one demuxed callback, use `start_all_pinned_readers_demux` below.
pub fn start_all_pinned_readers(
    pin_prefix: &str,
    on_edr_sys: impl Fn(&[u8]) + Send + 'static,
    on_net: impl Fn(&[u8]) + Send + 'static,
    on_wx: impl Fn(&[u8]) + Send + 'static,
    on_file_perf: impl Fn(&[u8]) + Send + 'static,
) -> AnyResult<()> {
    let base = pin_prefix.trim_end_matches('/');
    let hint_prefix = pin_prefix.to_string();

    let ensure = |path: &str| -> AnyResult<()> {
        if Path::new(path).exists() {
            Ok(())
        } else {
            log::error!("pinned subscribe failed: missing {path}");
            log::error!("hint: run edr_attach_any -p {hint_prefix}");
            Err(anyhow!("missing pin: {path}"))
        }
    };

    let edr_path = format!("{base}/edr_events_rb");
    ensure(&edr_path)?;
    log::info!("subscribe: events_reader source=ringbuf path={edr_path}");
    start_ringbuf_reader_from_pinned(&edr_path, on_edr_sys).map_err(|e| anyhow!(e))?;

    let net_path = format!("{base}/net_events");
    ensure(&net_path)?;
    log::info!("subscribe: net_flow_reader source=ringbuf path={net_path}");
    start_ringbuf_reader_from_pinned(&net_path, on_net).map_err(|e| anyhow!(e))?;

    let wx_path = format!("{base}/wx_events");
    ensure(&wx_path)?;
    log::info!("subscribe: wx_exec_reader source=ringbuf path={wx_path}");
    start_ringbuf_reader_from_pinned(&wx_path, on_wx).map_err(|e| anyhow!(e))?;

    let perf_path = format!("{base}/events");
    ensure(&perf_path)?;
    log::info!("subscribe: file_access_reader source=perf path={perf_path}");
    start_perf_reader_from_pinned(&perf_path, on_file_perf).map_err(|e| anyhow!(e))?;

    Ok(())
}

/// Same as above, but with a single demuxed callback that receives a `Source` tag.
pub fn start_all_pinned_readers_demux(
    pin_prefix: &str,
    on: impl Fn(Source, &[u8]) + Send + Sync + 'static + Clone,
) -> AnyResult<()> {
    let on1 = on.clone();
    let on2 = on.clone();
    let on3 = on.clone();
    start_all_pinned_readers(
        pin_prefix,
        move |b| on1(Source::EdrSys, b),
        move |b| on2(Source::Net, b),
        move |b| on3(Source::Wx, b),
        move |b| on(Source::FilePerf, b),
    )
}

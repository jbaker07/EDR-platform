// src/ebpf/syscall_reader.rs
//
// Minimal CO-RE eBPF ringbuf reader for syscall_tracer.bpf.o
// Requires libbpf-rs in Cargo.toml:
//   libbpf-rs = "0.21"
//   libbpf-sys = "1.*"
// Also ensure rlimit memlock is raised and BTF available at runtime.

#![allow(clippy::needless_return)]

use std::collections::HashMap;
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::{fs, thread, time::Duration};

use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::{TelemetryWriter, write_telemetry_record};
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::replay_writer::store_replay_event;

#[cfg(target_os = "linux")]
mod linux_impl {
    use super::*;
    use libbpf_rs::{MapFlags, ObjectBuilder, RingBufferBuilder};

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct SyscallEvent {
        pid: u32,
        syscall_id: u32,
    }

    fn syscall_to_tag(sys: u32) -> (&'static str, &'static str) {
        // (category, signal) — mapped to trust dimensions downstream
        match sys as i64 {
            libc::SYS_execve | libc::SYS_execveat => ("process", "process::exec"),
            libc::SYS_open | libc::SYS_openat | libc::SYS_rename | libc::SYS_unlink | libc::SYS_unlinkat => ("file", "file::fs_mutation"),
            libc::SYS_connect => ("network", "network::connect"),
            libc::SYS_read | libc::SYS_write => ("process", "process::io"),
            libc::SYS_clone | libc::SYS_fork | libc::SYS_vfork => ("process", "process::spawn"),
            libc::SYS_exit | libc::SYS_exit_group => ("process", "process::exit"),
            libc::SYS_setuid | libc::SYS_setreuid | libc::SYS_setresuid => ("privilege", "privilege::setuid"),
            libc::SYS_mount | libc::SYS_umount2 => ("privilege", "privilege::mount"),
            _ => ("process", "process::syscall"),
        }
    }

    fn read_proc_exe(pid: u32) -> Option<String> {
        let p = PathBuf::from(format!("/proc/{pid}/exe"));
        fs::read_link(p).ok().map(|p| p.to_string_lossy().to_string())
    }
    fn read_proc_cmdline(pid: u32) -> Option<String> {
        let p = PathBuf::from(format!("/proc/{pid}/cmdline"));
        fs::read(p).ok().map(|bytes| {
            let mut s = String::from_utf8_lossy(&bytes).to_string();
            s = s.replace('\0', " ").trim().to_string();
            s
        })
    }
    fn read_proc_cwd(pid: u32) -> Option<String> {
        let p = PathBuf::from(format!("/proc/{pid}/cwd"));
        fs::read_link(p).ok().map(|p| p.to_string_lossy().to_string())
    }

    pub fn start(writer: Arc<Mutex<TelemetryWriter>>) -> anyhow::Result<()> {
        // Raise memlock (best-effort)
        let rlim = libc::rlimit { rlim_cur: u64::MAX, rlim_max: u64::MAX };
        unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim as *const _) };

        // Open & load BPF object
        let obj_path = std::env::var("EDR_BPF_SYSCALL_OBJ")
            .unwrap_or_else(|_| "syscall_tracer.bpf.o".into());
        let open_obj = ObjectBuilder::default().open_file(&obj_path)?;
        let mut obj = open_obj.load()?;

        // Attach all tracepoint programs by parsing section names
        for prog in obj.progs_mut() {
            let sec = prog.section().unwrap_or_default();
            if let Some(rest) = sec.strip_prefix("tracepoint/") {
                if let Some((cat, name)) = rest.split_once('/') {
                    prog.attach_tracepoint(cat, name)?;
                }
            }
        }

        // Ring buffer setup
        let mut rb = RingBufferBuilder::new();
        let mut burst_guard: std::collections::HashMap<u32, u64> = HashMap::new();

        let mut map = obj.map_mut("events")?;
        rb.add(&mut map, move |data: &[u8]| {
            if data.len() < std::mem::size_of::<SyscallEvent>() { return 0; }
            let mut ev = SyscallEvent { pid: 0, syscall_id: 0 };
            unsafe {
                std::ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    &mut ev as *mut _ as *mut u8,
                    std::mem::size_of::<SyscallEvent>(),
                );
            }

            // basic burst guard per (pid) 10ms
            let now_ms = monotonic_ms();
            if let Some(last) = burst_guard.get(&ev.pid) {
                if now_ms.saturating_sub(*last) < 10 {
                    return 0;
                }
            }
            burst_guard.insert(ev.pid, now_ms);

            let (category, signal) = syscall_to_tag(ev.syscall_id);

            // Enrich from /proc (best effort)
            let mut data = HashMap::new();
            data.insert("pid".into(), ev.pid.to_string());
            data.insert("syscall_id".into(), ev.syscall_id.to_string());

            if let Some(exe) = read_proc_exe(ev.pid)       { data.insert("binary_path".into(), exe); }
            if let Some(cmd) = read_proc_cmdline(ev.pid)   { data.insert("command_line".into(), cmd); }
            if let Some(cwd) = read_proc_cwd(ev.pid)       { data.insert("cwd".into(), cwd); }

            let out = TelemetryOutput {
                category: category.to_string(),
                signal: signal.to_string(),
                confidence: 0.85,
                data,
            };

            // Fan into existing pipelines exactly like other realtime modules
            let mut map_line = out.data.clone();
            map_line.insert("category".into(), out.category.clone());
            map_line.insert("signal".into(), out.signal.clone());
            map_line.insert("confidence".into(), out.confidence.to_string());

            write_telemetry_record(map_line.clone());
            push_to_gnn_vector_log(map_line.clone());
            store_replay_event(map_line);

            // (Optional) If you want to push into the writer buffer, do it here:
            if let Ok(mut w) = writer.lock() {
                // If TelemetryWriter exposes a direct event API, call it; else ignore.
                let _ = &mut *w;
            }
            0
        })?;

        let rb = rb.build()?;
        let fd = rb.as_raw_fd();
        log_if("ebpf/syscall_reader", &format!("ringbuf fd={fd} attached OK"));

        // Poll loop
        thread::spawn(move || {
            loop {
                // 100 ms poll; libbpf returns number of records consumed
                let _ = rb.poll(100);
            }
        });

        Ok(())
    }

    #[inline]
    fn monotonic_ms() -> u64 {
        use std::time::Instant;
        static START: once_cell::sync::Lazy<Instant> = once_cell::sync::Lazy::new(Instant::now);
        START.elapsed().as_millis() as u64
    }

    fn log_if(_c: &str, _m: &str) {
        // hook into your logger if you want:
        // crate::logger::log(_c, _m);
        let _ = (_c, _m);
    }
}

#[cfg(not(target_os = "linux"))]
mod linux_impl {
    use super::*;
    pub fn start(_writer: Arc<Mutex<TelemetryWriter>>) -> anyhow::Result<()> {
        // No-op on non-Linux targets
        Ok(())
    }
}

pub fn start_syscall_reader(writer: Arc<Mutex<TelemetryWriter>>) {
    #[cfg(target_os = "linux")]
    {
        if let Err(e) = linux_impl::start(writer) {
            eprintln!("[ebpf/syscall_reader] failed: {e}");
        }
    }
}

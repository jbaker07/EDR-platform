// src/ebpf/raw_syscalls_reader.rs
// eBPF reader for raw_syscalls:sys_enter/sys_exit that feeds TelemetryWriter.

#![allow(dead_code)]

#[cfg(target_os = "linux")]
mod linux {
    use std::{mem, ptr, thread, time::Duration, sync::{Arc, Mutex}};

    use aya::{
        include_bytes_aligned,
        Bpf,
        maps::ring_buf::RingBuf,
        programs::TracePoint,
    };
    use chrono::Utc;

    use crate::telemetry_writer::TelemetryWriter;
    use crate::telemetry::TelemetryRecord;

    // Keep this layout in sync with your BPF event struct.
    // Typical sys_enter payload: pid, uid, syscall_id, ts_ns, args[6]
    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct SysEnterEvent {
        pub pid: u32,
        pub uid: u32,
        pub syscall_id: u32,
        pub _pad: u32,
        pub ts_ns: u64,
        pub args: [u64; 6],
    }

    fn syscall_name(id: u32) -> &'static str {
        // Minimal common map; fallback below.
        match id {
            59 => "execve",
            322 | 221 => "execveat",
            56 => "clone",
            57 => "fork",
            58 => "vfork",
            2  => "open",
            257 => "openat",
            3  => "close",
            0  => "read",
            1  => "write",
            257 => "openat",
            257 => "openat",
            42 => "connect",
            41 => "socket",
            9  => "mmap",
            10 => "mprotect",
            101 => "ptrace",
            321 | 321_u32 => "bpf",
            _ => "syscall",
        }
    }

    fn set_memlock_rlimit_unlimited() {
        unsafe {
            // Use libc directly to avoid new deps.
            let rlim = libc::rlimit { rlim_cur: libc::RLIM_INFINITY, rlim_max: libc::RLIM_INFINITY };
            let _ = libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim as *const libc::rlimit);
        }
    }

    /// Spawn the monitor thread. Loads `raw_syscalls_catcher.bpf.o`,
    /// attaches to raw_syscalls tracepoints, and forwards ringbuf events
    /// as TelemetryRecord rows into TelemetryWriter.
    pub fn spawn(writer: Arc<Mutex<TelemetryWriter>>) -> anyhow::Result<std::thread::JoinHandle<()>> {
        set_memlock_rlimit_unlimited();

        // Adjust the path if you keep the .o elsewhere.
        let obj = include_bytes_aligned!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/src/ebpf/raw_syscalls_catcher.bpf.o"));

        let mut bpf = Bpf::load(obj)?;

        // Attach sys_enter
        {
            let prog = bpf.program_mut("raw_sys_enter")
                .ok_or_else(|| anyhow::anyhow!("program raw_sys_enter not found"))?;
            let tp: &mut TracePoint = prog.try_into()?;
            tp.load()?;
            tp.attach("raw_syscalls", "sys_enter")?;
        }

        // Attach sys_exit if you emit it from BPF (optional)
        if let Some(prog) = bpf.program_mut("raw_sys_exit") {
            let tp: &mut TracePoint = prog.try_into()?;
            tp.load()?;
            // raw_syscalls/sys_exit may not exist on some kernels; ignore attach error softly.
            let _ = tp.attach("raw_syscalls", "sys_exit");
        }

        // Open ring buffer named "events"
        let mut rb = RingBuf::try_from(bpf.map_mut("events")
            .ok_or_else(|| anyhow::anyhow!("ringbuf map 'events' not found"))?)?;

        // Keep BPF object alive in thread by moving it in.
        let handle = thread::spawn(move || {
            // bpf is moved here to keep programs attached for thread lifetime
            let _keep_bpf_alive = bpf;

            loop {
                let _ = rb.poll(Duration::from_millis(1), |data| {
                    // SAFETY: event comes from BPF with this fixed layout.
                    if data.len() < mem::size_of::<SysEnterEvent>() { return; }
                    let ev = unsafe { ptr::read_unaligned(data.as_ptr() as *const SysEnterEvent) };

                    // Build a minimal TelemetryRecord (we enrich later)
                    let mut rec = TelemetryRecord {
                        timestamp: (Utc::now().timestamp_millis()) as u64,
                        pid: ev.pid as i32,
                        ppid: 0,
                        uid: ev.uid,
                        binary_path: String::new(),
                        command_line: String::new(),
                        cwd: String::new(),
                        tags: vec![format!("rawsys:{}", syscall_name(ev.syscall_id))],
                        risk_score: None,
                        ..Default::default()
                    };

                    // Decorate a bit: mark “interesting” syscalls
                    match syscall_name(ev.syscall_id) {
                        "execve" | "execveat" | "ptrace" | "bpf" | "mprotect" | "socket" | "connect" => {
                            rec.tags.push("sys_interest".into());
                        }
                        _ => {}
                    }

                    if let Ok(mut w) = writer.lock() {
                        // Use the existing API; push a single-element batch.
                        w.append_batch(vec![rec]);
                    }
                });

                // Yield lightly to avoid busy spin in quiet periods
                thread::sleep(Duration::from_millis(10));
            }
        });

        Ok(handle)
    }
}

#[cfg(target_os = "linux")]
pub use linux::spawn;

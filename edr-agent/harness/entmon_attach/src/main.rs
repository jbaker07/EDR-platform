use anyhow::{anyhow, Context, Result};
use aya::{
    maps::{perf::PerfEventArray, Array},
    programs::TracePoint,
    util::online_cpus,
    Ebpf, Pod,
};
use bytes::BytesMut;
use libc::{SYS_execve, SYS_mmap, SYS_openat};
use log::info;
use std::{mem, path::Path, thread, time::Duration};

#[repr(C)]
#[derive(Copy, Clone)]
struct EntropyEvent {
    // Match kernel order: pid, ppid, filename[256], syscall_name[8]
    pid: u32,
    ppid: u32,
    filename: [u8; 256], // may be empty for mmap; kernel may stuff syscall name here sometimes
    syscall: [u8; 8],    // "open", "openat", "mmap", "execve", etc.
}
unsafe impl Pod for EntropyEvent {}

fn find_bpf_obj() -> Result<String> {
    let candidates = [
        "src/ebpf/entropy_exec_monitor.bpf.o",
        "../../src/ebpf/entropy_exec_monitor.bpf.o",
        "../src/ebpf/entropy_exec_monitor.bpf.o",
    ];
    for c in candidates {
        if Path::new(c).exists() {
            return Ok(c.to_string());
        }
    }
    Err(anyhow!(
        "Could not find entropy_exec_monitor.bpf.o in expected locations"
    ))
}

// Proper C-string decoder: stop at first NUL byte
fn cstr(bytes: &[u8]) -> &str {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    std::str::from_utf8(&bytes[..end]).unwrap_or("")
}

fn print_event(e: &EntropyEvent) {
    let file = cstr(&e.filename);
    let sc = cstr(&e.syscall);
    // Prefer filename if present; otherwise show syscall name (mmap won’t have a path)
    if !file.is_empty() {
        println!("pid={} ppid={} syscall={} file={}", e.pid, e.ppid, sc, file);
    } else {
        println!("pid={} ppid={} syscall={}", e.pid, e.ppid, sc);
    }
}

fn main() -> Result<()> {
    env_logger::init();

    // --- Load & attach ---
    let obj = find_bpf_obj()?;
    let mut bpf = Ebpf::load_file(&obj).with_context(|| format!("loading BPF object at {obj}"))?;

    let prog: &mut TracePoint = bpf
        .program_mut("on_sys_enter")
        .ok_or_else(|| anyhow!("program 'on_sys_enter' not found in BPF object"))?
        .try_into()
        .context("cast to TracePoint")?;
    prog.load().context("TracePoint load (verifier log likely in this error)")?;
    prog.attach("raw_syscalls", "sys_enter")
        .context("attach raw_syscalls/sys_enter")?;
    info!("attached raw_syscalls/sys_enter");

    // Optional: if your kernel code checks numeric filters, keep these; harmless otherwise.
    let mut sysnrs: Array<_, u64> = Array::try_from(
        bpf.map_mut("sysnrs")
            .ok_or_else(|| anyhow!("map 'sysnrs' not found"))?,
    )
    .context("open sysnrs map")?;
    sysnrs.set(0, SYS_execve as u64, 0)?;
    sysnrs.set(1, SYS_openat as u64, 0)?;
    sysnrs.set(2, SYS_mmap as u64, 0)?;
    info!("initialized sysnrs: execve/openat/mmap");

    // Open perf readers (sync)
    let mut events = PerfEventArray::try_from(
        bpf.map_mut("events")
            .ok_or_else(|| anyhow!("map 'events' not found"))?,
    )
    .context("open events perf array")?;

    let cpus = online_cpus().map_err(|(m, e)| anyhow!("online_cpus failed: {m}: {e}"))?;
    let mut bufs = Vec::new();
    for cpu in cpus {
        let buf = events.open(cpu, None).with_context(|| format!("open perf buf cpu {cpu}"))?;
        bufs.push((cpu, buf));
    }
    info!("opened {} perf buffers", bufs.len());

    // Poll (simple harness)
    loop {
        for (_cpu, buf) in bufs.iter_mut() {
            let mut chunks = (0..16)
                .map(|_| BytesMut::with_capacity(1024))
                .collect::<Vec<_>>();
            match buf.read_events(&mut chunks) {
                Ok(ev) => {
                    for i in 0..ev.read {
                        if chunks[i].len() >= mem::size_of::<EntropyEvent>() {
                            // SAFETY: kernel wrote an EntropyEvent
                            let e: &EntropyEvent =
                                unsafe { &*(chunks[i].as_ptr() as *const EntropyEvent) };
                            print_event(e);
                        }
                    }
                }
                Err(e) => eprintln!("perf read error: {e}"),
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

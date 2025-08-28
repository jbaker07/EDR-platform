use anyhow::{Context, Result};
use aya::{
    Bpf,
    maps::{perf::AsyncPerfEventArray, Array},
    programs::TracePoint,
    util::online_cpus,
    Pod,
};
use bytes::BytesMut;
use libc::{SYS_execve, SYS_openat, SYS_mmap};
use log::{error, info};
use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
struct EntropyEvent {
    pid: u32,
    ppid: u32,
    syscall: u32,         // raw syscall nr
    filename: [u8; 256],  // null-terminated
}
unsafe impl Pod for EntropyEvent {}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    env_logger::init();

    // Load the BPF object
    let mut bpf = Bpf::load_file("src/ebpf/entropy_exec_monitor.bpf.o")
        .context("loading BPF object")?;

    // Cast & attach the tracepoint
    let prog: &mut TracePoint = bpf.program_mut("on_sys_enter")?
        .try_into()
        .context("cast to TracePoint")?;
    prog.load().context("TracePoint load (verifier log likely in this error)")?;
    prog.attach("raw_syscalls", "sys_enter")
        .context("attach raw_syscalls/sys_enter")?;
    info!("attached raw_syscalls/sys_enter");

    // Initialize syscall filter map (idx 0=execve, 1=openat, 2=mmap)
    let mut sysnrs: Array<_, u64> = Array::try_from(bpf.map_mut("sysnrs")?)
        .context("open sysnrs map")?;
    sysnrs.set(0, SYS_execve as u64, 0)?;
    sysnrs.set(1, SYS_openat as u64, 0)?;
    sysnrs.set(2, SYS_mmap as u64, 0)?;
    info!("initialized sysnrs");

    // Open perf readers (one per CPU)
    let mut events = AsyncPerfEventArray::try_from(bpf.map_mut("events")?)
        .context("open events perf array")?;
    for cpu in online_cpus().context("enumerate online CPUs")? {
        let mut buf = events.open(cpu, None)
            .with_context(|| format!("open perf buf cpu {cpu}"))?;
        tokio::spawn(async move {
            let mut bufs = (0..16).map(|_| BytesMut::with_capacity(1024)).collect::<Vec<_>>();
            loop {
                match buf.read_events(&mut bufs).await {
                    Ok(ev) => {
                        for i in 0..ev.read {
                            if bufs[i].len() >= mem::size_of::<EntropyEvent>() {
                                // SAFETY: kernel wrote an EntropyEvent
                                let e: &EntropyEvent = unsafe { &*(bufs[i].as_ptr() as *const EntropyEvent) };
                                let fname = std::str::from_utf8(&e.filename).unwrap_or("")
                                    .trim_end_matches('\0');
                                println!("pid={} ppid={} sysnr={} file={}",
                                         e.pid, e.ppid, e.syscall, fname);
                            }
                        }
                    }
                    Err(e) => {
                        error!("perf read error cpu {cpu}: {e}");
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    }
                }
            }
        });
    }

    tokio::signal::ctrl_c().await?;
    Ok(())
}

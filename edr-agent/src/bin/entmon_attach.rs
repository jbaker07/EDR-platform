use anyhow::{anyhow, Context, Result};
use aya::{
    maps::{perf::AsyncPerfEventArray, Array},
    programs::TracePoint,
    util::online_cpus,
    Ebpf,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bpf =
        Ebpf::load_file("src/ebpf/entropy_exec_monitor.bpf.o").context("load ebpf obj")?;

    // program_mut returns Option, so map None to an error
    let prog: &mut TracePoint = bpf
        .program_mut("on_sys_enter")
        .ok_or_else(|| anyhow!("program 'on_sys_enter' not found"))?
        .try_into()?;

    // same for maps:
    let mut sysnrs: Array<_, u64> = Array::try_from(
        bpf.map_mut("sysnrs")
            .ok_or_else(|| anyhow!("map 'sysnrs' not found"))?,
    )?;

    // and perf events:
    let mut events = AsyncPerfEventArray::try_from(
        bpf.map_mut("events")
            .ok_or_else(|| anyhow!("map 'events' not found"))?,
    )?;

    // online_cpus returns a (message, io::Error) tuple, so map it into anyhow
    let cpus = online_cpus().map_err(|(m, e)| anyhow!("online_cpus failed: {m}: {e}"))?;
    Ok(())
}

// TOP OF FILE
use anyhow::{anyhow, Context, Result};
use aya::{
    maps::{perf::AsyncPerfEventArray, Array},
    programs::TracePoint,
    util::online_cpus,
    Ebpf, // <- replace Bpf with Ebpf
};

// where you loaded the object file
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bpf =
        Ebpf::load_file("src/ebpf/entropy_exec_monitor.bpf.o").context("load ebpf obj")?;

    // getting a program by name returns Option; don't use `?` on Option
    let prog: &mut TracePoint = bpf
        .program_mut("on_sys_enter")
        .ok_or_else(|| anyhow!("program 'on_sys_enter' not found"))?
        .try_into()?; // keep your .load()/.attach() sequence below

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

    // online_cpus doesn't implement anyhow::Context because of its error type; map it
    let cpus = online_cpus().map_err(|(m, e)| anyhow!("online_cpus failed: {m}: {e}"))?;
    Ok(())
}

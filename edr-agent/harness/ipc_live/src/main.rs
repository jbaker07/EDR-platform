//! Live check for src/modules/suspicious_ipc.rs, using the agent's own code.
//!
//!   ipc_live watch <bpf_obj> <seconds>
//!       Every record the kernel sends, with the allowlist verdict. Shows what
//!       the BPF side filters out and what the allowlist lets through.
//!   ipc_live ptrace <seconds>
//!       Runs PtraceScanner over /proc once a second and prints every new
//!       attachment with the debugger allowlist verdict.
//!   ipc_live agent <seconds>
//!       Calls scan_ipc_activity() once a second, the same per cycle call the
//!       agent makes, and prints each TelemetryOutput it returns. Set
//!       EDR_BPF_IPC_OBJ to the object path first.
//!
//! Needs root, bpffs and tracefs, and the host pid namespace so the pids in
//! events match /proc.

use std::path::Path;
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};
use forensic_hooks::modules::suspicious_ipc as ipc;

fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    match (args.get(1).map(String::as_str), args.len()) {
        (Some("watch"), 4) => watch(&args[2], seconds(&args[3])?),
        (Some("ptrace"), 3) => ptrace(seconds(&args[2])?),
        (Some("agent"), 3) => agent(seconds(&args[2])?),
        _ => bail!(
            "usage: ipc_live watch <bpf_obj> <seconds> | ipc_live ptrace <seconds> | ipc_live agent <seconds>"
        ),
    }
}

fn seconds(arg: &str) -> Result<Duration> {
    let secs: u64 = arg
        .parse()
        .with_context(|| format!("bad seconds value {arg:?}"))?;
    Ok(Duration::from_secs(secs))
}

fn watch(obj_path: &str, run_for: Duration) -> Result<()> {
    println!(
        "[watch] loading {obj_path}, {} watched paths, running {}s",
        ipc::RUNTIME_SOCKETS.len(),
        run_for.as_secs()
    );
    let deadline = Instant::now() + run_for;
    ipc::run_socket_watch(
        obj_path,
        ipc::RUNTIME_SOCKETS,
        || Instant::now() < deadline,
        |event| {
            let comm = event.comm();
            let verdict = match ipc::allowlist_match(&comm, ipc::RUNTIME_CLIENT_ALLOWLIST) {
                Some(reason) => format!("ALLOWED ({reason})"),
                None => "FLAGGED".to_string(),
            };
            let alert = ipc::SocketAlert::new(*event, Path::new("/proc"));
            println!(
                "[watch] {verdict} pid={} tid={} uid={} comm={comm} path={} exe={}",
                event.tgid,
                event.tid,
                event.uid,
                event.path(),
                alert.exe.as_deref().unwrap_or("(exited)")
            );
        },
    )?;
    println!("[watch] done");
    Ok(())
}

fn ptrace(run_for: Duration) -> Result<()> {
    println!(
        "[ptrace] scanning /proc once a second for {}s",
        run_for.as_secs()
    );
    let deadline = Instant::now() + run_for;
    let mut scanner = ipc::PtraceScanner::default();
    while Instant::now() < deadline {
        for a in scanner.scan(Path::new("/proc"), ipc::MAX_PIDS_PER_SCAN) {
            let verdict = match a.allowed_by {
                Some(reason) => format!("ALLOWED ({reason})"),
                None => "FLAGGED".to_string(),
            };
            println!(
                "[ptrace] {verdict} traced_pid={} traced_comm={} tracer_pid={} tracer_comm={} tracer_uid={} tracer_exe={}",
                a.traced_pid,
                a.traced_comm,
                a.tracer_pid,
                a.tracer_comm,
                a.tracer_uid.map_or("?".to_string(), |u| u.to_string()),
                a.tracer_exe.as_deref().unwrap_or("?")
            );
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    println!("[ptrace] done");
    Ok(())
}

fn agent(run_for: Duration) -> Result<()> {
    println!(
        "[agent] scan_ipc_activity() once a second for {}s, EDR_BPF_IPC_OBJ={}",
        run_for.as_secs(),
        std::env::var("EDR_BPF_IPC_OBJ").unwrap_or_else(|_| "(unset)".into())
    );
    let deadline = Instant::now() + run_for;
    let mut cycle = 0u32;
    while Instant::now() < deadline {
        cycle += 1;
        let outputs = ipc::scan_ipc_activity();
        println!("[agent] cycle={cycle} outputs={}", outputs.len());
        for out in outputs {
            let get = |k: &str| out.data.get(k).cloned().unwrap_or_default();
            println!(
                "[agent] cycle={cycle} signal={} confidence={:.2} pid={} uid={} exe={} reason={}",
                out.signal,
                out.confidence,
                get("pid"),
                get("uid"),
                get("binary_path"),
                get("reason")
            );
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    println!("[agent] done after {cycle} cycles");
    Ok(())
}

//! Live check for src/guard (prevention mode), using the agent's own code.
//!
//!   guard_live run <seconds>
//!       Starts the guards with settings from the environment (EDR_GUARD_*),
//!       prints the setup and every decision, and stops gracefully at the
//!       deadline, or on SIGTERM or SIGINT once self protection lets them in.
//!   guard_live bench <count> <rounds>
//!       Times <count> connect() calls to a unix socket no guard watches,
//!       <rounds> times, and prints nanoseconds per connect.
//!   guard_live check-btf <btf file> <edr_guard.bpf.o>
//!       Runs the CO-RE relocations against another kernel's types (the 5.15
//!       blob from btf_5_15.c) and loads the result through this kernel's
//!       verifier without attaching anything.
//!
//! Needs root, securityfs and bpffs, and the host pid namespace for self
//! protection. It refuses to enforce without EDR_GUARD_SCOPE_CGROUP unless
//! GUARD_LIVE_HOST_WIDE=1, so a test on a shared kernel cannot block other
//! containers.

use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};
use forensic_hooks::guard::{self, GuardMode, GuardSettings};

static STOP_REQUESTED: AtomicBool = AtomicBool::new(false);

extern "C" fn on_stop_signal(_sig: libc::c_int) {
    STOP_REQUESTED.store(true, Ordering::SeqCst);
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args: Vec<String> = std::env::args().collect();
    match (args.get(1).map(String::as_str), args.len()) {
        (Some("run"), 3) => run(Duration::from_secs(number(&args[2])?)),
        (Some("bench"), 4) => bench(number(&args[2])? as usize, number(&args[3])? as usize),
        (Some("check-btf"), 4) => check_btf(&args[2], &args[3]),
        _ => bail!("usage: guard_live run <seconds> | guard_live bench <count> <rounds> | guard_live check-btf <btf> <obj>"),
    }
}

fn number(arg: &str) -> Result<u64> {
    arg.parse().with_context(|| format!("bad number {arg:?}"))
}

fn run(run_for: Duration) -> Result<()> {
    let settings = GuardSettings::from_env();
    let m = settings.modes;
    let enforcing = [m.socket, m.memfd, m.self_protect].contains(&GuardMode::Enforce);
    let host_wide = std::env::var("GUARD_LIVE_HOST_WIDE").is_ok_and(|v| v == "1");
    if enforcing && settings.scope_cgroup.is_none() && !host_wide {
        bail!("enforce without EDR_GUARD_SCOPE_CGROUP would act on every container on this kernel. Set a scope, or GUARD_LIVE_HOST_WIDE=1");
    }

    let handler: extern "C" fn(libc::c_int) = on_stop_signal;
    // SAFETY: the handler only stores to an atomic, which is async signal safe.
    unsafe {
        libc::signal(libc::SIGTERM, handler as libc::sighandler_t);
        libc::signal(libc::SIGINT, handler as libc::sighandler_t);
    }

    println!("[run] pid {} for {}s", std::process::id(), run_for.as_secs());
    let handle = match guard::start_guard_with(settings, |event, finding| {
        let get = |k: &str| finding.output.data.get(k).cloned().unwrap_or_default();
        println!(
            "[event] {} guard={} hook={} pid={} uid={} comm={} exe={} exe_id={}:{} target={} detail={} tags={}",
            get("action"),
            get("guard"),
            get("hook"),
            event.tgid,
            event.uid,
            event.comm(),
            get("binary_path"),
            event.exe_dev,
            event.exe_ino,
            get("target"),
            event.detail,
            get("tags")
        );
    }) {
        Ok(h) => h,
        Err(e) => {
            println!("[run] not started: {e:#}");
            return Ok(());
        }
    };

    let s = &handle.summary;
    println!(
        "[setup] modes socket={} memfd={} self={} agent_tgid={:?} scope_cgroup_id={} protected_ids={}",
        s.modes.socket.as_str(),
        s.modes.memfd.as_str(),
        s.modes.self_protect.as_str(),
        s.agent_tgid,
        s.scope_cgroup_id,
        s.protected_ids
    );
    for (path, id) in &s.socket_clients {
        println!("[setup] socket client {} dev={:#x} ino={}", path.display(), id.dev, id.ino);
    }
    for (path, id) in &s.memfd_allow {
        println!("[setup] memfd allow {} dev={:#x} ino={}", path.display(), id.dev, id.ino);
    }
    println!("[run] ready");

    let deadline = Instant::now() + run_for;
    while Instant::now() < deadline && !STOP_REQUESTED.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_millis(100));
        // What the agent's telemetry sweep would collect.
        for out in guard::take_guard_outputs() {
            println!("[sweep] signal={} confidence={:.2}", out.signal, out.confidence);
        }
    }
    let why = if STOP_REQUESTED.load(Ordering::SeqCst) { "signal" } else { "deadline" };
    println!("[run] stopping ({why})");
    handle.stop();
    println!("[run] stopped");
    Ok(())
}

fn bench(count: usize, rounds: usize) -> Result<()> {
    let dir = std::env::temp_dir().join(format!("guard_bench_{}", std::process::id()));
    std::fs::create_dir_all(&dir)?;
    let path = dir.join("bench.sock");
    let listener = UnixListener::bind(&path).with_context(|| format!("bind {}", path.display()))?;
    // Accept and drop every connection so the backlog never fills.
    std::thread::spawn(move || {
        for conn in listener.incoming() {
            drop(conn);
        }
    });
    for _ in 0..500 {
        UnixStream::connect(&path)?;
    }
    let mut per_connect = Vec::new();
    for round in 1..=rounds {
        let start = Instant::now();
        for _ in 0..count {
            // socket(), connect(), and close() when the stream drops.
            UnixStream::connect(&path)?;
        }
        let ns = start.elapsed().as_nanos() as f64 / count as f64;
        println!("[bench] round {round}: {count} connects, {ns:.0} ns each");
        per_connect.push(ns);
    }
    per_connect.sort_by(|a, b| a.total_cmp(b));
    // The fastest round has the least noise from the rest of the host.
    println!(
        "[bench] median {:.0} min {:.0} ns per connect",
        per_connect[per_connect.len() / 2],
        per_connect[0]
    );
    let _ = std::fs::remove_dir_all(&dir);
    Ok(())
}

/// Same method as harness/dns_live: open with btf_custom_path, load, close.
fn check_btf(btf_path: &str, obj_path: &str) -> Result<()> {
    use libbpf_rs::libbpf_sys;
    use std::ffi::CString;

    libbpf_rs::set_print(Some((libbpf_rs::PrintLevel::Debug, print_core_lines)));
    let obj_c = CString::new(obj_path)?;
    let btf_c = CString::new(btf_path)?;
    // libbpf-rs 0.21 has no setter for btf_custom_path, so this calls libbpf directly.
    let opts = libbpf_sys::bpf_object_open_opts {
        sz: std::mem::size_of::<libbpf_sys::bpf_object_open_opts>() as libbpf_sys::size_t,
        btf_custom_path: btf_c.as_ptr(),
        ..Default::default()
    };
    // SAFETY: obj_c, btf_c and opts outlive both calls. The object pointer is
    // checked for null before use and closed exactly once.
    let err = unsafe {
        let obj = libbpf_sys::bpf_object__open_file(obj_c.as_ptr(), &opts);
        if obj.is_null() {
            bail!("open {obj_path} failed");
        }
        let err = libbpf_sys::bpf_object__load(obj);
        libbpf_sys::bpf_object__close(obj);
        err
    };
    if err != 0 {
        bail!("CHECK-BTF FAILED: load returned {err}");
    }
    println!("CHECK-BTF OK: relocated against {btf_path} and accepted by the verifier");
    Ok(())
}

/// libbpf debug output cut down to the CO-RE relocation lines and any errors.
fn print_core_lines(_level: libbpf_rs::PrintLevel, msg: String) {
    if msg.contains("relo #") || msg.contains("failed") || msg.contains("rror") {
        print!("{msg}");
    }
}

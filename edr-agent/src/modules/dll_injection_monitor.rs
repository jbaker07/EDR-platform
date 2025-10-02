use aya::{
    include_bytes_aligned,
    maps::perf::PerfEventArray,
    programs::{KProbe, Program, RawTracePoint, TracePoint},
    util::online_cpus,
    Bpf,
};
use bytes::BytesMut;
use std::fs;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    convert::TryInto,
    mem,
    ptr::read_unaligned,
    sync::{
        atomic::{AtomicBool, Ordering},
        Once,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::logger::log;
use crate::{
    gnn_hook::{push_metadata_to_gnn_vector_log, push_to_gnn_vector_log},
    modules::replay_writer::store_replay_event,
    telemetry_types::{MemoryAnomalyType, TelemetryOutput},
    telemetry_writer::{push_memory_telemetry, write_telemetry_record},
    trust_hook::{submit_trust_event, TrustEvent},
};

static DLL_INJECTION_STARTED: AtomicBool = AtomicBool::new(false);
static DLL_INJECTION_ONCE: Once = Once::new();

#[repr(C)]
#[derive(Clone, Debug)]
pub struct DllInjectionEvent {
    pub pid: u32,
    pub timestamp: u64,
    pub target_pid: u32,
    pub injection_type: u32, // 1 = reflective, 2 = remote thread, etc.
}

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn parse_event(buf: &[u8]) -> Option<DllInjectionEvent> {
    if buf.len() < mem::size_of::<DllInjectionEvent>() {
        return None;
    }
    let ptr = buf.as_ptr() as *const DllInjectionEvent;
    Some(unsafe { read_unaligned(ptr) })
}

fn read_proc_value(pid: u32, field: &str) -> Option<String> {
    let path = match field {
        "exe" => PathBuf::from(format!("/proc/{}/exe", pid)),
        "cwd" => PathBuf::from(format!("/proc/{}/cwd", pid)),
        "cmdline" => PathBuf::from(format!("/proc/{}/cmdline", pid)),
        _ => return None,
    };

    if field == "cmdline" {
        fs::read(path).ok().map(|bytes| {
            String::from_utf8_lossy(&bytes)
                .replace('\0', " ")
                .trim()
                .to_string()
        })
    } else {
        fs::read_link(path).ok().map(|p| p.display().to_string())
    }
}

fn get_uid_for_pid(pid: u32) -> Option<u32> {
    let status_path = format!("/proc/{}/status", pid);
    if let Ok(contents) = fs::read_to_string(status_path) {
        for line in contents.lines() {
            if line.starts_with("Uid:") {
                return line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse::<u32>().ok());
            }
        }
    }
    None
}

/// Attach programs by variant without relying on `Program::section()`
/// or a generic `Program::load()`.
fn attach_all_programs(bpf: &mut Bpf) -> anyhow::Result<()> {
    for (prog_name, prog) in bpf.programs_mut() {
        let name = prog_name.to_string();

        match prog {
            Program::TracePoint(tp) => {
                // `tp` is already &mut TracePoint here – no try_into needed.
                tp.load()?;
                // Try common syscalls associated with injection behavior.
                let candidates = [
                    ("syscalls", "sys_enter_ptrace"),
                    ("syscalls", "sys_enter_mprotect"),
                    ("syscalls", "sys_enter_mmap"),
                ];
                let mut attached = false;
                for (cat, evt) in candidates {
                    match tp.attach(cat, evt) {
                        Ok(_link) => {
                            log(&format!("✔️ attached tracepoint {}/{}", cat, evt));
                            attached = true;
                            break;
                        }
                        Err(e) => {
                            log(&format!("ℹ️ attach {}/{} failed: {:?}", cat, evt, e));
                        }
                    }
                }
                if !attached {
                    // As a last resort, try ptrace
                    tp.attach("syscalls", "sys_enter_ptrace")?;
                    log("✔️ attached tracepoint syscalls/sys_enter_ptrace (fallback)");
                }
            }

            Program::RawTracePoint(rtp) => {
                rtp.load()?;
                // Broad entry point
                match rtp.attach("sys_enter") {
                    Ok(_link) => log("✔️ attached raw_tracepoint sys_enter"),
                    Err(e) => log(&format!(
                        "ℹ️ raw_tracepoint attach(sys_enter) failed: {:?}",
                        e
                    )),
                }
            }

            Program::KProbe(kp) => {
                kp.load()?;
                // Try several symbols that exist on different kernels
                let candidates = [
                    "do_mprotect_pkey",
                    "do_mprotect",
                    "__x64_sys_mprotect",
                    "__x64_sys_ptrace",
                    "ptrace_check_attach",
                    // Fallback: attempt to attach to the program name as a symbol
                    &name,
                ];
                let mut attached = false;
                for sym in candidates {
                    match kp.attach(sym, 0) {
                        Ok(_link) => {
                            log(&format!("✔️ attached kprobe {}", sym));
                            attached = true;
                            break;
                        }
                        Err(e) => log(&format!("ℹ️ kprobe attach({}) failed: {:?}", sym, e)),
                    }
                }
                if !attached {
                    log(&format!(
                        "⚠️ no kprobe candidate attached for program '{}'",
                        name
                    ));
                }
            }

            // Skip other variants cleanly.
            _other => {
                log(&format!(
                    "ℹ️ skipping unsupported program variant for '{}'",
                    name
                ));
            }
        }
    }
    Ok(())
}

#[cfg(all(target_os = "linux", feature = "with-ebpf"))]
const DLL_INJECTION_BPF: &[u8] = include_bytes_aligned!("../ebpf/dll_injection_monitor.bpf.o");

#[cfg(not(all(target_os = "linux", feature = "with-ebpf")))]
const DLL_INJECTION_BPF: &[u8] = &[];

pub fn start_ebpf_dll_injection_watch() {
    // If eBPF bytecode is not compiled in, no-op safely.
    if DLL_INJECTION_BPF.is_empty() {
        eprintln!("(stub) with-ebpf disabled or dll_injection_monitor.bpf.o missing; skipping dll-injection monitor");
        return;
    }

    thread::Builder::new()
        .name("ebpf_dll_injection_monitor".into())
        .spawn(move || {
            // Leak BPF so perf readers can run on 'static threads without lifetime issues
            let mut tmp = match Bpf::load(DLL_INJECTION_BPF) {
                Ok(bpf) => bpf,
                Err(e) => {
                    eprintln!("❌ Failed to load DLL injection BPF: {:?}", e);
                    return;
                }
            };
            let bpf: &'static mut Bpf = Box::leak(Box::new(tmp));

            if let Err(e) = attach_all_programs(bpf) {
                eprintln!("❌ Failed to attach BPF programs: {:?}", e);
                return;
            }

            // Mark started once the programs are successfully attached
            DLL_INJECTION_STARTED.store(true, Ordering::Release);
            println!("💉 [eBPF] DLL injection monitor attached successfully");

            // Open perf array with short, non-overlapping borrows
            let mut perf_array: PerfEventArray<_> = match bpf
                .map_mut("EVENTS")
                .and_then(|m| PerfEventArray::try_from(m).ok())
            {
                Some(p) => p,
                None => {
                    eprintln!("❌ EVENTS map not found or PerfEventArray::try_from failed");
                    return;
                }
            };

            for cpu_id in online_cpus().unwrap_or_default() {
                match perf_array.open(cpu_id, None) {
                    Ok(mut buf) => {
                        thread::spawn(move || {
                            let mut buffers = vec![BytesMut::with_capacity(1024); 8];

                            loop {
                                match buf.read_events(&mut buffers) {
                                    Ok(events) => {
                                        for slice in &buffers[..events.read] {
                                            if let Some(event) = parse_event(slice) {
                                                let pid = event.pid;
                                                let ppid = event.target_pid;
                                                let uid = get_uid_for_pid(pid).unwrap_or(0);

                                                let binary_path =
                                                    read_proc_value(pid, "exe").unwrap_or_else(|| "unknown".into());
                                                let command_line =
                                                    read_proc_value(pid, "cmdline").unwrap_or_else(|| "unknown".into());
                                                let cwd =
                                                    read_proc_value(pid, "cwd").unwrap_or_else(|| "unknown".into());

                                                // Memory telemetry side-effect
                                                let _ = push_memory_telemetry(
                                                    pid as i32,
                                                    ppid as i32,
                                                    uid,
                                                    &binary_path,
                                                    &command_line,
                                                    &cwd,
                                                    MemoryAnomalyType::DllInjection,
                                                    "Detected DLL Injection via thread start in remote process"
                                                        .into(),
                                                );

                                                println!(
                                                    "🧬 [DLL Injection] PID={} -> TGT={} TYPE={}",
                                                    pid, ppid, event.injection_type
                                                );

                                                let mut data = HashMap::new();
                                                data.insert("pid".into(), pid.to_string());
                                                data.insert("ppid".into(), ppid.to_string());
                                                data.insert("uid".into(), uid.to_string());
                                                data.insert("binary_path".into(), binary_path.clone());
                                                data.insert("command_line".into(), command_line.clone());
                                                data.insert("cwd".into(), cwd.clone());
                                                data.insert("injection_type".into(), event.injection_type.to_string());
                                                data.insert("timestamp".into(), now_ts().to_string());
                                                data.insert("event_type".into(), "dll_injection".into());
                                                data.insert("category".into(), "memory".into());
                                                data.insert("signal".into(), "dll_injection".into());
                                                data.insert("confidence".into(), "0.95".into());
                                                data.insert("replay_tag".into(), "dll_injection_detected".into());
                                                data.insert("gnn_escalate".into(), "true".into());

                                                // Fan-out
                                                write_telemetry_record(data.clone());
                                                push_to_gnn_vector_log(data.clone());
                                                push_metadata_to_gnn_vector_log(data.clone());
                                                store_replay_event(data);

                                                // Trust event
                                                let trust_event = TrustEvent::new_full(
                                                    now_ts(),
                                                    pid as i32,
                                                    ppid as i32,
                                                    uid,
                                                    binary_path,
                                                    command_line,
                                                    cwd,
                                                    "dll_injection".into(),
                                                    "memory::dll_injection".into(),
                                                    "dll_injection_monitor".into(),
                                                    Some("DLL injection detected".into()),
                                                    Some("memory::inject".into()),
                                                    Some(vec!["memory".into(), "dll_injection".into()]),
                                                    Some(8.5),
                                                );
                                                submit_trust_event(trust_event);
                                            }
                                        }

                                        if events.lost > 0 {
                                            eprintln!("⚠️ Lost {} dll-injection events", events.lost);
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("⚠️ Perf buffer read error: {:?}", e);
                                        thread::sleep(Duration::from_millis(50));
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("⚠️ Failed to open perf buffer on CPU {}: {:?}", cpu_id, e);
                    }
                }
            }
        })
        .unwrap();
}

pub fn scan_dll_injection_activity() -> Vec<TelemetryOutput> {
    DLL_INJECTION_ONCE.call_once(|| {
        if !DLL_INJECTION_STARTED.load(Ordering::Acquire) {
            let _ = thread::spawn(|| {
                // Direct call—no Tokio runtime needed
                start_ebpf_dll_injection_watch();
                // The thread above will set DLL_INJECTION_STARTED once attach succeeds
                println!("✅ dll_injection_monitor launch requested.");
            });
        }
    });

    let now = now_ts();
    let uid = get_uid_for_pid(1).unwrap_or(0);

    let trust_event = TrustEvent {
        timestamp: now,
        pid: 1,
        ppid: 0,
        uid,
        binary_path: "kernel".into(),
        command_line: "start_ebpf_dll_injection_watch".into(),
        cwd: "/".into(),
        anomaly_type: "Status".into(),
        component: "memory".into(),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("event".into(), "dll_injection_monitor_heartbeat".into());
            meta.insert("status".into(), "active".into());
            meta.insert("timestamp".into(), now.to_string());
            meta.insert("source".into(), "dll_injection_monitor".into());
            meta.insert("category".into(), "memory".into());
            meta.insert("signal".into(), "dll_injection_monitor_active".into());
            meta.insert(
                "replay_tag".into(),
                "dll_injection_monitor_heartbeat".into(),
            );
            meta.insert("features".into(), "[0.0]".into());
            meta
        },
        risk_score: 0.0,
        source_module: "dll_injection_monitor".into(),
        decay_context: Some("dll_injection_monitoring".into()),
        module: Some("dll_injection_monitor".into()),
        signal: Some("dll_injection_monitor_active".into()),
        signal_type: Some("monitor_heartbeat".into()),
        score: Some(100.0),
        raw_score: Some(0.0),
        tags: Some(vec!["monitor_alive".into(), "ebpf_monitor_ready".into()]),
        description: Some("DLL Injection eBPF monitor is alive and sending heartbeat.".into()),
    };
    submit_trust_event(trust_event);

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("event_type".into(), "dll_injection_monitor_active".into());
    data.insert("status".into(), "ebpf_monitor_spawned".into());
    data.insert("timestamp".into(), now.to_string());
    data.insert("category".into(), "memory".into());
    data.insert("signal".into(), "dll_injection_monitor_active".into());
    data.insert(
        "replay_tag".into(),
        "dll_injection_monitor_heartbeat".into(),
    );
    data.insert("uid".into(), uid.to_string());
    data.insert("pid".into(), "1".into());
    data.insert("ppid".into(), "0".into());

    vec![TelemetryOutput {
        category: "memory".into(),
        signal: "dll_injection_monitor_active".into(),
        confidence: 0.01,
        data,
    }]
}

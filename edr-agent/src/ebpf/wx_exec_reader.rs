// src/ebpf/wx_exec_reader.rs
//
// CO-RE ringbuf reader for wx_exec.bpf.o (write-xor-execute monitor)
// libbpf-rs 0.21
//
// Key points:
// - resolve object via events_reader::resolve_bpf("EDR_BPF_WX_OBJ", "wx_exec.bpf.o")
// - attach mprotect/mmap(/mmap2) enter/exit if present (names may be truncated)
// - prefer "wx_events" map (fallback to "EVENTS"/"events")
// - decode minimally inside the callback (no ebpf_ingest), emit telemetry
// - leak Link handles + forget(obj); poll at 1ms to reduce overruns

#![allow(clippy::needless_return)]
#![cfg(target_os = "linux")]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use libbpf_rs::{Link, ObjectBuilder, RingBufferBuilder};

use forensic_hooks::gnn_hook::push_to_gnn_vector_log;
use forensic_hooks::modules::replay_writer::store_replay_event;
use forensic_hooks::telemetry_types::TelemetryOutput;
use forensic_hooks::telemetry_writer::{write_telemetry_record, TelemetryWriter};

use crate::events_reader;

// Best-effort parse of the first 4 bytes as u32 pid (many wx_exec structs begin with pid)
#[inline]
fn read_pid_be(data: &[u8]) -> Option<u32> {
    if data.len() >= 4 {
        // eBPF ringbuf structs are native-endian; most x86_64/aarch64 are little-endian.
        // We'll interpret as native-endian safely via copy.
        let mut pid: u32 = 0;
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), &mut pid as *mut _ as *mut u8, 4);
        }
        Some(pid)
    } else {
        None
    }
}

pub fn start_wx_exec_reader(_writer: Arc<Mutex<TelemetryWriter>>) {
    // Raise memlock best-effort
    unsafe {
        let rlim = libc::rlimit { rlim_cur: u64::MAX, rlim_max: u64::MAX };
        let _ = libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim as *const _);
    }

    // Resolve the .o robustly (env override respected)
    let obj_path = events_reader::resolve_bpf("EDR_BPF_WX_OBJ", "wx_exec.bpf.o");

    // Open & load BPF object
    let open_obj = match ObjectBuilder::default().open_file(&obj_path) {
        Ok(o) => o,
        Err(e) => { eprintln!("[ebpf/wx] open {obj_path} failed: {e}"); return; }
    };
    let mut obj = match open_obj.load() {
        Ok(o) => o,
        Err(e) => { eprintln!("[ebpf/wx] load failed: {e}"); return; }
    };

    // Attach available programs (some section names are truncated by linker/kernel)
    let mut links: Vec<Link> = Vec::new();
    let mut attach_tp = |candidates: &[&str], cat: &str, name: &str| {
        for func in candidates {
            if let Some(p) = obj.prog_mut(func) {
                match p.attach_tracepoint(cat, name) {
                    Ok(l) => { links.push(l); return; }
                    Err(e) => eprintln!("[ebpf/wx] attach {func}@{cat}/{name} failed: {e}"),
                }
            }
        }
    };

    // mprotect enter/exit
    attach_tp(&["tp_enter_mprotect", "tp_enter_mprote"], "syscalls", "sys_enter_mprotect");
    attach_tp(&["tp_exit_mprotect",  "tp_exit_mprote" ], "syscalls", "sys_exit_mprotect");

    // mmap enter/exit
    attach_tp(&["tp_enter_mmap"], "syscalls", "sys_enter_mmap");
    attach_tp(&["tp_exit_mmap" ], "syscalls", "sys_exit_mmap");

    // mmap2 enter/exit (on some ABIs/kernels)
    attach_tp(&["tp_enter_mmap2"], "syscalls", "sys_enter_mmap2");
    attach_tp(&["tp_exit_mmap2" ], "syscalls", "sys_exit_mmap2");

    // Choose ringbuf map (prefer module-specific, allow env override)
    let map_name = match events_reader::choose_single_ringbuf_name(&obj) {
        Some(name) => name,
        None => {
            eprintln!("[ebpf/wx] ❌ ringbuf map not found (wx_events/net_events/edr_events_rb/EVENTS/events) in {obj_path}");
            return;
        }
    };
    eprintln!("[ebpf/wx] subscribing ringbuf map='{map_name}'");

    let mut map = match obj.map_mut(&map_name) {
        Some(m) => m,
        None => { eprintln!("[ebpf/wx] map '{map_name}' vanished after lookup"); return; }
    };

    // Build ringbuf & perform a minimal decode (no ebpf_ingest)
    let mut rb = RingBufferBuilder::new();
    rb.add(&mut map, move |data: &[u8]| {
        crate::EVENTS_IN.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        // Minimal, robust emission: report payload size and best-effort pid
        let pid = read_pid_be(data).unwrap_or(0);
        let mut m = HashMap::new();
        m.insert("pid".into(), pid.to_string());
        m.insert("payload_len".into(), data.len().to_string());
        m.insert("source".into(), "wx_exec".into());

        // Optional: include first few bytes for debugging/dedup (hex)
        let head_n = data.len().min(16);
        let head_hex = data[..head_n].iter().map(|b| format!("{:02x}", b)).collect::<String>();
        m.insert("payload_head_hex".into(), head_hex);

        let out = TelemetryOutput {
            category: "memory".into(),
            signal: "memory::wx_event".into(),
            confidence: if data.len() >= 24 { 0.60 } else { 0.35 },
            data: m,
        };

        let mut line = out.data.clone();
        line.insert("category".into(), out.category.clone());
        line.insert("signal".into(), out.signal.clone());
        line.insert("confidence".into(), format!("{:.2}", out.confidence));
        write_telemetry_record(line.clone());
        push_to_gnn_vector_log(line.clone());
        store_replay_event(line);
        0
    }).expect("ringbuf add");
    let rb = rb.build().expect("ringbuf build");

    // Keep attachments alive & keep object memory resident
    let _leaked_links: &'static mut Vec<Link> = Box::leak(Box::new(links));
    std::mem::forget(obj);

    // Tight poll to reduce overruns
    std::thread::spawn(move || loop {
        let _ = rb.poll(Duration::from_millis(1));
    });
}

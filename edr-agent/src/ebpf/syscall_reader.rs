// src/ebpf/syscall_reader.rs
#![allow(clippy::needless_return)]
#![cfg(target_os = "linux")]

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::sync::atomic::Ordering;

use libbpf_rs::{Link, ObjectBuilder, RingBufferBuilder};
use forensic_hooks::telemetry_writer::TelemetryWriter;

use crate::ebpf::events_reader; // resolve_bpf()
use crate::ebpf::ebpf_ingest::on_edr_event;

/// Attach a candidate function (if present) to a tracepoint category/name.
fn attach_tp_if_present(
    obj: &mut libbpf_rs::Object,
    links: &mut Vec<Link>,
    func_names: &[&str],
    cat: &str,
    name: &str,
) {
    for f in func_names {
        if let Some(p) = obj.prog_mut(f) {
            match p.attach_tracepoint(cat, name) {
                Ok(l) => {
                    eprintln!("[ebpf/syscalls] ✅ attached {f} -> {cat}/{name}");
                    links.push(l);
                    return;
                }
                Err(e) => {
                    eprintln!("[ebpf/syscalls] attach {f}@{cat}/{name} failed: {e}");
                }
            }
        }
    }
}

/// Return ringbuf map names to try, honoring EDR_SYS_RINGBUF if it exists in the object.
/// We’ll subscribe to ALL that are present (no longer “pick one”).
fn discover_ringbuf_maps(obj: &libbpf_rs::Object) -> Vec<String> {
    let mut ordered: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    // Highest priority: explicit env override (still include others later).
    if let Ok(over) = std::env::var("EDR_SYS_RINGBUF") {
        if obj.map(&over).is_some() && seen.insert(over.clone()) {
            ordered.push(over);
        }
    }

    // Common names used across our objects.
    for n in ["edr_events_rb", "events", "wx_events", "syscall_events", "sys_events", "EVENTS"] {
        if obj.map(n).is_some() && seen.insert(n.to_string()) {
            ordered.push(n.to_string());
        }
    }

    if ordered.is_empty() {
        eprintln!("[ebpf/syscalls] no known ringbuf maps found in object (edr_events_rb/events/wx_events/...)");
    } else {
        eprintln!("[ebpf/syscalls] ringbuf maps available: {}", ordered.join(", "));
    }
    ordered
}

pub fn start_syscall_reader(writer: Arc<Mutex<TelemetryWriter>>) {
    // Best-effort RLIMIT_MEMLOCK raise.
    unsafe {
        let rlim = libc::rlimit { rlim_cur: u64::MAX, rlim_max: u64::MAX };
        let _ = libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim as *const _);
    }

    // Default to proc_lifecycle.bpf.o; allow override via EDR_BPF_SYSCALL_OBJ.
    let obj_path = events_reader::resolve_bpf("EDR_BPF_SYSCALL_OBJ", "proc_lifecycle.bpf.o");

    let open = match ObjectBuilder::default().open_file(&obj_path) {
        Ok(o) => o,
        Err(e) => { eprintln!("[ebpf/syscalls] open {obj_path} failed: {e}"); return; }
    };
    let mut obj = match open.load() {
        Ok(o) => o,
        Err(e) => { eprintln!("[ebpf/syscalls] load failed: {e}"); return; }
    };

    // Attach a few likely tracepoints (we tolerate failures, logging the one that worked).
    let mut links: Vec<Link> = Vec::new();
    attach_tp_if_present(&mut obj, &mut links, &["tp_enter_execve"], "syscalls", "sys_enter_execve");
    attach_tp_if_present(&mut obj, &mut links, &["tp_enter_execveat"], "syscalls", "sys_enter_execveat");
    attach_tp_if_present(&mut obj, &mut links, &["tp_enter_connect","tp_enter_connec"], "syscalls", "sys_enter_connect");
    attach_tp_if_present(&mut obj, &mut links, &["tp_enter_mmap"], "syscalls", "sys_enter_mmap");
    attach_tp_if_present(&mut obj, &mut links, &["tp_enter_mprotect","tp_enter_mprote"], "syscalls", "sys_enter_mprotect");
    attach_tp_if_present(&mut obj, &mut links, &["tp_enter_clone"], "syscalls", "sys_enter_clone");
    attach_tp_if_present(&mut obj, &mut links, &["tp_sched_process_exec","tp_sched_process_exe"], "sched", "sched_process_exec");

    // Subscribe to ALL ringbuf maps we can find in THIS object.
    let maps = discover_ringbuf_maps(&obj);
    if maps.is_empty() {
        eprintln!("[ebpf/syscalls] ❌ no ringbuf maps to subscribe in {obj_path}");
        return;
    }

    let host_sz = std::mem::size_of::<crate::ebpf::ebpf_bridge::EdREvent>();
    eprintln!(
        "[ebpf/syscalls] ingest ready — host_struct_size={host_sz}; decoder also accepts legacy sizes: 312/332/376 and tcp_state: 48"
    );

    // Build one ring buffer per map to avoid borrow issues, and spawn a poller per rb.
    let mut started = 0usize;
    for name in maps {
        if let Some(mut m) = obj.map_mut(&name) {
            eprintln!("[ebpf/syscalls] subscribing ringbuf map='{name}' obj='{obj_path}'");
            let mut rb_builder = RingBufferBuilder::new();

            let w = writer.clone();
            if let Err(e) = rb_builder.add(&mut m, move |data: &[u8]| {
                // Count every raw frame delivered to userspace (pre-filter).
                crate::EVENTS_IN.fetch_add(1, Ordering::Relaxed);

                if let Some(rec) = on_edr_event(data, &w) {
                    // Persist the record
                    if let Ok(mut ww) = w.lock() {
                        ww.append(rec);
                    }
                    // Count accepted events (since in realtime mode we bypass the batching loop).
                    crate::EVENTS_ACCEPTED.fetch_add(1, Ordering::Relaxed);
                }
                0
            }) {
                eprintln!("[ebpf/syscalls] ❌ ringbuf add failed for map='{name}': {e}");
                continue;
            }

            match rb_builder.build() {
                Ok(rb) => {
                    started += 1;
                    std::thread::spawn(move || loop {
                        // poll fast to reduce overrun risk
                        let _ = rb.poll(Duration::from_millis(5));
                    });
                }
                Err(e) => {
                    eprintln!("[ebpf/syscalls] ❌ ringbuf build failed for map='{name}': {e}");
                }
            }
        } else {
            eprintln!("[ebpf/syscalls] ⚠️ ringbuf map '{name}' not found in object");
        }
    }

    if started == 0 {
        eprintln!("[ebpf/syscalls] ❌ no ring buffers started for {obj_path}");
        return;
    }

    // Keep BPF object and links alive for the lifetime of the readers.
    let _leaked_links: &'static mut Vec<Link> = Box::leak(Box::new(links));
    std::mem::forget(obj);
}

use aya::{include_bytes_aligned, Bpf};
use aya::maps::perf::PerfEventArray;
use aya::programs::TracePoint;
use aya::util::online_cpus;
use bytes::BytesMut;
use std::convert::TryInto;
use std::{thread, time::Duration};
use std::sync::{OnceLock, atomic::{AtomicBool, Ordering}};
use std::collections::HashMap;

use crate::telemetry_writer::{push_memory_telemetry, write_telemetry_record};
use crate::telemetry_types::{MemoryAnomalyType, TelemetryOutput};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct IPCAbuseEvent {
    pub pid: u32,
    pub target_pid: u32,
    pub syscall_id: u32,
    pub timestamp: u64,
}

pub static SCAN_SUSPICIOUS_IPC: OnceLock<AtomicBool> = OnceLock::new();

fn parse_ipc_event(buf: &[u8]) -> Option<IPCAbuseEvent> {
    use std::ptr::read_unaligned;
    let ptr = buf.as_ptr() as *const IPCAbuseEvent;
    Some(unsafe { read_unaligned(ptr) })
}

/// Starts the eBPF-powered suspicious IPC monitor
pub fn start_ebpf_ipc_abuse_watch() {
    SCAN_SUSPICIOUS_IPC.get_or_init(|| AtomicBool::new(true));

    thread::spawn(move || {
        let bpf = Bpf::load(include_bytes_aligned!(
            "../ebpf/suspicious_ipc.bpf.o"
        ));

        let mut bpf = match bpf {
            Ok(b) => b,
            Err(e) => {
                eprintln!("❌ Failed to load suspicious_ipc BPF: {:?}", e);
                return;
            }
        };

        let program = match bpf.program_mut("trace_suspicious_ipc") {
            Some(p) => p,
            None => {
                eprintln!("❌ Missing trace_suspicious_ipc program");
                return;
            }
        };

        let tp: &mut TracePoint = match program.try_into() {
            Ok(tp) => tp,
            Err(e) => {
                eprintln!("❌ Could not convert to TracePoint: {:?}", e);
                return;
            }
        };

        if let Err(e) = tp.load() {
            eprintln!("❌ Failed to load tracepoint: {:?}", e);
            return;
        }

        if let Err(e) = tp.attach("syscalls", "sys_enter_msgsnd") {
            eprintln!("❌ Failed to attach syscall: {:?}", e);
            return;
        }

        println!("💬 [eBPF] Suspicious IPC monitor tracepoint active");

        let map = match bpf.map_mut("EVENTS") {
            Ok(m) => m,
            Err(e) => {
                eprintln!("❌ No perf map found in suspicious_ipc_monitor: {:?}", e);
                return;
            }
        };

        let mut perf_array = match PerfEventArray::try_from(map) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("❌ PerfEventArray error: {:?}", e);
                return;
            }
        };

        for cpu_id in online_cpus().unwrap_or_default() {
            match perf_array.open(cpu_id, None) {
                Ok(mut buf) => {
                    thread::spawn(move || {
                        let mut buffers = vec![BytesMut::with_capacity(1024)];
                        loop {
                            match buf.read_events(&mut buffers) {
                                Ok(_) => {
                                    for buffer in &buffers {
                                        if let Some(evt) = parse_ipc_event(buffer) {
                                            let _ = push_memory_telemetry(
                                                evt.pid as i32,
                                                evt.target_pid as i32,
                                                0,
                                                "unknown".into(),
                                                "unknown".into(),
                                                "unknown".into(),
                                                MemoryAnomalyType::IpcAbuse,
                                                format!(
                                                    "Suspicious IPC: pid={} → pid={} via syscall_id={}",
                                                    evt.pid, evt.target_pid, evt.syscall_id
                                                ),
                                            )
                                            .map_err(|e| {
                                                eprintln!("⚠️ IPC telemetry failed: {:?}", e)
                                            });
                                        }
                                    }
                                    buffers.clear();
                                }
                                Err(e) => {
                                    eprintln!("⚠️ Read events error: {:?}", e);
                                }
                            }

                            thread::sleep(Duration::from_millis(25));
                        }
                    });
                }
                Err(e) => {
                    eprintln!("❌ Failed to open perf buffer on CPU {}: {:?}", cpu_id, e);
                }
            }
        }
    });
}

/// Passive fallback function for suspicious IPC scan (placeholder version)
pub fn scan_ipc_passive() -> Vec<TelemetryOutput> {
    let mut outputs = Vec::new();

    outputs.push(TelemetryOutput {
        category: "memory_anomaly".into(),
        signal: "ipc_abuse_simulated".into(),
        confidence: 0.5,
        data: HashMap::from([
            ("note".into(), "Fallback IPC scan not supported without live BPF".into()),
            ("severity".into(), "info".into()),
            ("replay_tag".into(), "simulated_ipc_abuse".into())
        ]),
    });

    outputs
}

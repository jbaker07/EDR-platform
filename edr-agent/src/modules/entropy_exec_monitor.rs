use aya::{
    include_bytes_aligned,
    maps::perf::PerfEventArray,
    util::online_cpus,
    Bpf,
};
use aya::programs::TracePoint;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{fs, mem, thread, time::Duration};

use crate::{
    trust_hook::{submit_trust_event, TrustEvent},
    gnn_hook::push_to_gnn_vector_log,
    telemetry_writer::write_telemetry_record,
    telemetry::calculate_entropy,
    logger::log,
};

use bytes::BytesMut;
use std::collections::HashMap;
use std::convert::TryInto;
use chrono::Utc;

static MONITOR_STARTED: AtomicBool = AtomicBool::new(false);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EntropyEvent {
    pub pid: u32,
    pub ppid: u32,
    pub filename: [u8; 256],
    pub syscall: [u8; 16],
}

pub fn start_entropy_exec_monitor() {
    if MONITOR_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    thread::spawn(|| {
        let mut bpf = match Bpf::load(include_bytes_aligned!(
            "../ebpf/entropy_exec_monitor.bpf.o"
        )) {
            Ok(bpf) => bpf,
            Err(e) => {
                log(&format!("Failed to load entropy_exec_monitor.bpf.o: {:?}", e));
                return;
            }
        };

        for prog_name in ["trace_execve", "trace_openat", "trace_mmap"] {
            if let Some(prog) = bpf.program_mut(prog_name) {
                let tp: &mut TracePoint = prog.try_into().unwrap();
                tp.load().unwrap();
                tp.attach(prog_name, prog_name).unwrap();
            } else {
                log(&format!("Program not found in BPF: {}", prog_name));
            }
        }

        let mut perf_array = PerfEventArray::try_from(bpf.map_mut("events").unwrap()).unwrap();

        for cpu_id in online_cpus().unwrap() {
            let mut buf = perf_array.open(cpu_id, None).unwrap();
            thread::spawn(move || {
                let mut bufs = (0..16)
                    .map(|_| BytesMut::with_capacity(4096))
                    .collect::<Vec<_>>();

                loop {
                    match buf.read_events(&mut bufs) {
                        Ok(_) => {
                            for buf in &bufs {
                                if buf.len() < mem::size_of::<EntropyEvent>() {
                                    continue;
                                }
                                let ptr = buf.as_ptr() as *const EntropyEvent;
                                let event = unsafe { *ptr };

                                let filename = String::from_utf8_lossy(&event.filename)
                                    .trim_end_matches('\0')
                                    .to_string();
                                let syscall = String::from_utf8_lossy(&event.syscall)
                                    .trim_end_matches('\0')
                                    .to_string();

                                if let Ok(contents) = fs::read(&filename) {
                                    let entropy = calculate_entropy(&contents);
                                    let severity = if entropy >= 7.9 { "high" } else { "medium" };

                                    let mut data: HashMap<String, String> = HashMap::new();
                                    data.insert("entropy".into(), entropy.to_string());
                                    data.insert("syscall".into(), syscall.clone());
                                    data.insert("path".into(), filename.clone());
                                    data.insert("timestamp".into(), Utc::now().timestamp().to_string());
                                    data.insert("pid".into(), event.pid.to_string());
                                    data.insert("ppid".into(), event.ppid.to_string());
                                    data.insert("category".into(), "file".to_string());
                                    data.insert("signal".into(), "entropy_exec_trigger".to_string());
                                    data.insert("confidence".into(), severity.to_string());
                                    data.insert("command_line".into(), format!("[{}]", syscall));
                                    data.insert("cwd".into(), "n/a".to_string());

                                    let mut tags = vec![
                                        "high_entropy".to_string(),
                                        format!("triggered_by_{}", syscall),
                                        "runtime_risk".to_string(),
                                    ];
                                    if filename.contains("/tmp") {
                                        tags.push("temp_path".to_string());
                                    }

                                    let trust_event = TrustEvent::from_parts(
                                        Utc::now().timestamp() as u64,
                                        event.pid as i32,
                                        event.ppid as i32,
                                        0, // uid unknown
                                        filename.clone(),
                                        "EncryptedPayload",
                                        "file",
                                        Some(data.clone()),
                                        if entropy > 7.9 { 0.85 } else { 0.5 },
                                        "entropy_exec_monitor",
                                        Some(format!(
                                            "File with entropy {:.2} executed via {}",
                                            entropy, syscall
                                        )),
                                        Some(tags),
                                        Some("entropy_exec_trigger".into()),
                                        Some(syscall.clone()),
                                        Some("file_behavior".into()),
                                        Some("entropy_exec_monitor".into()),
                                    );

                                    write_telemetry_record(data.clone());
                                    push_to_gnn_vector_log(data.clone());
                                    submit_trust_event(trust_event);
                                }
                            }
                        }
                        Err(_) => thread::sleep(Duration::from_millis(50)),
                    }
                }
            });
        }
    });
}

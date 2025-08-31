#![cfg(target_os = "linux")]

use std::collections::HashMap;
use aya::programs::TracePoint;

use crate::events_reader::{self, MapKind};

use forensic_hooks::forensic::utils::read_proc_value;
use forensic_hooks::gnn_hook::push_to_gnn_vector_log;
use forensic_hooks::modules::replay_writer::store_replay_event;
use forensic_hooks::telemetry_types::TelemetryOutput;
use forensic_hooks::telemetry_writer::write_telemetry_record;

#[repr(C)]
#[derive(Clone, Copy)]
struct FileAccessEvent {
    pid: u32,
    uid: u32,
    comm: [u8; 64],
    filename: [u8; 256],
}

fn parse_and_emit(buf: &[u8]) {
    use std::ptr::copy_nonoverlapping;
    if buf.len() < std::mem::size_of::<FileAccessEvent>() { return; }

    let mut e = FileAccessEvent { pid: 0, uid: 0, comm: [0;64], filename:[0;256] };
    unsafe {
        copy_nonoverlapping(
            buf.as_ptr(),
            &mut e as *mut _ as *mut u8,
            std::mem::size_of::<FileAccessEvent>(),
        );
    }

    let comm = String::from_utf8_lossy(&e.comm).trim_end_matches(char::from(0)).to_string();
    let path = String::from_utf8_lossy(&e.filename).trim_end_matches(char::from(0)).to_string();

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("pid".into(), e.pid.to_string());
    data.insert("uid".into(), e.uid.to_string());
    data.insert("comm".into(), comm.clone());
    data.insert("file_path".into(), path.clone());

    // Best-effort enrichment
    if let Ok(bin) = read_proc_value(e.pid, "exe")     { data.insert("binary_path".into(), bin); }
    if let Ok(cmd) = read_proc_value(e.pid, "cmdline") { data.insert("command_line".into(), cmd); }
    if let Ok(cwd) = read_proc_value(e.pid, "cwd")     { data.insert("cwd".into(), cwd); }

    // Keep it simple for now: read/open
    let out = TelemetryOutput {
        category: "file".into(),
        signal: "file_access".into(), // openat()
        confidence: 0.50,
        data,
    };

    // fan-out
    let mut m = out.data.clone();
    m.insert("category".into(), out.category.clone());
    m.insert("signal".into(), out.signal.clone());
    m.insert("confidence".into(), format!("{:.2}", out.confidence));
    write_telemetry_record(m.clone());
    push_to_gnn_vector_log(m.clone());
    store_replay_event(m);
}

fn attach(bpf: &mut aya::Bpf) -> Result<(), String> {
    let p = bpf.program_mut("trace_file_access").ok_or("missing prog trace_file_access")?;
    let tp: &mut TracePoint = p.try_into().map_err(|e| format!("{e:?}"))?;
    tp.load().map_err(|e| format!("load failed: {e:?}"))?;
    tp.attach("syscalls", "sys_enter_openat")
        .map(|_| ())
        .map_err(|e| format!("attach failed: {e:?}"))
}

pub fn start() -> Result<(), String> {
    let obj_path = events_reader::resolve_bpf("EDR_BPF_FILE_OBJ", "file_access_monitor.bpf.o");

    match events_reader::detect_events_map_kind(&obj_path)? {
        MapKind::Perf => {}
        other => return Err(format!("file_access_monitor expects PERF, got {:?}", other)),
    }

    let bytes = std::fs::read(&obj_path).map_err(|e| format!("read {}: {}", obj_path, e))?;
    events_reader::start_perf_reader_with_attach(&bytes, attach, parse_and_emit)
}

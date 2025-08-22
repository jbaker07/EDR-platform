// src/modules/ebpf_ingest.rs
use std::sync::{Arc, Mutex};
use crate::telemetry_writer::TelemetryWriter;
use crate::telemetry::{TelemetryRecord};
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::telemetry_writer::write_telemetry_record;
use crate::telemetry_types::TelemetryOutput;
use crate::ebpf_bridge::{EdREvent, edr_event_to_output, edr_event_to_record};

/// Call this from your ringbuf callback with the raw bytes (exact size of EdREvent).
/// Returns a TelemetryRecord you can batch (optional).
pub fn on_edr_event(bytes: &[u8], writer: &Arc<Mutex<TelemetryWriter>>) -> Option<TelemetryRecord> {
    // Safety: ringbuf record is exactly sizeof(EdREvent)
    if bytes.len() != std::mem::size_of::<EdREvent>() {
        eprintln!("[ebpf_ingest] size mismatch: got {}, want {}", bytes.len(), std::mem::size_of::<EdREvent>());
        return None;
    }
    let mut e = EdREvent {
        ts: 0, type_: 0, syscall_id: 0, tgid: 0, ppid: 0, uid: 0,
        fd: 0, ret: 0, flags: 0, aux_u32: 0, aux_u64: 0,
        fam: 0, proto: 0, lport: 0, laddr4: 0, laddr6: [0;16],
        rport: 0, raddr4: 0, raddr6: [0;16],
        path: [0;128], path2: [0;128], comm: [0;16],
    };
    // memcpy
    unsafe {
        std::ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            &mut e as *mut EdREvent as *mut u8,
            std::mem::size_of::<EdREvent>(),
        );
    }

    // 1) TelemetryOutput for JSONL / trust-engine path
    let out: TelemetryOutput = edr_event_to_output(&e);
    let mapped = {
        let mut m = out.data.clone();
        m.insert("tag0".into(), out.signal.clone());
        m
    };
    write_telemetry_record(mapped.clone());
    push_to_gnn_vector_log(mapped);

    // 2) TelemetryRecord for batch scoring / episode
    let rec = edr_event_to_record(&e);

    // 3) (optional) write flow edges into writer extras (if you maintain such)
    if let Ok(mut w) = writer.lock() {
        // Example: log sockets to your connection log
        if out.signal == "net_connect" || out.signal == "net_accept" {
            let _ = crate::modules::net_watch::log_open_connections(&mut *w);
        }
    }

    Some(rec)
}

// Ringbuf reader for wx_exec.bpf.o
// Signals:
//   - process::w_x_transition  (W->X within 5s window)
//   - memory::memfd_exec       (memfd written then exec in <=120s)

use std::collections::HashMap;
use std::os::fd::AsRawFd;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

use libbpf_rs::{ObjectBuilder, RingBufferBuilder};

use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::replay_writer::store_replay_event;
use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::{write_telemetry_record, TelemetryWriter};

#[repr(C)]
#[derive(Clone, Copy)]
struct WxEvt {
    pid: u32,
    kind: u32, // 1 W->X, 2 memfd_exec
    ts_ns: u64,
    addr: u64,
    len: u64,
    prot: u64,
    bytes: u64,
}

pub fn start_wx_exec_reader(writer: Arc<Mutex<TelemetryWriter>>) {
    unsafe {
        let rlim = libc::rlimit { rlim_cur: u64::MAX, rlim_max: u64::MAX };
        libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim as *const _);
    }

    let obj_path = std::env::var("EDR_BPF_WX_OBJ")
        .unwrap_or_else(|_| "wx_exec.bpf.o".into());
    let open = match ObjectBuilder::default().open_file(&obj_path) {
        Ok(o) => o,
        Err(e) => { eprintln!("[ebpf/wx] open failed: {e}"); return; }
    };
    let mut obj = match open.load() {
        Ok(o) => o,
        Err(e) => { eprintln!("[ebpf/wx] load failed: {e}"); return; }
    };

    for prog in obj.progs_mut() {
        if let Some(sec) = prog.section() {
            if sec.starts_with("tracepoint/") {
                if let Some((cat, name)) = sec.trim_start_matches("tracepoint/").split_once('/') {
                    if let Err(e) = prog.attach_tracepoint(cat, name) {
                        eprintln!("[ebpf/wx] attach {sec} failed: {e}");
                    }
                }
            }
        }
    }

    let mut rb = RingBufferBuilder::new();
    let mut map = match obj.map_mut("wx_events") {
        Ok(m) => m,
        Err(e) => { eprintln!("[ebpf/wx] map open failed: {e}"); return; }
    };

    rb.add(&mut map, move |data: &[u8]| {
        if data.len() < std::mem::size_of::<WxEvt>() { return 0; }
        let mut ev = WxEvt { pid:0, kind:0, ts_ns:0, addr:0, len:0, prot:0, bytes:0 };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                &mut ev as *mut _ as *mut u8,
                std::mem::size_of::<WxEvt>(),
            );
        }

        let mut fields = HashMap::new();
        fields.insert("pid".into(), ev.pid.to_string());
        fields.insert("ts_ns".into(), ev.ts_ns.to_string());

        match ev.kind {
            1 => {
                // W->X
                if ev.addr != 0 { fields.insert("addr".into(), format!("0x{:x}", ev.addr)); }
                if ev.len  != 0 { fields.insert("len".into(),  ev.len.to_string()); }
                if ev.prot != 0 { fields.insert("prot".into(), ev.prot.to_string()); }
                let out = TelemetryOutput {
                    category: "process".into(),
                    signal: "process::w_x_transition".into(),
                    confidence: 0.95,
                    data: fields,
                };
                emit(out);
            }
            2 => {
                // memfd -> exec chain
                if ev.bytes != 0 { fields.insert("bytes".into(), ev.bytes.to_string()); }
                let out = TelemetryOutput {
                    category: "memory".into(),
                    signal: "memory::memfd_exec".into(),
                    confidence: 0.95,
                    data: fields,
                };
                emit(out);
            }
            _ => {}
        }
        0
    }).expect("ringbuf add");

    let rb = rb.build().expect("ringbuf build");
    let _fd = rb.as_raw_fd();

    thread::spawn(move || loop {
        let _ = rb.poll(1);
        std::thread::sleep(Duration::from_millis(1));
    });

    fn emit(out: TelemetryOutput) {
        let mut m = out.data.clone();
        m.insert("category".into(), out.category.clone());
        m.insert("signal".into(), out.signal.clone());
        m.insert("confidence".into(), out.confidence.to_string());
        write_telemetry_record(m.clone());
        push_to_gnn_vector_log(m.clone());
        store_replay_event(m);
    }
}

// Minimal ringbuf reader for net_flow.bpf.o
// Emits TelemetryOutput with category "network" and signals:
//   - network::connect (addr/port)
//   - network::tx_bytes / network::rx_bytes (bytes)

use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};
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
struct NetEvt {
    pid: u32,
    kind: u32,     // 1 connect, 2 tx, 3 rx
    family: u16,   // AF_*
    dport: u16,    // BE
    is_v6: u8,
    daddr_v4: u32, // network byte order
    daddr_v6: [u8; 16],
    bytes: i64,
}

fn ntohs(x: u16) -> u16 {
    u16::from_be(x)
}

fn ip_to_string(is_v6: u8, v4: u32, v6: [u8; 16]) -> String {
    if is_v6 != 0 {
        let a = Ipv6Addr::from(v6);
        a.to_string()
    } else {
        let a = Ipv4Addr::from(v4);
        a.to_string()
    }
}

pub fn start_net_flow_reader(writer: Arc<Mutex<TelemetryWriter>>) {
    // raise memlock best-effort
    unsafe {
        let rlim = libc::rlimit { rlim_cur: u64::MAX, rlim_max: u64::MAX };
        libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim as *const _);
    }

    let obj_path = std::env::var("EDR_BPF_NET_OBJ")
        .unwrap_or_else(|_| "net_flow.bpf.o".into());
    let open = match ObjectBuilder::default().open_file(&obj_path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("[ebpf/net] open failed: {e}");
            return;
        }
    };
    let mut obj = match open.load() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("[ebpf/net] load failed: {e}");
            return;
        }
    };

    // attach all tracepoints in object
    for prog in obj.progs_mut() {
        if let Some(sec) = prog.section() {
            if sec.starts_with("tracepoint/") {
                if let Some((cat, name)) = sec.trim_start_matches("tracepoint/").split_once('/') {
                    if let Err(e) = prog.attach_tracepoint(cat, name) {
                        eprintln!("[ebpf/net] attach {sec} failed: {e}");
                    }
                }
            }
        }
    }

    let mut rb = RingBufferBuilder::new();
    let mut events_map = match obj.map_mut("net_events") {
        Ok(m) => m,
        Err(e) => {
            eprintln!("[ebpf/net] map open failed: {e}");
            return;
        }
    };

    rb.add(&mut events_map, move |data: &[u8]| {
        if data.len() < std::mem::size_of::<NetEvt>() {
            return 0;
        }
        let mut ev = NetEvt {
            pid: 0, kind: 0, family: 0, dport: 0, is_v6: 0, daddr_v4: 0, daddr_v6: [0; 16], bytes: 0,
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                &mut ev as *mut _ as *mut u8,
                std::mem::size_of::<NetEvt>(),
            );
        }

        let mut map = HashMap::new();
        map.insert("pid".into(), ev.pid.to_string());

        match ev.kind {
            1 => {
                // connect
                let ip = ip_to_string(ev.is_v6, ev.daddr_v4, ev.daddr_v6);
                let port = ntohs(ev.dport);
                map.insert("dst_addr".into(), ip);
                map.insert("dst_port".into(), port.to_string());
                map.insert("family".into(), ev.family.to_string());

                let out = TelemetryOutput {
                    category: "network".into(),
                    signal: "network::connect".into(),
                    confidence: 0.9,
                    data: map,
                };
                emit(out);
            }
            2 => {
                map.insert("bytes".into(), ev.bytes.to_string());
                let out = TelemetryOutput {
                    category: "network".into(),
                    signal: "network::tx_bytes".into(),
                    confidence: 0.75,
                    data: map,
                };
                emit(out);
            }
            3 => {
                map.insert("bytes".into(), ev.bytes.to_string());
                let out = TelemetryOutput {
                    category: "network".into(),
                    signal: "network::rx_bytes".into(),
                    confidence: 0.75,
                    data: map,
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
        let _ = rb.poll(100);
        // optional tiny sleep to yield
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

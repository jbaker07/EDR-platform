// src/ebpf/net_flow_reader.rs
// Minimal ringbuf reader for net_flow.bpf.o
// Emits TelemetryOutput with category "network" and signals:
//   - network::connect       (scored + gated; external/uncommon prioritized)
//   - network::tx_bytes / network::rx_bytes (suppressed unless non-trivial)

#![cfg(target_os = "linux")]

use std::collections::{HashMap, VecDeque};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::{Arc, Mutex};
use std::{thread, time::{Duration, Instant}};

use lazy_static::lazy_static;
use libbpf_rs::{ObjectBuilder, RingBufferBuilder};

use forensic_hooks::gnn_hook::push_to_gnn_vector_log;
use forensic_hooks::modules::replay_writer::store_replay_event;
use forensic_hooks::telemetry_types::TelemetryOutput;
use forensic_hooks::telemetry_writer::{write_telemetry_record, TelemetryWriter};

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

fn ntohs(x: u16) -> u16 { u16::from_be(x) }

fn ip_to_string(is_v6: u8, v4: u32, v6: [u8; 16]) -> String {
    if is_v6 != 0 { Ipv6Addr::from(v6).to_string() } else { Ipv4Addr::from(v4).to_string() }
}

/* ------------------- gating + scoring helpers ------------------- */

lazy_static! {
    static ref NF_DEDUPE:  std::sync::Mutex<HashMap<String, Instant>> = std::sync::Mutex::new(HashMap::new());
    static ref NF_RATE:    std::sync::Mutex<HashMap<String, VecDeque<Instant>>> = std::sync::Mutex::new(HashMap::new());
}

const NF_CONNECT_TTL_SECS: u64 = 90;
const NF_BYTES_TTL_SECS:   u64 = 10;
const NF_RATE_WINDOW_SECS: u64 = 60;
const NF_RATE_LIMIT_CONN:  usize = 500;
const NF_RATE_LIMIT_BYTES: usize = 800;
const BYTES_MIN_EMIT:      i64 = 4096;

fn is_private_ip(s: &str) -> bool {
    if s == "127.0.0.1" { return true; }
    if s.starts_with("10.") || s.starts_with("192.168.") { return true; }
    if s.starts_with("172.") {
        if let Some(seg) = s.split('.').nth(1).and_then(|x| x.parse::<u8>().ok()) {
            return (16..=31).contains(&seg);
        }
    }
    false
}
fn is_ephemeral_port(p: u16) -> bool { p >= 49152 }
fn is_common_service_port(p: u16) -> bool {
    matches!(p, 22 | 25 | 53 | 80 | 110 | 123 | 143 | 389 | 443 | 465 | 587 | 993 | 995)
}
const DENY_PORTS: &[u16] = &[31337, 4444, 1337, 6666, 6667];

fn rate_ok(signal: &str, limit: usize) -> bool {
    let mut r = NF_RATE.lock().unwrap();
    let q = r.entry(signal.to_string()).or_default();
    let now = Instant::now();
    let cutoff = now - Duration::from_secs(NF_RATE_WINDOW_SECS);
    while matches!(q.front(), Some(t) if *t < cutoff) { q.pop_front(); }
    if q.len() >= limit { return false; }
    q.push_back(now);
    true
}

fn dedupe_ok(key: &str, ttl_secs: u64) -> bool {
    let mut m = NF_DEDUPE.lock().unwrap();
    let now = Instant::now();
    if let Some(last) = m.get(key) {
        if now.duration_since(*last) < Duration::from_secs(ttl_secs) { return false; }
    }
    m.insert(key.to_string(), now);
    true
}

fn score_connect(dst_ip: &str, dst_port: u16) -> Option<(f32, u32, bool)> {
    if is_private_ip(dst_ip) { return None; }
    if is_ephemeral_port(dst_port) { return None; }

    let mut conf: f32 = 0.25;
    let mut risk: u32 = 12;
    let mut esc = false;

    if DENY_PORTS.contains(&dst_port) { conf = 0.85_f32; risk = 70; esc = true; }

    if dst_port < 1024 && !is_common_service_port(dst_port) {
        conf = conf.max(0.70_f32); risk = risk.max(55);
    } else if !is_common_service_port(dst_port) {
        conf = conf.max(0.45_f32); risk = risk.max(35);
    }

    if conf < 0.35_f32 { return None; }
    Some((conf, risk, esc))
}

/* ------------------------- reader ------------------------- */

pub fn start_net_flow_reader(_writer: Arc<Mutex<TelemetryWriter>>) {
    // raise memlock best-effort
    unsafe {
        let rlim = libc::rlimit { rlim_cur: u64::MAX, rlim_max: u64::MAX };
        libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim as *const _);
    }

    // Use repo-local objects by default (can override with EDR_BPF_NET_OBJ / EDR_BPF_DIR)
    let obj_path = crate::events_reader::resolve_bpf("EDR_BPF_NET_OBJ", "net_flow.bpf.o");

    let open = match ObjectBuilder::default().open_file(&obj_path) {
        Ok(o) => o,
        Err(e) => { eprintln!("[ebpf/net] open failed for {obj_path}: {e}"); return; }
    };
    let mut obj = match open.load() {
        Ok(o) => o,
        Err(e) => { eprintln!("[ebpf/net] load failed: {e}"); return; }
    };

    // Pick the map name immutably first, then borrow mutably once.
    let map_name = if obj.map("EVENTS").is_some() {
        "EVENTS"
    } else if obj.map("net_events").is_some() {
        "net_events"
    } else if obj.map("events").is_some() {
        "events"
    } else {
        eprintln!("[ebpf/net] map 'EVENTS'/'net_events'/'events' not found");
        return;
    };

    let mut events_map = match obj.map_mut(map_name) {
        Some(m) => m,
        None => { eprintln!("[ebpf/net] map '{map_name}' vanished after lookup"); return; }
    };

    let mut rb = RingBufferBuilder::new();
    rb.add(&mut events_map, move |data: &[u8]| {
        if data.len() < std::mem::size_of::<NetEvt>() { return 0; }
        let mut ev = NetEvt { pid: 0, kind: 0, family: 0, dport: 0, is_v6: 0,
                              daddr_v4: 0, daddr_v6: [0; 16], bytes: 0 };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                &mut ev as *mut _ as *mut u8,
                std::mem::size_of::<NetEvt>(),
            );
        }

        match ev.kind {
            1 => {
                let ip = ip_to_string(ev.is_v6, ev.daddr_v4, ev.daddr_v6);
                let port = ntohs(ev.dport);

                if is_private_ip(&ip) || is_ephemeral_port(port) { return 0; }
                if !rate_ok("network::connect", NF_RATE_LIMIT_CONN) { return 0; }
                let key = format!("pid={}|dst={}:{}/fam={}", ev.pid, ip, port, ev.family);
                if !dedupe_ok(&key, NF_CONNECT_TTL_SECS) { return 0; }

                if let Some((conf, risk, esc)) = score_connect(&ip, port) {
                    let mut map = HashMap::new();
                    map.insert("pid".into(), ev.pid.to_string());
                    map.insert("dst_addr".into(), ip);
                    map.insert("dst_port".into(), port.to_string());
                    map.insert("family".into(), ev.family.to_string());
                    map.insert("risk_score".into(), risk.to_string());
                    map.insert("gnn_escalate".into(), if esc { "true" } else { "false" }.into());
                    emit(TelemetryOutput {
                        category: "network".into(),
                        signal: "network::connect".into(),
                        confidence: conf,
                        data: map,
                    });
                }
            }
            2 | 3 => {
                if ev.bytes.abs() < BYTES_MIN_EMIT { return 0; }
                let sig = if ev.kind == 2 { "network::tx_bytes" } else { "network::rx_bytes" };
                if !rate_ok(sig, NF_RATE_LIMIT_BYTES) { return 0; }
                let key = format!("pid={}|{}", ev.pid, sig);
                if !dedupe_ok(&key, NF_BYTES_TTL_SECS) { return 0; }

                let mut map = HashMap::new();
                map.insert("pid".into(), ev.pid.to_string());
                map.insert("bytes".into(), ev.bytes.to_string());
                map.insert("risk_score".into(), "15".into());
                map.insert("gnn_escalate".into(), "false".into());
                emit(TelemetryOutput {
                    category: "network".into(),
                    signal: sig.into(),
                    confidence: 0.20,
                    data: map,
                });
            }
            _ => {}
        }

        0
    }).expect("ringbuf add");

    let rb = rb.build().expect("ringbuf build");

    thread::spawn(move || loop {
        let _ = rb.poll(Duration::from_millis(100));
        std::thread::sleep(Duration::from_millis(1));
    });

    fn emit(out: TelemetryOutput) {
        let mut m = out.data.clone();
        m.insert("category".into(), out.category.clone());
        m.insert("signal".into(), out.signal.clone());
        m.insert("confidence".into(), format!("{:.2}", out.confidence));
        write_telemetry_record(m.clone());
        push_to_gnn_vector_log(m.clone());
        store_replay_event(m);
    }
}

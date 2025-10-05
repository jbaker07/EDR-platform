// src/ebpf/net_flow_reader.rs
// Minimal ringbuf reader for net_flow.bpf.o
// Emits:
//   - network::connect
//   - network::tx_bytes / network::rx_bytes
//
// Debug / control envs (all optional):
//   EDR_NET_ALLOW_COMMON_PORTS=1      // include 80/443/53
//   EDR_NET_ALLOW_PRIVATE=1           // include 127.0.0.1 / RFC1918
//   EDR_NET_ALLOW_EPHEMERAL=1         // include dst ports >= 49152
//   EDR_NET_RATE_DISABLE=1            // disable rate limiting
//   EDR_NET_BYTES_MIN=<i64>           // default 4096
//   EDR_NET_SYNTH_CONNECT_FROM_BYTES=1// force synthetic connect on first bytes (also auto-enables if connect attach fails)

#![cfg(target_os = "linux")]

use std::collections::{HashMap, VecDeque};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use lazy_static::lazy_static;
use libbpf_rs::{ObjectBuilder, RingBufferBuilder};

use crate::events_reader;
use crate::pb_adapter;
use forensic_hooks::gnn_hook::push_to_gnn_vector_log;
use forensic_hooks::modules::replay_writer::store_replay_event;
use forensic_hooks::telemetry_types::TelemetryOutput;
use forensic_hooks::telemetry_writer::{write_telemetry_record, TelemetryWriter}; // for resolve_bpf()

#[repr(C)]
#[derive(Clone, Copy)]
struct NetEvt {
    pid: u32,
    kind: u32,   // 1 connect, 2 tx, 3 rx
    family: u16, // AF_*
    dport: u16,  // BE
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
        Ipv6Addr::from(v6).to_string()
    } else {
        Ipv4Addr::from(v4).to_string()
    }
}

/* ------------------- env helpers ------------------- */

fn env_bool(k: &str) -> bool {
    matches!(
        std::env::var(k).ok().as_deref(),
        Some("1" | "true" | "TRUE" | "yes" | "YES")
    )
}
fn env_i64(k: &str, d: i64) -> i64 {
    std::env::var(k)
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(d)
}

/* ------------------- gating + scoring helpers ------------------- */

lazy_static! {
    static ref NF_DEDUPE: std::sync::Mutex<HashMap<String, Instant>> =
        std::sync::Mutex::new(HashMap::new());
    static ref NF_RATE: std::sync::Mutex<HashMap<String, VecDeque<Instant>>> =
        std::sync::Mutex::new(HashMap::new());
}

const NF_CONNECT_TTL_SECS: u64 = 90;
const NF_BYTES_TTL_SECS: u64 = 10;
const NF_RATE_WINDOW_SECS: u64 = 60;
const NF_RATE_LIMIT_CONN: usize = 500;
const NF_RATE_LIMIT_BYTES: usize = 800;
const BYTES_MIN_EMIT: i64 = 4096;

fn is_private_ip(s: &str) -> bool {
    if s == "127.0.0.1" {
        return true;
    }
    if s.starts_with("10.") || s.starts_with("192.168.") {
        return true;
    }
    if s.starts_with("172.") {
        if let Some(seg) = s.split('.').nth(1).and_then(|x| x.parse::<u8>().ok()) {
            return (16..=31).contains(&seg);
        }
    }
    false
}
fn is_ephemeral_port(p: u16) -> bool {
    p >= 49152
}
fn is_common_service_port(p: u16) -> bool {
    matches!(
        p,
        22 | 25 | 53 | 80 | 110 | 123 | 143 | 389 | 443 | 465 | 587 | 993 | 995
    )
}
const DENY_PORTS: &[u16] = &[31337, 4444, 1337, 6666, 6667];

fn rate_ok(signal: &str, limit: usize) -> bool {
    let mut r = NF_RATE.lock().unwrap();
    let q = r.entry(signal.to_string()).or_default();
    let now = Instant::now();
    let cutoff = now - Duration::from_secs(NF_RATE_WINDOW_SECS);
    while matches!(q.front(), Some(t) if *t < cutoff) {
        q.pop_front();
    }
    if q.len() >= limit {
        return false;
    }
    q.push_back(now);
    true
}

fn dedupe_ok(key: &str, ttl_secs: u64) -> bool {
    let mut m = NF_DEDUPE.lock().unwrap();
    let now = Instant::now();
    if let Some(last) = m.get(key) {
        if now.duration_since(*last) < Duration::from_secs(ttl_secs) {
            return false;
        }
    }
    m.insert(key.to_string(), now);
    true
}

/// Returns (confidence, risk_score, escalate)
fn score_connect(dst_ip: &str, dst_port: u16) -> Option<(f32, u32, bool)> {
    if is_private_ip(dst_ip) {
        return None;
    }
    if is_ephemeral_port(dst_port) {
        return None;
    }

    let mut conf: f32 = 0.25;
    let mut risk: u32 = 12;
    let mut esc = false;

    if DENY_PORTS.contains(&dst_port) {
        conf = 0.85_f32;
        risk = 70;
        esc = true;
    }

    if dst_port < 1024 && !is_common_service_port(dst_port) {
        conf = conf.max(0.70_f32);
        risk = risk.max(55);
    } else if !is_common_service_port(dst_port) {
        conf = conf.max(0.45_f32);
        risk = risk.max(35);
    }

    if conf < 0.35_f32 {
        return None;
    }
    Some((conf, risk, esc))
}

/* ------------------------- reader ------------------------- */

pub fn start_net_flow_reader(_writer: Arc<Mutex<TelemetryWriter>>) {
    // raise memlock best-effort
    unsafe {
        let rlim = libc::rlimit {
            rlim_cur: u64::MAX,
            rlim_max: u64::MAX,
        };
        let _ = libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim as *const _);
    }

    let prefix = std::env::var("EDR_PIN_PREFIX").unwrap_or_else(|_| "/sys/fs/bpf/edr".to_string());
    let force = std::env::var("EDR_FORCE_PINNED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let base = prefix.trim_end_matches('/');

    // Env knobs (debug-friendly)
    let allow_common = env_bool("EDR_NET_ALLOW_COMMON_PORTS");
    let allow_private = env_bool("EDR_NET_ALLOW_PRIVATE");
    let allow_ephemeral = env_bool("EDR_NET_ALLOW_EPHEMERAL");
    let rate_disable = env_bool("EDR_NET_RATE_DISABLE");
    let bytes_min_emit = env_i64("EDR_NET_BYTES_MIN", BYTES_MIN_EMIT);
    let synth_connect_env = env_bool("EDR_NET_SYNTH_CONNECT_FROM_BYTES");

    let handler_builder = {
        let allow_common = allow_common;
        let allow_private = allow_private;
        let allow_ephemeral = allow_ephemeral;
        let rate_disable = rate_disable;
        let bytes_min_emit = bytes_min_emit;
        move |synth_connect: bool| -> Arc<dyn Fn(&[u8]) + Send + Sync> {
            let allow_common = allow_common;
            let allow_private = allow_private;
            let allow_ephemeral = allow_ephemeral;
            let rate_disable = rate_disable;
            let bytes_min_emit = bytes_min_emit;
            let handler: Arc<dyn Fn(&[u8]) + Send + Sync> = Arc::new(move |data: &[u8]| {
                crate::metrics::events_in().fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                if data.len() < std::mem::size_of::<NetEvt>() {
                    return;
                }
                let mut ev = NetEvt {
                    pid: 0,
                    kind: 0,
                    family: 0,
                    dport: 0,
                    is_v6: 0,
                    daddr_v4: 0,
                    daddr_v6: [0; 16],
                    bytes: 0,
                };
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        data.as_ptr(),
                        &mut ev as *mut _ as *mut u8,
                        std::mem::size_of::<NetEvt>(),
                    );
                }

                let ip = ip_to_string(ev.is_v6, ev.daddr_v4, ev.daddr_v6);
                let port = ntohs(ev.dport);

                match ev.kind {
                    1 => {
                        if !allow_private && is_private_ip(&ip) {
                            return;
                        }
                        if !allow_ephemeral && is_ephemeral_port(port) {
                            return;
                        }
                        if !rate_disable && !rate_ok("network::connect", NF_RATE_LIMIT_CONN) {
                            return;
                        }

                        let key = format!("pid={}|dst={}:{}/fam={}", ev.pid, ip, port, ev.family);
                        if !dedupe_ok(&key, NF_CONNECT_TTL_SECS) {
                            return;
                        }

                        let (conf, risk, esc) = match score_connect(&ip, port) {
                            Some(t) => t,
                            None if allow_common || allow_private || allow_ephemeral => {
                                (0.20, 10, false)
                            }
                            None => return,
                        };

                        if !allow_common && is_common_service_port(port) {
                            return;
                        }

                        emit(TelemetryOutput {
                            category: "network".into(),
                            signal: "network::connect".into(),
                            confidence: conf,
                            data: {
                                let mut m = HashMap::new();
                                m.insert("pid".into(), ev.pid.to_string());
                                m.insert("dst_addr".into(), ip);
                                m.insert("dst_port".into(), port.to_string());
                                m.insert("family".into(), ev.family.to_string());
                                m.insert("risk_score".into(), risk.to_string());
                                m.insert(
                                    "gnn_escalate".into(),
                                    if esc { "true" } else { "false" }.into(),
                                );
                                m
                            },
                        });
                    }
                    2 | 3 => {
                        if ev.bytes.abs() < bytes_min_emit {
                            return;
                        }
                        let sig = if ev.kind == 2 {
                            "network::tx_bytes"
                        } else {
                            "network::rx_bytes"
                        };
                        if !rate_disable && !rate_ok(sig, NF_RATE_LIMIT_BYTES) {
                            return;
                        }
                        let key_bytes = format!("pid={}|{}", ev.pid, sig);
                        if !dedupe_ok(&key_bytes, NF_BYTES_TTL_SECS) {
                            return;
                        }

                        if synth_connect {
                            let key_conn =
                                format!("pid={}|dst={}:{}/fam={}", ev.pid, ip, port, ev.family);
                            if dedupe_ok(&key_conn, NF_CONNECT_TTL_SECS) {
                                emit(TelemetryOutput {
                                    category: "network".into(),
                                    signal: "network::connect".into(),
                                    confidence: 0.15,
                                    data: {
                                        let mut m = HashMap::new();
                                        m.insert("pid".into(), ev.pid.to_string());
                                        m.insert("dst_addr".into(), ip.clone());
                                        m.insert("dst_port".into(), port.to_string());
                                        m.insert("family".into(), ev.family.to_string());
                                        m.insert("risk_score".into(), "10".into());
                                        m.insert("gnn_escalate".into(), "false".into());
                                        m.insert("synthesized".into(), "true".into());
                                        m
                                    },
                                });
                            }
                        }

                        emit(TelemetryOutput {
                            category: "network".into(),
                            signal: sig.into(),
                            confidence: 0.20,
                            data: {
                                let mut m = HashMap::new();
                                m.insert("pid".into(), ev.pid.to_string());
                                m.insert("bytes".into(), ev.bytes.to_string());
                                m.insert("risk_score".into(), "15".into());
                                m.insert("gnn_escalate".into(), "false".into());
                                m
                            },
                        });
                    }
                    _ => {}
                }
            });
            handler
        }
    };

    if force || super::pins_available(&prefix) {
        let path = format!("{base}/net_events");
        if !Path::new(&path).exists() {
            log::error!("pinned subscribe failed: missing {path}");
            log::error!("hint: run edr_attach_any -p {prefix}");
            return;
        }
        log::info!("subscribe: net_flow_reader source=ringbuf path={path}");
        let handler = handler_builder(synth_connect_env);
        let handler_pinned = handler.clone();
        if let Err(e) = events_reader::start_ringbuf_reader_from_pinned(&path, move |data| {
            handler_pinned.as_ref()(data);
        }) {
            log::error!("pinned subscribe failed for net_flow_reader: {e}");
        }
        return;
    }

    #[cfg(not(all(target_os = "linux", feature = "with-ebpf-load")))]
    {
        log::info!("loader path disabled; use pins");
        return;
    }

    #[cfg(all(target_os = "linux", feature = "with-ebpf-load"))]
    {
        log::warn!("loader: net_flow_reader using .bpf.o (feature explicitly enabled)");

        let obj_path = events_reader::resolve_bpf("EDR_BPF_NET_OBJ", "net_flow.bpf.o");
        eprintln!("[ebpf/net] object = {}", obj_path);

        let open = match ObjectBuilder::default().open_file(&obj_path) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("[ebpf/net] open {obj_path} failed: {e}");
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

        let mut links: Vec<libbpf_rs::Link> = Vec::new();
        let mut attached = |candidates: &[&str], cat: &str, name: &str| -> bool {
            for func in candidates {
                if let Some(p) = obj.prog_mut(func) {
                    match p.attach_tracepoint(cat, name) {
                        Ok(l) => {
                            links.push(l);
                            return true;
                        }
                        Err(_) => {}
                    }
                }
            }
            eprintln!(
                "[ebpf/net] attach failed for any of {:?} @ {}/{}",
                candidates, cat, name
            );
            false
        };

        let connect_attached = attached(
            &["tp_enter_connect", "tp_enter_connec"],
            "syscalls",
            "sys_enter_connect",
        );
        let _ = attached(&["tp_exit_sendto"], "syscalls", "sys_exit_sendto");
        let _ = attached(&["tp_exit_sendmsg"], "syscalls", "sys_exit_sendmsg");
        let _ = attached(
            &["tp_exit_recvfrom", "tp_exit_recvfro"],
            "syscalls",
            "sys_exit_recvfrom",
        );
        let _ = attached(&["tp_exit_recvmsg"], "syscalls", "sys_exit_recvmsg");

        let synth_connect = synth_connect_env || !connect_attached;
        if synth_connect && !connect_attached {
            eprintln!(
                "[ebpf/net] connect attach missing → enabling synthetic connect from first bytes"
            );
        }
        let handler = handler_builder(synth_connect);

        let map_name = match events_reader::choose_single_ringbuf_name(&obj) {
            Some(name) => name,
            None => {
                eprintln!(
                    "[ebpf/net] ❌ ringbuf map not found (wx_events/net_events/edr_events_rb/EVENTS/events)"
                );
                return;
            }
        };
        eprintln!("[ebpf/net] subscribing ringbuf map='{map_name}'");

        let mut events_map = match obj.map_mut(&map_name) {
            Some(m) => m,
            None => {
                eprintln!("[ebpf/net] map '{map_name}' vanished after lookup");
                return;
            }
        };

        let mut rb = RingBufferBuilder::new();
        let handler_rb = handler.clone();
        if let Err(e) = rb.add(&mut events_map, move |data: &[u8]| {
            handler_rb.as_ref()(data);
            0
        }) {
            eprintln!("[ebpf/net] ❌ ringbuf add failed for map='{map_name}': {e}");
            return;
        }

        let rb = match rb.build() {
            Ok(rb) => rb,
            Err(e) => {
                eprintln!("[ebpf/net] ringbuf build failed: {e}");
                return;
            }
        };

        let _leaked_links: &'static mut Vec<libbpf_rs::Link> = Box::leak(Box::new(links));
        std::mem::forget(obj);

        std::thread::spawn(move || loop {
            let _ = rb.poll(Duration::from_millis(10));
        });
    }
}

fn emit(out: TelemetryOutput) {
    pb_adapter::emit_adapter_facts(&out);

    let mut m = out.data.clone();
    m.insert("category".into(), out.category.clone());
    m.insert("signal".into(), out.signal.clone());
    m.insert("confidence".into(), format!("{:.2}", out.confidence));
    write_telemetry_record(m.clone());
    push_to_gnn_vector_log(m.clone());
    let _ = store_replay_event(m);
}

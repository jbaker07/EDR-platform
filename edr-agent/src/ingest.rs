// src/ingest.rs
// Compact kernel-event ingest: map eBPF reader outputs -> Event,
// window them into Episodes, run feature emitters + KRIM, emit tagged signals.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use tokio::sync::mpsc;

use crate::telemetry_types::TelemetryOutput;
use crate::telemetry_writer::TelemetryWriter;
use crate::modules::replay_writer::store_replay_event;
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::telemetry::push_feature_as_signal;

use crate::episode::Episode;
use crate::event::{Event, EventType};
use crate::services::krim_lite::analyze_episode_and_score;

static TX: OnceLock<mpsc::Sender<KernelEvent>> = OnceLock::new();

#[derive(Debug, Clone)]
pub enum KernelEventKind {
    Exec,
    Connect { family: u16, dport_be: u16, dst: String },
    TxBytes { bytes: u64 },
    RxBytes { bytes: u64 },
    Mmap { addr: u64, len: u64, prot: u64 },
    Mprotect { addr: u64, len: u64, prot: u64 },
    WXTransition { addr: u64, len: u64, prot: u64 },
    MemfdExec { bytes: u64 },
    Fork,
    Setuid,
    Capset,
    Other,
}

#[derive(Debug, Clone)]
pub struct KernelEvent {
    pub ts: DateTime<Utc>,
    pub pid: u32,
    pub kind: KernelEventKind,
    pub fields: HashMap<String, String>,
}

pub fn ingest_kernel_event(ev: KernelEvent) {
    if let Some(tx) = TX.get() {
        let _ = tx.try_send(ev);
    }
}

pub fn map_syscall_id_to_kind(id: i64) -> Option<KernelEventKind> {
    // Use libc syscall constants to stay portable.
    match id {
        x if x == libc::SYS_execve as i64 || x == libc::SYS_execveat as i64 => Some(KernelEventKind::Exec),
        x if x == libc::SYS_mmap as i64 => Some(KernelEventKind::Mmap { addr: 0, len: 0, prot: 0 }),
        x if x == libc::SYS_mprotect as i64 => Some(KernelEventKind::Mprotect { addr: 0, len: 0, prot: 0 }),
        x if x == libc::SYS_connect as i64 => Some(KernelEventKind::Connect { family: 0, dport_be: 0, dst: String::new() }),
        x if x == libc::SYS_clone as i64 || x == libc::SYS_fork as i64 || x == libc::SYS_vfork as i64 => Some(KernelEventKind::Fork),
        x if x == libc::SYS_setuid as i64 || x == libc::SYS_setreuid as i64 || x == libc::SYS_setresuid as i64 => Some(KernelEventKind::Setuid),
        x if x == libc::SYS_capset as i64 => Some(KernelEventKind::Capset),
        _ => None,
    }
}

// Map KernelEventKind -> EventType (Episode layer)
fn to_event_type(k: &KernelEventKind) -> EventType {
    use KernelEventKind::*;
    match k {
        Exec => EventType::Execve,
        Connect { .. } => EventType::BeaconTick,
        TxBytes { .. } => EventType::PipeToSocketBytes,   // reuse net-bytes marker
        RxBytes { .. } => EventType::FileToPipeBytes,     // symmetric marker
        Mmap { .. } => EventType::Mmap,
        Mprotect { .. } => EventType::Mprotect,
        WXTransition { .. } => EventType::Mprotect,       // W→X implies exec-prot flip
        MemfdExec { .. } => EventType::MemfdWrite,        // chain implies memfd activity
        Fork => EventType::Other,
        Setuid => EventType::Setuid,
        Capset => EventType::Capset,
        Other => EventType::Other,
    }
}

// Emit a TelemetryOutput (for tagging/scoring fan-in)
fn emit_signal(category: &str, signal: &str, mut data: HashMap<String, String>, confidence: f32) {
    use crate::telemetry_writer::write_telemetry_record;
    data.insert("category".into(), category.into());
    data.insert("signal".into(), signal.into());
    data.insert("confidence".into(), format!("{confidence}"));
    write_telemetry_record(data.clone());
    push_to_gnn_vector_log(data.clone());
    store_replay_event(data);
}

// Window state per host (you can shard by endpoint id if desired)
struct Window {
    started: Instant,
    events: Vec<Event>,
}

impl Window {
    fn new() -> Self {
        Self { started: Instant::now(), events: Vec::new() }
    }
    fn push(&mut self, ev: Event) {
        self.events.push(ev);
    }
    fn should_flush(&self, max_age: Duration, max_events: usize) -> bool {
        self.started.elapsed() >= max_age || self.events.len() >= max_events
    }
}

pub fn start_ingest_pipeline(_writer: Arc<tokio::sync::Mutex<TelemetryWriter>>) {
    // single global channel
    let (tx, mut rx) = mpsc::channel::<KernelEvent>(4096);
    let _ = TX.set(tx);

    // Tuning knobs
    let window_age = Duration::from_secs(10);
    let window_max_events = 512;

    tokio::spawn(async move {
        // Use hostname as Episode ctx
        let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
        let mut window = Window::new();

        loop {
            tokio::select! {
                Some(kev) = rx.recv() => {
                    // Map kernel event to Episode::Event
                    let et = to_event_type(&kev.kind);
                    let mut ev = Event { ts: kev.ts, event_type: et, ..Default::default() };

                    // Optional enrich: set byte counts / fields on Event
                    match kev.kind {
                        KernelEventKind::TxBytes { bytes } => { ev.bytes = Some(bytes); }
                        KernelEventKind::RxBytes { bytes } => { ev.bytes = Some(bytes); }
                        KernelEventKind::Mmap { addr, len, prot } |
                        KernelEventKind::Mprotect { addr, len, prot } |
                        KernelEventKind::WXTransition { addr, len, prot } => {
                            ev.addr = Some(addr);
                            ev.len  = Some(len);
                            ev.prot = Some(format!("{prot:#x}"));
                        }
                        KernelEventKind::Connect { family, dport_be, ref dst } => {
                            ev.dst = Some(format!("{dst}:{:?}", u16::from_be(dport_be)));
                            ev.family = Some(family as i32);
                        }
                        KernelEventKind::MemfdExec { bytes } => { ev.bytes = Some(bytes); }
                        _ => {}
                    }

                    window.push(ev);

                    if window.should_flush(window_age, window_max_events) {
                        flush_window(&host, &mut window);
                    }
                }
                // Periodic hard flush to avoid starvation
                _ = tokio::time::sleep(Duration::from_millis(500)) => {
                    if window.should_flush(window_age, window_max_events) {
                        flush_window(&host, &mut window);
                    }
                }
            }
        }
    });
}

fn flush_window(host: &str, window: &mut Window) {
    if window.events.is_empty() {
        window.started = Instant::now();
        return;
    }

    let mut ep = Episode::new(host.to_string());
    ep.context_id = Some("default".into());
    ep.role = Some("default".into());
    for e in window.events.drain(..) {
        ep.push(e);
    }
    window.started = Instant::now();

    // 1) Feature emitters
    let mut store = crate::baselines::BaselineStore::open("state/baselines")
        .unwrap_or_else(|_| crate::baselines::BaselineStore::in_memory());

    use crate::traits::FeatureEmitter;
    let mut feats = Vec::new();
    feats.extend(crate::memory_priv::MemoryPrivEmitter::new().emit(&ep, &mut store));
    feats.extend(crate::net_cadence::NetCadenceEmitter::new().emit(&ep, &mut store));
    let _ = store.flush();

    for f in feats.iter().filter(|f| f.z.abs() > 2.5) {
        // push a feature anomaly signal back into the pipeline
        push_feature_as_signal(f);
        let mut m = HashMap::new();
        m.insert("key".into(), f.key.clone());
        m.insert("value".into(), f.value.clone());
        m.insert("z".into(), format!("{:.2}", f.z));
        m.insert("family".into(), f.family.clone());
        emit_signal("feature", "feature::anomaly", m, 0.9);
    }

    // 2) KRIM-lite on the Episode graph
    let krim_events = analyze_episode_and_score(&ep, "state/krim_baseline.json", 4);
    for ev in krim_events {
        let mut m = HashMap::new();
        m.insert("pid".into(), ev.pid.to_string());
        m.insert("score".into(), format!("{:.2}", ev.score));
        m.insert("reason".into(), format!("{:?}", ev.reason));
        emit_signal("graph", "krim_alert", m, 0.95);
    }
}

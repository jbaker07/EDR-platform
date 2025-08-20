// src/ingest.rs
// Kernel-event ingest: map compact kernel events -> Event, window into Episodes,
// run feature emitters, and emit signals back into your existing scoring path.

use std::collections::{HashMap, VecDeque};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use tokio::sync::mpsc;

use crate::episode::Episode;
use crate::event::{Event, EventType};
use crate::telemetry::push_feature_as_signal;
use crate::telemetry_writer::write_telemetry_record;
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::modules::replay_writer::store_replay_event;

// ---------- Public API ----------

/// Send a kernel event into the ingest pipeline (used by eBPF readers).
pub fn ingest_kernel_event(ev: KernelEvent) {
    if let Some(tx) = TX.get() {
        let _ = tx.try_send(ev);
    }
}

/// Best-effort map from raw syscall id to a normalized kernel event kind.
/// On Linux we use libc::SYS_* constants; on other targets we return None.
pub fn map_syscall_id_to_kind(id: i64) -> Option<KernelEventKind> {
    #[cfg(target_os = "linux")]
    {
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
    #[cfg(not(target_os = "linux"))]
    {
        let _ = id;
        None
    }
}

/// Start the ingest task: drains kernel events, builds Episodes, runs emitters/KRIM,
/// and emits signals for scoring. Call this once during startup (Linux block is fine).
pub fn start_ingest_pipeline() {
    let (tx, mut rx) = mpsc::channel::<KernelEvent>(4096);
    let _ = TX.set(tx);

    // Tunables
    let window_age = Duration::from_secs(10);
    let window_max_events = 512;

    tokio::spawn(async move {
        let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
        let mut window = Window::new();

        loop {
            tokio::select! {
                Some(kev) = rx.recv() => {
                    // Map to Episode::Event and add to window
                    if let Some(ev) = map_kernel_to_event(kev) {
                        window.push(ev);
                    }
                    if window.should_flush(window_age, window_max_events) {
                        flush_window(&host, &mut window);
                    }
                }
                _ = tokio::time::sleep(Duration::from_millis(500)) => {
                    if window.should_flush(window_age, window_max_events) {
                        flush_window(&host, &mut window);
                    }
                }
            }
        }
    });
}

// ---------- Data types ----------

static TX: OnceLock<mpsc::Sender<KernelEvent>> = OnceLock::new();

#[derive(Debug, Clone)]
pub enum KernelEventKind {
    // Process/exec lineage
    Exec,
    Fork,
    // Network
    Connect { family: u16, dport_be: u16, dst: String },
    TxBytes { bytes: u64 },
    RxBytes { bytes: u64 },
    // Memory / WX / memfd
    Mmap { addr: u64, len: u64, prot: u64 },
    Mprotect { addr: u64, len: u64, prot: u64 },
    WXTransition { addr: u64, len: u64, prot: u64 },
    MemfdExec { bytes: u64 },
    // Privilege
    Setuid,
    Capset,
    // Fallback
    Other,
}

#[derive(Debug, Clone)]
pub struct KernelEvent {
    pub ts: DateTime<Utc>,
    pub pid: u32,
    pub kind: KernelEventKind,
    pub fields: HashMap<String, String>,
}

// ---------- Windowing / Episode building ----------

struct Window {
    started: Instant,
    events: Vec<Event>,
}
impl Window {
    fn new() -> Self { Self { started: Instant::now(), events: Vec::new() } }
    fn push(&mut self, ev: Event) { self.events.push(ev); }
    fn should_flush(&self, max_age: Duration, max_events: usize) -> bool {
        self.started.elapsed() >= max_age || self.events.len() >= max_events
    }
}

fn map_kernel_to_event(kev: KernelEvent) -> Option<Event> {
    let mut ev = Event { ts: kev.ts, event_type: EventType::Other, ..Default::default() };
    use KernelEventKind::*;
    ev.event_type = match kev.kind {
        Exec => EventType::Execve,
        Fork => EventType::Other,
        Connect { family, dport_be, dst } => {
            ev.family = Some(family as i32);
            ev.dst = Some(format!("{dst}:{}", u16::from_be(dport_be)));
            EventType::BeaconTick
        }
        TxBytes { bytes } => { ev.bytes = Some(bytes); EventType::PipeToSocketBytes }
        RxBytes { bytes } => { ev.bytes = Some(bytes); EventType::FileToPipeBytes }
        Mmap { addr, len, prot } => { ev.addr = Some(addr); ev.len = Some(len); ev.prot = Some(format!("{prot:#x}")); EventType::Mmap }
        Mprotect { addr, len, prot } => { ev.addr = Some(addr); ev.len = Some(len); ev.prot = Some(format!("{prot:#x}")); EventType::Mprotect }
        WXTransition { addr, len, prot } => { ev.addr = Some(addr); ev.len = Some(len); ev.prot = Some(format!("{prot:#x}")); EventType::Mprotect }
        MemfdExec { bytes } => { ev.bytes = Some(bytes); EventType::MemfdWrite }
        Setuid => EventType::Setuid,
        Capset => EventType::Capset,
        Other => EventType::Other,
    };
    Some(ev)
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
        // (Optional) add pid/uid linkage once your Event carries it
        ep.push(e);
    }
    window.started = Instant::now();

    run_feature_emitters(&ep);
    run_krim_on_episode(&ep);
}

// ---------- Emitters / Signals ----------

fn run_feature_emitters(ep: &Episode) {
    let mut store = crate::baselines::BaselineStore::open("state/baselines")
        .unwrap_or_else(|_| crate::baselines::BaselineStore::in_memory());

    use crate::traits::FeatureEmitter;
    let mut feats = Vec::new();
    feats.extend(crate::memory_priv::MemoryPrivEmitter::new().emit(ep, &mut store));
    feats.extend(crate::net_cadence::NetCadenceEmitter::new().emit(ep, &mut store));
    let _ = store.flush();

    for f in feats.into_iter().filter(|f| f.z.abs() > 2.5) {
        // 1) tag via your existing helper (feeds trust engine)
        push_feature_as_signal(&f);

        // 2) also write a TelemetryOutput-ish record for logs/GNN/replay
        let mut m = HashMap::new();
        m.insert("category".into(), "feature".into());
        m.insert("signal".into(), "feature::anomaly".into());
        m.insert("key".into(), f.key.clone());
        m.insert("value".into(), f.value.clone());
        m.insert("family".into(), f.family.clone());
        m.insert("z".into(), format!("{:.2}", f.z));

        write_telemetry_record(m.clone());
        push_to_gnn_vector_log(m.clone());
        store_replay_event(m);
    }
}

fn run_krim_on_episode(ep: &Episode) {
    // Build a lightweight graph from Episode events → nodes/edges for KRIM-lite.
    // If your graph types differ, adjust this adapter in one place.
    use crate::graph_builder::{GraphEdge, GraphNode};

    let mut nodes: Vec<GraphNode> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();

    for (i, _) in ep.events.iter().enumerate() {
        let id = format!("e-{}", i);
        // Trust proxy: earlier events get higher “trust”, later get lower (toy heuristic).
        let trust_score = 1.0f32 - (i as f32 / (ep.events.len().max(1) as f32));
        nodes.push(GraphNode {
            id: id.clone(),
            trust_score,
            ..Default::default()
        });
        if i > 0 {
            edges.push(GraphEdge {
                source: format!("e-{}", i - 1),
                target: id,
                ..Default::default()
            });
        }
    }

    let baseline_path = "state/krim_baseline.json";
    let events = crate::services::krim_lite::analyze_graph_and_score(&nodes, &edges, 4, baseline_path);

    for ev in events {
        let mut m = HashMap::new();
        m.insert("category".into(), "graph".into());
        m.insert("signal".into(), "krim_alert".into());
        m.insert("pid".into(), ev.pid.to_string());
        m.insert("ppid".into(), ev.ppid.to_string());
        m.insert("score".into(), format!("{:.2}", ev.score));
        m.insert("reason".into(), format!("{:?}", ev.reason));

        write_telemetry_record(m.clone());
        push_to_gnn_vector_log(m.clone());
        store_replay_event(m);
    }
}

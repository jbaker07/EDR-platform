mod logger;
mod config;
mod telemetry;
pub mod telemetry_writer;
pub mod telemetry_types;
pub mod services;
mod trust_hook;
mod gnn_hook;
pub mod utils;
pub mod modules;
pub mod relay;
pub mod forensic;
pub mod adaptive_threshold_engine;
pub mod trust_digest_engine;
pub mod score_reason;
pub mod session_trust_curve;

use crate::telemetry_writer::TelemetryWriter;
use crate::logger::init_logger;
use crate::config::load_and_verify_policy;
use crate::telemetry::{
    get_current_telemetry_snapshot, run_sideeffect_monitors_and_collect, TelemetryRecord,
    start_realtime_monitors,
};
use crate::telemetry_types::TelemetryOutput;
use crate::utils::time::now_ts;
use crate::services::trust_engine_final::{evaluate_and_dispatch_trust_score, TelemetryData, TrustResult};

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::OnceLock;

use crate::mahalanobis::{Mahala, MahalaCfg};
use crate::elliptic_envelope::EllipticEnvelope;
use crate::krim::analyze_graph_and_score;
use crate::graph_builder::{GraphNode, GraphEdge};

// —— global detectors state ——
static MAHALA_ENGINE: OnceLock<std::sync::Mutex<Mahala>> = OnceLock::new();
static ENV_BASELINE: OnceLock<std::sync::Mutex<VecDeque<Vec<f64>>>> = OnceLock::new();
static ENV_MODEL:   OnceLock<std::sync::Mutex<Option<EllipticEnvelope>>> = OnceLock::new();

fn mahala_engine() -> &'static std::sync::Mutex<Mahala> {
    MAHALA_ENGINE.get_or_init(|| std::sync::Mutex::new(Mahala::default()))
}
fn env_baseline() -> &'static std::sync::Mutex<VecDeque<Vec<f64>>> {
    ENV_BASELINE.get_or_init(|| std::sync::Mutex::new(VecDeque::with_capacity(1024)))
}
fn env_model() -> &'static std::sync::Mutex<Option<EllipticEnvelope>> {
    ENV_MODEL.get_or_init(|| std::sync::Mutex::new(None))
}

// —— helpers ——
impl std::convert::TryFrom<TelemetryOutput> for TelemetryRecord {
    type Error = String;
    fn try_from(output: TelemetryOutput) -> Result<Self, Self::Error> {
        let mut rec = TelemetryRecord {
            timestamp: now_ts(),
            pid: 0,
            ppid: 0,
            uid: 0,
            binary_path: output.data.get("binary_path").cloned().unwrap_or_default(),
            command_line: output.data.get("command_line").cloned().unwrap_or_default(),
            cwd: output.data.get("cwd").cloned().unwrap_or_default(),
            ..Default::default()
        };
        // tag + confidence → risk
        rec.tags.push(output.signal);
        rec.risk_score = Some((output.confidence * 100.0) as u32);
        Ok(rec)
    }
}

fn build_episode_from_records(records: &[TelemetryRecord]) -> Option<crate::episode::Episode> {
    use chrono::Utc;
    use crate::event::{Event, EventType};

    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
    let mut ep = crate::episode::Episode::new(host);
    ep.context_id = Some("default".into());
    ep.role = Some("default".into());

    for r in records {
        if !r.binary_path.is_empty() || !r.command_line.is_empty() {
            ep.push(Event { ts: Utc::now(), event_type: EventType::Execve, ..Default::default() });
        }
        if !r.cwd.is_empty() {
            ep.push(Event { ts: Utc::now(), event_type: EventType::Other, ..Default::default() });
        }
    }
    Some(ep)
}

fn build_graph_from_records(records: &[TelemetryRecord]) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    use std::collections::HashMap;
    let mut nodes: Vec<GraphNode> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();
    let mut proc_id_map: HashMap<i32, String> = HashMap::new();

    for r in records {
        let nid = format!("proc:{}", r.pid);
        proc_id_map.insert(r.pid, nid.clone());

        nodes.push(GraphNode {
            id: nid.clone(),
            trust_score: 1.0f32 - ((r.risk_score.unwrap_or(0).min(100) as f32) / 100.0),
            uid: Some(r.uid),
            pid: Some(r.pid as u32),
            ppid: Some(r.ppid as u32),
            binary_path: if r.binary_path.is_empty() { None } else { Some(r.binary_path.clone()) },
            command_line: if r.command_line.is_empty() { None } else { Some(r.command_line.clone()) },
            cwd: if r.cwd.is_empty() { None } else { Some(r.cwd.clone()) },
            tags: r.tags.clone(),
            anchor_ids: Vec::new(),
            ..Default::default()
        });

        if r.ppid > 0 {
            if let Some(parent) = proc_id_map.get(&r.ppid).cloned() {
                edges.push(GraphEdge { source: parent, target: nid.clone(), ..Default::default() });
            }
        }
        if !r.binary_path.is_empty() {
            let file_id = format!("file:{}", r.binary_path);
            nodes.push(GraphNode { id: file_id.clone(), ..Default::default() });
            edges.push(GraphEdge { source: nid.clone(), target: file_id, ..Default::default() });
        }
        if !r.cwd.is_empty() {
            let dir_id = format!("file:{}", r.cwd);
            nodes.push(GraphNode { id: dir_id.clone(), ..Default::default() });
            edges.push(GraphEdge { source: nid.clone(), target: dir_id, ..Default::default() });
        }
    }
    (nodes, edges)
}

fn tag_batch_outliers_with_mahala_and_envelope(
    records: &mut [TelemetryRecord],
    ep: &crate::episode::Episode,
) {
    let x = Mahala::vectorize_episode(ep);
    if x.is_empty() { return; }
    let p = x.len();

    let now = chrono::Utc::now();
    let mut engine = mahala_engine().lock().unwrap();
    engine.observe("default", &x, now);
    let (d2, _damp, support) = engine.distance_and_damp("default", &x, now);
    drop(engine);

    // χ²_p(0.99) ≈ p + z * sqrt(2p)
    let z = 2.326_347_874_040_840_8_f64;
    let thr = (p as f64) + z * (2.0 * p as f64).sqrt();
    if (d2 as f64) > thr && support >= MahalaCfg::d_support() {
        for r in records.iter_mut() { r.tags.push("mahalanobis_outlier".into()); }
    }

    {
        let mut buf = env_baseline().lock().unwrap();
        if buf.len() == buf.capacity() { buf.pop_front(); }
        buf.push_back(x.clone());
    }
    let model_ready = { let buf = env_baseline().lock().unwrap(); buf.len() >= 256 };

    if model_ready {
        let mut env_lock = env_model().lock().unwrap();
        if env_lock.is_none() || (support % 200 == 0) {
            let buf = env_baseline().lock().unwrap();
            if let Some(env) = EllipticEnvelope::fit(&buf.iter().cloned().collect::<Vec<_>>(), 25, 0.01) {
                *env_lock = Some(env);
            }
        }
        if let Some(env) = env_lock.as_ref() {
            let margin = env.decision_function(&x);
            if margin < 0.0 {
                for r in records.iter_mut() { r.tags.push("elliptic_outlier".into()); }
            }
        }
    }
}

fn tag_records_from_krim(records: &mut [TelemetryRecord]) {
    let (nodes, edges) = build_graph_from_records(records);
    if nodes.len() < 3 { return; }
    let baseline_path = "state/krim_baseline.json";
    let events = analyze_graph_and_score(&nodes, &edges, 4, baseline_path);

    for ev in events {
        let mut tagged = false;
        if ev.pid != 0 {
            for r in records.iter_mut() {
                if r.pid == ev.pid as i32 {
                    r.tags.push("krim_alert".into());
                    tagged = true;
                }
            }
        }
        if !tagged {
            if let Some(last) = records.last_mut() { last.tags.push("krim_alert".into()); }
        }
    }
}

// —— fanout hub ——
fn fanout_all(res: &TrustResult, records: &[TelemetryRecord], data: &TelemetryData) {
    println!(
        "📣 [{}] total={:.2} escalated={} reasons={:?} batch_recs={}",
        res.endpoint_id, res.total_score, res.escalated, res.reasons, records.len()
    );

    let _ = crate::session_explainer::explain(res, records, data)
        .map(|n| println!("🧠 {}", n));
    let _ = crate::risk_alert_writer::persist(res, records, data);

    let _ = crate::stdout_sink::emit(res, records, data);
    let _ = crate::ecs_sink::emit(res, records, data);
    let _ = crate::elastic_bulk::emit(res, records, data);
    let _ = crate::splunk_hec::emit(res, records, data);
    let _ = crate::slack_webhook::emit(res, records, data);

    let _ = crate::graph_snapshot_writer::maybe_snapshot(res, records, data);
    let _ = crate::gnn_bridge::emit(res, records, data);
    let _ = crate::replay_scheduler::consider(res, records, data);
}


// Somewhere in your loader callback
if let Some(rec) = crate::modules::ebpf_ingest::on_edr_event(bytes, &writer) {
    // you can choose to push each record immediately…
    // or collect in a channel and drain into your main batch every 30s
    // Example: send on a crossbeam channel:
    let _ = ebpf_tx.send(rec);
}
// drain eBPF channel if you used one
while let Ok(rec) = ebpf_rx.try_recv() {
    records.push(rec);
}




#[tokio::main(flavor = "multi_thread")]
async fn main() {
    init_logger();

    let _policy = load_and_verify_policy("policy.json")
        .expect("❌ Failed to load or verify policy");

    let writer = Arc::new(Mutex::new(TelemetryWriter::new()));

    #[cfg(target_os = "linux")]
    { start_realtime_monitors(writer.clone()); }

    {
        let flush_writer = writer.clone();
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(Duration::from_secs(30));
            loop {
                tick.tick().await;
                if let Ok(mut locked) = flush_writer.lock() {
                    let _ = locked.flush();
                }
            }
        });
    }

    loop {
        let mut records: Vec<TelemetryRecord> = get_current_telemetry_snapshot(writer.clone());

        for output in run_sideeffect_monitors_and_collect() {
            println!("📥 Passive Signal: {:?}", output);
            match TelemetryRecord::try_from(output) {
                Ok(rec) => records.push(rec),
                Err(_)  => println!("⚠️ Failed to convert TelemetryOutput → TelemetryRecord"),
            }
        }

        if records.is_empty() {
            println!("⚠️  No telemetry records collected this cycle.");
        } else {
            if let Some(ep) = build_episode_from_records(&records) {
                let mut store = crate::baselines::BaselineStore::open("state/baselines")
                    .unwrap_or_else(|_| crate::baselines::BaselineStore::in_memory());

                use crate::traits::FeatureEmitter;
                let mut feats = Vec::new();
                feats.extend(crate::memory_priv::MemoryPrivEmitter::new().emit(&ep, &mut store));
                feats.extend(crate::net_cadence::NetCadenceEmitter::new().emit(&ep, &mut store));
                let _ = store.flush();

                for f in feats.iter().filter(|f| f.z.abs() > 2.0) {
                    println!("🔎 feature: {}={} z={:.2} fam={}", f.key, f.value, f.z, f.family);
                    crate::telemetry::push_feature_as_signal(f);
                }

                tag_batch_outliers_with_mahala_and_envelope(&mut records, &ep);
                tag_records_from_krim(&mut records);
            }

            for rec in &records {
                println!("🟡 Raw Telemetry: {:?}", rec);
                let telemetry_data = TelemetryData::from_record(rec);
                let result: TrustResult = evaluate_and_dispatch_trust_score(&telemetry_data);
                println!("✅ Trust Result: {:?}", result);
                fanout_all(&result, &records, &telemetry_data);
            }
        }

        if let Ok(mut locked) = writer.clone().lock() { locked.append_batch(records); }
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

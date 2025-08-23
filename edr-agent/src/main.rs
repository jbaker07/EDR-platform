use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use chrono::Utc;

use forensic_hooks::elliptic_envelope::EllipticEnvelope;
use forensic_hooks::episode::Episode;
use forensic_hooks::event::{Event, EventType};
use forensic_hooks::graph_builder::{GraphEdge, GraphNode};
use forensic_hooks::mahalanobis::{Mahala, MahalaCfg};
use forensic_hooks::memory_priv::MemoryPrivEmitter;
use forensic_hooks::net_cadence::NetCadenceEmitter;
use forensic_hooks::pcb_emitter::PcbEmitter;
use forensic_hooks::baselines::BaselineStore;
use forensic_hooks::services::krim_lite::analyze_graph_and_score;
use forensic_hooks::services::trust_engine_final::{
    evaluate_and_dispatch_trust_score, TelemetryData, TrustResult,
};
use forensic_hooks::telemetry::{
    get_current_telemetry_snapshot, run_sideeffect_monitors_and_collect, start_realtime_monitors,
    TelemetryRecord,
};
use forensic_hooks::telemetry_types::TelemetryOutput;
use forensic_hooks::utils::time::now_ts;
use forensic_hooks::calibration::RollingCalibrator;

// —— global calibrator ——
static CALIBRATOR: OnceLock<std::sync::Mutex<RollingCalibrator>> = OnceLock::new();
fn calibrator() -> &'static std::sync::Mutex<RollingCalibrator> {
    CALIBRATOR.get_or_init(|| std::sync::Mutex::new(RollingCalibrator::default()))
}

// —— global detectors state ——
static MAHALA_ENGINE: OnceLock<std::sync::Mutex<Mahala>> = OnceLock::new();
static ENV_BASELINE: OnceLock<std::sync::Mutex<VecDeque<Vec<f64>>>> = OnceLock::new();
static ENV_MODEL: OnceLock<std::sync::Mutex<Option<EllipticEnvelope>>> = OnceLock::new();

fn mahala_engine() -> &'static std::sync::Mutex<Mahala> {
    MAHALA_ENGINE.get_or_init(|| std::sync::Mutex::new(Mahala::default()))
}
fn env_baseline() -> &'static std::sync::Mutex<VecDeque<Vec<f64>>> {
    ENV_BASELINE.get_or_init(|| std::sync::Mutex::new(VecDeque::with_capacity(1024)))
}
fn env_model() -> &'static std::sync::Mutex<Option<EllipticEnvelope>> {
    ENV_MODEL.get_or_init(|| std::sync::Mutex::new(None))
}

// —— local converter (avoid orphan rule) ——
fn try_convert_output(output: TelemetryOutput) -> Result<TelemetryRecord, String> {
    let mut rec = TelemetryRecord {
        timestamp: now_ts(),
        pid: 0,
        ppid: 0,
        uid: 0,
        binary_path: output.data.get("binary_path").cloned().unwrap_or_default(),
        command_line: output
            .data
            .get("command_line")
            .cloned()
            .unwrap_or_default(),
        cwd: output.data.get("cwd").cloned().unwrap_or_default(),
        ..Default::default()
    };
    rec.tags.push(output.signal);
    rec.risk_score = Some((output.confidence * 100.0) as u32);
    Ok(rec)
}

// —— hygiene: drop noisy/empty records & dedup spammy MFA bypasses ——
fn filter_records(records: &mut Vec<TelemetryRecord>) {
    // Drop empty process placeholders & obvious kernel-thread noise (ppid==2)
    records.retain(|r| {
        let has_proc = !r.binary_path.is_empty() || !r.command_line.is_empty();
        let not_kernel_ppid2 = r.ppid != 2;
        has_proc && not_kernel_ppid2
    });

    // Strip heartbeat/no-op tags
    let noisy = [
        "auth_monitor_active",
        "dll_injection_monitor_active",
        "encrypted_payload_monitor_active",
        "file_hash_monitor_active",
        "file_tamper_monitor_active",
        "no_cron_anomaly_detected",
        "geo_ip_no_anomaly",
    ];
    let noisy_set: HashSet<&'static str> = noisy.iter().copied().collect();
    for r in records.iter_mut() {
        r.tags.retain(|t| !noisy_set.contains(t.as_str()));
    }
}

fn dedup_records(records: &mut Vec<TelemetryRecord>) {
    // One-minute bucket dedup for MFA-bypass signals
    let mut seen: HashSet<(i64, String)> = HashSet::new();
    records.retain(|r| {
        // find any tag that contains "mfa_bypass"
        if let Some(tag) = r.tags.iter().find(|t| t.contains("mfa_bypass")) {
            let bucket = (r.timestamp as i64) / 60;
            let key = (bucket, tag.clone());
            if seen.contains(&key) {
                false
            } else {
                seen.insert(key);
                true
            }
        } else {
            true
        }
    });
}

// —— episode/graph builders ——
fn build_episode_from_records(records: &[TelemetryRecord]) -> Option<Episode> {
    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
    let mut ep = Episode::new(host);
    ep.context_id = Some("default".into());
    ep.role = Some("default".into());

    for r in records {
        if !r.binary_path.is_empty() || !r.command_line.is_empty() {
            ep.push(Event {
                ts: Utc::now(),
                event_type: EventType::Execve,
                ..Default::default()
            });
        }
        if !r.cwd.is_empty() {
            ep.push(Event {
                ts: Utc::now(),
                event_type: EventType::Other,
                ..Default::default()
            });
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

// —— anomaly taggers ——
fn tag_batch_outliers_with_mahala_and_envelope(records: &mut [TelemetryRecord], ep: &Episode) {
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
    let baseline_path = std::env::var("KRIM_BASELINE")
        .unwrap_or_else(|_| "state/krim_baseline.json".into());
    let events = analyze_graph_and_score(&nodes, &edges, 4, &baseline_path);

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
        "📣 [{}] score={:.2} escalated={} reasons={:?} batch_recs={}",
        res.endpoint_id, res.score, res.escalated, res.reasons, records.len()
    );

    // TrustResult doesn’t expose events; pass empty slice for sinks
    let events: &[forensic_hooks::trust_hook::TrustEvent] = &[];

    let _ = forensic_hooks::explain::explain(events, records, data);
    let _ = forensic_hooks::persist::persist(events, records, data);

    let _ = forensic_hooks::stdout_sink::emit(events, records, data);
    let _ = forensic_hooks::ecs_sink::emit(events, records, data);
    let _ = forensic_hooks::elastic_bulk::emit(events, records, data);
    let _ = forensic_hooks::splunk_hec::emit(events, records, data);
    let _ = forensic_hooks::slack_webhook::emit(events, records, data);
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // Use env_logger since logger::init_logger() doesn’t exist
    let _ = env_logger::try_init();

    let _policy = forensic_hooks::config::load_and_verify_policy("policy.json")
        .expect("❌ Failed to load or verify policy");

    let writer = Arc::new(Mutex::new(forensic_hooks::telemetry_writer::TelemetryWriter::new()));

    #[cfg(target_os = "linux")]
    {
        start_realtime_monitors(writer.clone());
    }

    if let Ok(mut cal) = calibrator().lock() {
        cal.flush();
    }

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
            match try_convert_output(output) {
                Ok(rec) => records.push(rec),
                Err(_)  => println!("⚠️ Failed to convert TelemetryOutput → TelemetryRecord"),
            }
        }

        // Hygiene
        filter_records(&mut records);
        dedup_records(&mut records);

        if records.is_empty() {
            println!("⚠️  No telemetry records collected this cycle.");
        } else {
            // —— PCB snapshots: structured + optional raw dump
            let pcb_snaps = forensic_hooks::pcb::collect_for_records(&records);
            let _ = forensic_hooks::pcb::append_snapshots_ndjson("state/pcb_snaps.ndjson", &pcb_snaps);

            let dump_raw = std::env::var("EDR_PCB_DUMP_RAW")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
            if dump_raw {
                let _ = forensic_hooks::pcb::dump_records_raw(&records, "state/pcb_raw", false);
            }

            if let Some(ep) = build_episode_from_records(&records) {
                // Feature emitters with in-memory baseline
                let mut store = BaselineStore::default();

                use forensic_hooks::traits::FeatureEmitter;
                let mut feats = Vec::new();
                feats.extend(MemoryPrivEmitter::new().emit(&ep, &mut store));
                feats.extend(NetCadenceEmitter::new().emit(&ep, &mut store));
                feats.extend(PcbEmitter::new().emit(&ep, &mut store));
                let _ = store.flush();

                for f in feats.iter().filter(|f| f.z.abs() > 2.0) {
                    println!("🔎 feature: {}={} z={:.2} fam={}", f.key, f.value, f.z, f.family);
                    // If/when type aligns: forensic_hooks::telemetry::push_feature_as_signal(f);
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

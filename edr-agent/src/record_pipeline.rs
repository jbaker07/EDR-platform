// src/record_pipeline.rs
// The record path: decoded records to the rules, the anomaly taggers, the live graph,
// the trust scores and kv_telemetry.jsonl.
//
// The decoders push records into a bounded queue (pb_adapter::queue::FactQueue, the same
// queue the playbook facts use). One OS thread takes them in chunks and runs every stage
// on the chunk. When the queue is full a decoder waits up to the push timeout, so a burst
// is held in the kernel ring buffer instead of being dropped here. Before this, records
// went through a 200,000 slot channel to a loop that woke every 5 s, and under a flood
// that loop fell behind and the channel dropped 200,466 records in one live run.
//
// Every stage's time is counted, so /pipeline/status shows where a chunk's time goes.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use parking_lot::RwLock;
use serde_json::{json, Value};
use tokio::sync::broadcast;

use forensic_hooks::baselines::BaselineStore;
use forensic_hooks::pb_adapter::queue::FactQueue;
use forensic_hooks::pipeline::Evaluator;
use forensic_hooks::services::trust_engine_final::{
    self, evaluate_and_dispatch_trust_score, TelemetryData, TrustResult,
};
use forensic_hooks::telemetry::Telemetry as NormTelemetry;
use forensic_hooks::telemetry::{run_sideeffect_monitors_and_collect, TelemetryRecord};
use forensic_hooks::telemetry_writer::TelemetryWriter;
use forensic_hooks::{ioc, pb_adapter, pb_introspect};

use crate::graph::GraphState;

/// Default record queue size. EDR_RECORD_QUEUE changes it.
pub const DEFAULT_CAPACITY: usize = 131_072;
/// Most records one pass takes.
pub const CHUNK: usize = 4096;
/// How long a pass waits to fill a chunk before it runs on what it has.
const GATHER: Duration = Duration::from_millis(200);
/// Side-effect monitors (auth log, sockets, scripts) run this often.
const SIDE_EFFECT_EVERY: Duration = Duration::from_secs(5);

// Stage names, in the order a chunk runs through them.
pub const STAGES: &[&str] = &[
    "filter",
    "pcb",
    "taggers",
    "ioc",
    "decider",
    "rules_and_graph",
    "trust",
    "persist",
    "side_effects",
];

#[derive(Default)]
pub struct RecordStats {
    pub records_total: AtomicU64,
    pub chunks_total: AtomicU64,
    pub rules_evaluated_total: AtomicU64,
    pub rule_hits_total: AtomicU64,
    pub largest_chunk: AtomicU64,
    stage_us: [AtomicU64; 9],
}

impl RecordStats {
    fn add(&self, stage: usize, since: Instant) {
        self.stage_us[stage].fetch_add(since.elapsed().as_micros() as u64, Ordering::Relaxed);
    }

    pub fn json(&self) -> Value {
        let stages: serde_json::Map<String, Value> = STAGES
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let ms = self.stage_us[i].load(Ordering::Relaxed) / 1000;
                (name.to_string(), json!(ms))
            })
            .collect();
        json!({
            "records_total": self.records_total.load(Ordering::Relaxed),
            "chunks_total": self.chunks_total.load(Ordering::Relaxed),
            "largest_chunk": self.largest_chunk.load(Ordering::Relaxed),
            "rules_evaluated_total": self.rules_evaluated_total.load(Ordering::Relaxed),
            "rule_hits_total": self.rule_hits_total.load(Ordering::Relaxed),
            "trust_output_writes_total": trust_engine_final::output_writes_total(),
            "stage_ms_total": stages,
        })
    }
}

pub fn stats() -> &'static RecordStats {
    static S: OnceLock<RecordStats> = OnceLock::new();
    S.get_or_init(RecordStats::default)
}

static QUEUE: OnceLock<Arc<FactQueue<TelemetryRecord>>> = OnceLock::new();

/// Create the record queue. EDR_RECORD_QUEUE sets the size and
/// EDR_RECORD_PUSH_TIMEOUT_MS how long a full queue holds a decoder.
pub fn init_queue() -> Arc<FactQueue<TelemetryRecord>> {
    QUEUE
        .get_or_init(|| {
            let cap = std::env::var("EDR_RECORD_QUEUE")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .filter(|n| *n > 0)
                .unwrap_or(DEFAULT_CAPACITY);
            let wait = std::env::var("EDR_RECORD_PUSH_TIMEOUT_MS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .map(Duration::from_millis)
                .unwrap_or(pb_adapter::queue::DEFAULT_PUSH_TIMEOUT);
            Arc::new(FactQueue::new(cap, wait))
        })
        .clone()
}

/// Queue one record. False, counted as a drop, when the queue stayed full for the
/// whole push timeout.
pub fn push(rec: TelemetryRecord) -> bool {
    match QUEUE.get() {
        Some(q) => q.push(rec),
        None => false,
    }
}

pub fn queue_stats() -> Option<pb_adapter::queue::QueueStats> {
    QUEUE.get().map(|q| q.stats())
}

/// Everything a chunk needs. Owned by the record thread.
pub struct RecordPipeline {
    pub evaluator: Evaluator,
    pub graph: Arc<RwLock<GraphState>>,
    pub alerts: broadcast::Sender<String>,
    pub incidents: broadcast::Sender<String>,
    pub writer: Arc<Mutex<TelemetryWriter>>,
    /// Set false in tests to skip the side-effect monitors, which read the host.
    pub side_effects: bool,
}

impl RecordPipeline {
    /// Run every stage on one chunk.
    pub fn process(&mut self, mut records: Vec<TelemetryRecord>) {
        let st = stats();
        let t = Instant::now();
        crate::filter_records(&mut records);
        st.add(0, t);
        if records.is_empty() {
            return;
        }
        st.records_total.fetch_add(records.len() as u64, Ordering::Relaxed);
        st.chunks_total.fetch_add(1, Ordering::Relaxed);
        st.largest_chunk.fetch_max(records.len() as u64, Ordering::Relaxed);

        // PCB snapshots
        let t = Instant::now();
        let pcb_snaps = forensic_hooks::pcb::collect_for_records(&records);
        crate::remember_pcb(&pcb_snaps);
        let _ = forensic_hooks::pcb::append_snapshots_ndjson("state/pcb_snaps.ndjson", &pcb_snaps);
        if pcb_dump_raw() {
            let _ = forensic_hooks::pcb::dump_records_raw(&records, "state/pcb_raw", false);
        }
        st.add(1, t);

        // Feature and anomaly taggers. The rules read their tags, so they run first.
        let t = Instant::now();
        if let Some(ep) = crate::build_episode_from_records(&records) {
            let mut store = BaselineStore::default();
            use forensic_hooks::traits::FeatureEmitter;
            let mut feats = Vec::new();
            feats.extend(forensic_hooks::memory_priv::MemoryPrivEmitter::new().emit(&ep, &mut store));
            feats.extend(forensic_hooks::net_cadence::NetCadenceEmitter::new().emit(&ep, &mut store));
            feats.extend(forensic_hooks::pcb_emitter::PcbEmitter::new().emit(&ep, &mut store));
            let _ = store.flush();
            if crate::verbose() {
                for f in feats.iter().filter(|f| f.z.abs() > 2.0) {
                    println!("feature: {}={} z={:.2} fam={}", f.key, f.value, f.z, f.family);
                }
            }
            let _ = crate::tag_batch_outliers_with_mahala_and_envelope(&mut records, &ep);
            crate::tag_records_from_krim(&mut records);
        }
        st.add(2, t);

        // IOC matches become pending playbook evidence.
        let t = Instant::now();
        match ioc::tag_records(&mut records) {
            Ok(hits) => {
                for h in hits {
                    pb_introspect::add_or_update_pending(pb_introspect::PendingEvidence {
                        id: h.id,
                        playbook_id: "ioc_layer".to_string(),
                        ts: h.ts,
                        slot_id: h.slot,
                        fact: h.fact,
                        summary: h.summary,
                        tags: h.tags,
                        ttl_sec: 900,
                    });
                }
            }
            Err(e) => eprintln!("ioc::tag_records error: {e}"),
        }
        st.add(3, t);

        // DecisionEngine, from the tags.
        let t = Instant::now();
        crate::feed_decider_from_record_tags(&records, &self.alerts, &self.incidents);
        st.add(4, t);

        // Per-record rules and the live graph.
        let t = Instant::now();
        let host = crate::agent_host();
        for rec in &records {
            let mut tel = NormTelemetry::from(rec.clone());
            let (cat, evt) = crate::categorize_record(rec);
            tel.category = cat;
            tel.event = evt;
            self.graph_ingest(rec, &tel);

            st.rules_evaluated_total.fetch_add(1, Ordering::Relaxed);
            match self.evaluator.evaluate(&tel) {
                Ok(Some(alert)) => {
                    st.rule_hits_total.fetch_add(1, Ordering::Relaxed);
                    crate::ALERTS_OUT.fetch_add(1, Ordering::Relaxed);
                    if let Ok(v) = serde_json::to_value(&alert) {
                        crate::handle_rule_alert(&v, &host, &self.alerts, &self.incidents);
                    }
                }
                Ok(None) => {}
                Err(e) => eprintln!("evaluator error: {e}"),
            }
        }
        st.add(5, t);

        // Trust scores.
        let t = Instant::now();
        for rec in &records {
            if crate::verbose() {
                println!("Raw Telemetry: {:?}", rec);
            }
            let data = TelemetryData::from_record(rec);
            let result: TrustResult = evaluate_and_dispatch_trust_score(&data);
            if crate::verbose() {
                println!("Trust Result: {:?}", result);
            }
            crate::fanout_all(&result, &records, &data);
        }
        // The trust files and GNN lines, newest value per endpoint, once a second.
        trust_engine_final::flush_outputs(false);
        st.add(6, t);

        // kv_telemetry and the other record files.
        let t = Instant::now();
        if let Ok(w) = self.writer.lock() {
            w.append_batch(records);
        }
        st.add(7, t);
    }

    fn graph_ingest(&self, rec: &TelemetryRecord, tel: &NormTelemetry) {
        let mut g = self.graph.write();
        let pid = rec.pid.max(0) as u32;
        let ppid = rec.ppid.max(0) as u32;
        let comm = if !rec.binary_path.is_empty() {
            rec.binary_path.clone()
        } else {
            rec.command_line
                .split_whitespace()
                .next()
                .unwrap_or("-")
                .to_string()
        };
        let score = (rec.risk_score.unwrap_or(0) as f64) / 100.0;
        let conf = (0.1 + score).min(0.95);
        match (tel.category.as_str(), tel.event.as_str()) {
            ("process", "exec") => g.ingest_exec(ppid, "-", pid, &comm, score, conf),
            ("network", "connect") => {
                let ip_port = rec
                    .command_line
                    .split_whitespace()
                    .find(|s| s.contains(':') && s.chars().any(|c| c.is_ascii_digit()))
                    .unwrap_or("-");
                g.ingest_connect(pid, &comm, ip_port, score, conf);
            }
            ("file", "write") | ("file", "open") => {
                let path = if !rec.cwd.is_empty() { rec.cwd.clone() } else { "-".into() };
                g.ingest_file_access(pid, &comm, &path, tel.event == "write", score, conf);
            }
            _ => {}
        }
    }

}

/// Records from the side-effect monitors (auth log, sockets, scripts), which read the
/// host instead of a ring buffer. They run on a thread of their own every 5 s and queue
/// their records like the ring readers do: a pass takes about 200 ms, and on the record
/// thread that pause was long enough to fill the queue in a burst.
fn spawn_side_effects(queue: Arc<FactQueue<TelemetryRecord>>, rt: tokio::runtime::Handle) -> std::io::Result<()> {
    std::thread::Builder::new()
        .name("side-effects".into())
        .spawn(move || {
            let _rt = rt.enter();
            loop {
                let t = Instant::now();
                for output in run_sideeffect_monitors_and_collect() {
                    match crate::try_convert_output(output) {
                        Ok(rec) => {
                            if queue.push(rec) {
                                forensic_hooks::metrics::events_accepted().fetch_add(1, Ordering::Relaxed);
                            }
                        }
                        Err(_) => eprintln!("could not convert a side-effect output to a record"),
                    }
                }
                stats().add(8, t);
                std::thread::sleep(SIDE_EFFECT_EVERY.saturating_sub(t.elapsed()));
            }
        })
        .map(|_| ())
}

fn pcb_dump_raw() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var("EDR_PCB_DUMP_RAW")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    })
}

/// Start the record thread. It enters the tokio runtime so the stages that spawn
/// tasks (the trust engine's bus publish) work from it.
pub fn spawn(
    queue: Arc<FactQueue<TelemetryRecord>>,
    mut pipeline: RecordPipeline,
    rt: tokio::runtime::Handle,
) -> std::io::Result<()> {
    if pipeline.side_effects {
        spawn_side_effects(queue.clone(), rt.clone())?;
    }
    std::thread::Builder::new()
        .name("record-pipeline".into())
        .spawn(move || {
            let _rt = rt.enter();
            queue.set_consumer_alive(true);
            loop {
                let mut chunk = queue.pop_batch(CHUNK, Duration::from_secs(1));
                // A small chunk waits a little for more, so the per-chunk stages
                // (PCB, taggers, the file writes) do not run for every few records.
                if !chunk.is_empty() && chunk.len() < CHUNK {
                    let until = Instant::now() + GATHER;
                    while chunk.len() < CHUNK {
                        let left = until.saturating_duration_since(Instant::now());
                        if left.is_zero() {
                            break;
                        }
                        let more = queue.pop_batch(CHUNK - chunk.len(), left);
                        if more.is_empty() {
                            break;
                        }
                        chunk.extend(more);
                    }
                }
                if chunk.is_empty() {
                    trust_engine_final::flush_outputs(false);
                } else {
                    pipeline.process(chunk);
                }
            }
        })
        .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use forensic_hooks::pipeline::Thresholds;
    use std::collections::HashMap;
    use std::io::BufRead;

    /// One kv_telemetry.jsonl row as a decoder's record: ids, path, command and the
    /// decoder tags.
    fn record_from_kv(row: &HashMap<String, String>) -> TelemetryRecord {
        let num = |k: &str| row.get(k).and_then(|v| v.parse::<i64>().ok());
        let text = |ks: &[&str]| ks.iter().find_map(|k| row.get(*k)).cloned().unwrap_or_default();
        let mut tags: Vec<String> = Vec::new();
        for k in ["tag0", "tag1", "tag2", "signal"] {
            if let Some(v) = row.get(k) {
                tags.push(v.clone());
            }
        }
        if let Some(c) = row.get("comm") {
            tags.push(format!("comm:{c}"));
        }
        TelemetryRecord {
            timestamp: num("timestamp").unwrap_or(1_800_000_000) as u64,
            pid: num("pid").or_else(|| num("tgid")).unwrap_or(0) as i32,
            ppid: num("ppid").unwrap_or(0) as i32,
            uid: num("uid").unwrap_or(0) as u32,
            binary_path: text(&["binary_path", "exe", "path"]),
            command_line: text(&["command_line", "cmdline", "comm"]),
            cwd: text(&["cwd"]),
            env_vars: None,
            tags,
            risk_score: row
                .get("confidence")
                .and_then(|c| c.parse::<f64>().ok())
                .map(|c| (c * 100.0) as u32),
        }
    }

    /// Replays a kv_telemetry.jsonl file through every stage and prints where the time
    /// went. Run from a scratch directory, since the stages write state/, logs/ and
    /// json_files/ under the working directory:
    ///   EDR_REPLAY_KV=/path/kv.jsonl cargo test --release --bin forensic_hooks replay -- --ignored --nocapture
    #[test]
    #[ignore]
    fn replay() {
        let Ok(path) = std::env::var("EDR_REPLAY_KV") else {
            eprintln!("set EDR_REPLAY_KV");
            return;
        };
        let limit: usize = std::env::var("EDR_REPLAY_N")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(usize::MAX);
        let mut evaluator = Evaluator::new(Thresholds { low: 0.30, medium: 0.55, high: 0.80 });
        let rules = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/rules.json");
        evaluator.registry.add_rules_from(&rules).unwrap();
        evaluator.registry.add(Box::new(forensic_hooks::detectors::tag_flags::TagFlagsDetector));
        let (alerts, _a) = broadcast::channel(1024);
        let (incidents, _i) = broadcast::channel(256);
        let mut p = RecordPipeline {
            evaluator,
            graph: crate::graph_state(),
            alerts,
            incidents,
            writer: Arc::new(Mutex::new(TelemetryWriter::new())),
            side_effects: false,
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _g = rt.enter();
        // Read and run one chunk at a time, so the replay holds no more than a chunk.
        let file = std::fs::File::open(&path).unwrap();
        let mut n = 0usize;
        let mut chunk = Vec::with_capacity(CHUNK);
        let t = Instant::now();
        for line in std::io::BufReader::new(file).lines().take(limit) {
            let Ok(line) = line else { continue };
            if let Ok(row) = serde_json::from_str::<HashMap<String, String>>(&line) {
                chunk.push(record_from_kv(&row));
                n += 1;
            }
            if chunk.len() == CHUNK {
                p.process(std::mem::replace(&mut chunk, Vec::with_capacity(CHUNK)));
            }
        }
        if !chunk.is_empty() {
            p.process(chunk);
        }
        trust_engine_final::flush_outputs(true);
        let secs = t.elapsed().as_secs_f64();
        let hwm = std::fs::read_to_string("/proc/self/status")
            .ok()
            .and_then(|s| s.lines().find(|l| l.starts_with("VmHWM")).map(str::to_string))
            .unwrap_or_default();
        eprintln!(
            "replayed {n} records in {secs:.2} s ({:.0} records/s, reading and parsing the file included), {hwm}: {}",
            n as f64 / secs,
            stats().json()
        );
    }
}

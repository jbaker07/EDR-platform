mod ebpf;
// main.rs (Phase 2 + Phase 3 Decision Engine integration)
// - Safer TS parsing (secs/ms)
// - Two-pass graph builder (no missing PPID edges)
// - Periodic flush actually hits disk
// - DEMO mode unchanged (now increments alerts_out)
// - Rules path fallback
// - /healthz + nodes_count/edges_count in graph response for easy jq
// - Metrics: events/events_in (received), events_accepted (post-filter), alerts_out
// - Relaxed acceptance filter: tag-implied events count as accepted
// - Compat aliases in /metrics: events_accepted_total, alerts_out_total
// - /graph/live + /graph/stream (SSE) + metrics/healthz include graph counts
// - Incidents manager + SSE + REST, action endpoints, BPF drops metric, EDR_PORT, Explain++
// - NEW (Phase 3): DecisionEngine (consensus + correlation + cooldown + allow/suppress) wired in
// - NEW (Phase PB): Playbook Engine + IOC/YARA wired (pending/hits, metrics, endpoints)

use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::Infallible;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use std::env;
use axum::{
    extract::{Path as AxPath, Query, State},
    http::{StatusCode, header::CACHE_CONTROL, HeaderValue},
    response::{
        sse::{Event, Sse},
        IntoResponse,
    },
    routing::{delete, get, post, get_service},
    Json, Router,
};
use chrono::{TimeZone, Utc};
use futures_util::stream::{unfold, Stream};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha1::{Digest, Sha1};
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};
use forensic_hooks::{ioc, pb_api, pb_introspect, pb_wiring, pending, tamper, yara_sidecar};
use forensic_hooks::pb_engine::PlaybookEngine;


// ---------- metrics counters ----------
static EVENTS_IN: AtomicU64 = AtomicU64::new(0);          // received (pre-filter)
static EVENTS_ACCEPTED: AtomicU64 = AtomicU64::new(0);     // accepted (post-filter, after persist)
static ALERTS_OUT: AtomicU64 = AtomicU64::new(0);
// NEW: BPF ringbuffer/loss metric hook
pub static BPF_DROPS: AtomicU64 = AtomicU64::new(0);
pub fn bpf_drops_add(n: u64) { BPF_DROPS.fetch_add(n, Ordering::Relaxed); }

// ---------- your pipeline / telemetry ----------
use forensic_hooks::baselines::BaselineStore;
use forensic_hooks::calibration::RollingCalibrator;
use forensic_hooks::elliptic_envelope::EllipticEnvelope;
use forensic_hooks::episode::Episode;
use forensic_hooks::event::{Event as FHEvent, EventType};
use forensic_hooks::graph_builder::{GraphEdge, GraphNode};
use forensic_hooks::mahalanobis::{Mahala, MahalaCfg};
use forensic_hooks::memory_priv::MemoryPrivEmitter;
use forensic_hooks::net_cadence::NetCadenceEmitter;
use forensic_hooks::pcb_emitter::PcbEmitter;
use forensic_hooks::pipeline::{Evaluator, Thresholds};
use forensic_hooks::services::krim_lite::analyze_graph_and_score;
use forensic_hooks::services::trust_engine_final::{
    evaluate_and_dispatch_trust_score, TelemetryData, TrustResult,
};
use forensic_hooks::telemetry::{
    get_current_telemetry_snapshot, run_sideeffect_monitors_and_collect, start_realtime_monitors,
    TelemetryRecord,
};
use forensic_hooks::telemetry::Telemetry as NormTelemetry;
use forensic_hooks::telemetry_types::TelemetryOutput;
use forensic_hooks::utils::time::now_ts;


// === expose your local detectors (src/detectors/...) ===
mod detectors {
    pub mod rules_hot;
    pub mod tag_flags;
}
use crate::detectors::rules_hot::HotRulesDetector;
use crate::detectors::tag_flags::TagFlagsDetector;

// === add the reader modules directly ===
#[path = "ebpf/events_reader.rs"]
mod events_reader; // kept for later; we don't call a missing start()
#[path = "ebpf/file_access_reader.rs"]
mod file_access_reader;
#[path = "ebpf/net_flow_reader.rs"]
mod net_flow_reader;

// === use your existing graph module at src/graph.rs ===
mod graph;
use crate::graph::{spawn_writer, GraphState};
use parking_lot::RwLock;

// === Decision Engine (Phase 3) ===
mod decision;
use crate::decision::{
    AllowSuppressList, DecisionEngine, DecisionPolicy, DetectorKind, FinalAlert, Signal, Severity,
};

mod rgcn_bridge;
use crate::rgcn_bridge::{start_rgcn_bridge, annotate_graph, rgcn_metrics};


// A globally shared live-graph state (optional, does not block UI graph endpoint).
static GRAPH_STATE: OnceLock<Arc<RwLock<GraphState>>> = OnceLock::new();
fn graph_state() -> Arc<RwLock<GraphState>> {
    GRAPH_STATE
        .get_or_init(|| Arc::new(RwLock::new(GraphState::new(300)))) // keep last 5 minutes
        .clone()
}

// —— global calibrator ——
static CALIBRATOR: OnceLock<Mutex<RollingCalibrator>> = OnceLock::new();
fn calibrator() -> &'static Mutex<RollingCalibrator> {
    CALIBRATOR.get_or_init(|| Mutex::new(RollingCalibrator::default()))
}

// —— global detectors state ——
static MAHALA_ENGINE: OnceLock<Mutex<Mahala>> = OnceLock::new();
static ENV_BASELINE: OnceLock<Mutex<VecDeque<Vec<f64>>>> = OnceLock::new();
static ENV_MODEL: OnceLock<Mutex<Option<EllipticEnvelope>>> = OnceLock::new();

fn mahala_engine() -> &'static Mutex<Mahala> {
    MAHALA_ENGINE.get_or_init(|| Mutex::new(Mahala::default()))
}
fn env_baseline() -> &'static Mutex<VecDeque<Vec<f64>>> {
    ENV_BASELINE.get_or_init(|| Mutex::new(VecDeque::with_capacity(1024)))
}
fn env_model() -> &'static Mutex<Option<EllipticEnvelope>> {
    ENV_MODEL.get_or_init(|| Mutex::new(None))
}

// —— Playbook Engine singleton ——
// —— Playbook Engine singleton ——
// —— Playbook Engine singleton ——
static PB_ENGINE: OnceLock<Mutex<PlaybookEngine>> = OnceLock::new();
fn pb_engine() -> &'static Mutex<PlaybookEngine> {
    // Use the constructor that actually exists
    PB_ENGINE.get_or_init(|| Mutex::new(PlaybookEngine::empty()))
}

// ======================= Ontology: loader + enrichment =======================
#[derive(Clone, Default)]
struct OntologySet {
    core_ontology: serde_json::Value,                        // core_ontology.json
    inverse_edges: std::collections::HashMap<String, String>,// inverse_edges.json
    semantic_tags: serde_json::Value,                        // semantic_tags.json (optional)
    unknown_risk_profile: serde_json::Value,                 // unknown_risk_profile.json (optional)
    anchor_definitions: std::collections::HashMap<String, serde_json::Value>, // anchor_definitions.json (optional)
    dir: String,
}

#[derive(Serialize)]
struct HostHealth {
    cpu_pct: Option<f32>,
    mem_pct: Option<f32>,
    agent_lag_ms: Option<u64>,
    event_rate_per_s: Option<f32>,
    events_in: Option<u64>,
    alerts_out: Option<u64>,
    bpf_drops: Option<u64>,
    nodes_count: Option<u64>,
    edges_count: Option<u64>,
    rgcn_p95: Option<f32>,
}

async fn host_self_health() -> impl IntoResponse {
    let (nodes_count, edges_count) = read_graph_counts_from_file();
    Json(HostHealth {
        cpu_pct: None,
        mem_pct: None,
        agent_lag_ms: None,
        event_rate_per_s: None,
        events_in: Some(EVENTS_IN.load(Ordering::Relaxed)),
        alerts_out: Some(ALERTS_OUT.load(Ordering::Relaxed)),
        bpf_drops: Some(BPF_DROPS.load(Ordering::Relaxed)),
        nodes_count: Some(nodes_count),
        edges_count: Some(edges_count),
        rgcn_p95: None,
    })
}


static ONTOLOGY: OnceLock<Arc<Mutex<OntologySet>>> = OnceLock::new();
fn ontology() -> Arc<Mutex<OntologySet>> {
    ONTOLOGY
        .get_or_init(|| Arc::new(Mutex::new(load_ontology_default())))
        .clone()
}

fn normalize_and_annotate_graph(mut v: serde_json::Value) -> serde_json::Value {
    // Ensure "edges" exists (prefer edges; fall back to links)
    let has_edges = v.get("edges").map(|e| !e.is_null()).unwrap_or(false);
    if !has_edges {
        if let Some(links) = v.get("links").cloned() {
            if let Some(obj) = v.as_object_mut() {
                obj.insert("edges".to_string(), links);
            }
        } else if let Some(obj) = v.as_object_mut() {
            obj.insert("edges".to_string(), serde_json::Value::Array(vec![]));
        }
    }

    // Fill rel from kind when missing, so the UI can label edges nicely
    if let Some(edges) = v.get_mut("edges").and_then(|x| x.as_array_mut()) {
        for e in edges.iter_mut() {
            let has_rel = e.get("rel").is_some();
            if !has_rel {
                if let Some(kind) = e.get("kind").and_then(|x| x.as_str()) {
                    let rel = match kind {
                        "EXEC" => "spawned",
                        "CONNECT" => "connected-to",
                        "READ" | "WRITE" => "accessed",
                        _ => "linked-to",
                    };
                    if let Some(obj) = e.as_object_mut() {
                        obj.insert("rel".into(), serde_json::Value::String(rel.to_string()));
                    }
                }
            }
        }
    }

    // Add node kind if absent (helps the UI shapes/colors + R-GCN features)
    if let Some(nodes) = v.get_mut("nodes").and_then(|x| x.as_array_mut()) {
        for n in nodes.iter_mut() {
            let id = n.get("id").and_then(|x| x.as_str()).unwrap_or("");
            let has_kind = n.get("kind").is_some();
            if !has_kind {
                let inferred = if id.starts_with("proc:") {
                    "Process"
                } else if id.starts_with("file:") {
                    "File"
                } else if id.starts_with("ip:") {
                    "IP"
                } else if id.starts_with("user:") {
                    "User"
                } else if id.starts_with("domain:") {
                    "Domain"
                } else {
                    "Other"
                };
                if let Some(obj) = n.as_object_mut() {
                    obj.insert("kind".into(), serde_json::Value::String(inferred.to_string()));
                }
            }
        }
    }

    // Overlay R-GCN scores (no-op if bridge hasn’t scored yet)
    rgcn_bridge::annotate_graph(&mut v);

    v
}

fn load_json_if(path: &std::path::Path) -> Option<serde_json::Value> {
    std::fs::read_to_string(path).ok().and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
}

fn map_from_obj(v: &serde_json::Value) -> std::collections::HashMap<String, String> {
    let mut out = std::collections::HashMap::new();
    if let Some(obj) = v.as_object() {
        for (k, vv) in obj {
            if let Some(s) = vv.as_str() {
                out.insert(k.clone(), s.to_string());
            }
        }
    }
    out
}

fn load_ontology_from_dir(dir: &str) -> OntologySet {
    let p = |name: &str| std::path::Path::new(dir).join(name);
    let core = load_json_if(&p("core_ontology.json")).unwrap_or_else(|| json!({}));
    let inv  = load_json_if(&p("inverse_edges.json")).unwrap_or_else(|| json!({}));
    let sem  = load_json_if(&p("semantic_tags.json")).unwrap_or_else(|| json!({}));
    let unk  = load_json_if(&p("unknown_risk_profile.json")).unwrap_or_else(|| json!({}));
    let anchors_v = load_json_if(&p("anchor_definitions.json")).unwrap_or_else(|| json!([]));
    let mut anchors = std::collections::HashMap::new();
    if let Some(arr) = anchors_v.as_array() {
        for it in arr {
            if let Some(id) = it.get("id").and_then(|x| x.as_str()) {
                anchors.insert(id.to_string(), it.clone());
            }
        }
    }
    OntologySet {
        core_ontology: core,
        inverse_edges: map_from_obj(&inv),
        semantic_tags: sem,
        unknown_risk_profile: unk,
        anchor_definitions: anchors,
        dir: dir.to_string(),
    }
}

fn candidates_ontology_dirs() -> Vec<String> {
    // Priority: explicit env var, then common workspace layouts
    let mut c = Vec::new();
    if let Ok(s) = std::env::var("ONTOLOGY_DIR") {
        if !s.trim().is_empty() { c.push(s); }
    }
    c.push("edr-platform/ontology".to_string());
    c.push("../edr-platform/ontology".to_string());
    c.push("../../edr-platform/ontology".to_string());
    c.push("ontology".to_string());
    c.push("../ontology".to_string());
    c.push("../../ontology".to_string());
    c.dedup();
    c
}

fn dir_has_core_files(dir: &str) -> bool {
    let p = |name: &str| std::path::Path::new(dir).join(name);
    p("core_ontology.json").exists() && p("inverse_edges.json").exists()
}

fn resolve_ontology_dir_auto() -> String {
    for d in candidates_ontology_dirs() {
        if dir_has_core_files(&d) {
            return d;
        }
    }
    // Fallback: env var if set, else "ontology" (even if missing, so API still answers)
    std::env::var("ONTOLOGY_DIR").unwrap_or_else(|_| "ontology".to_string())
}

fn load_ontology_default() -> OntologySet {
    load_ontology_from_dir(&resolve_ontology_dir_auto())
}


// --- minimal helpers (no extra crates) ---
fn looks_like_ipv4(s: &str) -> bool {
    // "1.2.3.4" or "1.2.3.4:443"
    let ip = s.split_once(':').map(|(a, _)| a).unwrap_or(s);
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 { return false; }
    parts.iter().all(|p| p.parse::<u8>().is_ok())
}

fn infer_otype_from_node(n: &serde_json::Value) -> String {
    // Prefer explicit info in node fields
    if n.get("binary_path").and_then(|v| v.as_str()).is_some() { return "Process".into(); }
    if n.get("pid").and_then(|v| v.as_u64()).is_some()       { return "Process".into(); }
    if n.get("command_line").and_then(|v| v.as_str()).is_some(){ return "Process".into(); }
    if let Some(id) = n.get("id").and_then(|v| v.as_str()) {
        if id.starts_with("proc:") { return "Process".into(); }
        if id.starts_with("file:") { return "File".into(); }
        if id.starts_with("user:") { return "User".into(); }
        if looks_like_ipv4(id)     { return "IP".into(); }
        if id.contains('/') || id.contains('\\') { return "File".into(); }
    }
    if let Some(k) = n.get("kind").and_then(|v| v.as_str()) {
        let k = k.to_ascii_lowercase();
        if k.contains("proc")  { return "Process".into(); }
        if k.contains("file")  { return "File".into(); }
        if k == "ip" || k == "net" { return "IP".into(); }
        if k.contains("dns")   { return "Domain".into(); }
        if k.contains("user")  { return "User".into(); }
    }
    "Unknown".into()
}

fn default_rel_for(st: &str, tt: &str) -> String {
    match (st, tt) {
        ("Process", "Process") => "spawned",
        ("Process", "IP")      => "connected-to",
        ("Process", "File")    => "accessed",
        ("User",    "Process") => "launched",
        ("File",    "Process") => "executed-by",
        _ => "linked-to",
    }.to_string()
}

fn enrich_graph_json(raw: &serde_json::Value) -> serde_json::Value {
    let ont = ontology();
    let ont = ont.lock().unwrap().clone();

    let mut out = raw.clone();

    // Get mutable node/edge arrays (copy-on-write from raw)
    let nodes_in = raw.get("nodes").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let edges_in = raw.get("edges")
        .or_else(|| raw.get("links"))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    // 1) Enrich nodes: add otype and details
    let mut node_type_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut nodes_out: Vec<serde_json::Value> = Vec::with_capacity(nodes_in.len());

    for n in nodes_in {
        let mut nn = n.clone();
        let otype = nn.get("otype")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| infer_otype_from_node(&nn));
        if let Some(id) = nn.get("id").and_then(|v| v.as_str()) {
            node_type_map.insert(id.to_string(), otype.clone());
        }
        // attach otype
        nn.as_object_mut().unwrap().insert("otype".into(), json!(otype));

        // attach details (best-effort)
        let mut det = serde_json::Map::new();
        if let Some(v) = nn.get("pid").and_then(|x| x.as_i64())      { det.insert("pid".into(), json!(v)); }
        if let Some(v) = nn.get("pid").and_then(|x| x.as_u64()).map(|x| x as u32)    { det.insert("ppid".into(), json!(v)); }
        if let Some(v) = nn.get("binary_path").and_then(|x| x.as_str()){ det.insert("exe".into(), json!(v)); }
        if let Some(v) = nn.get("command_line").and_then(|x| x.as_str()){ det.insert("cmdline".into(), json!(v)); }
        if let Some(v) = nn.get("uid").and_then(|x| x.as_i64())      { det.insert("uid".into(), json!(v)); }
        if !det.is_empty() {
            nn.as_object_mut().unwrap().insert("details".into(), serde_json::Value::Object(det));
        }

        nodes_out.push(nn);
    }

    // 2) Enrich edges: normalize, infer rel, attach inverse + validity
    let mut edges_out: Vec<serde_json::Value> = Vec::with_capacity(edges_in.len());
    for e in edges_in {
        let mut ee = e.clone();
        if ee.get("source").is_none() {
            if let Some(v) = ee.get("from").cloned() {
                ee.as_object_mut().unwrap().insert("source".into(), v);
            }
        }
        if ee.get("target").is_none() {
            if let Some(v) = ee.get("to").cloned() {
                ee.as_object_mut().unwrap().insert("target".into(), v);
            }
        }
        let sid = ee.get("source").and_then(|v| v.as_str()).unwrap_or("");
        let tid = ee.get("target").and_then(|v| v.as_str()).unwrap_or("");
        let st  = node_type_map.get(sid).cloned().unwrap_or_else(|| "Unknown".into());
        let tt  = node_type_map.get(tid).cloned().unwrap_or_else(|| "Unknown".into());

        // relation (keep provided, else infer)
        let rel = ee.get("rel")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| default_rel_for(&st, &tt));
        ee.as_object_mut().unwrap().insert("rel".into(), json!(rel.clone()));

        // inverse (if known)
        if let Some(inv) = ont.inverse_edges.get(&rel) {
            ee.as_object_mut().unwrap().insert("rel_inverse".into(), json!(inv));
        }

        // ontology validation (if relation exists under source type)
        if let Some(allowed) = ont.core_ontology
            .get(&st).and_then(|t| t.get("edges"))
            .and_then(|m| m.get(&rel)).and_then(|arr| arr.as_array())
        {
            let allowed: Vec<String> = allowed.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect();
            let ok = allowed.iter().any(|t| t == &tt);
            ee.as_object_mut().unwrap().insert("rel_valid".into(), json!(ok));
            if !ok {
                ee.as_object_mut().unwrap().insert("expected_targets".into(), json!(allowed));
                ee.as_object_mut().unwrap().insert("violation".into(),
                    json!(format!("edge '{}' not allowed: {} -> {}", rel, st, tt)));
            }
        }

        edges_out.push(ee);
    }

    // 3) Recompose; also mirror edges to links for UI compatibility if needed
    out.as_object_mut().unwrap().insert("nodes".into(), json!(nodes_out));
    out.as_object_mut().unwrap().insert("edges".into(), json!(edges_out.clone()));
    if out.get("links").is_some() {
        out.as_object_mut().unwrap().insert("links".into(), json!(edges_out));
    }

    // lightweight counts for jq/health
    let n_cnt = out.get("nodes").and_then(|x| x.as_array()).map(|a| a.len()).unwrap_or(0);
    let e_cnt = out.get("edges").and_then(|x| x.as_array()).map(|a| a.len()).unwrap_or(0);
    out.as_object_mut().unwrap().insert("nodes_count".into(), json!(n_cnt));
    out.as_object_mut().unwrap().insert("edges_count".into(), json!(e_cnt));

    out
}

// ======================= Ontology endpoints =======================
async fn ontology_status() -> impl IntoResponse {
    let ont_arc = ontology();
    let guard = ont_arc.lock().unwrap();
    let dir = guard.dir.clone();
    let candidates = candidates_ontology_dirs();

    let exists = |base: &str, name: &str| std::path::Path::new(base).join(name).exists();

    Json(json!({
        "dir": dir,
        "candidates_tried": candidates,
        "present": {
            "core_ontology.json": exists(&guard.dir, "core_ontology.json"),
            "inverse_edges.json": exists(&guard.dir, "inverse_edges.json"),
            "semantic_tags.json": exists(&guard.dir, "semantic_tags.json"),
            "unknown_risk_profile.json": exists(&guard.dir, "unknown_risk_profile.json"),
            "anchor_definitions.json": exists(&guard.dir, "anchor_definitions.json")
        }
    }))
}


async fn ontology_reload(
    Query(q): Query<std::collections::HashMap<String, String>>
) -> impl IntoResponse {
    // Decide target dir without holding a lock
    let target_dir = match q.get("dir").map(|s| s.as_str()) {
        Some("auto") | None => resolve_ontology_dir_auto(),
        Some(other) => other.to_string(),
    };

    // Reload
    let ont_arc = ontology();
    {
        let mut guard = ont_arc.lock().unwrap();
        *guard = load_ontology_from_dir(&target_dir);
    }

    Json(json!({"ok": true, "dir": target_dir}))
}



async fn ontology_validate() -> impl IntoResponse {
    let raw = if let Ok(s) = fs::read_to_string("json_files/graph_live.json") {
        serde_json::from_str::<serde_json::Value>(&s).unwrap_or_else(|_| json!({"nodes":[],"edges":[]}))
    } else { json!({"nodes":[],"edges":[]}) };
    let enriched = enrich_graph_json(&raw);

    let nodes = enriched.get("nodes").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let edges = enriched.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();

    let mut nodes_by_type: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for n in &nodes {
        let t = n.get("otype").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
        *nodes_by_type.entry(t).or_insert(0) += 1;
    }

    let mut edges_with_rel = 0usize;
    let mut edges_valid = 0usize;
    let mut edges_invalid = 0usize;
    let mut samples_invalid: Vec<serde_json::Value> = vec![];

    for e in &edges {
        if e.get("rel").is_some() { edges_with_rel += 1; }
        if let Some(v) = e.get("rel_valid").and_then(|v| v.as_bool()) {
            if v { edges_valid += 1; } else {
                edges_invalid += 1;
                if samples_invalid.len() < 10 {
                    let mut slim = serde_json::Map::new();
                    for k in ["source","target","rel","rel_inverse","violation"].iter() {
                        if let Some(val) = e.get(*k) { slim.insert((*k).to_string(), val.clone()); }
                    }
                    samples_invalid.push(serde_json::Value::Object(slim));
                }
            }
        }
    }

    Json(json!({
        "nodes_by_type": nodes_by_type,
        "edges_total": edges.len(),
        "edges_with_rel": edges_with_rel,
        "edges_valid": edges_valid,
        "edges_invalid": edges_invalid,
        "samples_invalid": samples_invalid
    }))
}


// —— DecisionEngine singleton ——
static DECISION_ENGINE: OnceLock<Mutex<DecisionEngine>> = OnceLock::new();
fn decision_engine() -> &'static Mutex<DecisionEngine> {
    DECISION_ENGINE.get_or_init(|| {
        // Start conservative; we can hot-load JSON later
        let policy = DecisionPolicy::default();
        let allow = AllowSuppressList::default();
        Mutex::new(DecisionEngine::new(policy, allow))
    })
}

#[allow(dead_code)]
fn decision_load_from_files(policy_path: &str, allow_path: &str) {
    let policy = std::fs::read_to_string(policy_path)
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .and_then(|v| v.get("policy").cloned().or(Some(v)))
        .and_then(|p| serde_json::from_value::<DecisionPolicy>(p).ok())
        .unwrap_or_default();
    let allow = std::fs::read_to_string(allow_path)
        .ok()
        .and_then(|t| AllowSuppressList::from_json_str(&t).ok())
        .unwrap_or_default();
    let mut eng = decision_engine().lock().unwrap();
    *eng = DecisionEngine::new(policy, allow);
}

// ======================= Incidents (Phase 2) =======================

/// Simple rolling incident manager:
/// - group by (exe, primary technique, time bucket)
/// - maintains compact JSON objects for UI & SSE
struct IncidentsMgr {
    window_secs: i64,
    items: HashMap<String, Value>, // id -> incident JSON
}

impl IncidentsMgr {
    fn new(window_secs: i64) -> Self {
        Self { window_secs, items: HashMap::new() }
    }

    fn list(&self) -> Vec<Value> {
        let mut v: Vec<Value> = self.items.values().cloned().collect();
        v.sort_by(|a, b| {
            let ta = a.get("last_ts").and_then(|x| x.as_i64()).unwrap_or(0);
            let tb = b.get("last_ts").and_then(|x| x.as_i64()).unwrap_or(0);
            tb.cmp(&ta)
        });
        v
    }

    fn get(&self, id: &str) -> Option<Value> { self.items.get(id).cloned() }

    /// Ingest an alert JSON; returns updated incident JSON for SSE.
    fn ingest_alert_json(&mut self, alert: &Value) -> Option<Value> {
        let ts = alert.get("ts")
            .or_else(|| alert.get("time"))
            .or_else(|| alert.get("timestamp"))
            .and_then(|x| x.as_i64())
            .unwrap_or(now_ts() as i64);

        let exe = alert.pointer("/event/exe")
            .or_else(|| alert.pointer("/event/binary_path"))
            .and_then(|x| x.as_str())
            .unwrap_or("-")
            .to_string();

        let techniques = extract_mitre(alert);
        let primary_t = techniques.get(0).cloned().unwrap_or_else(|| "-".into());
        let bucket = ts / self.window_secs.max(1);

        // id = sha1(exe|t|bucket)
        let mut h = Sha1::new();
        h.update(exe.as_bytes());
        h.update(b"|");
        h.update(primary_t.as_bytes());
        h.update(b"|");
        h.update(bucket.to_string().as_bytes());
        let id = format!("{:x}", h.finalize());

        let risk = alert.get("risk")
            .or_else(|| alert.get("score"))
            .and_then(|x| x.as_f64())
            .unwrap_or(0.0);

        let sev = alert.get("severity")
            .and_then(|x| x.as_str()).unwrap_or("none").to_string();

        // pull a compact neighborhood graph ±5m around this alert
        let recs = window_records(ts, 300);
        let (nodes, edges) = build_graph_from_records(&recs);
        let rgcn = rgcn_proxy_score(&nodes);

        // Avoid moving `id` into the JSON so we can use it below
        let mut item = if let Some(existing) = self.items.remove(&id) {
            existing
        } else {
            json!({
                "id": id.clone(),
                "exe": exe,
                "primary_technique": primary_t,
                "techniques": techniques,
                "bucket": bucket,
                "first_ts": ts,
                "last_ts": ts,
                "alerts_count": 0u64,
                "risk_max": 0.0f64,
                "severity_max": "none",
                "sample_ids": [],
                "rgcn_score": rgcn,
                "graph": { "nodes": nodes, "edges": edges },
            })
        };

        // update fields
        {
            let obj = item.as_object_mut().unwrap();
            // times
            let first_ts = obj.get("first_ts").and_then(|x| x.as_i64()).unwrap_or(ts);
            let last_ts  = obj.get("last_ts").and_then(|x| x.as_i64()).unwrap_or(ts);
            obj.insert("first_ts".into(), json!(first_ts.min(ts)));
            obj.insert("last_ts".into(), json!(last_ts.max(ts)));

            // counts
            let n = obj.get("alerts_count").and_then(|x| x.as_u64()).unwrap_or(0);
            obj.insert("alerts_count".into(), json!(n + 1));

            // severity max (by rank)
            let cur = obj.get("severity_max").and_then(|x| x.as_str()).unwrap_or("none");
            let new_max = if severity_rank(&sev) > severity_rank(cur) { sev.clone() } else { cur.to_string() };
            obj.insert("severity_max".into(), json!(new_max));

            // risk max
            let rmax = obj.get("risk_max").and_then(|x| x.as_f64()).unwrap_or(0.0);
            obj.insert("risk_max".into(), json!(rmax.max(risk)));

            // techniques merged
            let mut tch: Vec<String> = obj.get("techniques")
                .and_then(|x| x.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();
            for t in techniques {
                if !tch.iter().any(|x| x == &t) { tch.push(t); }
            }
            obj.insert("techniques".into(), json!(tch));

            // keep a tiny sample of alert ids (if present)
            if let Some(aid) = alert.get("id").and_then(|x| x.as_str()) {
                let mut sids: Vec<String> = obj.get("sample_ids")
                    .and_then(|x| x.as_array())
                    .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_default();
                if !sids.iter().any(|x| x == aid) {
                    if sids.len() >= 6 { sids.remove(0); }
                    sids.push(aid.to_string());
                }
                obj.insert("sample_ids".into(), json!(sids));
            }

            // refresh graph/rgcn snapshot using latest ts window (cheap, bounded)
            obj.insert("rgcn_score".into(), json!(rgcn));
            obj.insert("graph".into(), json!({ "nodes": nodes, "edges": edges }));
        }

        self.items.insert(id.clone(), item.clone());
        Some(item)
    }
}

fn severity_rank(s: &str) -> i32 {
    match s.to_ascii_lowercase().as_str() {
        "high" => 3,
        "medium" => 2,
        "low" => 1,
        _ => 0,
    }
}

// Global incidents store
static INCIDENTS: OnceLock<Arc<Mutex<IncidentsMgr>>> = OnceLock::new();
fn incidents() -> Arc<Mutex<IncidentsMgr>> {
    INCIDENTS
        .get_or_init(|| Arc::new(Mutex::new(IncidentsMgr::new(15 * 60))))
        .clone()
}

// ======================= API state & handlers =======================
fn synth_ppid_edges_if_missing(v: &mut serde_json::Value) {
    // Only synthesize if we have nodes and edges are missing/empty.
    let nodes_arr = match v.get("nodes").and_then(|x| x.as_array()) {
        Some(n) if !n.is_empty() => n,
        _ => return,
    };
    let edges_empty = v
        .get("edges")
        .and_then(|x| x.as_array())
        .map(|a| a.is_empty())
        .unwrap_or(true);
    if !edges_empty {
        return;
    }

    // Track which ids exist and map pid -> node id for proc nodes.
    let mut have_id: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut pid_to_id: std::collections::HashMap<u32, String> = std::collections::HashMap::new();

    for n in nodes_arr {
        if let Some(id) = n.get("id").and_then(|x| x.as_str()) {
            have_id.insert(id.to_string());
            if id.starts_with("proc:") {
                if let Ok(pid) = id.trim_start_matches("proc:").parse::<u32>() {
                    pid_to_id.insert(pid, id.to_string());
                }
            }
        }
        // Also consider explicit pid/ppid fields.
        if let (Some(pid), Some(id)) = (
            n.get("pid").and_then(|x| x.as_u64()).map(|x| x as u32),
            n.get("id").and_then(|x| x.as_str()),
        ) {
            pid_to_id.entry(pid).or_insert_with(|| id.to_string());
            have_id.insert(id.to_string());
        }
    }

    // Synthesize EXEC edges: parent(ppid) -> child(pid)
    let mut out_edges: Vec<serde_json::Value> = Vec::new();
    for n in nodes_arr {
        let pid  = n.get("pid").and_then(|x| x.as_u64()).map(|x| x as u32);
        let ppid = n.get("ppid").and_then(|x| x.as_u64()).map(|x| x as u32);
        if let (Some(pid), Some(ppid)) = (pid, ppid) {
            if pid == 0 || ppid == 0 || pid == ppid { continue; }
            let src = pid_to_id
                .get(&ppid)
                .cloned()
                .unwrap_or_else(|| format!("proc:{ppid}"));
            let dst = pid_to_id
                .get(&pid)
                .cloned()
                .unwrap_or_else(|| format!("proc:{pid}"));

            if have_id.contains(&src) && have_id.contains(&dst) {
                out_edges.push(serde_json::json!({
                    "kind": "EXEC",
                    "rel": "spawned",
                    "rel_inverse": "spawned-by",
                    "rel_valid": true,
                    "source": src,
                    "target": dst,
                    "last_seen": now_ts() as i64,
                    "weight": 1.0
                }));
            }
        }
    }

    v.as_object_mut()
        .unwrap()
        .insert("edges".into(), serde_json::Value::Array(out_edges));
}

#[derive(Clone)]
struct ApiState {
    tx: broadcast::Sender<String>,           // alerts SSE
    graph_tx: broadcast::Sender<String>,     // graph SSE (whole snapshot JSON)
    inc_tx: broadcast::Sender<String>,       // incidents SSE
    integrations: Arc<Mutex<HashMap<String, Value>>>,
}

// small adapter: broadcast::Receiver<String> -> Stream<Item=Result<Event, Infallible>>
fn sse_stream_from_broadcast(
    rx: broadcast::Receiver<String>,
) -> impl Stream<Item = Result<Event, Infallible>> {
    unfold(rx, |mut rx| async move {
        match rx.recv().await {
            Ok(line) => Some((Ok(Event::default().data(line)), rx)),
            Err(_) => None, // channel closed => end stream
        }
    })
}

async fn alerts_sse(
    State(st): State<ApiState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = st.tx.subscribe();
    Sse::new(sse_stream_from_broadcast(rx))
}

async fn graph_sse(
    State(st): State<ApiState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = st.graph_tx.subscribe();
    Sse::new(sse_stream_from_broadcast(rx))
}

// NEW: incidents SSE
async fn incidents_sse(
    State(st): State<ApiState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = st.inc_tx.subscribe();
    Sse::new(sse_stream_from_broadcast(rx))
}

// current graph JSON (from file)
// current graph JSON (from file) — enriched with ontology fields
// current graph JSON (from file) — synthesize edges if missing, then annotate with R-GCN
async fn graph_live() -> impl axum::response::IntoResponse {
    use axum::Json;
    use serde_json::Value;
    use std::fs;

    let body = fs::read_to_string("json_files/graph_live.json").unwrap_or_else(|_| "{}".into());
    let v: Value = serde_json::from_str(&body).unwrap_or(serde_json::json!({}));
    let v = normalize_and_annotate_graph(v);
    Json(v)
}





// helper: read counts so metrics/health can show graph size
fn read_graph_counts_from_file() -> (u64, u64) {
    if let Ok(s) = fs::read_to_string("json_files/graph_live.json") {
        if let Ok(v) = serde_json::from_str::<Value>(&s) {
            let n = v.get("nodes").and_then(|x| x.as_array()).map(|a| a.len()).unwrap_or(0);
            let e = v.get("edges").or_else(|| v.get("links"))
                .and_then(|x| x.as_array()).map(|a| a.len()).unwrap_or(0);
            return (n as u64, e as u64);
        }
    }
    (0, 0)
}
async fn metrics_json() -> Json<Value> {
    // fast-path: use atomics
    let mut ev_in = EVENTS_IN.load(Ordering::Relaxed);
    let ev_acc = EVENTS_ACCEPTED.load(Ordering::Relaxed);
    let mut al = ALERTS_OUT.load(Ordering::Relaxed);

    // fallback to disk so UI isn't blank if counters were zeroed or miswired
    if ev_in == 0 {
        let mut total = 0u64;
        if let Ok(rd) = fs::read_dir("telemetry_output") {
            for e in rd.flatten() {
                let p = e.path();
                if let Some(n) = p.file_name().and_then(|n| n.to_str()) {
                    if n.starts_with("telemetry_") && n.ends_with(".jsonl") {
                        if let Ok(s) = fs::read_to_string(&p) {
                            total += s.lines().count() as u64;
                        }
                    }
                }
            }
        }
        if total > 0 {
            ev_in = total;
        }
    }
    if al == 0 {
        if let Ok(s) = fs::read_to_string("logs/alerts.jsonl") {
            al = s.lines().count() as u64;
        }
    }

    let (nodes_count, edges_count) = read_graph_counts_from_file();
    let pbm = pb_introspect::metrics();
    let iocm = ioc::metrics();

    // Build the existing metrics payload first
    let base = json!({
        // canonical keys
        "events_in": ev_in,                 // received (pre-filter)
        "events_accepted": ev_acc,          // accepted (post-filter, persisted)
        "alerts_out": al,
        "nodes_count": nodes_count,
        "edges_count": edges_count,
        "bpf_drops_total": BPF_DROPS.load(Ordering::Relaxed),

        // compatibility aliases
        "events": ev_in,
        "events_accepted_total": ev_acc,
        "alerts_out_total": al,

        // PB/IOC gauges
        "pb_hits_total": pbm.pb_hits_total,
        "pb_pending_current": pbm.pb_pending_current,
        "ioc_matches_total": iocm.matches_total,
        "yara_matches_total": iocm.yara_total
    });

    // Merge in R-GCN overlay metrics (no-op if bridge is disabled/empty)
    let mut obj = base.as_object().unwrap().clone();
    let rg = rgcn_metrics();
    if let Some(m) = rg.as_object() {
        for (k, v) in m {
            obj.insert(k.clone(), v.clone());
        }
    }

    Json(Value::Object(obj))
}


// /healthz -> JSON with counters + graph counts + bpf drops
async fn healthz() -> impl IntoResponse {
    let ev_in = EVENTS_IN.load(Ordering::Relaxed);
    let ev_acc = EVENTS_ACCEPTED.load(Ordering::Relaxed);
    let al = ALERTS_OUT.load(Ordering::Relaxed);
    let (n, e) = read_graph_counts_from_file();
    Json(json!({
        "ok": true,
        "events_in": ev_in,
        "events_accepted": ev_acc,
        "alerts_out": al,
        "nodes_count": n,
        "edges_count": e,
        "bpf_drops_total": BPF_DROPS.load(Ordering::Relaxed)
    }))
}

// --- IOC endpoints (simple, stateless handlers) ---
async fn ioc_reload_handler() -> impl IntoResponse {
    match ioc::reload() {
        Ok(_) => (StatusCode::OK, Json(json!({"ok": true}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"ok": false, "error": e.to_string()}))),
    }
}
async fn ioc_stats_handler() -> Json<Value> {
    let m = ioc::metrics();
    Json(json!({
        "ioc_matches_total": m.matches_total,
        "yara_matches_total": m.yara_total
    }))
}

// quick diag: where am I serving from + file presence
async fn diag_where() -> Json<Value> {
    let cwd = env::current_dir().map(|p| p.display().to_string()).unwrap_or_else(|_| "<err>".into());

    let ui_dir = "ui";
    let jf_dir = "json_files";

    let ui_abs  = fs::canonicalize(ui_dir).ok().map(|p| p.display().to_string());
    let jf_abs  = fs::canonicalize(jf_dir).ok().map(|p| p.display().to_string());

    let exists = |p: &str| Path::new(p).exists();

    Json(json!({
        "cwd": cwd,
        "dirs": {
            "ui":        { "configured": ui_dir, "resolved": ui_abs },
            "json_files":{ "configured": jf_dir, "resolved": jf_abs }
        },
        "files": {
            "ui/index.html":           exists("ui/index.html"),
            "ui/app.js":               exists("ui/app.js"),
            "ui/graph.html":           exists("ui/graph.html"),
            "json_files/graph_live.json": exists("json_files/graph_live.json")
        }
    }))
}

// --- Graph endpoint: build process/file graph for an endpoint and attach an R-GCN-like score ---

#[derive(Deserialize)]
struct GraphQ {
    window_sec: Option<u64>,
}

// robust timestamp reader: supports seconds + milliseconds
fn parse_ts_any(v: &Value, default: i64) -> i64 {
    v.get("timestamp").and_then(|x| x.as_i64())
        .or_else(|| v.get("ts").and_then(|x| x.as_i64()))
        .or_else(|| v.get("time").and_then(|x| x.as_i64()))
        .or_else(|| v.get("ts_ms").and_then(|x| x.as_i64()).map(|ms| ms / 1000))
        .unwrap_or(default)
}

async fn graph_for_endpoint(
    AxPath(endpoint): AxPath<String>,
    Query(q): Query<GraphQ>,
) -> impl IntoResponse {
    let window = q.window_sec.unwrap_or(900); // 15m
    let mut records: Vec<TelemetryRecord> = Vec::new();

    if let Ok(s) = fs::read_to_string("state/pcb_snaps.ndjson") {
        let now = now_ts() as i64;
        for line in s.lines().rev().take(5000) {
            if let Ok(v) = serde_json::from_str::<Value>(line) {
                let pid = v.get("pid").and_then(|x| x.as_i64()).unwrap_or_default() as i32;
                let ppid = v.get("ppid").and_then(|x| x.as_i64()).unwrap_or_default() as i32;
                let ts = parse_ts_any(&v, now);
                if (now - ts) as u64 <= window {
                    records.push(TelemetryRecord {
                        timestamp: ts as u64,
                        pid,
                        ppid,
                        uid: 0,
                        binary_path: v.get("exe").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
                        command_line: v.get("cmdline").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
                        cwd: String::new(),
                        env_vars: None,
                        tags: vec![],
                        risk_score: Some(0),
                    });
                }
            }
        }
    }

    let (nodes, edges) = build_graph_from_records(&records);
    let mut risks = Vec::new();
    for n in &nodes {
        let s = 1.0 - n.trust_score.max(0.0).min(1.0);
        risks.push(s as f64);
    }
    let rgcn_score = if risks.is_empty() {
        0.0
    } else {
        risks.iter().sum::<f64>() / risks.len() as f64
    };
    Json(json!({
        "endpoint": endpoint,
        "rgcn_score": rgcn_score,
        "nodes": nodes,
        "edges": edges,
        // handy for: curl ... | jq '.nodes_count, .edges_count'
        "nodes_count": risks.len(),
        "edges_count": edges.len()
    }))
}

// =========================================
// EXPLAIN API (rich, context-heavy)
// =========================================

fn tail_file_lines(path: &str, max: usize) -> Vec<String> {
    if let Ok(s) = fs::read_to_string(path) {
        let lines: Vec<&str> = s.lines().collect();
        return lines
            .into_iter()
            .rev()
            .take(max)
            .rev()
            .map(|x| x.to_string())
            .collect();
    }
    Vec::new()
}

fn synth_id(v: &Value) -> String {
    let mut h = Sha1::new();
    let ts = v
        .get("ts")
        .or_else(|| v.get("time"))
        .or_else(|| v.get("timestamp"))
        .and_then(|x| x.as_i64())
        .unwrap_or(0)
        .to_string();
    let exe = v
        .pointer("/event/exe")
        .or_else(|| v.pointer("/event/binary_path"))
        .and_then(|x| x.as_str())
        .unwrap_or("-");
    let cmd = v
        .pointer("/event/cmdline")
        .or_else(|| v.pointer("/event/command_line"))
        .and_then(|x| x.as_str())
        .unwrap_or("-");
    h.update(ts.as_bytes());
    h.update(exe.as_bytes());
    h.update(cmd.as_bytes());
    format!("{:x}", h.finalize())
}

fn load_alert_by_id(id: &str) -> Option<Value> {
    for line in tail_file_lines("logs/alerts.jsonl", 10000) {
        if let Ok(v) = serde_json::from_str::<Value>(&line) {
            if let Some(aid) = v.get("id").and_then(|x| x.as_str()) {
                if aid == id {
                    return Some(v);
                }
            } else {
                let sid = synth_id(&v);
                if sid == id {
                    return Some(v);
                }
            }
        }
    }
    None
}

fn window_records(ts_center: i64, secs: i64) -> Vec<TelemetryRecord> {
    let mut out = Vec::new();
    for line in tail_file_lines("state/pcb_snaps.ndjson", 20000) {
        if let Ok(v) = serde_json::from_str::<Value>(&line) {
            let ts = parse_ts_any(&v, ts_center);
            if (ts - ts_center).abs() <= secs {
                let pid = v.get("pid").and_then(|x| x.as_i64()).unwrap_or(0) as i32;
                let ppid = v.get("ppid").and_then(|x| x.as_i64()).unwrap_or(0) as i32;
                let exe = v.get("exe").and_then(|x| x.as_str()).unwrap_or("").to_string();
                let cmd = v.get("cmdline").and_then(|x| x.as_str()).unwrap_or("").to_string();
                out.push(TelemetryRecord {
                    timestamp: ts as u64,
                    pid,
                    ppid,
                    uid: 0,
                    binary_path: exe,
                    command_line: cmd,
                    cwd: String::new(),
                    env_vars: None,
                    tags: vec![],
                    risk_score: Some(0),
                });
            }
        }
    }
    out
}

fn rgcn_proxy_score(nodes: &[GraphNode]) -> f64 {
    if nodes.is_empty() {
        return 0.0;
    }
    let mut sum = 0.0;
    for n in nodes {
        let trust = n.trust_score.max(0.0).min(1.0) as f64;
        sum += (1.0 - trust);
    }
    (sum / nodes.len() as f64).max(0.0).min(1.0)
}

fn extract_entities(alert: &Value) -> Value {
    let user = alert
        .pointer("/event/user")
        .or_else(|| alert.get("user"))
        .cloned()
        .unwrap_or(Value::Null);
    let host = alert
        .pointer("/event/host")
        .or_else(|| alert.get("host"))
        .cloned()
        .unwrap_or(Value::Null);
    let exe = alert
        .pointer("/event/exe")
        .or_else(|| alert.pointer("/event/binary_path"))
        .cloned()
        .unwrap_or(Value::Null);
    let pid = alert.pointer("/event/pid").cloned().unwrap_or(Value::Null);
    let ppid = alert.pointer("/event/ppid").cloned().unwrap_or(Value::Null);
    let sha = alert
        .pointer("/event/sha256")
        .or_else(|| alert.get("sha256"))
        .cloned()
        .unwrap_or(Value::Null);
    let src = alert.get("src_ip").cloned().unwrap_or(Value::Null);
    let dst = alert.get("dst_ip").cloned().unwrap_or(Value::Null);
    json!({
        "user": user, "host": host, "exe": exe, "pid": pid, "ppid": ppid, "sha256": sha, "src_ip": src, "dst_ip": dst
    })
}

fn extract_mitre(alert: &Value) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(f) = alert.get("findings").and_then(|x| x.as_array()) {
        for it in f {
            if let Some(lab) = it.get("label").and_then(|x| x.as_str()) {
                if lab.starts_with('T') {
                    out.push(lab.to_string());
                }
            }
        }
    }
    if let Some(t) = alert.get("technique").and_then(|x| x.as_str()) {
        if t.starts_with('T') {
            out.push(t.to_string());
        }
    }
    out.sort();
    out.dedup();
    out
}

fn load_enrichment(ip: Option<&str>, sha256: Option<&str>) -> Value {
    if let Ok(s) = fs::read_to_string("state/enrich_cache.json") {
        if let Ok(v) = serde_json::from_str::<Value>(&s) {
            let mut m = serde_json::Map::new();
            if let Some(ip) = ip {
                if let Some(val) = v.get(&format!("ip:{ip}")) {
                    m.insert("ip".into(), val.clone());
                }
            }
            if let Some(sha) = sha256 {
                if let Some(val) = v.get(&format!("sha256:{sha}")) {
                    m.insert("sha256".into(), val.clone());
                }
            }
            return Value::Object(m);
        }
    }
    json!({})
}

impl Default for Verdict {
    fn default() -> Self { Verdict::Benign }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Verdict { Benign, Leaning, Malicious }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ExplainSummary {
    verdict: Verdict,
    confidence: f32,           // 0.0 .. 1.0
    reason_codes: Vec<String>, // short machine-readable reasons
}

fn pick_verdict_from_stats(avg_trust: f64, sev: &str, rgcn_avg_risk: f64) -> ExplainSummary {
    let mut reasons = vec![];
    let sev_l = sev.to_ascii_lowercase();
    if avg_trust < 0.6 { reasons.push("trust_low".into()); }
    else if avg_trust < 0.85 { reasons.push("trust_medium".into()); }
    if rgcn_avg_risk >= 0.8 { reasons.push("rgcn_high".into()); }
    if sev_l == "high" { reasons.push("sev_high".into()); }
    if sev_l == "medium" { reasons.push("sev_medium".into()); }

    let (verdict, confidence) = if avg_trust < 0.6 || sev_l == "high" || rgcn_avg_risk >= 0.9 {
        (Verdict::Malicious, 0.9)
    } else if avg_trust < 0.85 || sev_l == "medium" || rgcn_avg_risk >= 0.75 {
        (Verdict::Leaning, 0.7)
    } else {
        (Verdict::Benign, 0.85)
    };

    ExplainSummary { verdict, confidence, reason_codes: reasons }
}

async fn explain_alert(AxPath(alert_id): AxPath<String>) -> impl IntoResponse {
    let mut alert = if let Some(v) = load_alert_by_id(&alert_id) {
        v
    } else {
        return (StatusCode::NOT_FOUND, "alert not found").into_response();
    };
    if alert.get("id").is_none() {
        let sid = synth_id(&alert);
        alert.as_object_mut().unwrap().insert("id".into(), Value::String(sid));
    }

    let sev = alert.get("severity").and_then(|x| x.as_str()).unwrap_or("none").to_string();
    let risk = alert.get("risk").or_else(|| alert.get("score")).and_then(|x| x.as_f64()).unwrap_or(0.0);
    let ts = alert.get("ts").or_else(|| alert.get("time")).or_else(|| alert.get("timestamp"))
        .and_then(|x| x.as_i64()).unwrap_or(now_ts() as i64);

    // neighborhood & graph
    let recs = window_records(ts, 300);
    let (nodes, edges) = build_graph_from_records(&recs);
    let rgcn = rgcn_proxy_score(&nodes);

    let findings = alert.get("findings").cloned().unwrap_or(json!([]));

    let mut anomalies: Vec<String> = Vec::new();
    for r in recs.iter() {
        for t in &r.tags {
            if t.contains("mahal") || t.contains("elliptic") || t.contains("krim") {
                anomalies.push(t.clone());
            }
        }
    }
    anomalies.sort();
    anomalies.dedup();

    let entities = extract_entities(&alert);
    let mitre = extract_mitre(&alert);

    let ip = alert.get("src_ip").and_then(|x| x.as_str());
    let sha = alert.get("sha256").and_then(|x| x.as_str())
        .or_else(|| alert.pointer("/event/sha256").and_then(|x| x.as_str()));
    let enrichment = load_enrichment(ip, sha);

    let reasons = alert.get("reasons").cloned().unwrap_or_else(|| {
        let mut rs = Vec::new();
        if let Some(arr) = findings.as_array() {
            for f in arr {
                let src = f.get("source").and_then(|x| x.as_str()).unwrap_or("detector");
                let sc = f.get("score").and_then(|x| x.as_f64()).unwrap_or(0.0);
                rs.push(json!({"source": src, "weight": sc}));
            }
        }
        json!(rs)
    });

    let technique_names: Vec<String> = mitre.clone().into_iter().map(|t| match t.as_str() {
        "T1055" => "Process Injection".into(),
        "T1059.001" => "Command & Scripting: PowerShell".into(),
        "T1547.001" => "Boot/Logon Autostart: Run Keys".into(),
        _ => t,
    }).collect();

    let confidence = alert.get("confidence").and_then(|x| x.as_f64()).unwrap_or_else(|| rgcn.max(0.0).min(1.0));

    // ----- compute these BEFORE json! -----
    let mahala_d2: f64 = {
        let ep = build_episode_from_records(&recs);
        let mut d2: f64 = 0.0;
        if let Some(ep) = ep {
            let x = Mahala::vectorize_episode(&ep);
            if !x.is_empty() {
                let now = chrono::Utc::now();
                if let Ok(mut eng) = mahala_engine().lock() {
                    let (dist2, _damp, _sup) = eng.distance_and_damp("default", &x, now);
                    d2 = dist2 as f64;
                }
            }
        }
        d2
    };

    let elliptic_margin: f64 = {
        let ep = build_episode_from_records(&recs);
        let mut margin: f64 = 0.0;
        if let Some(ep) = ep {
            let x = Mahala::vectorize_episode(&ep);
            if !x.is_empty() {
                if let Ok(env_lock) = env_model().lock() {
                    if let Some(env) = env_lock.as_ref() {
                        margin = env.decision_function(&x);
                    }
                }
            }
        }
        margin
    };
    // --------------------------------------

    // Verdict summary
    let avg_trust = if nodes.is_empty() {
        0.75
    } else {
        let s: f64 = nodes.iter().map(|n| n.trust_score as f64).sum();
        (s / nodes.len() as f64).clamp(0.0, 1.0)
    };
    let summary = pick_verdict_from_stats(avg_trust, &sev, rgcn);

    let resp = json!({
        "id": alert.get("id"),
        "headline": format!(
            "{} · {} · {}",
            sev.to_uppercase(),
            technique_names.get(0).map(|s| s.as_str()).unwrap_or("-"),
            alert.pointer("/event/exe").and_then(|x| x.as_str()).unwrap_or("-")
        ),
        "severity": sev,
        "risk": risk,
        "techniques": mitre,
        "technique_names": technique_names,
        "ts": ts,
        "entities": entities,
        "rgcn_score": rgcn,
        "confidence": confidence,
        "model_version": "phase-2.0",
        "detector_versions": {
            "rules_hot": ">=1",
            "mahala": "1.0",
            "elliptic": "1.0",
            "krim_lite": "1.0"
        },
        "why_now": { "mahalanobis_d2": mahala_d2, "elliptic_margin": elliptic_margin },
        "why": findings,
        "anomalies": anomalies,
        "graph": { "nodes": nodes, "edges": edges },
        "timeline": recs.iter().map(|r| json!({
            "ts": r.timestamp, "pid": r.pid, "ppid": r.ppid, "exe": r.binary_path, "cmd": r.command_line
        })).collect::<Vec<_>>(),
        "enrichment": enrichment,
        "risk_breakdown": reasons,
        "recommendations": recommend_from_mitre(&extract_mitre(&alert)),
        "summary": summary
    });

    Json(resp).into_response()
}



fn recommend_from_mitre(mitre: &[String]) -> Vec<String> {
    let mut actions: Vec<String> = Vec::new();
    if mitre.iter().any(|t| t.starts_with("T1055")) {
        actions.push("Isolate host; capture memory; hunt for unsigned DLLs injected into signed parents.".into());
    }
    if mitre.iter().any(|t| t.starts_with("T1059.001")) {
        actions.push("Block outbound for the user; review PowerShell transcript logs and AMSI events.".into());
    }
    if mitre.iter().any(|t| t.starts_with("T1547.001")) {
        actions.push("Check autorun keys and Startup folder; verify owner and signer of binaries.".into());
    }
    if actions.is_empty() {
        actions.push("Triage process tree, parent-child chain, and recent network egress for this endpoint.".into());
    }
    actions
}

// ======================= Utilities / mapping =======================

fn try_convert_output(output: TelemetryOutput) -> Result<TelemetryRecord, String> {
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
    rec.tags.push(output.signal);
    rec.risk_score = Some((output.confidence * 100.0) as u32);
    Ok(rec)
}

// ----------- RELAXED acceptance filter -----------
fn filter_records(records: &mut Vec<TelemetryRecord>) {
    records.retain(|r| {
        let has_proc = !r.binary_path.is_empty() || !r.command_line.is_empty();

        // Treat certain tags as implying a real-world entity even if paths/cmdline are blank
        let has = |s: &str| r.tags.iter().any(|t| t.contains(s));
        let fileish = r.tags.iter().any(|t| t.starts_with("file_") || t.contains("write") || t.contains("open"));
        let tag_implies_entity =
            has("dns") || has("net_") || has("connect") || fileish || has("auth") || has("logon") || has("usb");

        let not_kernel_ppid2 = r.ppid != 2;
        (has_proc || tag_implies_entity) && not_kernel_ppid2
    });

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
    let mut seen: HashSet<(i64, String)> = HashSet::new();
    records.retain(|r| {
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

fn categorize_record(rec: &TelemetryRecord) -> (String, String) {
    if !rec.binary_path.is_empty() || !rec.command_line.is_empty() {
        return ("process".into(), "exec".into());
    }
    let has = |needle: &str| rec.tags.iter().any(|x| x.contains(needle));
    if has("dns") || has("net_") || has("connect") {
        return ("network".into(), "connect".into());
    }
    if has("file_") || has("write") || has("open") {
        return ("file".into(), "write".into());
    }
    if has("auth") || has("logon") {
        return ("auth".into(), "authenticate".into());
    }
    if has("usb") {
        return ("usb".into(), "device".into());
    }
    ("unknown".into(), "unknown".into())
}

// ======================= Episode / graph helpers =======================

fn build_episode_from_records(records: &[TelemetryRecord]) -> Option<Episode> {
    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
    let mut ep = Episode::new(host);
    ep.context_id = Some("default".into());
    ep.role = Some("default".into());

    for r in records {
        // Use record timestamp, not "now", to keep temporal ordering sensible.
        let ts = Utc
            .timestamp_opt(r.timestamp as i64, 0)
            .single()
            .unwrap_or_else(Utc::now);

        if !r.binary_path.is_empty() || !r.command_line.is_empty() {
            ep.push(FHEvent {
                ts,
                event_type: EventType::Execve,
                ..Default::default()
            });
        }
        if !r.cwd.is_empty() {
            ep.push(FHEvent {
                ts,
                event_type: EventType::Other,
                ..Default::default()
            });
        }
    }
    Some(ep)
}

fn build_graph_from_records(records: &[TelemetryRecord]) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    use std::collections::{HashMap as Map, HashSet as Set};

    let mut nodes: Vec<GraphNode> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();

    let mut proc_id_map: Map<i32, String> = Map::new();
    let mut seen_nodes: Set<String> = Set::new();
    let mut seen_edges: Set<(String, String)> = Set::new();

    // pass 1: create all proc nodes (so PPID lookups always work)
    for r in records {
        let nid = format!("proc:{}", r.pid);
        if seen_nodes.insert(nid.clone()) {
            proc_id_map.insert(r.pid, nid.clone());
            nodes.push(GraphNode {
                id: nid,
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
        }
    }

    // pass 2: connect proc->proc and proc->file/dir
    for r in records {
        let nid = format!("proc:{}", r.pid);

        if r.ppid > 0 {
            if let Some(parent) = proc_id_map.get(&r.ppid).cloned() {
                if seen_edges.insert((parent.clone(), nid.clone())) {
                    edges.push(GraphEdge {
                        source: parent,
                        target: nid.clone(),
                        ..Default::default()
                    });
                }
            }
        }
        if !r.binary_path.is_empty() {
            let file_id = format!("file:{}", r.binary_path);
            if seen_nodes.insert(file_id.clone()) {
                nodes.push(GraphNode { id: file_id.clone(), ..Default::default() });
            }
            if seen_edges.insert((nid.clone(), file_id.clone())) {
                edges.push(GraphEdge { source: nid.clone(), target: file_id, ..Default::default() });
            }
        }
        if !r.cwd.is_empty() {
            let dir_id = format!("file:{}", r.cwd);
            if seen_nodes.insert(dir_id.clone()) {
                nodes.push(GraphNode { id: dir_id.clone(), ..Default::default() });
            }
            if seen_edges.insert((nid.clone(), dir_id.clone())) {
                edges.push(GraphEdge { source: nid.clone(), target: dir_id, ..Default::default() });
            }
        }
    }
    (nodes, edges)
}

// —— anomaly taggers ——
fn tag_batch_outliers_with_mahala_and_envelope(
    records: &mut [TelemetryRecord],
    ep: &Episode,
) -> (Option<f64>, Option<f64>) {
    let x = Mahala::vectorize_episode(ep);
    if x.is_empty() {
        return (None, None);
    }
    let p = x.len();

    let now = chrono::Utc::now();

    // Track diagnostics we’ll return
    let mut mahal_diag: Option<f64> = None;
    let mut env_margin: Option<f64> = None;

    // Mahalanobis
    let mut engine = mahala_engine().lock().unwrap();
    engine.observe("default", &x, now);
    let (d2, _damp, support) = engine.distance_and_damp("default", &x, now);
    mahal_diag = Some(d2 as f64);
    drop(engine);

    // χ²_p(0.99) ≈ p + z * sqrt(2p)
    let z = 2.326_347_874_040_840_8_f64;
    let thr = (p as f64) + z * (2.0 * p as f64).sqrt();
    if (d2 as f64) > thr && support >= MahalaCfg::d_support() {
        for r in records.iter_mut() {
            r.tags.push("mahalanobis_outlier".into());
        }
    }

    // Keep/update rolling baseline for EllipticEnvelope
    {
        let mut buf = env_baseline().lock().unwrap();
        if buf.len() == buf.capacity() {
            buf.pop_front();
        }
        buf.push_back(x.clone());
    }
    let model_ready = { env_baseline().lock().unwrap().len() >= 256 };

    if model_ready {
        let mut env_lock = env_model().lock().unwrap();
        if env_lock.is_none() || (support % 200 == 0) {
            let buf = env_baseline().lock().unwrap();
            if let Some(env) =
                EllipticEnvelope::fit(&buf.iter().cloned().collect::<Vec<_>>(), 25, 0.01)
            {
                *env_lock = Some(env);
            }
        }
        if let Some(env) = env_lock.as_ref() {
            let margin = env.decision_function(&x);
            env_margin = Some(margin);
            if margin < 0.0 {
                for r in records.iter_mut() {
                    r.tags.push("elliptic_outlier".into());
                }
            }
        }
    }

    (mahal_diag, env_margin)
}

fn tag_records_from_krim(records: &mut [TelemetryRecord]) {
    let (nodes, edges) = build_graph_from_records(records);
    if nodes.len() < 3 {
        return;
    }
    let baseline_path =
        std::env::var("KRIM_BASELINE").unwrap_or_else(|_| "state/krim_baseline.json".to_string());
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
            if let Some(last) = records.last_mut() {
                last.tags.push("krim_alert".into());
            }
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

// ---- small helpers ----
fn resolve_rules_path() -> Option<PathBuf> {
    let candidates = [
        "edr-agent/src/rules.json",
        "src/rules.json",
        "rules.json",
    ];
    for c in candidates {
        if Path::new(c).exists() {
            return Some(PathBuf::from(c));
        }
    }
    None
}

// ======================= DecisionEngine helpers =======================

fn sev_to_str(s: Severity) -> &'static str {
    match s {
        Severity::High => "high",
        Severity::Medium => "medium",
        Severity::Low => "low",
    }
}

fn first_token(s: &str) -> String {
    s.split_whitespace().next().unwrap_or("").to_string()
}

fn final_alert_to_json(a: &FinalAlert) -> Value {
    let primary_ttp = a
        .tags
        .iter()
        .find(|t| t.starts_with('T'))
        .cloned()
        .unwrap_or_else(|| "-".into());

    json!({
        "id": format!("da:{}:{}:{}", a.exe, a.user, a.ts),
        "ts": a.ts as i64,
        "severity": sev_to_str(a.severity),
        "risk": (a.risk as f64),
        "support": (a.support as f64),
        "technique": primary_ttp,
        "tags": a.tags,
        "detectors": a.detectors.iter().map(|d| json!({"kind": format!("{:?}", d.kind), "score": d.score})).collect::<Vec<_>>(),
        "rationale": a.rationale,
        "event": {
            "exe": a.exe,
            "cmdline": a.cmd_prefix,
            "user": a.user,
            "host": a.host
        },
        "findings": a.detectors.iter().map(|d| {
            json!({"source": format!("{:?}", d.kind), "score": d.score, "label": primary_ttp})
        }).collect::<Vec<_>>()
    })
}

fn emit_final_incident(
    a: FinalAlert,
    alert_tx: &broadcast::Sender<String>,
    inc_tx: &broadcast::Sender<String>,
) {
    let v = final_alert_to_json(&a);
    let line = v.to_string();

    // Persist alongside Phase-2 alerts so Explain/Incidents keep working
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/alerts.jsonl")
        .and_then(|mut f| {
            use std::io::Write;
            writeln!(f, "{line}")
        });

    // Mirror DecisionEngine incident as a Playbooks “hit” for operator visibility
    pb_introspect::push_hit(pb_introspect::PlaybookHit {
        id: format!("de:{}", v.get("id").and_then(|x| x.as_str()).unwrap_or("")),
        playbook_id: "decision_engine".to_string(),
        ts: v.get("ts").and_then(|x| x.as_i64()).unwrap_or(0).max(0) as u64,
        filled_slots: vec!["consensus".to_string()],   // minimal shim until PB engine feeds real slots
        missing_slots: vec![],
        tags: v.get("tags")
            .and_then(|t| t.as_array())
            .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default(),
        rationale: vec![format!(
            "DE {} risk={:.2} tech={}",
            v.get("severity").and_then(|x| x.as_str()).unwrap_or("low"),
            v.get("risk").and_then(|x| x.as_f64()).unwrap_or(0.0),
            v.get("technique").and_then(|x| x.as_str()).unwrap_or("-")
        )],
    });

    // Broadcast to existing streams
    let _ = alert_tx.send(line);
    if let Some(out) = incidents().lock().unwrap().ingest_alert_json(&v) {
        let _ = inc_tx.send(out.to_string());
    }

    ALERTS_OUT.fetch_add(1, Ordering::Relaxed);
}

fn signal_from_tag(rec: &TelemetryRecord, tag: &str) -> Option<Signal> {
    let exe = if !rec.binary_path.is_empty() {
        rec.binary_path.clone()
    } else {
        first_token(&rec.command_line)
    };
    if exe.is_empty() {
        return None;
    }

    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
    let user = if rec.uid == 0 {
        "root".into()
    } else {
        format!("uid:{}", rec.uid)
    };

    let (det, score) = match tag {
        t if t.eq_ignore_ascii_case("krim_alert") => (DetectorKind::Krim, 0.8),
        t if t.eq_ignore_ascii_case("mahalanobis_outlier") => (DetectorKind::Mahala, 0.7),
        t if t.eq_ignore_ascii_case("elliptic_outlier") => (DetectorKind::Envelope, 0.65),
        "MEMFD_EXEC" | "RWX_MAP" => (DetectorKind::Rules, 1.0),
        t if t.starts_with('T') => (DetectorKind::Rules, 0.9),
        _ => return None,
    };

    Some(Signal {
        detector: det,
        score,
        ts: rec.timestamp,
        exe: Some(exe),
        user: Some(user),
        host: Some(host),
        cmd_prefix: Some(first_token(&rec.command_line)),
        parent: None,
        signer: None,
        hash: None,
        pid: Some(rec.pid),
        tags: vec![tag.to_string()],
    })
}

fn feed_decider_from_record_tags(
    records: &[TelemetryRecord],
    alert_tx: &broadcast::Sender<String>,
    inc_tx: &broadcast::Sender<String>,
) {
    let mut eng = decision_engine().lock().unwrap();
    for rec in records {
        for tag in &rec.tags {
            if let Some(sig) = signal_from_tag(rec, tag) {
                let outs = eng.ingest(sig);
                for a in outs {
                    emit_final_incident(a, alert_tx, inc_tx);
                }
            }
        }
    }
}

// ======================= Boot =======================

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let _ = env_logger::try_init();
    {
    let cwd = std::env::current_dir().unwrap();
    eprintln!("🗂️  cwd        : {}", cwd.display());
    eprintln!("🗂️  ui         : {}", std::fs::canonicalize("ui").map(|p| p.display().to_string()).unwrap_or_else(|_| "MISSING".into()));
    eprintln!("🗂️  json_files : {}", std::fs::canonicalize("json_files").map(|p| p.display().to_string()).unwrap_or_else(|_| "MISSING".into()));
    eprintln!("🗎   ui/index.html present? {}", std::path::Path::new("ui/index.html").exists());
    eprintln!("🗎   ui/app.js     present? {}", std::path::Path::new("ui/app.js").exists());
    eprintln!("🗎   ui/graph.html present? {}", std::path::Path::new("ui/graph.html").exists());
    }

    let _policy = forensic_hooks::config::load_and_verify_policy("policy.json")
        .expect("❌ Failed to load or verify policy");

    let writer = Arc::new(Mutex::new(
        forensic_hooks::telemetry_writer::TelemetryWriter::new(),
    ));

    // === IOC lists & YARA ===
    if let Err(e) = ioc::init() {
        eprintln!("⚠️ ioc::init failed: {e}");
    }

    // === Playbook engine: load all YAMLs from ./playbooks ===
    // === Playbook engine: load all YAMLs from ./playbooks ===
    {
        let pb_dir = std::env::var("PB_DIR").unwrap_or_else(|_| "playbooks".to_string());
        match pb_engine().lock() {
            Ok(mut eng) => {
                match PlaybookEngine::load_dir(&pb_dir) {
                    Ok(new_eng) => {
                        *eng = new_eng;
                        eprintln!("✅ playbook_engine loaded from {pb_dir}");
                    }
                    Err(e) => eprintln!("⚠️ playbook_engine load_dir({pb_dir}) failed: {e}"),
                }
            }
            Err(_) => eprintln!("⚠️ playbook_engine lock poisoned"),
        }
    }


    // alerts / graph / incidents channels
    let (alert_tx, _alert_rx) = broadcast::channel::<String>(1024);
    let (graph_tx, _graph_rx) = broadcast::channel::<String>(128);
    let (inc_tx, _inc_rx)     = broadcast::channel::<String>(256);

    // Optional PB SSE bus (for pb_api)
    let (pb_tx, _pb_rx) = tokio::sync::broadcast::channel::<String>(256);
    pb_introspect::set_sse_sender(pb_tx.clone());
    let pb_state = pb_api::PbApiState { tx: pb_tx.clone() };

    // ensure layout exists
    let _ = std::fs::create_dir_all("logs");
    let _ = std::fs::create_dir_all("edr-agent/logs"); // also support logger paths that expect this prefix
    let _ = std::fs::create_dir_all("state");
    let _ = std::fs::create_dir_all("ui");
    let _ = std::fs::create_dir_all("json_files"); // ensure live-graph path exists
    let cwd = env::current_dir().unwrap();
    eprintln!("🗂️  cwd        : {}", cwd.display());
    eprintln!("🗂️  ui         : {}", fs::canonicalize("ui").map(|p| p.display().to_string()).unwrap_or_else(|_| "MISSING".into()));
    eprintln!("🗂️  json_files : {}", fs::canonicalize("json_files").map(|p| p.display().to_string()).unwrap_or_else(|_| "MISSING".into()));
    eprintln!("🗎   ui/index.html present? {}", Path::new("ui/index.html").exists());
    eprintln!("🗎   ui/app.js     present? {}", Path::new("ui/app.js").exists());
    eprintln!("🗎   ui/graph.html present? {}", Path::new("ui/graph.html").exists());

    // ---- initialize live graph writer using your graph.rs ----
    let gs = graph_state();
let _graph_task = spawn_writer(gs.clone(), PathBuf::from("json_files/graph_live.json"));

{
    let rgcn_enabled = std::env::var("RGCN_ENABLED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if rgcn_enabled {
        use crate::rgcn_bridge::{start_rgcn_bridge, Config as RgcnCfg};

        let url        = std::env::var("RGCN_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:9000/infer".into());
        let poll_ms    = std::env::var("RGCN_POLL_MS").ok().and_then(|s| s.parse().ok()).unwrap_or(1500);
        let timeout_ms = std::env::var("RGCN_TIMEOUT_MS").ok().and_then(|s| s.parse().ok()).unwrap_or(2000);
        let max_nodes  = std::env::var("RGCN_MAX_NODES").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);
        let max_edges  = std::env::var("RGCN_MAX_EDGES").ok().and_then(|s| s.parse().ok()).unwrap_or(6000);

        let cfg = rgcn_bridge::Config {
        url,
        poll_ms: 2000,
        timeout_ms: 3000,
        max_nodes: 3000,
        max_edges: 6000,
        graph_path: "json_files/graph_live.annotated.json".into(), // 👈 annotated
    };
    start_rgcn_bridge(cfg);

        eprintln!("✅ R-GCN bridge started");
    } else {
        eprintln!("ℹ️ R-GCN bridge disabled (set RGCN_ENABLED=1 to enable)");
    }
}



    // ==== MVP event-level detection wiring (rules -> JSONL alerts) ====
    let mut evaluator = Evaluator::new(Thresholds {
        low: 0.30,
        medium: 0.55,
        high: 0.80,
    });

    if let Some(path) = resolve_rules_path() {
        if let Ok(hot) = HotRulesDetector::new(&path, Duration::from_secs(2)) {
            evaluator.registry.add(Box::new(hot));
        } else if let Err(e) = evaluator.registry.add_rules_from(&path) {
            eprintln!("⚠️ rules detector not loaded from {:?}: {e}", path);
        }
    } else {
        eprintln!("⚠️ rules.json not found in expected locations; continuing without hot rules");
    }

    evaluator.registry.add(Box::new(TagFlagsDetector));

    if let Err(e) = evaluator.set_output(Path::new("logs/alerts.jsonl")) {
        eprintln!("⚠️ alerts file not set: {e}");
    }

    // --- integrations state (load from disk if present) ---
    let integrations_map: HashMap<String, Value> = {
        let mut out = HashMap::new();
        if let Ok(s) = fs::read_to_string("state/integrations.json") {
            if let Ok(v) = serde_json::from_str::<Value>(&s) {
                if let Some(obj) = v.as_object() {
                    for (k, vv) in obj {
                        out.insert(k.to_string(), vv.clone());
                    }
                }
            }
        }
        out
    };

    let api_state = ApiState {
        tx: alert_tx.clone(),
        graph_tx: graph_tx.clone(),
        inc_tx: inc_tx.clone(),
        integrations: Arc::new(Mutex::new(integrations_map)),
    };

    // publisher: push graph JSON to SSE channel when file changes
    // publisher: push graph JSON to SSE channel when file changes (enriched)
    // publisher: push graph JSON to SSE channel when file changes (annotated with R-GCN overlay)
{
    let graph_tx = graph_tx.clone();
    tokio::spawn(async move {
        use std::fs;
        let mut tick = tokio::time::interval(std::time::Duration::from_millis(900));
        let mut last = String::new();
        loop {
            tick.tick().await;
            match fs::read_to_string("json_files/graph_live.json") {
                Ok(raw) if !raw.is_empty() => {
                    // Parse, normalize, annotate, re-emit
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) {
                        let v = normalize_and_annotate_graph(v);
                        if let Ok(out) = serde_json::to_string(&v) {
                            if out != last {
                                last = out.clone();
                                let _ = graph_tx.send(out);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    });
}





   // --- Build the PB router (separate state) and main app router ---
// --- Build the PB router (separate state) and main app router ---
let pb_router = Router::new()
    .route("/pending", get(pb_api::pending))
    .route("/hits", get(pb_api::hits))
    .route("/reload", post(pb_api::reload))
    .route("/stream", get(pb_api::stream))
    .with_state(pb_state.clone());

// ---------- Static UI (no-cache) ----------
let no_cache = SetResponseHeaderLayer::overriding(
    CACHE_CONTROL,
    HeaderValue::from_static("no-store, max-age=0, must-revalidate"),
);

// Serve UI explicitly for the main files, plus a directory fallback.
// Using a child Router lets us apply the no-cache layer cleanly.
let ui_router = Router::new()
    .route("/index.html", get_service(ServeFile::new("ui/index.html")))
    .route("/graph.html", get_service(ServeFile::new("ui/graph.html")))
    .route("/app.js",     get_service(ServeFile::new("ui/app.js")))
    .fallback_service(get_service(ServeDir::new("ui")))
    .layer(no_cache.clone());

// ---------- Main app ----------
// ---------- Main app ----------
let app = Router::new()
    // APIs
    .route("/alerts", get(alerts_sse))
    .route("/metrics", get(metrics_json))
    .route("/alerts/:id/explain", get(explain_alert))
    .route("/graph/:id", get(graph_for_endpoint))
    .route("/graph/live", get(graph_live))
    .route("/graph/stream", get(graph_sse))
    // Ontology
    .route("/ontology/status", get(ontology_status))
    .route("/ontology/reload", post(ontology_reload))
    .route("/ontology/validate", get(ontology_validate))
    // Health row
    .route("/hosts/self/health", get(host_self_health))
    // Incidents (Phase 2)
    .route("/incidents/stream", get(incidents_sse))
    .route("/incidents", get(|| async {
        let list = incidents().lock().unwrap().list();
        Json(list)
    }))
    .route("/incidents/:id", get(|AxPath(id): AxPath<String>| async move {
        if let Some(it) = incidents().lock().unwrap().get(&id) {
            Json(it).into_response()
        } else {
            (StatusCode::NOT_FOUND, "not found").into_response()
        }
    }))
    // Safe action stubs (Phase 2)
    .route("/actions/isolate_host", post(|Json(v): Json<Value>| async move {
        eprintln!("✳️ action:isolate_host {}", v);
        (StatusCode::ACCEPTED, Json(json!({"ok":true,"action":"isolate_host","echo":v})))
    }))
    .route("/actions/kill_proc", post(|Json(v): Json<Value>| async move {
        eprintln!("✳️ action:kill_proc {}", v);
        (StatusCode::ACCEPTED, Json(json!({"ok":true,"action":"kill_proc","echo":v})))
    }))
    .route("/actions/block_egress", post(|Json(v): Json<Value>| async move {
        eprintln!("✳️ action:block_egress {}", v);
        (StatusCode::ACCEPTED, Json(json!({"ok":true,"action":"block_egress","echo":v})))
    }))
    // Integrations
    .route("/integrations/catalog", get(integrations_catalog))
    .route("/integrations", get(integrations_list).post(integrations_add))
    .route("/integrations/:id", delete(integrations_delete))
    // IOC endpoints
    .route("/ioc/reload", post(ioc_reload_handler))
    .route("/ioc/stats", get(ioc_stats_handler))
    // PB endpoints (nested with separate state)
    .nest("/playbooks", pb_router)
    // Diag/health
    .route("/where", get(diag_where))
    .route("/__where", get(diag_where))
    .route("/healthz", get(healthz))
    // Static
    .route("/", get_service(ServeFile::new("ui/index.html")))
    .nest("/ui", ui_router)
    .nest_service("/json_files", get_service(ServeDir::new("json_files")))
    // Layers/state
    .with_state(api_state.clone())
    .layer(CorsLayer::permissive());


// Allow port override (EDR_PORT), default 8080
{
    let port: u16 = std::env::var("EDR_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
    tokio::spawn(async move {
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
        eprintln!("🌐 UI available at http://{}/  (dashboard) | http://{}/ui/graph.html (live graph)", addr, addr);
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
}



    // ==== startup of eBPF readers ====
    let use_legacy = std::env::var("EDR_USE_LEGACY_REALTIME")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    #[cfg(target_os = "linux")]
    {
        if use_legacy {
            start_realtime_monitors(writer.clone());
        } else {
            // Local readers (binary-side)
            net_flow_reader::start_net_flow_reader(writer.clone());
            crate::ebpf::wx_exec_reader::start_wx_exec_reader(writer.clone());
            crate::ebpf::syscall_reader::start_syscall_reader(writer.clone());

            if let Err(e) = file_access_reader::start() {
                eprintln!("⛔ file_access_reader disabled: {e}");
            } else {
                eprintln!("✅ file_access_reader started");
            }

            // NOTE: events_reader::start(...) is NOT called here because this
            // module does not expose a `start` function in your tree.
            // If you add one later, you can enable it here.
        }
    }

    if let Ok(mut cal) = calibrator().lock() {
        cal.flush();
    }

    // periodic writer flush (actually flushes to disk now)
    {
        let flush_writer = writer.clone();
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(Duration::from_secs(30));
            loop {
                tick.tick().await;
                if let Ok(locked) = flush_writer.lock() {
                    // call the real flush_to_disk; `flush()` is a no-op shim
                    locked.flush_to_disk();
                }
            }
        });
    }

    // metrics logger (optional)
    {
        let mut tick = tokio::time::interval(Duration::from_secs(60));
        tokio::spawn(async move {
            loop {
                tick.tick().await;
                let ev_in = EVENTS_IN.load(Ordering::Relaxed);
                let ev_acc = EVENTS_ACCEPTED.load(Ordering::Relaxed);
                let al = ALERTS_OUT.load(Ordering::Relaxed);
                let drops = BPF_DROPS.load(Ordering::Relaxed);
                eprintln!("📊 metrics: received={} accepted={} alerts_out={} bpf_drops={}", ev_in, ev_acc, al, drops);
            }
        });
    }

    // ======================= main loop =======================
    loop {
        let mut records: Vec<TelemetryRecord> = get_current_telemetry_snapshot(writer.clone());

        for output in run_sideeffect_monitors_and_collect() {
            println!("📥 Passive Signal: {:?}", output);
            match try_convert_output(output) {
                Ok(rec) => records.push(rec),
                Err(_) => println!("⚠️ Failed to convert TelemetryOutput → TelemetryRecord"),
            }
        }

        // Count received BEFORE hygiene so "events_in" reflects raw ingress
        let received_len = records.len();

        // Hygiene
        filter_records(&mut records);
        dedup_records(&mut records);

        // Count accepted AFTER hygiene
        let accepted_len = records.len();

        // bump counters (accepted will be bumped after persist)
        EVENTS_IN.fetch_add(received_len as u64, Ordering::Relaxed);

        if records.is_empty() {
            println!("⚠️  No telemetry records collected this cycle.");
        } else {
            // —— PCB snapshots
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
                }
                let (_mahalanobis_d2, _elliptic_margin) =
                    tag_batch_outliers_with_mahala_and_envelope(&mut records, &ep);
                tag_records_from_krim(&mut records);
            }

            // 6a) IOC enrichment: tags records + reflect hits into PB pending
            match ioc::tag_records(&mut records) {
                Ok(hits) => {
                    for h in hits {
                        pb_introspect::add_or_update_pending(pb_introspect::PendingEvidence {
                            id: h.id,                // stable id from ioc::Hit
                            playbook_id: "ioc_layer".to_string(),
                            ts: h.ts,
                            slot_id: h.slot,         // e.g., "artifact_ioc" / "binary_ioc"
                            fact: h.fact,            // e.g., "Net", "FileIO", "Exec"
                            summary: h.summary,      // short human string
                            tags: h.tags,            // ["IOC", "YARA", ...]
                            ttl_sec: 900,
                        });
                    }
                }
                Err(e) => eprintln!("⚠️ ioc::tag_records error: {e}"),
            }

            // 6b) PlaybookEngine: ingest this batch and emit pending/hits
            {
                let now = now_ts();
            if let Ok(mut eng) = pb_engine().lock() {
                // Delegate conversion of TelemetryRecord -> Facts and ingest+collect to pb_wiring
                let out = pb_wiring::ingest_batch(&mut *eng, &records, now);
                for p in out.pending {
                    pb_introspect::add_or_update_pending(p);
                }
                for h in out.hits {
                    pb_introspect::push_hit(h);
                }
            }}


            // ⬇️ Feed anomalies & structural tags to DecisionEngine before rules
            feed_decider_from_record_tags(&records, &alert_tx, &inc_tx);

            // === Run event-level rules on each record and emit JSONL alerts + SSE ===
            for rec in &records {
                let mut t = NormTelemetry::from(rec.clone());
                let (cat, evt) = categorize_record(rec);
                t.category = cat;
                t.event = evt;

                // ---- Live Graph ingest (fixed types: u32 + f64) ----
                {
                    let mut gsw = gs.write();

                    let pid_i  = rec.pid;
                    let ppid_i = rec.ppid;
                    let pid_u  = if pid_i < 0 { 0 } else { pid_i as u32 };
                    let ppid_u = if ppid_i < 0 { 0 } else { ppid_i as u32 };

                    let comm = if !rec.binary_path.is_empty() {
                        rec.binary_path.clone()
                    } else {
                        rec.command_line
                            .split_whitespace()
                            .next()
                            .unwrap_or("-")
                            .to_string()
                    };

                    let score_f64 = (rec.risk_score.unwrap_or(0) as f64) / 100.0_f64;
                    let conf_f64  = (0.1_f64 + score_f64).min(0.95_f64);

                    match (t.category.as_str(), t.event.as_str()) {
                        ("process", "exec") => {
                            gsw.ingest_exec(ppid_u, "-", pid_u, &comm, score_f64, conf_f64);
                        }
                        ("network", "connect") => {
                            let ip_port = rec
                                .command_line
                                .split_whitespace()
                                .find(|s| s.contains(':') && s.chars().any(|c| c.is_ascii_digit()))
                                .unwrap_or("-");
                            gsw.ingest_connect(pid_u, &comm, ip_port, score_f64, conf_f64);
                        }
                        ("file", "write") | ("file", "open") => {
                            let path = if !rec.cwd.is_empty() { rec.cwd.clone() } else { "-".into() };
                            let is_write = t.event == "write";
                            gsw.ingest_file_access(pid_u, &comm, &path, is_write, score_f64, conf_f64);
                        }
                        _ => {}
                    }
                }

                match evaluator.evaluate(&t) {
                    Ok(Some(alert)) => {
                        // Keep legacy alert path (Phase 2)
                        ALERTS_OUT.fetch_add(1, Ordering::Relaxed);
                        if let Ok(line) = serde_json::to_string(&alert) {
                            let _ = alert_tx.send(line.clone());

                            // Phase 2: feed incidents and broadcast
                            if let Ok(v) = serde_json::from_str::<Value>(&line) {
                                if let Some(out) = incidents().lock().unwrap().ingest_alert_json(&v) {
                                    let _ = inc_tx.send(out.to_string());
                                }

                                // ⬇️ also feed this as a Rules signal into DecisionEngine
                                let exe = v
                                    .pointer("/event/exe")
                                    .or_else(|| v.pointer("/event/binary_path"))
                                    .and_then(|x| x.as_str())
                                    .unwrap_or("-")
                                    .to_string();
                                let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
                                let user = v
                                    .pointer("/event/user")
                                    .and_then(|x| x.as_str())
                                    .unwrap_or("unknown")
                                    .to_string();
                                let cmd = v
                                    .pointer("/event/cmdline")
                                    .or_else(|| v.pointer("/event/command_line"))
                                    .and_then(|x| x.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let ts = v
                                    .get("ts")
                                    .or_else(|| v.get("time"))
                                    .or_else(|| v.get("timestamp"))
                                    .and_then(|x| x.as_i64())
                                    .unwrap_or(now_ts() as i64) as u64;
                                let score = v
                                    .get("risk").and_then(|x| x.as_f64())
                                    .or_else(|| v.get("score").and_then(|x| x.as_f64()))
                                    .unwrap_or(0.8)
                                    .clamp(0.0, 1.5) as f32;

                                let tags: Vec<String> = v.get("findings")
                                    .and_then(|x| x.as_array())
                                    .map(|arr| arr.iter()
                                         .filter_map(|f| f.get("label").and_then(|l| l.as_str()).map(|s| s.to_string()))
                                         .collect()
                                    ).unwrap_or_default();

                                let sig = Signal {
                                    detector: DetectorKind::Rules,
                                    score: score.min(1.0),
                                    ts,
                                    exe: Some(exe),
                                    user: Some(user),
                                    host: Some(host),
                                    cmd_prefix: Some(first_token(&cmd)),
                                    parent: None,
                                    signer: None,
                                    hash: None,
                                    pid: None,
                                    tags,
                                };
                                let outs = decision_engine().lock().unwrap().ingest(sig);
                                for a in outs {
                                    emit_final_incident(a, &alert_tx, &inc_tx);
                                }
                            }
                        }
                    }
                    Ok(None) => {}
                    Err(e) => eprintln!("⚠️ evaluator error: {e}"),
                }
            }

            // Trust engine fanout
            for rec in &records {
                println!("🟡 Raw Telemetry: {:?}", rec);
                let telemetry_data = TelemetryData::from_record(rec);
                let result: TrustResult = evaluate_and_dispatch_trust_score(&telemetry_data);
                println!("✅ Trust Result: {:?}", result);
                fanout_all(&result, &records, &telemetry_data);
            }
        }

        // Persist the batch, then bump accepted so metrics reflect on-disk state
        let accepted_len_u64 = accepted_len as u64;
        if let Ok(locked) = writer.clone().lock() {
            locked.append_batch(records);
            EVENTS_ACCEPTED.fetch_add(accepted_len_u64, Ordering::Relaxed);
        }
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

// --- Integrations minimal controller (catalog/list/add/delete) ---

#[derive(Serialize, Deserialize, Default)]
struct IntegrationCreate {
    id: String,
    name: String,
    creds: Value,
}

async fn integrations_catalog() -> impl IntoResponse {
    let path = Path::new("integrations/catalog.json");
    let body = fs::read_to_string(path).unwrap_or_else(|_| "{\"version\":1,\"vendors\":[]}".into());
    (StatusCode::OK, body)
}

async fn integrations_list(State(st): State<ApiState>) -> impl IntoResponse {
    let m = st.integrations.lock().unwrap();
    let items: Vec<Value> = m.values().cloned().collect();
    Json(json!({ "items": items }))
}

async fn integrations_add(
    State(st): State<ApiState>,
    Json(req): Json<IntegrationCreate>,
) -> impl IntoResponse {
    let mut m = st.integrations.lock().unwrap();
    m.insert(
        req.id.clone(),
        json!({
            "id": req.id,
            "name": req.name,
            "creds": req.creds,
            "status": "syncing",
            "last_ok": Value::Null
        }),
    );
    let _ = fs::create_dir_all("state");
    let _ = fs::write(
        "state/integrations.json",
        serde_json::to_vec_pretty(&*m).unwrap(),
    );
    StatusCode::ACCEPTED
}

async fn integrations_delete(
    State(st): State<ApiState>,
    AxPath(id): AxPath<String>,
) -> impl IntoResponse {
    let mut m = st.integrations.lock().unwrap();
    m.remove(&id);
    let _ = fs::write(
        "state/integrations.json",
        serde_json::to_vec_pretty(&*m).unwrap(),
    );
    StatusCode::NO_CONTENT
}

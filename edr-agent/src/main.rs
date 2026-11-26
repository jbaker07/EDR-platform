// main.rs (Phase 2 + Phase 3 Decision Engine integration)
// - Safer TS parsing (secs/ms)
// - Two-pass graph builder (no missing PPID edges)
// - Periodic flush actually hits disk
// - DEMO mode unchanged (now increments alerts_out)
// - Rules path fallback
// - /healthz (no token, only ok:true) + nodes_count/edges_count in graph response for easy jq
// - Metrics: events/events_in (received), events_accepted (post-filter), alerts_out
// - Relaxed acceptance filter: tag-implied events count as accepted
// - Compat aliases in /metrics: events_accepted_total, alerts_out_total
// - /graph/live + /graph/stream (SSE) + metrics include graph counts
// - Incidents manager + SSE + REST, action endpoints, BPF drops metric, EDR_PORT, Explain++
// - NEW (Phase 3): DecisionEngine (consensus + correlation + cooldown + allow/suppress) wired in
// - NEW (Phase PB): Playbook Engine + IOC wired (pending/hits, metrics, endpoints)

use anyhow::{anyhow, Result};
use axum::{
    extract::{Path as AxPath, Query, State},
    http::StatusCode,
    response::{
        sse::{Event, Sse},
        IntoResponse,
    },
    Json,
};
use chrono::{TimeZone, Utc};
use forensic_hooks::pb_engine::PlaybookEngine;
use forensic_hooks::{ioc, pb_adapter, pb_api, pb_introspect, pb_wiring};
use futures_util::stream::{unfold, Stream};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha1::{Digest, Sha1};
use std::collections::{HashMap, VecDeque};
use std::convert::Infallible;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use tokio::{
    net::TcpListener,
    sync::{broadcast, oneshot},
};

// ---------- metrics counters ----------
static ALERTS_OUT: AtomicU64 = AtomicU64::new(0);
static PUBLISH_WARN_TS: AtomicU64 = AtomicU64::new(0);
static PUBLISH_BUS_URL: Lazy<Option<String>> = Lazy::new(|| {
    if env::var("PUBLISH_BUS_DISABLED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return None;
    }
    match env::var("PUBLISH_BUS_URL") {
        Ok(url) if !url.trim().is_empty() => Some(url),
        _ => Some("http://127.0.0.1:8787/api/incidents".to_string()),
    }
});
static PUBLISH_BUS_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .expect("publish bus client")
});

// ---------- pipeline / telemetry ----------
use forensic_hooks::elliptic_envelope::EllipticEnvelope;
use forensic_hooks::episode::Episode;
use forensic_hooks::event::{Event as FHEvent, EventType};
use forensic_hooks::graph_builder::{GraphEdge, GraphNode};
use forensic_hooks::mahalanobis::{Mahala, MahalaCfg};
use forensic_hooks::pipeline::{Evaluator, Thresholds};
use forensic_hooks::services::krim_lite::analyze_graph_and_score;
use forensic_hooks::services::trust_engine_final::{TelemetryData, TrustResult};
use forensic_hooks::telemetry::TelemetryRecord;
use forensic_hooks::telemetry_types::TelemetryOutput;
use forensic_hooks::telemetry_writer::TelemetryWriter;
use forensic_hooks::utils::time::now_ts;
use fs2::FileExt;

#[cfg(target_os = "linux")]
pub mod ebpf; // <-- this tells Rust there’s a module at src/ebpf/mod.rs

#[cfg(target_os = "linux")]
use crate::ebpf::ebpf_ingest::{
    on_edr_event, on_edr_event_file_only, on_edr_event_net_only, on_edr_event_wx_only,
};
#[cfg(target_os = "linux")]
use crate::ebpf::events_reader;
#[cfg(target_os = "linux")]
use log::warn;

#[cfg(target_os = "linux")]
pub fn pins_available(prefix: &str) -> bool {
    // Accept if ANY known source exists
    let rb = ["edr_events_rb", "net_events", "wx_events"];
    let perf = ["events"]; // file-access perf pin
    rb.iter()
        .any(|n| Path::new(&format!("{prefix}/{n}")).exists())
        || perf
            .iter()
            .any(|n| Path::new(&format!("{prefix}/{n}")).exists())
}

#[cfg(target_os = "linux")]
fn pinned_mode_selected() -> (bool, String) {
    let prefix = std::env::var("EDR_PIN_PREFIX").unwrap_or_else(|_| "/sys/fs/bpf/edr".to_string());
    let force = std::env::var("EDR_FORCE_PINNED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let present = pins_available(&prefix);
    (force || present, prefix)
}

// === detectors come from the library crate ===
use forensic_hooks::detectors::rules_hot::HotRulesDetector;
use forensic_hooks::detectors::tag_flags::TagFlagsDetector;

// === add the reader modules directly ===
#[cfg(not(target_os = "linux"))]
#[path = "ebpf/events_reader.rs"]
mod events_reader; // kept for later; we don't call a missing start()

// === graph module at src/graph.rs ===
mod graph;
use crate::graph::{spawn_writer, GraphState};
use forensic_hooks::metrics::{self, Metrics};
use parking_lot::RwLock;

// === Decision Engine (Phase 3) ===
// The library's copy, so decision.rs is compiled once. crate::decision still resolves.
use forensic_hooks::decision;
use crate::decision::{
    AllowSuppressList, DecisionEngine, DecisionPolicy, DetectorKind, FinalAlert, Severity, Signal,
};

mod rgcn_bridge;
mod record_pipeline;
use crate::rgcn_bridge::{rgcn_metrics, start_rgcn_bridge};

// A globally shared live-graph state (optional, does not block UI graph endpoint).
static GRAPH_STATE: OnceLock<Arc<RwLock<GraphState>>> = OnceLock::new();
fn graph_state() -> Arc<RwLock<GraphState>> {
    GRAPH_STATE
        .get_or_init(|| Arc::new(RwLock::new(GraphState::new(300)))) // keep last 5 minutes
        .clone()
}

// ---- global detectors state ----
static MAHALA_ENGINE: OnceLock<Mutex<Mahala>> = OnceLock::new();
static ENV_BASELINE: OnceLock<Mutex<VecDeque<Vec<f64>>>> = OnceLock::new();
static ENV_MODEL: OnceLock<Mutex<Option<EllipticEnvelope>>> = OnceLock::new();
static DECODE_MISS: AtomicU64 = AtomicU64::new(0);

fn mahala_engine() -> &'static Mutex<Mahala> {
    MAHALA_ENGINE.get_or_init(|| Mutex::new(Mahala::default()))
}
fn env_baseline() -> &'static Mutex<VecDeque<Vec<f64>>> {
    ENV_BASELINE.get_or_init(|| Mutex::new(VecDeque::with_capacity(1024)))
}
fn env_model() -> &'static Mutex<Option<EllipticEnvelope>> {
    ENV_MODEL.get_or_init(|| Mutex::new(None))
}

// ---- Playbook Engine singleton ----
static PB_ENGINE: OnceLock<Arc<Mutex<PlaybookEngine>>> = OnceLock::new();
fn pb_engine() -> Arc<Mutex<PlaybookEngine>> {
    PB_ENGINE
        .get_or_init(|| Arc::new(Mutex::new(PlaybookEngine::empty())))
        .clone()
}

// ======================= Ontology: loader + enrichment =======================
#[derive(Clone, Default)]
struct OntologySet {
    core_ontology: serde_json::Value, // core_ontology.json
    inverse_edges: std::collections::HashMap<String, String>, // inverse_edges.json
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
        events_in: Some(metrics::events_in().load(Ordering::Relaxed)),
        alerts_out: Some(ALERTS_OUT.load(Ordering::Relaxed)),
        bpf_drops: Some(metrics::bpf_drops_total().load(Ordering::Relaxed)),
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
                    obj.insert(
                        "kind".into(),
                        serde_json::Value::String(inferred.to_string()),
                    );
                }
            }
        }
    }

    // Overlay R-GCN scores (no-op if bridge hasn’t scored yet)
    rgcn_bridge::annotate_graph(&mut v);

    v
}

fn load_json_if(path: &std::path::Path) -> Option<serde_json::Value> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
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
    let inv = load_json_if(&p("inverse_edges.json")).unwrap_or_else(|| json!({}));
    OntologySet {
        core_ontology: core,
        inverse_edges: map_from_obj(&inv),
        dir: dir.to_string(),
    }
}

fn candidates_ontology_dirs() -> Vec<String> {
    // Priority: explicit env var, then common workspace layouts
    let mut c = Vec::new();
    if let Ok(s) = std::env::var("ONTOLOGY_DIR") {
        if !s.trim().is_empty() {
            c.push(s);
        }
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
    if parts.len() != 4 {
        return false;
    }
    parts.iter().all(|p| p.parse::<u8>().is_ok())
}

fn infer_otype_from_node(n: &serde_json::Value) -> String {
    // Prefer explicit info in node fields
    if n.get("binary_path").and_then(|v| v.as_str()).is_some() {
        return "Process".into();
    }
    if n.get("pid").and_then(|v| v.as_u64()).is_some() {
        return "Process".into();
    }
    if n.get("command_line").and_then(|v| v.as_str()).is_some() {
        return "Process".into();
    }
    if let Some(id) = n.get("id").and_then(|v| v.as_str()) {
        if id.starts_with("proc:") {
            return "Process".into();
        }
        if id.starts_with("file:") {
            return "File".into();
        }
        if id.starts_with("user:") {
            return "User".into();
        }
        if looks_like_ipv4(id) {
            return "IP".into();
        }
        if id.contains('/') || id.contains('\\') {
            return "File".into();
        }
    }
    if let Some(k) = n.get("kind").and_then(|v| v.as_str()) {
        let k = k.to_ascii_lowercase();
        if k.contains("proc") {
            return "Process".into();
        }
        if k.contains("file") {
            return "File".into();
        }
        if k == "ip" || k == "net" {
            return "IP".into();
        }
        if k.contains("dns") {
            return "Domain".into();
        }
        if k.contains("user") {
            return "User".into();
        }
    }
    "Unknown".into()
}

fn default_rel_for(st: &str, tt: &str) -> String {
    match (st, tt) {
        ("Process", "Process") => "spawned",
        ("Process", "IP") => "connected-to",
        ("Process", "File") => "accessed",
        ("User", "Process") => "launched",
        ("File", "Process") => "executed-by",
        _ => "linked-to",
    }
    .to_string()
}

fn enrich_graph_json(raw: &serde_json::Value) -> serde_json::Value {
    let ont = ontology();
    let ont = ont.lock().unwrap().clone();

    let mut out = raw.clone();

    // Get mutable node/edge arrays (copy-on-write from raw)
    let nodes_in = raw
        .get("nodes")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let edges_in = raw
        .get("edges")
        .or_else(|| raw.get("links"))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    // 1) Enrich nodes: add otype and details
    let mut node_type_map: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut nodes_out: Vec<serde_json::Value> = Vec::with_capacity(nodes_in.len());

    for n in nodes_in {
        let mut nn = n.clone();
        let otype = nn
            .get("otype")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| infer_otype_from_node(&nn));
        if let Some(id) = nn.get("id").and_then(|v| v.as_str()) {
            node_type_map.insert(id.to_string(), otype.clone());
        }
        // attach otype
        nn.as_object_mut()
            .unwrap()
            .insert("otype".into(), json!(otype));

        // attach details (best-effort)
        let mut det = serde_json::Map::new();
        if let Some(v) = nn.get("pid").and_then(|x| x.as_i64()) {
            det.insert("pid".into(), json!(v));
        }
        if let Some(v) = nn.get("ppid").and_then(|x| x.as_u64()).map(|x| x as u32) {
            det.insert("ppid".into(), json!(v));
        }
        if let Some(v) = nn.get("binary_path").and_then(|x| x.as_str()) {
            det.insert("exe".into(), json!(v));
        }
        if let Some(v) = nn.get("command_line").and_then(|x| x.as_str()) {
            det.insert("cmdline".into(), json!(v));
        }
        if let Some(v) = nn.get("uid").and_then(|x| x.as_i64()) {
            det.insert("uid".into(), json!(v));
        }
        if !det.is_empty() {
            nn.as_object_mut()
                .unwrap()
                .insert("details".into(), serde_json::Value::Object(det));
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
        let st = node_type_map
            .get(sid)
            .cloned()
            .unwrap_or_else(|| "Unknown".into());
        let tt = node_type_map
            .get(tid)
            .cloned()
            .unwrap_or_else(|| "Unknown".into());

        // relation (keep provided, else infer)
        let rel = ee
            .get("rel")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| default_rel_for(&st, &tt));
        ee.as_object_mut()
            .unwrap()
            .insert("rel".into(), json!(rel.clone()));

        // inverse (if known)
        if let Some(inv) = ont.inverse_edges.get(&rel) {
            ee.as_object_mut()
                .unwrap()
                .insert("rel_inverse".into(), json!(inv));
        }

        // ontology validation (if relation exists under source type)
        if let Some(allowed) = ont
            .core_ontology
            .get(&st)
            .and_then(|t| t.get("edges"))
            .and_then(|m| m.get(&rel))
            .and_then(|arr| arr.as_array())
        {
            let allowed: Vec<String> = allowed
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            let ok = allowed.iter().any(|t| t == &tt);
            ee.as_object_mut()
                .unwrap()
                .insert("rel_valid".into(), json!(ok));
            if !ok {
                ee.as_object_mut()
                    .unwrap()
                    .insert("expected_targets".into(), json!(allowed));
                ee.as_object_mut().unwrap().insert(
                    "violation".into(),
                    json!(format!("edge '{}' not allowed: {} -> {}", rel, st, tt)),
                );
            }
        }

        edges_out.push(ee);
    }

    // 3) Recompose; also mirror edges to links for UI compatibility if needed
    out.as_object_mut()
        .unwrap()
        .insert("nodes".into(), json!(nodes_out));
    out.as_object_mut()
        .unwrap()
        .insert("edges".into(), json!(edges_out.clone()));
    if out.get("links").is_some() {
        out.as_object_mut()
            .unwrap()
            .insert("links".into(), json!(edges_out));
    }

    // lightweight counts for jq/health
    let n_cnt = out
        .get("nodes")
        .and_then(|x| x.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let e_cnt = out
        .get("edges")
        .and_then(|x| x.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    out.as_object_mut()
        .unwrap()
        .insert("nodes_count".into(), json!(n_cnt));
    out.as_object_mut()
        .unwrap()
        .insert("edges_count".into(), json!(e_cnt));

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
    Query(q): Query<std::collections::HashMap<String, String>>,
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
        serde_json::from_str::<serde_json::Value>(&s)
            .unwrap_or_else(|_| json!({"nodes":[],"edges":[]}))
    } else {
        json!({"nodes":[],"edges":[]})
    };
    let enriched = enrich_graph_json(&raw);

    let nodes = enriched
        .get("nodes")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let edges = enriched
        .get("edges")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut nodes_by_type: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for n in &nodes {
        let t = n
            .get("otype")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();
        *nodes_by_type.entry(t).or_insert(0) += 1;
    }

    let mut edges_with_rel = 0usize;
    let mut edges_valid = 0usize;
    let mut edges_invalid = 0usize;
    let mut samples_invalid: Vec<serde_json::Value> = vec![];

    for e in &edges {
        if e.get("rel").is_some() {
            edges_with_rel += 1;
        }
        if let Some(v) = e.get("rel_valid").and_then(|v| v.as_bool()) {
            if v {
                edges_valid += 1;
            } else {
                edges_invalid += 1;
                if samples_invalid.len() < 10 {
                    let mut slim = serde_json::Map::new();
                    for k in ["source", "target", "rel", "rel_inverse", "violation"].iter() {
                        if let Some(val) = e.get(*k) {
                            slim.insert((*k).to_string(), val.clone());
                        }
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

// ---- DecisionEngine singleton ----
static DECISION_ENGINE: OnceLock<Mutex<DecisionEngine>> = OnceLock::new();
fn decision_engine() -> &'static Mutex<DecisionEngine> {
    DECISION_ENGINE.get_or_init(|| {
        // Start conservative; we can hot-load JSON later
        let policy = DecisionPolicy::default();
        let allow = AllowSuppressList::default();
        Mutex::new(DecisionEngine::new(policy, allow))
    })
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
        Self {
            window_secs,
            items: HashMap::new(),
        }
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

    fn get(&self, id: &str) -> Option<Value> {
        self.items.get(id).cloned()
    }

    /// Adds shield proposals to an incident, newest version of each id kept.
    fn attach_proposals(&mut self, id: &str, proposals: Vec<Value>) -> Option<Value> {
        let item = self.items.get_mut(id)?;
        let obj = item.as_object_mut()?;
        let mut list: Vec<Value> = obj
            .get("shield_proposals")
            .and_then(|x| x.as_array())
            .cloned()
            .unwrap_or_default();
        for p in proposals {
            list.retain(|old| old.get("id") != p.get("id"));
            list.push(p);
        }
        if list.len() > 8 {
            let drop = list.len() - 8;
            list.drain(..drop);
        }
        obj.insert("shield_proposals".into(), json!(list));
        let item = item.clone();
        forensic_hooks::forwarder::queue_incident(&item);
        Some(item)
    }

    /// Ingest an alert JSON; returns updated incident JSON for SSE.
    fn ingest_alert_json(&mut self, alert: &Value) -> Option<Value> {
        let ts = alert
            .get("ts")
            .or_else(|| alert.get("time"))
            .or_else(|| alert.get("timestamp"))
            .and_then(|x| x.as_i64())
            .unwrap_or(now_ts() as i64);

        let exe = alert
            .pointer("/event/exe")
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

        let risk = alert
            .get("risk")
            .or_else(|| alert.get("score"))
            .and_then(|x| x.as_f64())
            .unwrap_or(0.0);

        let sev = alert
            .get("severity")
            .and_then(|x| x.as_str())
            .unwrap_or("none")
            .to_string();

        // A compact neighborhood graph of the processes seen 5 minutes either side. An
        // incident keeps its graph for GRAPH_REFRESH_SECS of alert time, so a rule that
        // matches thousands of times in a burst does not rebuild it thousands of times.
        const GRAPH_REFRESH_SECS: i64 = 30;
        let reuse = self.items.get(&id).and_then(|it| {
            let at = it.get("graph_ts").and_then(|x| x.as_i64())?;
            ((ts - at).abs() < GRAPH_REFRESH_SECS).then(|| {
                (
                    it.get("graph").cloned().unwrap_or(Value::Null),
                    it.get("rgcn_score").cloned().unwrap_or(Value::Null),
                    at,
                )
            })
        });
        let (graph, rgcn, graph_ts) = match reuse {
            Some(g) => g,
            None => {
                let recs = window_records(ts, 300);
                let (nodes, edges) = build_graph_from_records(&recs);
                let rgcn = rgcn_proxy_score(&nodes);
                (json!({ "nodes": nodes, "edges": edges }), json!(rgcn), ts)
            }
        };

        // Avoid moving `id` into the JSON so we can use it below
        let source = alert
            .get("source")
            .and_then(|x| x.as_str())
            .unwrap_or("rules")
            .to_string();
        // The alert's own title, a playbook's name, or the names of the rules that matched.
        let rule_names: Vec<String> = alert
            .get("findings")
            .and_then(|x| x.as_array())
            .map(|f| {
                f.iter()
                    .filter_map(|it| it.pointer("/details/name").and_then(|n| n.as_str()).map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        let title = alert
            .get("title")
            .or_else(|| alert.get("playbook_name"))
            .and_then(|x| x.as_str())
            .map(str::to_string)
            .or_else(|| (!rule_names.is_empty()).then(|| rule_names.join(", ")));
        let mut item = if let Some(existing) = self.items.remove(&id) {
            existing
        } else {
            json!({
                "id": id.clone(),
                "host": alert
                    .pointer("/event/host")
                    .and_then(|x| x.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into())),
                "title": title.clone().unwrap_or_else(|| format!("{primary_t} by {exe}")),
                "sources": [],
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
                "graph": graph,
                "graph_ts": graph_ts,
            })
        };

        // update fields
        {
            let obj = item.as_object_mut().unwrap();
            // times
            let first_ts = obj.get("first_ts").and_then(|x| x.as_i64()).unwrap_or(ts);
            let last_ts = obj.get("last_ts").and_then(|x| x.as_i64()).unwrap_or(ts);
            obj.insert("first_ts".into(), json!(first_ts.min(ts)));
            obj.insert("last_ts".into(), json!(last_ts.max(ts)));

            // counts
            let n = obj
                .get("alerts_count")
                .and_then(|x| x.as_u64())
                .unwrap_or(0);
            obj.insert("alerts_count".into(), json!(n + 1));

            // severity max (by rank)
            let cur = obj
                .get("severity_max")
                .and_then(|x| x.as_str())
                .unwrap_or("none");
            let new_max = if severity_rank(&sev) > severity_rank(cur) {
                sev.clone()
            } else {
                cur.to_string()
            };
            obj.insert("severity_max".into(), json!(new_max));

            // risk max
            let rmax = obj.get("risk_max").and_then(|x| x.as_f64()).unwrap_or(0.0);
            obj.insert("risk_max".into(), json!(rmax.max(risk)));

            // techniques merged
            let mut tch: Vec<String> = obj
                .get("techniques")
                .and_then(|x| x.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();
            for t in techniques {
                if !tch.iter().any(|x| x == &t) {
                    tch.push(t);
                }
            }
            obj.insert("techniques".into(), json!(tch));

            // keep a tiny sample of alert ids (if present)
            if let Some(aid) = alert.get("id").and_then(|x| x.as_str()) {
                let mut sids: Vec<String> = obj
                    .get("sample_ids")
                    .and_then(|x| x.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                if !sids.iter().any(|x| x == aid) {
                    if sids.len() >= 6 {
                        sids.remove(0);
                    }
                    sids.push(aid.to_string());
                }
                obj.insert("sample_ids".into(), json!(sids));
            }

            // Where the alerts came from (rules, playbook, guard, decision_engine), and
            // the newest alert's details block (the guard name, caller, target and
            // mode for a guard alert, the playbook for a playbook hit).
            let mut sources: Vec<String> = obj
                .get("sources")
                .and_then(|x| x.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
                .unwrap_or_default();
            if !sources.contains(&source) {
                sources.push(source.clone());
            }
            obj.insert("sources".into(), json!(sources));
            if let Some(d) = alert.get("details") {
                obj.insert("details".into(), d.clone());
            } else if let Some(pb) = alert.get("playbook_id") {
                obj.insert(
                    "details".into(),
                    json!({ "playbook_id": pb, "playbook_name": alert.get("playbook_name") }),
                );
            }
            if let Some(t) = &title {
                if source == "guard" || source == "playbook" {
                    obj.insert("title".into(), json!(t));
                }
            }

            obj.insert("rgcn_score".into(), rgcn);
            obj.insert("graph".into(), graph);
            obj.insert("graph_ts".into(), json!(graph_ts));
        }

        self.items.insert(id.clone(), item.clone());
        forensic_hooks::forwarder::queue_incident(&item);
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

// Response actions (src/response.rs), set up once from the signed policy.
static RESPONDER: OnceLock<forensic_hooks::response::Responder> = OnceLock::new();

/// Runs a response action on a blocking thread (it may start nft or iptables) and turns
/// the outcome into an HTTP response.
async fn run_action<F>(f: F) -> axum::response::Response
where
    F: FnOnce(&'static forensic_hooks::response::Responder) -> forensic_hooks::response::Outcome
        + Send
        + 'static,
{
    use axum::http::StatusCode;
    let Some(r) = RESPONDER.get() else {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"ok": false, "error": "response actions are not set up"}))).into_response();
    };
    match tokio::task::spawn_blocking(move || f(r)).await {
        Ok(o) => (
            StatusCode::from_u16(o.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(o.body),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"ok": false, "error": e.to_string()}))).into_response(),
    }
}

/// POST /shields: the exact document bytes and the hex Ed25519 signature over them.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ShieldPost {
    document: String,
    signature: String,
}

/// An optional {"reason": "..."} body on promote and delete.
fn shield_reason(body: &[u8]) -> String {
    serde_json::from_slice::<Value>(body)
        .ok()
        .and_then(|v| v.get("reason").and_then(|r| r.as_str()).map(str::to_string))
        .unwrap_or_default()
}

/// Runs a shield change on a blocking thread and turns the outcome into HTTP.
async fn run_shield<F>(f: F) -> axum::response::Response
where
    F: FnOnce(&'static std::sync::Arc<forensic_hooks::shield::ShieldManager>) -> forensic_hooks::response::Outcome
        + Send
        + 'static,
{
    use axum::http::StatusCode;
    let Some(m) = forensic_hooks::shield::manager() else {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"ok": false, "error": "shields are not set up"}))).into_response();
    };
    match tokio::task::spawn_blocking(move || f(m)).await {
        Ok(o) => (StatusCode::from_u16(o.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR), Json(o.body)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"ok": false, "error": e.to_string()}))).into_response(),
    }
}

/// A JSON body that does not parse is a 400 with the reason, not axum's plain text.
fn parse_body<T: serde::de::DeserializeOwned>(body: &[u8]) -> Result<T, axum::response::Response> {
    serde_json::from_slice(body).map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"ok": false, "error": format!("bad request body: {e}")})),
        )
            .into_response()
    })
}

// Global incidents store
static INCIDENTS: OnceLock<Arc<Mutex<IncidentsMgr>>> = OnceLock::new();
fn incidents() -> Arc<Mutex<IncidentsMgr>> {
    INCIDENTS
        .get_or_init(|| Arc::new(Mutex::new(IncidentsMgr::new(15 * 60))))
        .clone()
}

// ======================= API state & handlers =======================
#[derive(Clone)]
struct ApiState {
    tx: broadcast::Sender<String>,       // alerts SSE
    graph_tx: broadcast::Sender<String>, // graph SSE (whole snapshot JSON)
    inc_tx: broadcast::Sender<String>,   // incidents SSE
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

#[derive(Deserialize, Default)]
struct RecentIncidentsQuery {
    limit: Option<usize>,
}

async fn incidents_recent(Query(q): Query<RecentIncidentsQuery>) -> Json<Value> {
    let limit = q.limit.unwrap_or(50);
    let items = pb_introspect::get_hits(limit);
    Json(json!({ "items": items }))
}

// current graph JSON (from file): synthesize edges if missing, then annotate with R-GCN
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
            let n = v
                .get("nodes")
                .and_then(|x| x.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            let e = v
                .get("edges")
                .or_else(|| v.get("links"))
                .and_then(|x| x.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            return (n as u64, e as u64);
        }
    }
    (0, 0)
}
async fn metrics_json() -> Json<Value> {
    // fast-path: use atomics
    let ev_in = metrics::events_in().load(Ordering::Relaxed);
    let ev_acc = metrics::events_accepted().load(Ordering::Relaxed);
    let al = ALERTS_OUT.load(Ordering::Relaxed);

    // optional fallback counter derived from persisted telemetry rows
    let telemetry_rows_total = {
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
        total
    };
    let (nodes_count, edges_count) = read_graph_counts_from_file();
    let pbm = pb_introspect::metrics();
    let iocm = ioc::metrics();
    let adapter = pb_adapter::metrics_handle();
    let facts_in = adapter
        .as_ref()
        .map(|m| m.facts_in.load(Ordering::Relaxed))
        .unwrap_or(0);
    let mapper_dropped = adapter
        .as_ref()
        .map(|m| m.mapper_dropped_facts_total.load(Ordering::Relaxed))
        .unwrap_or(0);
    let playbooks_fired_total = pb_introspect::playbooks_fired_total();

    // Build the existing metrics payload first
    log::debug!(
        "/metrics live: events_in={} accepted={} rows={}",
        ev_in,
        ev_acc,
        telemetry_rows_total
    );

    let base = json!({
        // canonical keys
        "events_in": ev_in,                 // received (pre-filter)
        "events_accepted": ev_acc,          // accepted (post-filter, persisted)
        "alerts_out": al,
        "nodes_count": nodes_count,
        "edges_count": edges_count,
        "bpf_drops_total": metrics::bpf_drops_total().load(Ordering::Relaxed),
        "decode_miss_total": DECODE_MISS.load(Ordering::Relaxed),
        "facts_in": facts_in,
        "mapper_dropped_facts_total": mapper_dropped,
        "playbooks_fired_total": playbooks_fired_total,
        "telemetry_rows_total": telemetry_rows_total,

        // adapter compatibility aliases
        "adapter_facts_in_total": facts_in,
        "adapter_mapper_dropped_facts_total": mapper_dropped,

        // compatibility aliases
        "events": ev_in,
        "events_accepted_total": ev_acc,
        "alerts_out_total": al,

        // PB/IOC gauges
        "adapter_queue_depth": pb_adapter::queue_stats().map(|(q, _)| q.depth).unwrap_or(0),
        "adapter_queue_high_water": pb_adapter::queue_stats().map(|(q, _)| q.high_water).unwrap_or(0),
        "adapter_queue_waited_total": pb_adapter::queue_stats().map(|(q, _)| q.waited_total).unwrap_or(0),
        "adapter_queue_dropped_total": pb_adapter::queue_stats().map(|(q, _)| q.dropped_total).unwrap_or(0),
        "redactions_total": forensic_hooks::redact::redactions_total(),
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

/// GET /pipeline/status
async fn pipeline_status() -> Json<Value> {
    let (queue, skipped) = match pb_adapter::queue_stats() {
        Some((q, s)) => (serde_json::to_value(q).unwrap_or(Value::Null), s),
        None => (Value::Null, 0),
    };
    let (mut loaded, sizes): (Vec<String>, (usize, usize, usize)) = pb_engine()
        .lock()
        .map(|e| (e.playbooks.keys().cloned().collect(), e.sizes()))
        .unwrap_or_default();
    loaded.sort();
    Json(json!({
        "adapter_queue": queue,
        "adapter_skipped_outputs_total": skipped,
        "adapter_dropped_facts_total": metrics::mapper_dropped_facts_total().load(Ordering::Relaxed),
        "record_queue": record_pipeline::queue_stats(),
        "records_dropped_total": record_pipeline::queue_stats().map(|q| q.dropped_total).unwrap_or(0),
        "record_pipeline": record_pipeline::stats().json(),
        "facts_in": metrics::facts_in().load(Ordering::Relaxed),
        "playbooks_loaded": loaded.len(),
        "engine": { "counting_windows": sizes.0, "events_in_windows": sizes.1, "hypotheses": sizes.2 },
        "playbooks": loaded,
        "redactions_total": forensic_hooks::redact::redactions_total(),
        "exec_from_kernel_copy_total": forensic_hooks::pb_adapter::exec_from_kernel_copy_total(),
        "exec_stale_proc_total": forensic_hooks::pb_adapter::exec_stale_proc_total(),
        "forwarder": forensic_hooks::forwarder::stats(),
        "ground_truth": {
            "written_total": pb_wiring::ground_truth_stats().0,
            "failed_total": pb_wiring::ground_truth_stats().1,
        },
    }))
}

// /healthz is the one route without a token, so it says only that the API is up.
// The counters it used to carry are on /metrics, behind the token.
async fn healthz() -> impl IntoResponse {
    Json(json!({ "ok": true }))
}

// --- IOC endpoints (simple, stateless handlers) ---
async fn ioc_reload_handler() -> impl IntoResponse {
    match ioc::reload() {
        Ok(_) => (StatusCode::OK, Json(json!({"ok": true}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"ok": false, "error": e.to_string()})),
        ),
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
    let cwd = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "<err>".into());

    let ui_dir = "ui";
    let jf_dir = "json_files";

    let ui_abs = fs::canonicalize(ui_dir)
        .ok()
        .map(|p| p.display().to_string());
    let jf_abs = fs::canonicalize(jf_dir)
        .ok()
        .map(|p| p.display().to_string());

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
    v.get("timestamp")
        .and_then(|x| x.as_i64())
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
                        binary_path: v
                            .get("exe")
                            .and_then(|x| x.as_str())
                            .unwrap_or_default()
                            .to_string(),
                        command_line: v
                            .get("cmdline")
                            .and_then(|x| x.as_str())
                            .unwrap_or_default()
                            .to_string(),
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

/// The newest process snapshots (time, pid, exe), the same ones the record thread
/// appends to state/pcb_snaps.ndjson. Incidents take their neighborhood graph from here.
/// Reading the file instead, for every alert, cost a full read of a file that only grows
/// plus 20,000 JSON parses per alert, and stalled the record thread in a live burst.
const RECENT_PCB_CAP: usize = 20_000;

fn recent_pcb() -> &'static Mutex<VecDeque<(i64, i32, String)>> {
    static R: OnceLock<Mutex<VecDeque<(i64, i32, String)>>> = OnceLock::new();
    R.get_or_init(|| Mutex::new(VecDeque::with_capacity(RECENT_PCB_CAP)))
}

/// Keep the newest snapshots for window_records.
fn remember_pcb(snaps: &[forensic_hooks::pcb::PcbSnapshot]) {
    if let Ok(mut q) = recent_pcb().lock() {
        for s in snaps {
            if q.len() >= RECENT_PCB_CAP {
                q.pop_front();
            }
            q.push_back(((s.ts_ms / 1000) as i64, s.pid, s.exe.clone().unwrap_or_default()));
        }
    }
}

/// Processes seen within `secs` of `ts_center`, from the newest snapshots.
fn window_records(ts_center: i64, secs: i64) -> Vec<TelemetryRecord> {
    let Ok(q) = recent_pcb().lock() else {
        return Vec::new();
    };
    q.iter()
        .filter(|(ts, _, _)| (ts - ts_center).abs() <= secs)
        .map(|(ts, pid, exe)| TelemetryRecord {
            timestamp: *ts as u64,
            pid: *pid,
            ppid: 0,
            uid: 0,
            binary_path: exe.clone(),
            command_line: String::new(),
            cwd: String::new(),
            env_vars: None,
            tags: vec![],
            risk_score: Some(0),
        })
        .collect()
}

fn rgcn_proxy_score(nodes: &[GraphNode]) -> f64 {
    if nodes.is_empty() {
        return 0.0;
    }
    let mut sum = 0.0;
    for n in nodes {
        let trust = n.trust_score.max(0.0).min(1.0) as f64;
        sum += 1.0 - trust;
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
            // A rules.json finding is labeled with the rule id and carries its
            // techniques in details.mitre.
            if let Some(m) = it.pointer("/details/mitre").and_then(|x| x.as_array()) {
                out.extend(m.iter().filter_map(|t| t.as_str()).filter(|t| t.starts_with('T')).map(str::to_string));
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
    fn default() -> Self {
        Verdict::Benign
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Verdict {
    Benign,
    Leaning,
    Malicious,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ExplainSummary {
    verdict: Verdict,
    confidence: f32,           // 0.0 .. 1.0
    reason_codes: Vec<String>, // short machine-readable reasons
}

fn pick_verdict_from_stats(avg_trust: f64, sev: &str, rgcn_avg_risk: f64) -> ExplainSummary {
    let mut reasons = vec![];
    let sev_l = sev.to_ascii_lowercase();
    if avg_trust < 0.6 {
        reasons.push("trust_low".into());
    } else if avg_trust < 0.85 {
        reasons.push("trust_medium".into());
    }
    if rgcn_avg_risk >= 0.8 {
        reasons.push("rgcn_high".into());
    }
    if sev_l == "high" {
        reasons.push("sev_high".into());
    }
    if sev_l == "medium" {
        reasons.push("sev_medium".into());
    }

    let (verdict, confidence) = if avg_trust < 0.6 || sev_l == "high" || rgcn_avg_risk >= 0.9 {
        (Verdict::Malicious, 0.9)
    } else if avg_trust < 0.85 || sev_l == "medium" || rgcn_avg_risk >= 0.75 {
        (Verdict::Leaning, 0.7)
    } else {
        (Verdict::Benign, 0.85)
    };

    ExplainSummary {
        verdict,
        confidence,
        reason_codes: reasons,
    }
}

async fn explain_alert(AxPath(alert_id): AxPath<String>) -> impl IntoResponse {
    let mut alert = if let Some(v) = load_alert_by_id(&alert_id) {
        v
    } else {
        return (StatusCode::NOT_FOUND, "alert not found").into_response();
    };
    if alert.get("id").is_none() {
        let sid = synth_id(&alert);
        alert
            .as_object_mut()
            .unwrap()
            .insert("id".into(), Value::String(sid));
    }

    let sev = alert
        .get("severity")
        .and_then(|x| x.as_str())
        .unwrap_or("none")
        .to_string();
    let risk = alert
        .get("risk")
        .or_else(|| alert.get("score"))
        .and_then(|x| x.as_f64())
        .unwrap_or(0.0);
    let ts = alert
        .get("ts")
        .or_else(|| alert.get("time"))
        .or_else(|| alert.get("timestamp"))
        .and_then(|x| x.as_i64())
        .unwrap_or(now_ts() as i64);

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
    let sha = alert
        .get("sha256")
        .and_then(|x| x.as_str())
        .or_else(|| alert.pointer("/event/sha256").and_then(|x| x.as_str()));
    let enrichment = load_enrichment(ip, sha);

    let reasons = alert.get("reasons").cloned().unwrap_or_else(|| {
        let mut rs = Vec::new();
        if let Some(arr) = findings.as_array() {
            for f in arr {
                let src = f
                    .get("source")
                    .and_then(|x| x.as_str())
                    .unwrap_or("detector");
                let sc = f.get("score").and_then(|x| x.as_f64()).unwrap_or(0.0);
                rs.push(json!({"source": src, "weight": sc}));
            }
        }
        json!(rs)
    });

    let technique_names: Vec<String> = mitre
        .clone()
        .into_iter()
        .map(|t| match t.as_str() {
            "T1055" => "Process Injection".into(),
            "T1059.001" => "Command & Scripting: PowerShell".into(),
            "T1547.001" => "Boot/Logon Autostart: Run Keys".into(),
            _ => t,
        })
        .collect();

    let confidence = alert
        .get("confidence")
        .and_then(|x| x.as_f64())
        .unwrap_or_else(|| rgcn.max(0.0).min(1.0));

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
        actions.push(
            "Isolate host; capture memory; hunt for unsigned DLLs injected into signed parents."
                .into(),
        );
    }
    if mitre.iter().any(|t| t.starts_with("T1059.001")) {
        actions.push(
            "Block outbound for the user; review PowerShell transcript logs and AMSI events."
                .into(),
        );
    }
    if mitre.iter().any(|t| t.starts_with("T1547.001")) {
        actions.push(
            "Check autorun keys and Startup folder; verify owner and signer of binaries.".into(),
        );
    }
    if actions.is_empty() {
        actions.push(
            "Triage process tree, parent-child chain, and recent network egress for this endpoint."
                .into(),
        );
    }
    actions
}

// ======================= Utilities / mapping =======================

fn try_convert_output(output: TelemetryOutput) -> Result<TelemetryRecord, String> {
    pb_adapter::emit_adapter_facts(&output);

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
    // TEMP: very permissive accept, keep if we have any signal at all
    records.retain(|r| {
        let has_proc = !r.binary_path.is_empty() || !r.command_line.is_empty();
        let has_id = r.pid != 0 || r.ppid != 0 || r.uid != 0;
        let has_tags = !r.tags.is_empty();
        has_proc || has_id || has_tags
    });
}

fn categorize_record(rec: &TelemetryRecord) -> (String, String) {
    // Decoder tags first. The file and network decoders also fill binary_path,
    // and populate_proc_fields fills command_line for any live pid, so the
    // path check below would label those records process/exec.
    let tag = |t: &str| rec.tags.iter().any(|x| x == t);
    let tag_prefix = |p: &str| rec.tags.iter().any(|x| x.starts_with(p));
    if tag("file_access") || tag("file_open") {
        return ("file".into(), "open".into());
    }
    if tag_prefix("file_") || (tag("file") && tag("write")) {
        return ("file".into(), "write".into());
    }
    if tag("tcp_state") || tag_prefix("net_") || tag_prefix("network::") {
        return ("network".into(), "connect".into());
    }
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
                binary_path: if r.binary_path.is_empty() {
                    None
                } else {
                    Some(r.binary_path.clone())
                },
                command_line: if r.command_line.is_empty() {
                    None
                } else {
                    Some(r.command_line.clone())
                },
                cwd: if r.cwd.is_empty() {
                    None
                } else {
                    Some(r.cwd.clone())
                },
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
                nodes.push(GraphNode {
                    id: file_id.clone(),
                    ..Default::default()
                });
            }
            if seen_edges.insert((nid.clone(), file_id.clone())) {
                edges.push(GraphEdge {
                    source: nid.clone(),
                    target: file_id,
                    ..Default::default()
                });
            }
        }
        if !r.cwd.is_empty() {
            let dir_id = format!("file:{}", r.cwd);
            if seen_nodes.insert(dir_id.clone()) {
                nodes.push(GraphNode {
                    id: dir_id.clone(),
                    ..Default::default()
                });
            }
            if seen_edges.insert((nid.clone(), dir_id.clone())) {
                edges.push(GraphEdge {
                    source: nid.clone(),
                    target: dir_id,
                    ..Default::default()
                });
            }
        }
    }
    (nodes, edges)
}

// ---- anomaly taggers ----
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
    let mahal_diag: Option<f64>;
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
    // The chunk is the outlier, not each record in it, so the tag goes on one record
    // (the newest). Tagging every record made the tag_flags rule raise one alert per
    // record: 11,103 rule hits and about 2,300 incidents in one live burst.
    if (d2 as f64) > thr && support >= MahalaCfg::d_support() {
        if let Some(r) = records.last_mut() {
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
                if let Some(r) = records.last_mut() {
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

/// EDR_VERBOSE=1 (or RUST_LOG with telemetry=trace) prints every record and trust
/// result. Off by default: at burst rates the three lines per record wrote about 1 GB of
/// log in 10 minutes and slowed the batch loop to 11 s a pass.
fn verbose() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| {
        std::env::var("EDR_VERBOSE").map(|v| v == "1" || v.eq_ignore_ascii_case("true")).unwrap_or(false)
            || std::env::var("RUST_LOG").map(|v| v.contains("telemetry=trace")).unwrap_or(false)
    })
}

/// The host name incidents and signals carry: HOSTNAME, else "localhost".
fn agent_host() -> String {
    static H: OnceLock<String> = OnceLock::new();
    H.get_or_init(|| std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into()))
        .clone()
}

/// A rules.json rule matched: send the alert, make it a RuleHit fact for the playbooks,
/// an incident, and a DecisionEngine signal.
fn handle_rule_alert(
    v: &Value,
    host: &str,
    at: &broadcast::Sender<String>,
    it: &broadcast::Sender<String>,
) {
    let _ = at.send(v.to_string());
    // The rule hit is also a RuleHit fact for the playbooks (edr_tamper reads T1562 hits).
    pb_adapter::emit_rule_hit(v);
    if let Some(out) = incidents().lock().unwrap().ingest_alert_json(v) {
        let _ = it.send(out.to_string());
    }
    let text = |ptrs: &[&str]| {
        ptrs.iter()
            .find_map(|p| v.pointer(p).and_then(|x| x.as_str()))
            .map(str::to_string)
    };
    let exe = text(&["/event/exe", "/event/binary_path"]).unwrap_or_else(|| "-".into());
    let user = text(&["/event/user"]).unwrap_or_else(|| "unknown".into());
    let cmd = text(&["/event/cmdline", "/event/command_line"]).unwrap_or_default();
    let ts = v
        .get("ts")
        .or_else(|| v.get("time"))
        .or_else(|| v.get("timestamp"))
        .and_then(|x| x.as_i64())
        .unwrap_or(now_ts() as i64) as u64;
    let score = v
        .get("risk")
        .and_then(|x| x.as_f64())
        .or_else(|| v.get("score").and_then(|x| x.as_f64()))
        .unwrap_or(0.8)
        .clamp(0.0, 1.5) as f32;
    let tags: Vec<String> = v
        .get("findings")
        .and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|f| f.get("label").and_then(|l| l.as_str()).map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    let sig = Signal {
        detector: DetectorKind::Rules,
        score: score.min(1.0),
        ts,
        exe: Some(exe),
        user: Some(user),
        host: Some(host.to_string()),
        cmd_prefix: Some(first_token(&cmd)),
        parent: None,
        signer: None,
        hash: None,
        pid: None,
        tags,
    };
    let outs = decision_engine().lock().unwrap().ingest(sig);
    for a in outs {
        emit_final_incident(a, at, it);
    }
}

// ---- trust result line ----
// Prints one line per trust result with EDR_VERBOSE=1. The agent has no SIEM sinks.
fn fanout_all(res: &TrustResult, records: &[TelemetryRecord], _data: &TelemetryData) {
    if !verbose() {
        return;
    }
    println!(
        "📣 [{}] score={:.2} escalated={} reasons={:?} batch_recs={}",
        res.endpoint_id,
        res.score,
        res.escalated,
        res.reasons,
        records.len()
    );
}

// ---- small helpers ----
fn resolve_rules_path() -> Option<PathBuf> {
    let candidates = ["edr-agent/src/rules.json", "src/rules.json", "rules.json"];
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

    let correlation_group_id = format!("{}::{}::{}", a.host, a.user, a.exe);

    json!({
        "id": format!("da:{}:{}:{}", a.exe, a.user, a.ts),
        "ts": a.ts as i64,
        "severity": sev_to_str(a.severity),
        "risk": (a.risk as f64),
        "support": (a.support as f64),
        "correlation_group_id": correlation_group_id,
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

fn publish_bus_log_warning(message: String) {
    let now = Utc::now().timestamp().max(0) as u64;
    let last = PUBLISH_WARN_TS.load(Ordering::Relaxed);
    if now.saturating_sub(last) >= 60 {
        PUBLISH_WARN_TS.store(now, Ordering::Relaxed);
        log::warn!("{}", message);
    } else {
        log::debug!("{}", message);
    }
}

fn queue_publish_bus(payload: Value) {
    if let Some(url) = PUBLISH_BUS_URL.as_ref() {
        let client = Client::clone(&PUBLISH_BUS_CLIENT);
        let url = url.clone();
        tokio::spawn(async move {
            match client.post(&url).json(&payload).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        let status = resp.status();
                        let body = resp.text().await.unwrap_or_default();
                        publish_bus_log_warning(format!(
                            "publish bus responded with status {} and body {}",
                            status,
                            body
                        ));
                    }
                }
                Err(err) => {
                    publish_bus_log_warning(format!(
                        "failed to post incident to publish bus: {}",
                        err
                    ));
                }
            }
        });
    }
}

fn emit_final_incident(
    a: FinalAlert,
    alert_tx: &broadcast::Sender<String>,
    inc_tx: &broadcast::Sender<String>,
) {
    let v = final_alert_to_json(&a);
    let line = v.to_string();

    queue_publish_bus(v.clone());

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
        filled_slots: vec!["consensus".to_string()], // minimal shim until PB engine feeds real slots
        missing_slots: vec![],
        tags: v
            .get("tags")
            .and_then(|t| t.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(|s| s.to_string()))
                    .collect()
            })
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
async fn main() -> Result<()> {
    let lock_path = "/var/run/forensic_hooks.lock";
    let _lock_file = {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(lock_path)
            .expect("lock file open failed");
        if let Err(_) = file.try_lock_exclusive() {
            eprintln!("agent already running (lock {})", lock_path);
            std::process::exit(1);
        }
        file
    };

    {
        let cwd = std::env::current_dir().unwrap();
        eprintln!("🗂️  cwd        : {}", cwd.display());
        eprintln!(
            "🗂️  ui         : {}",
            std::fs::canonicalize("ui")
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "MISSING".into())
        );
        eprintln!(
            "🗂️  json_files : {}",
            std::fs::canonicalize("json_files")
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "MISSING".into())
        );
        eprintln!(
            "🗎   ui/index.html present? {}",
            std::path::Path::new("ui/index.html").exists()
        );
        eprintln!(
            "🗎   ui/app.js     present? {}",
            std::path::Path::new("ui/app.js").exists()
        );
        eprintln!(
            "🗎   ui/graph.html present? {}",
            std::path::Path::new("ui/graph.html").exists()
        );
    }

    // A policy with a bad or missing signature stops the agent here, before any
    // sensor or guard starts. EDR_DEV_NO_AUTH=1 turns this into a warning. A policy
    // with a serial under the last one applied is refused too, and the copy kept at
    // that apply is used instead (anti-rollback across restarts).
    let policy_path = PathBuf::from("policy.json");
    let pubkey_path = forensic_hooks::config::pubkey_path();
    let policy_store = forensic_hooks::config::PolicyStore::new(&forensic_hooks::config::state_dir());
    let loaded_policy = match forensic_hooks::config::load_with_floor(
        &policy_path,
        &pubkey_path,
        &policy_store,
        forensic_hooks::agent_api::dev_no_auth(),
    ) {
        Ok((p, notes)) => {
            for n in notes {
                eprintln!("⚠️ policy: {n}");
            }
            p
        }
        Err(e) => {
            eprintln!("❌ {e:#}");
            eprintln!("   sign it with scripts/policy_signer.py, or set EDR_DEV_NO_AUTH=1 on a dev machine");
            std::process::exit(1);
        }
    };
    if let Err(e) = policy_store.record(&loaded_policy) {
        eprintln!("⚠️ policy: could not record serial {} in {}: {e}", loaded_policy.policy.serial, policy_store.serial_path().display());
    }
    forensic_hooks::policy_watch::started(&loaded_policy);
    let policy = loaded_policy.policy.clone();
    // The signed policy's settings, each enforced below: the log level (when RUST_LOG
    // is unset), command line masking, the lowest guard mode, the response actions and
    // the ports isolation keeps open.
    let _ = env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(policy.logging.level.as_str()),
    )
    .try_init();
    forensic_hooks::redact::init(policy.redact_command_lines);
    eprintln!(
        "policy: serial {}, guard_mode at least {}, redact_command_lines {}, response_actions {:?}, management_ports {:?}, shields max_active {} max_lifetime_sec {}",
        policy.serial, policy.guard_mode, policy.redact_command_lines, policy.response_actions, policy.management_ports,
        policy.shields.max_active, policy.shields.max_lifetime_sec
    );
    // Prevention mode (BPF LSM). Falls back to detect-only when the kernel lacks it.
    let prevention_guard = {
        let (settings, warnings) = forensic_hooks::guard::settings_for_policy(&policy);
        for w in warnings {
            eprintln!("⚠️ guard: {w}");
            log::warn!("guard: {w}");
        }
        forensic_hooks::guard::start_guard_with(settings, |_, _| {}).ok()
    };

    // Incident forwarding to the backend (EDR_BACKEND_URL). A bad setting is logged and
    // the agent runs without it.
    let forward_cfg = match forensic_hooks::forwarder::config_from(|k| std::env::var(k).ok()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ incident forwarding off: {e}");
            None
        }
    };
    let api_port: u16 = std::env::var("EDR_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    let keep_ports_for = move |p: &forensic_hooks::config::RawPolicy| {
        let mut keep_ports = p.management_ports.clone();
        if !keep_ports.contains(&api_port) {
            keep_ports.push(api_port);
        }
        keep_ports
    };
    let backend_ips: Vec<std::net::IpAddr> = forward_cfg
        .as_ref()
        .map(|c| c.resolved().into_iter().map(|(ip, _)| ip).collect())
        .unwrap_or_default();
    {
        let keep_ports = keep_ports_for(&policy);
        let backend = forward_cfg.as_ref().map(|c| c.resolved()).unwrap_or_default();
        let cfg = forensic_hooks::response::ResponseConfig::from_env(
            keep_ports,
            backend,
            policy.response_actions.clone(),
        );
        let _ = RESPONDER.set(forensic_hooks::response::Responder::new(
            cfg,
            Box::new(forensic_hooks::response::SystemRunner),
            Path::new("/proc"),
            std::process::id() as i64,
        ));
    }

    // Runtime shields (src/shield). They need the BPF LSM like the guard. Without it
    // the shield API answers but refuses to apply anything.
    let shield_thread = start_shields(&policy, &pubkey_path, &policy_store, prevention_guard.as_ref(), &backend_ips);

    // Live policy reload: a newer signed policy applies to the running agent.
    {
        let guard_control = prevention_guard.as_ref().map(|g| g.control.clone());
        let backend_ips = backend_ips.clone();
        let own = own_files(&pubkey_path, &policy_store);
        let apply: forensic_hooks::policy_watch::ApplyFn = Box::new(move |lp| {
            let p = &lp.policy;
            let mut notes = Vec::new();
            if std::env::var("RUST_LOG").is_err() {
                if let Ok(level) = p.logging.level.parse::<log::LevelFilter>() {
                    log::set_max_level(level);
                }
            }
            forensic_hooks::redact::policy_requires(p.redact_command_lines);
            if let Some(control) = &guard_control {
                let (settings, warnings) = forensic_hooks::guard::settings_for_policy(p);
                notes.extend(warnings);
                let (socket, memfd) = settings.allowlists();
                match control.apply(settings.modes, &socket, &memfd) {
                    Ok(m) => notes.push(format!(
                        "guard modes runtime_socket={} memfd_exec={} self_protect={}, allowlists {} socket clients and {} memfd programs, one bank switch",
                        m.socket.as_str(), m.memfd.as_str(), m.self_protect.as_str(), socket.len(), memfd.len()
                    )),
                    Err(e) => notes.push(format!("guard maps not updated: {e:#}")),
                }
            }
            if let Some(m) = forensic_hooks::shield::manager() {
                let never = forensic_hooks::shield::NeverBlock::build(&p.shields.never_block, &own, &backend_ips);
                notes.extend(m.apply_policy(p.shields.clone(), never));
            }
            if let Some(r) = RESPONDER.get() {
                r.set_policy(p.response_actions.clone(), keep_ports_for(p));
            }
            notes
        });
        let pem = std::fs::read(&pubkey_path).unwrap_or_default();
        let watch = forensic_hooks::policy_watch::Watch {
            policy_path: policy_path.clone(),
            pubkey_pem: pem,
            store: forensic_hooks::config::PolicyStore::new(&forensic_hooks::config::state_dir()),
            audit_path: PathBuf::from(std::env::var("EDR_POLICY_AUDIT").unwrap_or_else(|_| "logs/policy_audit.jsonl".into())),
            allow_unsigned: forensic_hooks::agent_api::dev_no_auth(),
            every: Duration::from_secs(1),
        };
        if let Err(e) = watch.spawn(loaded_policy.clone(), apply) {
            eprintln!("❌ policy watch did not start: {e}");
        }
    }

    // Expired shields and expired firewall blocks leave the agent's state once a second.
    // The kernel already stopped enforcing them on time.
    std::thread::Builder::new()
        .name("expiry_sweep".into())
        .spawn(|| loop {
            std::thread::sleep(Duration::from_secs(1));
            if let Some(m) = forensic_hooks::shield::manager() {
                m.sweep();
            }
            if let Some(r) = RESPONDER.get() {
                r.sweep();
            }
        })
        .ok();

    let writer = Arc::new(Mutex::new(TelemetryWriter::new()));

    let metrics = Arc::new(Metrics::new());
    metrics.reset_on_startup();
    metrics::init(metrics.clone());
    metrics::events_in().store(0, Ordering::Relaxed);
    pb_adapter::set_metrics_handle(metrics.clone());

    pb_adapter::set_engine_handle(pb_engine());

    // Decoders push facts into a bounded queue. One OS thread drains it in batches into
    // the playbook engine. A full queue makes the producer wait (see pb_adapter/queue.rs).
    let fact_queue = pb_adapter::init_queue();
    if let Err(e) = pb_adapter::runner::spawn_adapter_thread(fact_queue) {
        eprintln!("❌ playbook engine thread did not start: {e}");
    }
    if let Some(cfg) = forward_cfg.clone() {
        tokio::spawn(forensic_hooks::forwarder::run(cfg));
    }
    // CPU samples for the HostPerf facts, every 5 s on a thread of their own.
    forensic_hooks::modules::host_perf::spawn();

    // === IOC lists & YARA ===
    if let Err(e) = ioc::init() {
        eprintln!("⚠️ ioc::init failed: {e}");
    }

    // === Playbook engine: load all YAMLs from ./playbooks ===
    {
        let pb_dir = std::env::var("PB_DIR").unwrap_or_else(|_| "playbooks".to_string());
        match pb_engine().lock() {
            Ok(mut eng) => match PlaybookEngine::load_dir(&pb_dir) {
                Ok(new_eng) => {
                    *eng = new_eng;
                    eprintln!("✅ playbook_engine loaded from {pb_dir}");
                }
                Err(e) => eprintln!("⚠️ playbook_engine load_dir({pb_dir}) failed: {e}"),
            },
            Err(_) => eprintln!("⚠️ playbook_engine lock poisoned"),
        }
    }

    // alerts / graph / incidents channels
    let (alert_tx, _alert_rx) = broadcast::channel::<String>(1024);
    let (graph_tx, _graph_rx) = broadcast::channel::<String>(128);
    let (inc_tx, _inc_rx) = broadcast::channel::<String>(256);

    // Optional PB SSE bus (for pb_api)
    let (pb_tx, _pb_rx) = tokio::sync::broadcast::channel::<String>(256);
    pb_introspect::set_sse_sender(pb_tx.clone());
    let pb_state = pb_api::PbApiState { tx: pb_tx.clone() };

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // ensure layout exists
    let _ = std::fs::create_dir_all("logs");
    let _ = std::fs::create_dir_all("edr-agent/logs");
    let _ = std::fs::create_dir_all("state");
    let _ = std::fs::create_dir_all("ui");
    let _ = std::fs::create_dir_all("json_files"); // ensure live-graph path exists
    {
        let cwd = env::current_dir().unwrap();
        eprintln!("🗂️  cwd        : {}", cwd.display());
        eprintln!(
            "🗂️  ui         : {}",
            fs::canonicalize("ui")
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "MISSING".into())
        );
        eprintln!(
            "🗂️  json_files : {}",
            fs::canonicalize("json_files")
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "MISSING".into())
        );
        eprintln!(
            "🗎   ui/index.html present? {}",
            Path::new("ui/index.html").exists()
        );
        eprintln!(
            "🗎   ui/app.js     present? {}",
            Path::new("ui/app.js").exists()
        );
        eprintln!(
            "🗎   ui/graph.html present? {}",
            Path::new("ui/graph.html").exists()
        );
    }

    // ---- initialize live graph writer (graph.rs) ----
    let gs = graph_state();
    let _graph_task = spawn_writer(gs.clone(), PathBuf::from("json_files/graph_live.json"));

    // ---- optional R-GCN bridge ----
    {
        let rgcn_enabled = std::env::var("RGCN_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        if rgcn_enabled {
            let url =
                std::env::var("RGCN_URL").unwrap_or_else(|_| "http://127.0.0.1:9000/infer".into());
            let poll_ms = std::env::var("RGCN_POLL_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1500);
            let timeout_ms = std::env::var("RGCN_TIMEOUT_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2000);
            let max_nodes = std::env::var("RGCN_MAX_NODES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3000);
            let max_edges = std::env::var("RGCN_MAX_EDGES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(6000);

            let cfg = rgcn_bridge::Config {
                url,
                poll_ms,
                timeout_ms,
                max_nodes,
                max_edges,
                graph_path: "json_files/graph_live.annotated.json".into(),
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
        // The file holds integration credentials, so it is owner-only (0600).
        forensic_hooks::agent_api::tighten_to_owner_only(Path::new("state/integrations.json"));
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

    // ---------------- Integrations API ----------------
    {
        use axum::http::StatusCode;
        use axum::http::{header, HeaderValue};
        use axum::routing::{delete, get, get_service, post};
        use axum::{extract::State, response::IntoResponse, Json, Router};
        use serde::{Deserialize, Serialize};
        use serde_json::{json, Value};
        use std::{fs, path::Path};
        use tower_http::services::{ServeDir, ServeFile};
        use tower_http::set_header::SetResponseHeaderLayer;

        #[derive(Serialize, Deserialize, Default)]
        struct IntegrationCreate {
            id: String,
            name: String,
            creds: Value,
        }

        async fn integrations_catalog() -> impl IntoResponse {
            let path = Path::new("integrations/catalog.json");
            let body = fs::read_to_string(path)
                .unwrap_or_else(|_| "{\"version\":1,\"vendors\":[]}".into());
            (StatusCode::OK, body)
        }

        // Credentials are masked on the way out. The stored file is owner-only, and a
        // write error never prints its contents.
        async fn integrations_list(State(st): State<ApiState>) -> impl IntoResponse {
            let m = st.integrations.lock().unwrap();
            Json(forensic_hooks::agent_api::integrations_view(&m))
        }

        fn save_integrations(m: &HashMap<String, Value>) {
            let _ = fs::create_dir_all("state");
            let bytes = serde_json::to_vec_pretty(m).unwrap_or_else(|_| b"{}".to_vec());
            let path = Path::new("state/integrations.json");
            if let Err(e) = forensic_hooks::agent_api::write_private_file(path, &bytes) {
                eprintln!("⚠️ failed writing state/integrations.json: {e}");
            }
        }

        async fn integrations_add(
            State(st): State<ApiState>,
            Json(req): Json<IntegrationCreate>,
        ) -> impl IntoResponse {
            let mut m = st.integrations.lock().unwrap();
            m.insert(
                req.id.clone(),
                json!({
                    "id": req.id.clone(),
                    "name": req.name.clone(),
                    "creds": req.creds.clone(),
                    "status": "syncing",
                    "last_ok": Value::Null
                }),
            );
            save_integrations(&m);
            (StatusCode::ACCEPTED, Json(json!({ "ok": true })))
        }

        async fn integrations_delete(
            State(st): State<ApiState>,
            axum::extract::Path(id): axum::extract::Path<String>,
        ) -> impl IntoResponse {
            let mut m = st.integrations.lock().unwrap();
            m.remove(&id);
            save_integrations(&m);
            StatusCode::NO_CONTENT
        }

        // ---------------- PB router + Static UI + Main app ----------------
        let pb_router = Router::new()
            .route("/pending", get(pb_api::pending))
            .route("/hits", get(pb_api::hits))
            .route("/reload", post(pb_api::reload))
            .route("/stream", get(pb_api::stream))
            .with_state(pb_state.clone());

        // no-cache for UI assets
        let no_cache = SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store, max-age=0, must-revalidate"),
        );

        // UI subtree (served under /ui)
        let ui_router = Router::new()
            .route("/index.html", get_service(ServeFile::new("ui/index.html")))
            .route("/graph.html", get_service(ServeFile::new("ui/graph.html")))
            .route("/app.js", get_service(ServeFile::new("ui/app.js")))
            .fallback_service(get_service(ServeDir::new("ui")))
            .layer(no_cache.clone());

        // Data routes. agent_api::secure_router puts the bearer token check on every
        // one of them, the SSE streams and /json_files included.
        let protected = Router::new()
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
            .route("/incidents/recent", get(incidents_recent))
            .route(
                "/incidents",
                get(|| async {
                    let list = incidents().lock().unwrap().list();
                    Json(list)
                }),
            )
            .route(
                "/incidents/:id",
                get(
                    |axum::extract::Path(id): axum::extract::Path<String>| async move {
                        if let Some(it) = incidents().lock().unwrap().get(&id) {
                            Json(it).into_response()
                        } else {
                            (StatusCode::NOT_FOUND, "not found").into_response()
                        }
                    },
                ),
            )
            // Response actions (src/response.rs). Each needs a reason or an incident
            // id, supports dry_run, must be allowed by the signed policy, and is written
            // to logs/actions_audit.jsonl.
            .route(
                "/actions/kill_proc",
                post(|body: axum::body::Bytes| async move {
                    match parse_body::<forensic_hooks::response::KillRequest>(&body) {
                        Ok(req) => run_action(move |r| {
                            r.kill_proc(req, &|id| incidents().lock().map(|m| m.get(id).is_some()).unwrap_or(false))
                        })
                        .await,
                        Err(resp) => resp,
                    }
                }),
            )
            .route(
                "/actions/block_egress",
                post(|body: axum::body::Bytes| async move {
                    match parse_body::<forensic_hooks::response::BlockRequest>(&body) {
                        Ok(req) => run_action(move |r| r.block_egress(req)).await,
                        Err(resp) => resp,
                    }
                }),
            )
            .route(
                "/actions/isolate_host",
                post(|body: axum::body::Bytes| async move {
                    match parse_body::<forensic_hooks::response::IsolateRequest>(&body) {
                        Ok(req) => run_action(move |r| r.isolate_host(req)).await,
                        Err(resp) => resp,
                    }
                }),
            )
            .route(
                "/actions/release",
                post(|body: axum::body::Bytes| async move {
                    match parse_body::<forensic_hooks::response::ReleaseRequest>(&body) {
                        Ok(req) => run_action(move |r| r.release(req)).await,
                        Err(resp) => resp,
                    }
                }),
            )
            .route(
                "/actions/state",
                get(|| async {
                    Json(RESPONDER.get().map(|r| r.state_json()).unwrap_or(json!({"firewall": "none"})))
                }),
            )
            // Runtime shields (src/shield). POST takes a signed document, every change
            // and refusal goes to logs/shields_audit.jsonl.
            .route(
                "/shields",
                get(|| async {
                    Json(forensic_hooks::shield::manager().map(|m| m.list()).unwrap_or(json!({"enforcement": "none", "shields": []})))
                })
                .post(|body: axum::body::Bytes| async move {
                    match parse_body::<ShieldPost>(&body) {
                        Ok(req) => run_shield(move |m| m.apply_signed(&req.document, &req.signature)).await,
                        Err(resp) => resp,
                    }
                }),
            )
            .route(
                "/shields/:id/promote",
                post(|axum::extract::Path(id): axum::extract::Path<String>, body: axum::body::Bytes| async move {
                    let reason = shield_reason(&body);
                    run_shield(move |m| m.promote(&id, &reason)).await
                }),
            )
            .route(
                "/shields/:id",
                delete(|axum::extract::Path(id): axum::extract::Path<String>, body: axum::body::Bytes| async move {
                    let reason = shield_reason(&body);
                    run_shield(move |m| m.remove(&id, &reason)).await
                }),
            )
            // The signed policy in force, and the last reload or refusal.
            .route("/policy/status", get(|| async { Json(forensic_hooks::policy_watch::status_json()) }))
            // Pipeline health: the fact queue (drops, waits, depth), the loaded
            // playbooks, masking and forwarding counters.
            .route("/pipeline/status", get(pipeline_status))
            // Integrations
            .route("/integrations/catalog", get(integrations_catalog))
            .route(
                "/integrations",
                get(integrations_list).post(integrations_add),
            )
            .route("/integrations/:id", delete(integrations_delete))
            // IOC endpoints
            .route("/ioc/reload", post(ioc_reload_handler))
            .route("/ioc/stats", get(ioc_stats_handler))
            // PB endpoints (nested with separate state)
            .nest("/playbooks", pb_router)
            // Diag
            .route("/where", get(diag_where))
            .route("/__where", get(diag_where))
            // Static data
            .nest_service("/json_files", get_service(ServeDir::new("json_files")))
            .with_state(api_state.clone());

        // No token here: a liveness probe, and the UI shell (HTML and JS from the repo,
        // no data) that holds the sign-in field.
        let public = Router::new()
            .route("/healthz", get(healthz))
            .route("/", get_service(ServeFile::new("ui/index.html")))
            .nest("/ui", ui_router);

        // ---- Serve UI/API: loopback only, and only with a token ----
        match forensic_hooks::agent_api::AgentAuth::from_env() {
            Err(why) => {
                eprintln!("❌ agent API not started: {why}. Sensing and detection keep running.");
                log::error!("agent API not started: {why}");
            }
            Ok(auth) => {
                if auth.is_open() {
                    eprintln!("⚠️ WARNING: agent API has NO authentication (EDR_DEV_NO_AUTH=1). Dev machines only.");
                }
                // A token file is re-read every 2 s, so the token can be rotated without
                // a restart.
                if let Ok(path) = std::env::var(forensic_hooks::agent_api::TOKEN_FILE_ENV) {
                    auth.watch_token_file(PathBuf::from(path), Duration::from_secs(2));
                }
                let server_app = forensic_hooks::agent_api::secure_router(
                    public,
                    protected,
                    auth,
                    forensic_hooks::agent_api::cors_from_env(),
                );
                let port: u16 = std::env::var("EDR_PORT")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(8080);

                tokio::spawn(async move {
                    // Bind directly with Tokio (loopback by default)
                    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
                    let listener = match TcpListener::bind(addr).await {
                        Ok(l) => l,
                        Err(e) => {
                            eprintln!("❌ bind({addr}) failed: {e}");
                            return;
                        }
                    };

                    let local = listener.local_addr().unwrap_or(addr);
                    eprintln!(
                        "🌐 UI listening on http://{}/  (dashboard) | http://{}/ui/graph.html (live graph)",
                        local, local
                    );
                    eprintln!("▶️ axum server task starting…");

                    let shutdown = async {
                        let _ = shutdown_rx.await;
                    };

                    if let Err(e) = axum::serve(listener, server_app)
                        .with_graceful_shutdown(shutdown)
                        .await
                    {
                        eprintln!("❌ axum serve error: {e}");
                    } else {
                        eprintln!("❌ axum serve ended unexpectedly (no error)");
                    }
                });
            }
        }

        // ---------------- decoders → record queue → record thread ----------------
        // Records for the rules, the taggers, the live graph, the trust scores and the
        // telemetry files. A bounded queue with backpressure feeds one OS thread that
        // runs them in chunks (record_pipeline.rs). A full queue holds the decoder up to
        // the push timeout, and only a record that still finds no room is dropped and
        // counted (records_dropped_total on /pipeline/status). Playbook facts have their
        // own queue (pb_adapter).
        let record_queue = record_pipeline::init_queue();
        let record_thread = record_pipeline::spawn(
            record_queue,
            record_pipeline::RecordPipeline {
                evaluator,
                graph: gs.clone(),
                alerts: alert_tx.clone(),
                incidents: inc_tx.clone(),
                writer: writer.clone(),
                side_effects: true,
            },
            tokio::runtime::Handle::current(),
        );
        if let Err(e) = record_thread {
            eprintln!("record thread did not start: {e}");
        }

        // ---------------- startup of eBPF readers ----------------

        #[cfg(target_os = "linux")]
        {
            use std::sync::atomic::Ordering;

            use forensic_hooks::telemetry::TelemetryRecord;

            fn write_record_and_counters(rec: TelemetryRecord) {
                if record_pipeline::push(rec) {
                    metrics::events_accepted().fetch_add(1, Ordering::Relaxed);
                }
            }

            std::thread::spawn(|| loop {
                let in_ = metrics::events_in().load(Ordering::Relaxed);
                let acc = metrics::events_accepted().load(Ordering::Relaxed);
                let drops = metrics::bpf_drops_total().load(Ordering::Relaxed);
                let missed = in_.saturating_sub(acc);
                // The agent's own frames that reached user space (raw sensor, file_access).
                let (self_raw, self_file) = crate::ebpf::ebpf_ingest::self_frames_dropped();
                // exec frames whose process was gone from /proc: filled from the kernel
                // copy, or left without exe and argv.
                let exec_kcopy = forensic_hooks::pb_adapter::exec_from_kernel_copy_total();
                let exec_lost = forensic_hooks::pb_adapter::exec_unresolved_total();

                let drop_rate = if in_ > 0 {
                    100.0 * missed as f64 / in_ as f64
                } else {
                    0.0
                };

                eprintln!(
                    "[metrics] events_in={} accepted={} missed={} perf_drops={} drop_rate={:.2}% self_raw={} self_file={} exec_kernel_copy={} exec_unresolved={}",
                    in_, acc, missed, drops, drop_rate, self_raw, self_file, exec_kcopy, exec_lost
                );

                // Give freed heap pages back to the kernel. glibc keeps what a burst, a
                // shield snapshot or a large GET /shields freed, so the agent's RSS only
                // grew: measured with 30,000 shields, three listings added 115 MB that
                // never came back. Every 5 s is cheap next to the work between calls.
                #[cfg(all(target_os = "linux", target_env = "gnu"))]
                unsafe {
                    libc::malloc_trim(0);
                }

                std::thread::sleep(Duration::from_secs(5));
            });
            let (use_pins, pin_prefix) = pinned_mode_selected();

            log::info!("Pinned-reader mode: {use_pins} (EDR_PIN_PREFIX={pin_prefix:?})");

            // Raw hex copy of every frame, next to the decoded record. Off by default because
            // it doubled every event in kv_telemetry.jsonl. EDR_DEBUG_RAW_EVENTS=1 turns it on.
            fn emit_debug_raw(src_name: &str, bytes: &[u8]) {
                static RAW_ON: OnceLock<bool> = OnceLock::new();
                let on = *RAW_ON.get_or_init(|| {
                    std::env::var("EDR_DEBUG_RAW_EVENTS")
                        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                        .unwrap_or(false)
                });
                if !on {
                    return;
                }
                let mut m = std::collections::HashMap::new();
                m.insert("category".into(), "debug".into());
                m.insert("signal".into(), "debug::raw_event".into());
                m.insert("source".into(), src_name.into());
                m.insert("payload_len".into(), bytes.len().to_string());
                let head_n = bytes.len().min(16);
                let head_hex = bytes[..head_n]
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>();
                m.insert("payload_head_hex".into(), head_hex);
                forensic_hooks::telemetry_writer::write_telemetry_record(m);
            }

            let demux = {
                let w = writer.clone();
                move |src: events_reader::Source, bytes: &[u8]| match src {
                    events_reader::Source::EdrSys => {
                        emit_debug_raw("edr_events_rb", bytes);
                        if let Some(rec) = on_edr_event(bytes, &w) {
                            write_record_and_counters(rec);
                        }
                    }
                    events_reader::Source::Net => {
                        emit_debug_raw("net_events", bytes);
                        if let Some(rec) = on_edr_event_net_only(bytes, &w) {
                            write_record_and_counters(rec);
                        }
                    }
                    events_reader::Source::Wx => {
                        emit_debug_raw("wx_events", bytes);
                        if let Some(rec) = on_edr_event_wx_only(bytes, &w) {
                            write_record_and_counters(rec);
                        }
                    }
                    events_reader::Source::FilePerf => {
                        emit_debug_raw("events", bytes);
                        if let Some(rec) = on_edr_event_file_only(bytes, &w) {
                            write_record_and_counters(rec);
                        }
                    }
                }
            };

            if !use_pins {
                log::error!(
                    "No pins at {pin_prefix}. Hint: load the sensors first (edr_attach_any -p {pin_prefix} <object.bpf.o>)."
                );
                return Err(anyhow!("no pinned sensor maps at {pin_prefix}"));
            }
            // Self filters for raw_syscalls_catcher and file_access_monitor, and the
            // default allowlist for raw_syscalls_catcher, before any
            // reader starts.
            crate::ebpf::raw_config::configure(&pin_prefix);
            events_reader::start_all_pinned_readers_demux(&pin_prefix, demux)?;
            // The sensors' maps, programs and pins come under the guard too.
            if let Some(g) = prevention_guard.as_ref() {
                forensic_hooks::guard::spawn_protect_held(g.control.clone(), PathBuf::from(&pin_prefix));
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            log::warn!("eBPF readers are disabled on non-Linux targets.");
        }

        // ---------------- incidents from playbooks and the guard ----------------
        {
            use std::time::Duration;
            let it = inc_tx.clone();

            tokio::spawn(async move {
                loop {
                    // Playbooks that completed since the last pass become incidents and are
                    // kept in logs/playbook_hits.jsonl. The engine thread runs tick and the
                    // open_flow sweep itself (pb_adapter/runner.rs).
                    {
                        let done = pb_wiring::take_completed();
                        if !done.is_empty() {
                            let _ = std::fs::create_dir_all("logs");
                            let mut log = std::fs::OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open("logs/playbook_hits.jsonl")
                                .ok();
                            for hit in &done {
                                let alert = pb_wiring::hit_alert_json(hit);
                                if let Some(f) = log.as_mut() {
                                    use std::io::Write;
                                    let _ = writeln!(f, "{alert}");
                                }
                                // A statement of its own, so the incidents lock is released here.
                                // Inside `if let Some(inc) = incidents().lock()...` the guard
                                // would live to the end of the block, and attach_proposals
                                // below takes the same lock: the first live run deadlocked
                                // there and the whole API stopped answering.
                                let ingested = incidents().lock().unwrap().ingest_alert_json(&alert);
                                if let Some(inc) = ingested {
                                    let inc_id = inc.get("id").and_then(|x| x.as_str()).unwrap_or("-").to_string();
                                    eprintln!("incident {inc_id} from playbook {}", hit.playbook_id);
                                    let _ = it.send(inc.to_string());
                                    // Shield proposals from the playbook's templates, on the
                                    // incident and ready to sign. Applied here only when the
                                    // signed policy lets this kind apply by itself.
                                    let props = forensic_hooks::shield::propose::from_hit(
                                        hit,
                                        &inc_id,
                                        forensic_hooks::utils::time::now_ts(),
                                        Path::new("/proc"),
                                        Path::new(forensic_hooks::shield::propose::CGROUP_ROOT),
                                    );
                                    if !props.is_empty() {
                                        let mut views = Vec::new();
                                        for p in props {
                                            let mut v = p.json();
                                            if let Some(m) = forensic_hooks::shield::manager() {
                                                let o = m.apply_auto(p.doc.clone(), &hit.playbook_id);
                                                v["auto_apply"] = json!({ "status": o.status, "result": o.body });
                                                if o.status < 300 {
                                                    eprintln!("shield {} applied by itself in monitor mode ({})", p.doc.id, p.summary);
                                                }
                                            }
                                            eprintln!("shield proposal {} on incident {inc_id}: {}", p.doc.id, p.summary);
                                            views.push(v);
                                        }
                                        if let Some(inc) = incidents().lock().unwrap().attach_proposals(&inc_id, views) {
                                            let _ = it.send(inc.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Guard blocks and would-blocks become incidents with the guard name,
                    // the caller exe, the target and the mode.
                    {
                        let guard_alerts = forensic_hooks::guard::take_guard_alerts();
                        if !guard_alerts.is_empty() {
                            let _ = std::fs::create_dir_all("logs");
                            let mut log = std::fs::OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open("logs/guard_alerts.jsonl")
                                .ok();
                            for alert in &guard_alerts {
                                // A shield hit takes the technique of the incident that
                                // proposed the shield.
                                let mut alert = alert.clone();
                                if alert.get("source").and_then(|x| x.as_str()) == Some("shield") {
                                    if let Some(linked) = alert
                                        .pointer("/details/incident_id")
                                        .and_then(|x| x.as_str())
                                        .and_then(|i| incidents().lock().unwrap().get(i))
                                    {
                                        let t = linked.get("primary_technique").cloned().unwrap_or(json!("-"));
                                        if t != json!("-") {
                                            alert["findings"] = json!([{ "label": t.clone() }]);
                                        }
                                        alert["technique"] = t;
                                    }
                                }
                                let alert = &alert;
                                if let Some(f) = log.as_mut() {
                                    use std::io::Write;
                                    let _ = writeln!(f, "{alert}");
                                }
                                if let Some(inc) = incidents().lock().unwrap().ingest_alert_json(alert) {
                                    eprintln!(
                                        "incident {} from guard {} ({})",
                                        inc.get("id").and_then(|x| x.as_str()).unwrap_or("-"),
                                        alert.pointer("/details/guard").and_then(|x| x.as_str()).unwrap_or("-"),
                                        alert.pointer("/details/mode").and_then(|x| x.as_str()).unwrap_or("-"),
                                    );
                                    let _ = it.send(inc.to_string());
                                }
                            }
                        }
                    }

                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            });
        }
    }

    // --- keep the process alive until Ctrl-C (so the server keeps listening) ---
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl-C handler");
    eprintln!("🛑 received Ctrl-C, shutting down…");
    if let Some(prevention_guard) = prevention_guard {
        prevention_guard.stop();
    }
    // The shield programs stay attached through their pins, and every row ends on time.
    #[cfg(target_os = "linux")]
    if let Some(t) = shield_thread {
        t.stop();
    }
    #[cfg(not(target_os = "linux"))]
    let _ = shield_thread;
    let _ = shutdown_tx.send(());
    Ok(())
}

/// Files the guard write protects and no shield may name: the agent's binary, its
/// public key and its anti-rollback state.
fn own_files(pubkey: &Path, store: &forensic_hooks::config::PolicyStore) -> Vec<PathBuf> {
    let mut v = Vec::new();
    if let Ok(exe) = std::fs::read_link("/proc/self/exe") {
        v.push(exe);
    }
    v.push(pubkey.to_path_buf());
    v.extend(store.files());
    let state = forensic_hooks::config::state_dir();
    v.push(state.join("shields.json"));
    v.push(state.join("shield_serials.json"));
    v.push(state.join("shields.journal"));
    v
}

#[cfg(target_os = "linux")]
type ShieldThread = forensic_hooks::shield::loader::ShieldThread;
#[cfg(not(target_os = "linux"))]
type ShieldThread = ();

/// Loads the shield programs (pinned), restores the saved shields, protects the
/// shields' BPF objects and the agent's files with the guard, and installs the
/// process wide shield manager.
fn start_shields(
    policy: &forensic_hooks::config::RawPolicy,
    pubkey: &Path,
    store: &forensic_hooks::config::PolicyStore,
    guard: Option<&forensic_hooks::guard::GuardHandle>,
    backend_ips: &[std::net::IpAddr],
) -> Option<ShieldThread> {
    use forensic_hooks::shield::{self, NeverBlock, ShieldManager, SystemClock};
    let state = forensic_hooks::config::state_dir();
    let _ = std::fs::create_dir_all(&state);
    let audit = PathBuf::from(std::env::var("EDR_SHIELDS_AUDIT").unwrap_or_else(|_| "logs/shields_audit.jsonl".into()));
    let own = own_files(pubkey, store);
    let never = NeverBlock::build(&policy.shields.never_block, &own, backend_ips);
    let pem = std::fs::read(pubkey).unwrap_or_default();
    let mut thread: Option<ShieldThread> = None;
    let mut maps: Option<Box<dyn shield::RuleMaps>> = None;
    #[cfg(target_os = "linux")]
    if let Some(g) = guard {
        let obj = std::env::var("EDR_BPF_SHIELD_OBJ").unwrap_or_else(|_| match std::env::var("EDR_BPF_DIR") {
            Ok(d) => format!("{}/edr_shield.bpf.o", d.trim_end_matches('/')),
            Err(_) => "src/ebpf/edr_shield.bpf.o".into(),
        });
        let pin_dir = PathBuf::from(std::env::var("EDR_SHIELD_PIN_DIR").unwrap_or_else(|_| shield::loader::DEFAULT_PIN_DIR.into()));
        let on_event: Box<dyn FnMut(&[u8]) + Send> = Box::new(|data: &[u8]| {
            let Some(e) = shield::abi::ShieldEvent::from_bytes(data) else {
                log::warn!("shields: dropped a {} byte record", data.len());
                return;
            };
            let active = shield::manager().and_then(|m| m.by_num(e.shield_id));
            let exe = std::fs::read_link(format!("/proc/{}/exe", e.tgid)).ok().map(|p| p.display().to_string());
            if let Some((finding, alert)) = shield::shield_finding(&e, active.as_ref(), exe.as_deref()) {
                forensic_hooks::guard::emit_with_alert(finding, alert);
            }
        });
        match shield::loader::start(&obj, &pin_dir, g.summary.scope_cgroup_id, on_event) {
            Ok(st) => {
                for n in &st.notes {
                    eprintln!("shields: {n}");
                }
                if let Err(e) = g.control.protect_ids(&st.ids) {
                    eprintln!("⚠️ shields: could not protect the shield objects: {e:#}");
                }
                let mut files = st.pins.clone();
                files.extend(own.iter().cloned());
                files.push(PathBuf::from(&obj));
                files.push(PathBuf::from(&g.summary.obj_path));
                eprintln!(
                    "shields: {} ({} programs, maps and links protected), pinned in {}",
                    obj,
                    st.ids.len(),
                    pin_dir.display()
                );
                maps = Some(Box::new(st.maps));
                thread = Some(st.thread);
                // The state files must exist before their inodes are protected.
                let m = ShieldManager::new(policy.shields.clone(), never.clone(), maps.take().unwrap(), Box::new(SystemClock), pem.clone(), &state, &audit, "kernel");
                for n in m.restore() {
                    eprintln!("shields: {n}");
                }
                match g.control.protect_files(&files) {
                    Ok(missing) if !missing.is_empty() => eprintln!("⚠️ guard: not protected, missing: {}", missing.join(", ")),
                    Ok(_) => eprintln!("guard: {} agent files and shield pins write protected", files.len()),
                    Err(e) => eprintln!("⚠️ guard: file protection: {e:#}"),
                }
                shield::set_manager(std::sync::Arc::new(m));
                return thread;
            }
            Err(e) => eprintln!("⚠️ shields: not loaded, the shield API refuses new shields: {e:#}"),
        }
    }
    let _ = (&mut thread, guard);
    let m = ShieldManager::new(
        policy.shields.clone(),
        never,
        maps.unwrap_or_else(|| Box::new(shield::MemMaps::default())),
        Box::new(SystemClock),
        pem,
        &state,
        &audit,
        "none",
    );
    shield::set_manager(std::sync::Arc::new(m));
    thread
}

// Every decoder stamps TelemetryRecord.timestamp in Unix seconds, so a playbook window
// can mix sensors. The kernel stamp is time since boot and is converted, not divided.
#[cfg(test)]
mod timestamp_tests {
    use crate::ebpf::abi::{self, test_bytes::*, SharedFrame};
    use crate::ebpf::ebpf_bridge::{edr_event_to_output, edr_event_to_record};
    use crate::ebpf::ebpf_ingest::{decode_proc_lifecycle, decode_wx};
    use forensic_hooks::pb_adapter::clock;

    const T: u64 = 1_800_000_000;

    #[test]
    fn raw_syscall_records_use_unix_seconds() {
        let mut b = vec![0u8; 376];
        put_u64(&mut b, 0, clock::unix_secs_to_mono_ns(T) + 400_000_000); // ts
        put_u32(&mut b, 8, 14); // EVT_UNLINK
        let e = abi::from_ring_frame::<crate::ebpf::ebpf_bridge::EdREvent>(&b).unwrap();
        // tgid 0, so no /proc lookup runs.
        assert_eq!(edr_event_to_record(&e).timestamp, T);
        assert_eq!(edr_event_to_output(&e).data["timestamp"], T.to_string());
    }

    #[test]
    fn tcp_state_addresses_and_ports_decode_in_network_order() {
        // Offsets of struct edr_event: fam 56, lport 58, laddr4 60, laddr6 64, rport 80,
        // raddr4 84, raddr6 88. The kernel stores addresses and ports big endian.
        let mut b = vec![0u8; 376];
        put_u32(&mut b, 8, 200); // EVT_TCP_STATE
        b[56] = 2;
        b[58..60].copy_from_slice(&42754u16.to_be_bytes());
        b[60..64].copy_from_slice(&[127, 0, 0, 1]);
        b[80..82].copy_from_slice(&8080u16.to_be_bytes());
        b[84..88].copy_from_slice(&[203, 0, 113, 10]);
        let e = abi::from_ring_frame::<crate::ebpf::ebpf_bridge::EdREvent>(&b).unwrap();
        let d = edr_event_to_output(&e).data;
        assert_eq!(d["laddr4"], "127.0.0.1", "was printed reversed as 1.0.0.127");
        assert_eq!(d["raddr4"], "203.0.113.10");
        assert_eq!(d["lport"], "42754");
        assert_eq!(d["rport"], "8080");
        // IPv6, with a mapped IPv4 peer shown as IPv4.
        b[56] = 10;
        b[64..80].copy_from_slice(&"fd00::5".parse::<std::net::Ipv6Addr>().unwrap().octets());
        b[88..104].copy_from_slice(&"::ffff:203.0.113.9".parse::<std::net::Ipv6Addr>().unwrap().octets());
        let e = abi::from_ring_frame::<crate::ebpf::ebpf_bridge::EdREvent>(&b).unwrap();
        let d = edr_event_to_output(&e).data;
        assert_eq!(d["laddr6"], "fd00::5");
        assert_eq!(d["raddr6"], "203.0.113.9");
    }

    #[test]
    fn wx_and_proc_records_use_the_kernel_stamp() {
        let ns = clock::unix_secs_to_mono_ns(T);
        let d = decode_wx(&wx(555, 2, ns, 0, 0, 0, 8192), T + 3).unwrap();
        assert_eq!(d.record.timestamp, T, "kernel stamp, not decode time");
        // A frame with no stamp keeps the decode time.
        let d = decode_wx(&wx(555, 2, 0, 0, 0, 0, 8192), T + 3).unwrap();
        assert_eq!(d.record.timestamp, T + 3);

        let mut b = proc_lc(1001, 1234, 1000, 0, "sh", 0);
        put_u64(&mut b, 0, ns + 2_000_000_000);
        let SharedFrame::ProcLifecycle(p) = abi::classify_shared_frame(&b) else {
            panic!("not a proc_lifecycle frame");
        };
        assert_eq!(decode_proc_lifecycle(&p, T + 9).unwrap().record.timestamp, T + 2);
    }
}

// Secrets on a command line are masked before anything is stored: the kv rows, the
// telemetry files, the playbook facts and the incident list are checked on disk and in
// memory for the raw values.
#[cfg(test)]
mod redaction_tests {
    use crate::ebpf::abi::{self, test_bytes::*, SharedFrame};
    use crate::ebpf::ebpf_ingest::{decode_proc_lifecycle, fill_exec_from_kernel_copy};
    use forensic_hooks::pb_adapter::{adapter_facts_with, ProcInfo};
    use forensic_hooks::pb_wiring::{FixedMounts, Mapper};
    use forensic_hooks::telemetry_writer::{write_telemetry_record_to, TelemetryWriter};
    use serde_json::json;

    const T: u64 = 1_800_000_000;

    // Built at run time so no source line holds a whole fake secret.
    fn secrets() -> Vec<String> {
        vec![
            format!("{}{}", "Hunter2", "Pass"),
            format!("{}{}", "tok_9f8e7d6c", "5b4a3210"),
            format!("{}{}", "AKIA", "IOSFODNN7EXAMPLE"),
            format!("{}{}", "wJalrXUtnFEMI/K7MDENG/", "bPxRfiCYEXAMPLEKEY"),
            format!("{}{}", "eyJhbGciOi", "JIUzI1NiJ9abc"),
            format!("{}{}", "urlpw", "S3cret"),
        ]
    }

    fn read_all(dir: &std::path::Path) -> String {
        let mut text = String::new();
        for e in std::fs::read_dir(dir).unwrap().flatten() {
            text.push_str(&std::fs::read_to_string(e.path()).unwrap_or_default());
        }
        text
    }

    #[test]
    fn argv_secrets_never_reach_disk_facts_or_incidents() {
        let s = secrets();
        let args: Vec<String> = vec![
            "mysql".into(),
            "-uroot".into(),
            format!("-p{}", s[0]),
            "--api-token".into(),
            s[1].clone(),
            format!("AWS_ACCESS_KEY_ID={}", s[2]),
            format!("AWS_SECRET_ACCESS_KEY={}", s[3]),
            "Authorization:".into(),
            "Bearer".into(),
            s[4].clone(),
            format!("https://bob:{}@db.example/x", s[5]),
        ];
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        let b = proc_lc_exec(4242, "mysql", "/usr/bin/mysql", &argv);
        let SharedFrame::ProcLifecycle(p) = abi::classify_shared_frame(&b) else {
            panic!("not a proc_lifecycle frame");
        };
        let d = decode_proc_lifecycle(&p, T).unwrap();
        let dir = std::env::temp_dir().join(format!("edr_redact_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);

        // The kv row, as sink() writes it.
        let mut m = d.out.data.clone();
        m.insert("category".into(), d.out.category.clone());
        m.insert("signal".into(), d.out.signal.clone());
        write_telemetry_record_to(&dir, m);

        // The record, filled from the kernel copy because /proc has no pid 4242, as the
        // batch loop stores it.
        let mut rec = d.record.clone();
        fill_exec_from_kernel_copy(&mut rec, &p);
        assert!(rec.command_line.contains("[REDACTED]"));
        let w = TelemetryWriter::with_dir(&dir);
        w.append_batch(vec![rec.clone()]);
        w.flush_to_disk();

        // The playbook facts.
        let facts: Vec<_> = adapter_facts_with(&d.out, T, &|_| ProcInfo::default())
            .iter()
            .flat_map(|af| Mapper::with_mounts(Box::new(FixedMounts(vec![]))).facts(af))
            .collect();
        let cmd = facts[0].cmd.clone().unwrap();
        assert!(cmd.starts_with("mysql -uroot -p[REDACTED] --api-token [REDACTED]"), "{cmd}");

        // The incident made from an alert on this record.
        let alert = json!({
            "id": "a1", "ts": T, "severity": "high", "technique": "T1059",
            "findings": [{"label": "T1059"}],
            "event": { "exe": rec.binary_path, "cmdline": rec.command_line },
        });
        let mut mgr = crate::IncidentsMgr::new(900);
        let inc = mgr.ingest_alert_json(&alert).unwrap().to_string();

        let on_disk = read_all(&dir);
        assert!(on_disk.contains("[REDACTED]"), "the rows were written: {on_disk}");
        for secret in &s {
            assert!(!on_disk.contains(secret.as_str()), "{secret} reached disk");
            assert!(!cmd.contains(secret.as_str()), "{secret} reached a fact");
            assert!(!inc.contains(secret.as_str()), "{secret} reached an incident");
            assert!(!format!("{:?}", d.out.data).contains(secret.as_str()));
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_rule_alert_incident_names_the_rule_and_its_techniques() {
        let alert = json!({
            "ts": T, "severity": "medium", "risk": 0.6,
            "findings": [{"source": "rules", "score": 0.7, "label": "proc_exec_from_tmp",
                          "details": {"name": "Exec from /tmp", "mitre": ["T1204", "T1036"]}}],
            "event": { "exe": "/tmp/x" },
        });
        let inc = crate::IncidentsMgr::new(900).ingest_alert_json(&alert).unwrap();
        assert_eq!(inc["title"], "Exec from /tmp");
        assert_eq!(inc["techniques"], json!(["T1036", "T1204"]));
        assert_eq!(inc["sources"], json!(["rules"]));
    }

    #[test]
    fn a_guard_block_becomes_an_incident_with_guard_caller_target_and_mode() {
        let mut data = std::collections::HashMap::new();
        for (k, v) in [
            ("source", "edr_guard"),
            ("timestamp", "1800000000"),
            ("pid", "4321"),
            ("uid", "0"),
            ("comm", "docker"),
            ("binary_path", "/tmp/g6_spoof/docker"),
            ("guard", "runtime_socket"),
            ("hook", "unix_stream_connect"),
            ("action", "blocked"),
            ("target", "/run/docker.sock"),
            ("mitre", "T1611,T1610"),
            ("reason", "blocked: docker (pid 4321) connect() to /run/docker.sock"),
        ] {
            data.insert(k.to_string(), v.to_string());
        }
        let out = forensic_hooks::telemetry_types::TelemetryOutput {
            category: "process".into(),
            signal: "guard::runtime_socket::blocked".into(),
            confidence: 0.6,
            data,
        };
        let alert = forensic_hooks::guard::guard_alert_json(&out);
        let mut mgr = crate::IncidentsMgr::new(900);
        let inc = mgr.ingest_alert_json(&alert).unwrap();
        assert_eq!(inc["sources"], json!(["guard"]));
        assert_eq!(inc["exe"], "/tmp/g6_spoof/docker");
        assert_eq!(inc["primary_technique"], "T1610");
        assert_eq!(inc["severity_max"], "high");
        assert_eq!(inc["details"]["guard"], "runtime_socket");
        assert_eq!(inc["details"]["caller_exe"], "/tmp/g6_spoof/docker");
        assert_eq!(inc["details"]["target"], "/run/docker.sock");
        assert_eq!(inc["details"]["mode"], "enforce");
        assert!(inc["title"].as_str().unwrap().starts_with("Guard runtime_socket blocked"));
        // The same caller again in the window adds to the incident.
        let again = mgr.ingest_alert_json(&alert).unwrap();
        assert_eq!(again["alerts_count"], 2);
        assert_eq!(mgr.list().len(), 1);
    }
}

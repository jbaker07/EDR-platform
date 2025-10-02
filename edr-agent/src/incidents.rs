//! incidents.rs
//!
//! Minimal, file-local incident aggregator for the EDR demo.
//! - Groups alerts by (exe, primary MITRE technique, time bucket)
//! - Maintains a rolling map of incident JSON objects the UI can consume
//! - Builds a compact neighborhood graph (±5m) from `state/pcb_snaps.ndjson`
//! - Computes an R-GCN-like proxy score from node trust scores
//!
//! NOTE: This module is self-contained and does not require `forensic_hooks`.
//!       It defines the minimal types it needs locally.

// std
use std::collections::HashMap;
use std::fs;

// third-party
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha1::{Digest, Sha1};

// -----------------------------------------------------------------------------
// Minimal local types (so we don't depend on `forensic_hooks` here)
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GraphNode {
    pub id: String,
    /// 0..1 (1 = fully trusted). We compute risk proxy as (1 - trust_score).
    pub trust_score: f32,

    // optional/handy metadata (kept to match your UI/other code expectations)
    pub uid: Option<u32>,
    pub pid: Option<u32>,
    pub ppid: Option<u32>,
    pub binary_path: Option<String>,
    pub command_line: Option<String>,
    pub cwd: Option<String>,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub anchor_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
}

#[derive(Clone, Debug, Default)]
struct TelemetryRecord {
    pub timestamp: u64,
    pub pid: i32,
    pub ppid: i32,
    pub uid: u32,
    pub binary_path: String,
    pub command_line: String,
    pub cwd: String,
    pub env_vars: Option<HashMap<String, String>>,
    pub tags: Vec<String>,
    pub risk_score: Option<u32>,
}

fn now_ts() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// -----------------------------------------------------------------------------
// Incidents manager
// -----------------------------------------------------------------------------

/// Rolling incident manager. Not thread-safe on its own; wrap with `Mutex`.
pub struct IncidentsMgr {
    /// Grouping bucket size (seconds). Typical: 15 * 60.
    pub window_secs: i64,
    items: HashMap<String, Value>, // id -> incident JSON
}

impl IncidentsMgr {
    pub fn new(window_secs: i64) -> Self {
        Self {
            window_secs,
            items: HashMap::new(),
        }
    }

    /// Return incidents, newest first.
    pub fn list(&self) -> Vec<Value> {
        let mut v: Vec<Value> = self.items.values().cloned().collect();
        v.sort_by(|a, b| {
            let ta = a.get("last_ts").and_then(|x| x.as_i64()).unwrap_or(0);
            let tb = b.get("last_ts").and_then(|x| x.as_i64()).unwrap_or(0);
            tb.cmp(&ta)
        });
        v
    }

    /// Get a single incident by id (if present).
    pub fn get(&self, id: &str) -> Option<Value> {
        self.items.get(id).cloned()
    }

    /// Ingest a single alert JSON (as produced by your Evaluator).
    /// Returns the updated incident JSON (for SSE fanout) if successful.
    pub fn ingest_alert_json(&mut self, alert: &Value) -> Option<Value> {
        // Basic fields
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
        let id = make_incident_id(&exe, &primary_t, bucket);

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

        // Build a small neighborhood graph around this alert time (±300s)
        let recs = window_records(ts, 300);
        let (nodes, edges) = build_graph_from_records(&recs);
        let rgcn = rgcn_proxy_score(&nodes);

        // Create or update incident
        let mut item = self.items.remove(&id).unwrap_or_else(|| {
            json!({
                "id": id,
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
        });

        {
            let obj = item.as_object_mut().expect("incident must be an object");

            // Time bounds
            let first_ts = obj.get("first_ts").and_then(|x| x.as_i64()).unwrap_or(ts);
            let last_ts = obj.get("last_ts").and_then(|x| x.as_i64()).unwrap_or(ts);
            obj.insert("first_ts".into(), json!(first_ts.min(ts)));
            obj.insert("last_ts".into(), json!(last_ts.max(ts)));

            // Counts
            let n = obj
                .get("alerts_count")
                .and_then(|x| x.as_u64())
                .unwrap_or(0);
            obj.insert("alerts_count".into(), json!(n + 1));

            // Severity max (ranked)
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

            // Risk max
            let rmax = obj.get("risk_max").and_then(|x| x.as_f64()).unwrap_or(0.0);
            obj.insert("risk_max".into(), json!(rmax.max(risk)));

            // Merge techniques (dedup)
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

            // Keep small sample of alert ids (if present)
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

            // Refresh graph snapshot & proxy score
            obj.insert("rgcn_score".into(), json!(rgcn));
            obj.insert("graph".into(), json!({ "nodes": nodes, "edges": edges }));
        }

        // Prune stale incidents (simple rolling GC)
        self.prune(ts);

        self.items.insert(
            item.get("id").and_then(|x| x.as_str()).unwrap().to_string(),
            item.clone(),
        );
        Some(item)
    }

    fn prune(&mut self, now_ts_sec: i64) {
        let horizon = now_ts_sec - (self.window_secs * 4); // keep ~4 buckets
        self.items
            .retain(|_, v| v.get("last_ts").and_then(|x| x.as_i64()).unwrap_or(0) >= horizon);
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

/// Compute a stable incident id from (exe, primary technique, bucket).
fn make_incident_id(exe: &str, primary_t: &str, bucket: i64) -> String {
    let mut h = Sha1::new();
    h.update(exe.as_bytes());
    h.update(b"|");
    h.update(primary_t.as_bytes());
    h.update(b"|");
    h.update(bucket.to_string().as_bytes());
    format!("{:x}", h.finalize())
}

/// Rank severities (for "max" comparisons).
pub fn severity_rank(s: &str) -> i32 {
    match s.to_ascii_lowercase().as_str() {
        "high" => 3,
        "medium" => 2,
        "low" => 1,
        _ => 0,
    }
}

/// Pull MITRE technique ids from an alert JSON (labels in findings + top-level "technique").
pub fn extract_mitre(alert: &Value) -> Vec<String> {
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

/// Robust timestamp reader: supports seconds + milliseconds.
fn parse_ts_any(v: &Value, default: i64) -> i64 {
    v.get("timestamp")
        .and_then(|x| x.as_i64())
        .or_else(|| v.get("ts").and_then(|x| x.as_i64()))
        .or_else(|| v.get("time").and_then(|x| x.as_i64()))
        .or_else(|| v.get("ts_ms").and_then(|x| x.as_i64()).map(|ms| ms / 1000))
        .unwrap_or(default)
}

/// Load telemetry window around a center timestamp (±`secs`) from on-disk PCB snapshots.
fn window_records(ts_center: i64, secs: i64) -> Vec<TelemetryRecord> {
    let mut out = Vec::new();
    if let Ok(s) = fs::read_to_string("state/pcb_snaps.ndjson") {
        // Take the last N lines (no double rev required)
        let lines: Vec<&str> = s.lines().collect();
        let start = lines.len().saturating_sub(20_000);
        for &line in &lines[start..] {
            if let Ok(v) = serde_json::from_str::<Value>(line) {
                let ts = parse_ts_any(&v, ts_center);
                if (ts - ts_center).abs() <= secs {
                    let pid = v.get("pid").and_then(|x| x.as_i64()).unwrap_or(0) as i32;
                    let ppid = v.get("ppid").and_then(|x| x.as_i64()).unwrap_or(0) as i32;
                    let exe = v
                        .get("exe")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string();
                    let cmd = v
                        .get("cmdline")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string();
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
    }
    out
}

/// Build a small mixed graph from records (proc/file edges) with trust derived from risk_score.
fn build_graph_from_records(records: &[TelemetryRecord]) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    use std::collections::HashMap as Map;
    let mut nodes: Vec<GraphNode> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();
    let mut proc_id_map: Map<i32, String> = Map::new();

    // pass 1: all proc nodes (ensures PPID edges can resolve)
    for r in records {
        let nid = format!("proc:{}", r.pid);
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
        });
    }

    // pass 2: connect proc->proc and proc->file/dir
    for r in records {
        let nid = format!("proc:{}", r.pid);

        if r.ppid > 0 {
            if let Some(parent) = proc_id_map.get(&r.ppid).cloned() {
                edges.push(GraphEdge {
                    source: parent,
                    target: nid.clone(),
                });
            }
        }
        if !r.binary_path.is_empty() {
            let file_id = format!("file:{}", r.binary_path);
            nodes.push(GraphNode {
                id: file_id.clone(),
                ..Default::default()
            });
            edges.push(GraphEdge {
                source: nid.clone(),
                target: file_id,
            });
        }
        if !r.cwd.is_empty() {
            let dir_id = format!("file:{}", r.cwd);
            nodes.push(GraphNode {
                id: dir_id.clone(),
                ..Default::default()
            });
            edges.push(GraphEdge {
                source: nid.clone(),
                target: dir_id,
            });
        }
    }
    (nodes, edges)
}

/// R-GCN proxy: average (1 - trust_score) across nodes, clamped to [0,1].
fn rgcn_proxy_score(nodes: &[GraphNode]) -> f64 {
    if nodes.is_empty() {
        return 0.0;
    }
    let sum: f64 = nodes
        .iter()
        .map(|n| {
            let t = n.trust_score.max(0.0).min(1.0) as f64;
            1.0 - t
        })
        .sum();
    (sum / (nodes.len() as f64)).max(0.0).min(1.0)
}

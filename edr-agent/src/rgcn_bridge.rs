use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Config {
    pub url: String,
    pub poll_ms: u64,
    pub timeout_ms: u64,
    pub max_nodes: usize,
    pub max_edges: usize,
    pub graph_path: String,
}

#[derive(Default, Clone)]
struct Stats {
    model: String,
    last_ok_ts: u64,
    last_err: Option<String>,
    last_infer_ms: u64,
    nodes_scored: usize,
    calls_total: u64,
}

#[derive(Default)]
struct State {
    scores: HashMap<String, f32>, // node_id -> score
    etag: Option<String>,
    stats: Stats,
}

static STATE: OnceCell<RwLock<State>> = OnceCell::new();
fn state() -> &'static RwLock<State> {
    STATE.get_or_init(|| RwLock::new(State::default()))
}

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn short_etag(s: &str) -> String {
    use sha1::{Digest, Sha1};
    let mut h = Sha1::new();
    h.update(s.as_bytes());
    format!("{:x}", h.finalize())
}

fn build_infer_payload(v: &Value, max_nodes: usize, max_edges: usize) -> Value {
    // Keep essential fields only, and cap size
    let mut nodes_out = Vec::new();
    if let Some(nodes) = v.get("nodes").and_then(|x| x.as_array()) {
        for n in nodes.iter().take(max_nodes) {
            let id = n.get("id").and_then(|x| x.as_str()).unwrap_or("-");
            let kind = n
                .get("kind")
                .or_else(|| n.get("type"))
                .and_then(|x| x.as_str())
                .unwrap_or_else(|| {
                    if id.starts_with("proc:") {
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
                    }
                });
            let last_seen = n
                .get("last_seen")
                .or_else(|| n.get("ts"))
                .or_else(|| v.get("ts"))
                .and_then(|x| x.as_i64())
                .unwrap_or(now_ts() as i64);
            let trust = n
                .get("trust_score")
                .and_then(|x| x.as_f64())
                .unwrap_or(1.0);
            nodes_out.push(json!({
                "id": id, "kind": kind, "last_seen": last_seen, "trust_score": trust
            }));
        }
    }

    let mut edges_out = Vec::new();
    if let Some(edges) = v
        .get("edges")
        .or_else(|| v.get("links"))
        .and_then(|x| x.as_array())
    {
        for e in edges.iter().take(max_edges) {
            let s = e.get("source").and_then(|x| x.as_str()).unwrap_or("-");
            let t = e.get("target").and_then(|x| x.as_str()).unwrap_or("-");
            let rel = e.get("rel").and_then(|x| x.as_str()).unwrap_or_else(|| {
                // fallback from edge kind
                match e.get("kind").and_then(|x| x.as_str()).unwrap_or("") {
                    "EXEC" => "spawned",
                    "CONNECT" => "connected-to",
                    "READ" | "WRITE" => "accessed",
                    _ => "linked-to",
                }
            });
            let last_seen = e
                .get("last_seen")
                .and_then(|x| x.as_i64())
                .unwrap_or(now_ts() as i64);
            edges_out.push(json!({
                "source": s,
                "target": t,
                "rel": rel,
                "last_seen": last_seen
            }));
        }
    }

    json!({
        "ts": v.get("ts").and_then(|x| x.as_i64()).unwrap_or(now_ts() as i64),
        "nodes": nodes_out,
        "edges": edges_out
    })
}

pub fn annotate_graph(graph: &mut Value) {
    let st = state().read();
    if let Some(nodes) = graph.get_mut("nodes").and_then(|x| x.as_array_mut()) {
        for n in nodes {
            if let Some(id) = n.get("id").and_then(|x| x.as_str()) {
                if let Some(sc) = st.scores.get(id) {
                    n.as_object_mut()
                        .unwrap()
                        .insert("rgcn_score".into(), json!(*sc));
                }
            }
        }
    }
}

pub fn rgcn_metrics() -> Value {
    let st = state().read();
    json!({
        "rgcn_model": st.stats.model,
        "rgcn_last_ok_ts": st.stats.last_ok_ts,
        "rgcn_infer_ms": st.stats.last_infer_ms,
        "rgcn_nodes_scored": st.stats.nodes_scored,
        "rgcn_calls_total": st.stats.calls_total,
        "rgcn_last_err": st.stats.last_err
    })
}

// Helper: read, annotate with scores, write back (async, no locks held during await)
async fn annotate_live_graph_file(path: &str, scores: &HashMap<String, f32>) -> anyhow::Result<()> {
    let s = tokio::fs::read_to_string(path).await?;
    if s.is_empty() {
        return Ok(());
    }
    let mut v: Value = serde_json::from_str(&s)?;
    if let Some(nodes) = v.get_mut("nodes").and_then(|x| x.as_array_mut()) {
        for n in nodes {
            if let Some(id) = n.get("id").and_then(|x| x.as_str()) {
                if let Some(sc) = scores.get(id) {
                    n.as_object_mut()
                        .unwrap()
                        .insert("rgcn_score".into(), json!(*sc));
                }
            }
        }
    }
    let out = serde_json::to_vec_pretty(&v)?;
    tokio::fs::write(path, out).await?;
    Ok(())
}

// Accept both {"scores":{...}} and {...} shapes
#[derive(Deserialize)]
struct InferOut {
    scores: HashMap<String, f32>,
}

pub fn start_rgcn_bridge(cfg: Config) {
    let _ = state(); // init cell
    tokio::spawn(async move {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(cfg.timeout_ms))
            .build()
            .expect("rgcn_bridge: reqwest client");

        let mut last_sent_etag: Option<String> = None;

        loop {
            tokio::time::sleep(Duration::from_millis(cfg.poll_ms)).await;

            // 1) Read live graph (async, no locks)
            let s = match tokio::fs::read_to_string(&cfg.graph_path).await {
                Ok(x) => x,
                Err(_) => continue,
            };
            if s.is_empty() {
                continue;
            }

            // Avoid redundant work if file unchanged
            let et = short_etag(&s);
            if Some(et.clone()) == last_sent_etag {
                continue;
            }

            // 2) Build compact payload (no await)
            let v: Value = match serde_json::from_str(&s) {
                Ok(x) => x,
                Err(_) => continue,
            };
            let payload = build_infer_payload(&v, cfg.max_nodes, cfg.max_edges);

            // 3) Call sidecar (awaits with NO locks held)
            let t0 = std::time::Instant::now();
            let resp = match client.post(&cfg.url).json(&payload).send().await {
                Ok(r) => r,
                Err(e) => {
                    // record error (after await, acquire short-lived lock)
                    let mut st = state().write();
                    st.stats.calls_total += 1;
                    st.stats.last_err = Some(format!("reqwest {}", e));
                    continue;
                }
            };
            let elapsed = t0.elapsed().as_millis() as u64;

            // 3b) Parse body into a HashMap<String, f32>
            let text = match resp.text().await {
                Ok(t) => t,
                Err(e) => {
                    let mut st = state().write();
                    st.stats.calls_total += 1;
                    st.stats.last_err = Some(format!("http body {}", e));
                    continue;
                }
            };

            let map: HashMap<String, f32> = match serde_json::from_str::<InferOut>(&text)
                .map(|o| o.scores)
                .or_else(|_| serde_json::from_str::<HashMap<String, f32>>(&text))
            {
                Ok(m) => m,
                Err(e) => {
                    let mut st = state().write();
                    st.stats.calls_total += 1;
                    st.stats.last_err = Some(format!("json parse {}", e));
                    continue;
                }
            };

            // 4) Update in-memory state (short critical section, no await inside)
            {
                let mut st = state().write();
                st.stats.calls_total += 1;
                st.scores = map.clone();
                st.stats.nodes_scored = map.len();
                st.stats.last_infer_ms = elapsed;
                st.stats.last_ok_ts = now_ts();
                st.stats.last_err = None;
                // Optional: set model name if your sidecar provides it via /healthz elsewhere.
            }

            // 5) Annotate the live graph file so SSE/UI can consume rgcn_score
            if let Err(e) = annotate_live_graph_file(&cfg.graph_path, &map).await {
                eprintln!("rgcn_bridge: annotate_live_graph_file failed: {e}");
            } else {
                last_sent_etag = Some(et.clone());
            }
        }
    });
}

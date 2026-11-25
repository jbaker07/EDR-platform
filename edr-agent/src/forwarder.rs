// src/forwarder.rs
// Sends the agent's incidents to the backend, so the dashboard shows them next to the
// backend's own alerts. Every incident the agent creates or updates is queued here by
// id, newest version wins, and a task posts the queue every EDR_FORWARD_INTERVAL_SECS
// (default 5) to POST <EDR_BACKEND_URL>/api/incidents with the backend's bearer token.
// A failed post keeps the batch for the next round, unless a newer version of an
// incident arrived meanwhile.
//
// Configuration:
//   EDR_BACKEND_URL           backend base URL, such as https://edr.example:8000. Unset
//                             means no forwarding
//   EDR_API_TOKEN_FILE        the backend token in an owner-only file, re-read before
//                             every post so a rotation needs no restart
//   EDR_API_TOKEN             the same token from the environment
//   EDR_FORWARD_ALLOW_HTTP    1 allows plain http to a host that is not loopback. The token
//                             would cross the network in clear text, so it is off by default
//
// The payload leaves out the incident's graph snapshot and keeps the fields the backend
// stores: id, host, title, sources, exe, techniques, severity, risk, counts, times and
// the details block.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use serde_json::{json, Value};

use crate::utils::time::now_ts;

const MAX_PENDING: usize = 2000;
const MAX_BATCH: usize = 200;
pub const MIN_TOKEN_LENGTH: usize = 32;

#[derive(Clone, Debug)]
pub enum TokenSource {
    Value(String),
    File(PathBuf),
}

impl TokenSource {
    pub fn read(&self) -> Result<String, String> {
        let t = match self {
            Self::Value(v) => v.clone(),
            Self::File(p) => crate::agent_api::read_token_file(p)
                .map_err(|e| e.replace(crate::agent_api::TOKEN_FILE_ENV, "EDR_API_TOKEN_FILE"))?,
        };
        if t.chars().count() < MIN_TOKEN_LENGTH {
            return Err(format!("the backend token is shorter than {MIN_TOKEN_LENGTH} characters"));
        }
        Ok(t)
    }
}

#[derive(Clone, Debug)]
pub struct ForwardConfig {
    pub base_url: String,
    pub host: String,
    pub port: u16,
    pub token: TokenSource,
    pub interval: Duration,
}

impl ForwardConfig {
    pub fn endpoint(&self) -> String {
        format!("{}/api/incidents", self.base_url.trim_end_matches('/'))
    }

    /// The backend's addresses, which stay reachable while the host is isolated.
    pub fn resolved(&self) -> Vec<(std::net::IpAddr, u16)> {
        use std::net::ToSocketAddrs;
        (self.host.as_str(), self.port)
            .to_socket_addrs()
            .map(|it| it.map(|a| (a.ip(), a.port())).collect())
            .unwrap_or_default()
    }
}

fn is_loopback_host(host: &str) -> bool {
    host == "localhost"
        || host
            .trim_matches(|c| c == '[' || c == ']')
            .parse::<std::net::IpAddr>()
            .map(|ip| ip.is_loopback())
            .unwrap_or(false)
}

/// Reads the environment. Ok(None) when EDR_BACKEND_URL is not set.
pub fn config_from(var: impl Fn(&str) -> Option<String>) -> Result<Option<ForwardConfig>, String> {
    let Some(url) = var("EDR_BACKEND_URL").filter(|v| !v.trim().is_empty()) else {
        return Ok(None);
    };
    let url = url.trim().trim_end_matches('/').to_string();
    let (scheme, rest) = url
        .split_once("://")
        .ok_or_else(|| format!("EDR_BACKEND_URL {url} has no scheme"))?;
    let authority = rest.split('/').next().unwrap_or("");
    if authority.contains('@') {
        return Err("EDR_BACKEND_URL must not hold credentials, use EDR_API_TOKEN_FILE".into());
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((h, p)) if !h.ends_with(']') || h.starts_with('[') => match p.parse::<u16>() {
            Ok(p) => (h.to_string(), p),
            Err(_) => (authority.to_string(), if scheme == "https" { 443 } else { 80 }),
        },
        _ => (authority.to_string(), if scheme == "https" { 443 } else { 80 }),
    };
    match scheme {
        "https" => {}
        "http" if is_loopback_host(&host) => {}
        "http" if var("EDR_FORWARD_ALLOW_HTTP").as_deref() == Some("1") => {}
        "http" => {
            return Err(format!(
                "EDR_BACKEND_URL {url} is plain http to another host, so the token would cross \
                 the network in clear text. Use https, or EDR_FORWARD_ALLOW_HTTP=1 on a test network"
            ))
        }
        other => return Err(format!("EDR_BACKEND_URL scheme {other} is not http or https")),
    }
    let token = match (var("EDR_API_TOKEN_FILE"), var("EDR_API_TOKEN")) {
        (Some(f), _) if !f.is_empty() => TokenSource::File(PathBuf::from(f)),
        (_, Some(t)) if !t.is_empty() => TokenSource::Value(t),
        _ => return Err("EDR_BACKEND_URL is set but neither EDR_API_TOKEN_FILE nor EDR_API_TOKEN is".into()),
    };
    token.read()?;
    let interval = var("EDR_FORWARD_INTERVAL_SECS")
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(5);
    Ok(Some(ForwardConfig {
        base_url: url,
        host: host.trim_matches(|c| c == '[' || c == ']').to_string(),
        port,
        token,
        interval: Duration::from_secs(interval),
    }))
}

// ---- Queue ----

static ENABLED: AtomicBool = AtomicBool::new(false);
static PENDING: Mutex<BTreeMap<String, Value>> = Mutex::new(BTreeMap::new());
static SENT: AtomicU64 = AtomicU64::new(0);
static FAILED: AtomicU64 = AtomicU64::new(0);
static DROPPED: AtomicU64 = AtomicU64::new(0);
static LAST_OK: AtomicU64 = AtomicU64::new(0);
static LAST_ERROR: Mutex<String> = Mutex::new(String::new());

/// The part of an incident the backend stores.
pub fn compact(inc: &Value) -> Value {
    let pick = |k: &str| inc.get(k).cloned().unwrap_or(Value::Null);
    json!({
        "id": pick("id"),
        "host": pick("host"),
        "title": pick("title"),
        "sources": pick("sources"),
        "exe": pick("exe"),
        "primary_technique": pick("primary_technique"),
        "techniques": pick("techniques"),
        "severity": pick("severity_max"),
        "risk": pick("risk_max"),
        "alerts_count": pick("alerts_count"),
        "first_ts": pick("first_ts"),
        "last_ts": pick("last_ts"),
        "details": pick("details"),
    })
}

/// Queues the newest version of an incident. Does nothing when forwarding is off.
pub fn queue_incident(inc: &Value) {
    if !ENABLED.load(Ordering::Relaxed) {
        return;
    }
    queue_compact(compact(inc));
}

fn queue_compact(c: Value) {
    let Some(id) = c.get("id").and_then(|v| v.as_str()).map(str::to_string) else {
        return;
    };
    let mut q = PENDING.lock().unwrap_or_else(|e| e.into_inner());
    q.insert(id, c);
    while q.len() > MAX_PENDING {
        // Drop the incident that changed longest ago.
        let oldest = q
            .iter()
            .min_by_key(|(_, v)| v.get("last_ts").and_then(|t| t.as_i64()).unwrap_or(0))
            .map(|(k, _)| k.clone());
        if let Some(k) = oldest {
            q.remove(&k);
            DROPPED.fetch_add(1, Ordering::Relaxed);
        }
    }
}

fn take_batch() -> Vec<Value> {
    let mut q = PENDING.lock().unwrap_or_else(|e| e.into_inner());
    let keys: Vec<String> = q.keys().take(MAX_BATCH).cloned().collect();
    keys.iter().filter_map(|k| q.remove(k)).collect()
}

/// A failed batch goes back, except incidents that have a newer version queued.
fn requeue(batch: Vec<Value>) {
    let mut q = PENDING.lock().unwrap_or_else(|e| e.into_inner());
    for c in batch {
        if let Some(id) = c.get("id").and_then(|v| v.as_str()) {
            q.entry(id.to_string()).or_insert(c);
        }
    }
}

pub fn payload(host: &str, incidents: Vec<Value>) -> Value {
    json!({
        "agent": { "host": host, "version": env!("CARGO_PKG_VERSION") },
        "incidents": incidents,
    })
}

pub fn stats() -> Value {
    json!({
        "enabled": ENABLED.load(Ordering::Relaxed),
        "pending": PENDING.lock().map(|q| q.len()).unwrap_or(0),
        "sent_total": SENT.load(Ordering::Relaxed),
        "failed_posts_total": FAILED.load(Ordering::Relaxed),
        "dropped_total": DROPPED.load(Ordering::Relaxed),
        "last_ok_ts": LAST_OK.load(Ordering::Relaxed),
        "last_error": LAST_ERROR.lock().map(|e| e.clone()).unwrap_or_default(),
    })
}

/// One post of `batch`. Errors carry the status and a short body, never the token.
pub async fn post_batch(client: &reqwest::Client, cfg: &ForwardConfig, host: &str, batch: &[Value]) -> Result<(), String> {
    let token = cfg.token.read()?;
    let resp = client
        .post(cfg.endpoint())
        .bearer_auth(token)
        .json(&payload(host, batch.to_vec()))
        .send()
        .await
        .map_err(|e| format!("post to {}: {e}", cfg.endpoint()))?;
    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status();
        let body: String = resp.text().await.unwrap_or_default().chars().take(200).collect();
        Err(format!("backend answered {status}: {body}"))
    }
}

/// Turns forwarding on and runs the post loop. Call once from the tokio runtime.
pub async fn run(cfg: ForwardConfig) {
    ENABLED.store(true, Ordering::Relaxed);
    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
    let client = match reqwest::Client::builder().timeout(Duration::from_secs(10)).build() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ incident forwarding off: {e}");
            ENABLED.store(false, Ordering::Relaxed);
            return;
        }
    };
    eprintln!("incidents are forwarded to {} every {:?}", cfg.endpoint(), cfg.interval);
    let mut warned_at = 0u64;
    loop {
        tokio::time::sleep(cfg.interval).await;
        loop {
            let batch = take_batch();
            if batch.is_empty() {
                break;
            }
            let n = batch.len() as u64;
            match post_batch(&client, &cfg, &host, &batch).await {
                Ok(()) => {
                    SENT.fetch_add(n, Ordering::Relaxed);
                    LAST_OK.store(now_ts(), Ordering::Relaxed);
                }
                Err(e) => {
                    FAILED.fetch_add(1, Ordering::Relaxed);
                    if let Ok(mut le) = LAST_ERROR.lock() {
                        *le = e.clone();
                    }
                    let now = now_ts();
                    if now.saturating_sub(warned_at) >= 60 {
                        warned_at = now;
                        eprintln!("⚠️ incident forwarding: {e} (will retry)");
                    }
                    requeue(batch);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn vars(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let m: HashMap<String, String> = pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        move |k: &str| m.get(k).cloned()
    }

    const TOK: &str = "backend-token-0123456789-abcdefghijkl";

    #[test]
    fn config_refuses_clear_text_tokens_to_remote_hosts() {
        assert!(config_from(vars(&[])).unwrap().is_none());
        let ok = config_from(vars(&[("EDR_BACKEND_URL", "http://127.0.0.1:8000/"), ("EDR_API_TOKEN", TOK)])).unwrap().unwrap();
        assert_eq!(ok.endpoint(), "http://127.0.0.1:8000/api/incidents");
        assert_eq!((ok.host.as_str(), ok.port), ("127.0.0.1", 8000));
        let https = config_from(vars(&[("EDR_BACKEND_URL", "https://edr.example"), ("EDR_API_TOKEN", TOK)])).unwrap().unwrap();
        assert_eq!(https.port, 443);
        let err = config_from(vars(&[("EDR_BACKEND_URL", "http://edr.example:8000"), ("EDR_API_TOKEN", TOK)])).unwrap_err();
        assert!(err.contains("clear text"), "{err}");
        assert!(config_from(vars(&[("EDR_BACKEND_URL", "http://edr.example:8000"), ("EDR_API_TOKEN", TOK), ("EDR_FORWARD_ALLOW_HTTP", "1")])).unwrap().is_some());
        assert!(config_from(vars(&[("EDR_BACKEND_URL", "http://127.0.0.1:8000")])).unwrap_err().contains("EDR_API_TOKEN"));
        assert!(config_from(vars(&[("EDR_BACKEND_URL", "http://127.0.0.1:8000"), ("EDR_API_TOKEN", "short")])).unwrap_err().contains("shorter"));
        assert!(config_from(vars(&[("EDR_BACKEND_URL", "https://u:p@edr.example"), ("EDR_API_TOKEN", TOK)])).unwrap_err().contains("credentials"));
    }

    /// The shape the agent sends is the shape the backend test posts: both read
    /// tests/fixtures/agent_incident_batch.json.
    #[test]
    fn the_payload_matches_the_shared_fixture() {
        let fixture: Value = serde_json::from_str(include_str!("../tests/fixtures/agent_incident_batch.json")).unwrap();
        let incident = json!({
            "id": "4f9c2d0e7a1b3c5d6e8f90a1b2c3d4e5f6a7b8c9",
            "host": "edr-lab-1",
            "title": "Guard runtime_socket blocked: /run/docker.sock",
            "sources": ["guard"],
            "exe": "/tmp/g6_spoof/docker",
            "primary_technique": "T1610",
            "techniques": ["T1610", "T1611"],
            "severity_max": "high",
            "risk_max": 0.6,
            "alerts_count": 2,
            "first_ts": 1800000000,
            "last_ts": 1800000004,
            "bucket": 2000000,
            "sample_ids": ["guard:runtime_socket:blocked:4321:1800000000"],
            "rgcn_score": 0.0,
            "graph": { "nodes": [1, 2, 3], "edges": [] },
            "details": {
                "guard": "runtime_socket",
                "hook": "unix_stream_connect",
                "action": "blocked",
                "mode": "enforce",
                "caller_exe": "/tmp/g6_spoof/docker",
                "caller_pid": "4321",
                "caller_comm": "docker",
                "target": "/run/docker.sock",
                "reason": "blocked: docker (pid 4321) connect() to /run/docker.sock"
            }
        });
        let mut sent = payload("edr-lab-1", vec![compact(&incident)]);
        sent["agent"]["version"] = json!("0.1.0");
        assert_eq!(sent, fixture, "the graph, bucket and samples stay on the agent");
    }

    #[test]
    fn the_queue_keeps_the_newest_version_and_failed_batches_come_back() {
        let inc = |id: &str, n: u64, ts: i64| json!({ "id": id, "alerts_count": n, "last_ts": ts });
        queue_compact(compact(&inc("fwd-test-a", 1, 10)));
        queue_compact(compact(&inc("fwd-test-a", 2, 11)));
        queue_compact(compact(&inc("fwd-test-b", 1, 12)));
        let batch: Vec<Value> = take_batch().into_iter().filter(|v| v["id"].as_str().unwrap().starts_with("fwd-test")).collect();
        assert_eq!(batch.len(), 2);
        assert_eq!(batch[0]["alerts_count"], 2, "newest version");
        // While the post failed, incident a changed again: the newer one is kept.
        queue_compact(compact(&inc("fwd-test-a", 3, 13)));
        requeue(batch);
        let again: Vec<Value> = take_batch().into_iter().filter(|v| v["id"].as_str().unwrap().starts_with("fwd-test")).collect();
        assert_eq!(again.len(), 2);
        assert_eq!(again.iter().find(|v| v["id"] == "fwd-test-a").unwrap()["alerts_count"], 3);
    }

    /// The real post against a local HTTP listener: the bearer token goes in the
    /// Authorization header and the body is the payload.
    #[tokio::test]
    async fn post_sends_the_bearer_token_and_the_payload() {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = std::thread::spawn(move || {
            let (mut s, _) = listener.accept().unwrap();
            s.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
            let mut buf = Vec::new();
            let mut chunk = [0u8; 4096];
            loop {
                let n = s.read(&mut chunk).unwrap_or(0);
                if n == 0 {
                    break;
                }
                buf.extend_from_slice(&chunk[..n]);
                let text = String::from_utf8_lossy(&buf);
                if let Some(h) = text.find("\r\n\r\n") {
                    let len = text[..h]
                        .lines()
                        .find_map(|l| l.to_ascii_lowercase().strip_prefix("content-length:").map(|v| v.trim().parse::<usize>().unwrap_or(0)))
                        .unwrap_or(0);
                    if buf.len() >= h + 4 + len {
                        break;
                    }
                }
            }
            s.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}").unwrap();
            String::from_utf8_lossy(&buf).into_owned()
        });
        let cfg = config_from(vars(&[("EDR_BACKEND_URL", &format!("http://127.0.0.1:{port}")), ("EDR_API_TOKEN", TOK)])).unwrap().unwrap();
        let client = reqwest::Client::new();
        post_batch(&client, &cfg, "h1", &[json!({ "id": "x1", "title": "t" })]).await.unwrap();
        let req = server.join().unwrap();
        assert!(req.starts_with("POST /api/incidents HTTP/1.1"), "{req}");
        assert!(req.contains(&format!("authorization: Bearer {TOK}")) || req.contains(&format!("Authorization: Bearer {TOK}")));
        assert!(req.contains("\"incidents\":[{\"id\":\"x1\",\"title\":\"t\"}]"), "{req}");
    }
}

use parking_lot::RwLock;
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use tokio::{fs, task::JoinHandle, time::{interval, Duration}};

#[derive(Serialize, Clone)]
pub struct Node {
    pub id: String,          // "proc:1234", "file:/etc/passwd", "ip:1.2.3.4"
    pub label: String,       // "bash(1234)", "/etc/passwd", "1.2.3.4:443"
    pub kind: String,        // "proc" | "file" | "ip"
    pub score: f64,          // fused anomaly score (>=0)
    pub confidence: f64,     // [0,1]
    pub last_seen: u64,      // unix secs
}

#[derive(Serialize, Clone)]
pub struct Link {
    pub source: String,      // node.id
    pub target: String,      // node.id
    pub kind: String,        // "READ","WRITE","CONNECT","FORK","EXEC"
    pub weight: f64,         // simple intensity counter
    pub last_seen: u64,
}

#[derive(Serialize)]
pub struct Snapshot {
    pub ts: u64,
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
}

pub struct GraphState {
    nodes: HashMap<String, Node>,
    links: HashMap<String, Link>, // key: source|target|kind
    max_age_secs: u64,
}

impl GraphState {
    pub fn new(max_age_secs: u64) -> Self {
        Self { nodes: HashMap::new(), links: HashMap::new(), max_age_secs }
    }

    fn now() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    fn upsert_node(&mut self, id: String, label: String, kind: &str, score: f64, conf: f64, now: u64) {
        let e = self.nodes.entry(id.clone()).or_insert(Node {
            id, label, kind: kind.to_string(), score: 0.0, confidence: 0.0, last_seen: now,
        });
        // keep max score/conf for quick “hot” highlighting; update label if better
        e.last_seen = now;
        if score > e.score { e.score = score; }
        if conf > e.confidence { e.confidence = conf; }
    }

    fn upsert_link(&mut self, src: &str, dst: &str, kind: &str, bump: f64, now: u64) {
        let key = format!("{src}|{dst}|{kind}");
        let e = self.links.entry(key).or_insert(Link {
            source: src.to_string(), target: dst.to_string(), kind: kind.to_string(),
            weight: 0.0, last_seen: now,
        });
        e.weight += bump;
        e.last_seen = now;
    }

    pub fn ingest_file_access(&mut self, pid: u32, comm: &str, path: &str, is_write: bool, score: f64, conf: f64) {
        let now = Self::now();
        let p_id = format!("proc:{pid}");
        self.upsert_node(p_id.clone(), format!("{comm}({pid})"), "proc", score, conf, now);
        let f_id = format!("file:{path}");
        self.upsert_node(f_id.clone(), path.to_string(), "file", 0.0, 0.0, now);
        self.upsert_link(&p_id, &f_id, if is_write { "WRITE" } else { "READ" }, 1.0, now);
    }

    pub fn ingest_connect(&mut self, pid: u32, comm: &str, ipport: &str, score: f64, conf: f64) {
        let now = Self::now();
        let p_id = format!("proc:{pid}");
        self.upsert_node(p_id.clone(), format!("{comm}({pid})"), "proc", score, conf, now);
        let i_id = format!("ip:{ipport}");
        self.upsert_node(i_id.clone(), ipport.to_string(), "ip", 0.0, 0.0, now);
        self.upsert_link(&p_id, &i_id, "CONNECT", 1.0, now);
    }

    pub fn ingest_exec(&mut self, ppid: u32, parent_comm: &str, pid: u32, comm: &str, score: f64, conf: f64) {
        let now = Self::now();
        let parent_id = format!("proc:{ppid}");
        self.upsert_node(parent_id.clone(), format!("{parent_comm}({ppid})"), "proc", 0.0, 0.0, now);
        let child_id = format!("proc:{pid}");
        self.upsert_node(child_id.clone(), format!("{comm}({pid})"), "proc", score, conf, now);
        self.upsert_link(&parent_id, &child_id, "EXEC", 1.0, now);
    }

    fn gc(&mut self, now: u64) {
        let max_age = self.max_age_secs;
        self.nodes.retain(|_, n| now - n.last_seen <= max_age);
        self.links.retain(|_, e| now - e.last_seen <= max_age);
    }

    pub fn snapshot(&mut self) -> Snapshot {
        let now = Self::now();
        self.gc(now);
        Snapshot {
            ts: now,
            nodes: self.nodes.values().cloned().collect(),
            links: self.links.values().cloned().collect(),
        }
    }
}

pub fn spawn_writer(state: Arc<RwLock<GraphState>>, out_path: PathBuf) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_secs(1));
        loop {
            tick.tick().await;
            let snap = { state.write().snapshot() };
            if let Ok(json) = serde_json::to_vec_pretty(&snap) {
                let _ = fs::write(&out_path, json).await;
            }
        }
    })
}

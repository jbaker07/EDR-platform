// src/pb_introspect.rs
// Introspection layer for Playbook pending/hits + metrics + SSE broadcast fanout.
// This does NOT implement matching logic; your pb_engine / main should call the
// helpers here (push_hit / add_or_update_pending / remove_pending / set_sse_sender).

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, OnceLock};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookHit {
    pub id: String,
    pub playbook_id: String,
    pub ts: u64,
    pub filled_slots: Vec<String>,
    pub missing_slots: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub rationale: Vec<String>,
}

/// Matches what main.rs constructs in several places:
/// pb_introspect::add_or_update_pending(pb_introspect::PendingEvidence { ... })
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingEvidence {
    pub id: String,          // stable id for this pending evidence
    pub playbook_id: String, // which playbook this pending entry belongs to
    pub ts: u64,             // when it was created/observed
    pub slot_id: String,     // which slot is being filled / awaited
    pub fact: String,        // short source/fact label (e.g., "Net", "FileIO", "Exec")
    pub summary: String,     // short human summary
    #[serde(default)]
    pub tags: Vec<String>, // ["IOC","YARA",...]
    pub ttl_sec: u64,        // suggested time to keep it visible
}

#[derive(Default)]
struct Store {
    // ring buffer of last N hits
    hits: VecDeque<PlaybookHit>,
    hits_total: u64,
    // id -> pending evidence
    pending: HashMap<String, PendingEvidence>,
    // config
    cap_hits: usize,
}

static STORE: OnceLock<Mutex<Store>> = OnceLock::new();
fn store() -> &'static Mutex<Store> {
    STORE.get_or_init(|| {
        Mutex::new(Store {
            cap_hits: 1000,
            ..Default::default()
        })
    })
}

static PB_TX: OnceLock<broadcast::Sender<String>> = OnceLock::new();

pub fn set_sse_sender(tx: broadcast::Sender<String>) {
    // only set once
    let _ = PB_TX.set(tx);
}

pub fn push_hit(hit: PlaybookHit) {
    {
        let mut s = store().lock().unwrap();
        s.hits_total += 1;
        if s.hits.len() >= s.cap_hits {
            s.hits.pop_front();
        }
        s.hits.push_back(hit.clone());
    }
    // best-effort broadcast (JSON line per hit)
    if let Some(tx) = PB_TX.get() {
        let _ = tx.send(serde_json::to_string(&hit).unwrap_or_default());
    }
}

/// Replace the entire pending set (useful if an upstream engine recomputes a snapshot)
pub fn set_pending_snapshot(list: Vec<PendingEvidence>) {
    let mut s = store().lock().unwrap();
    s.pending.clear();
    for it in list {
        s.pending.insert(it.id.clone(), it);
    }
}

/// Upsert a single pending evidence entry by id.
pub fn add_or_update_pending(p: PendingEvidence) {
    let mut s = store().lock().unwrap();
    s.pending.insert(p.id.clone(), p);
}

/// Remove a pending entry by id.
pub fn remove_pending(id: &str) {
    let mut s = store().lock().unwrap();
    s.pending.remove(id);
}

/// Fetch latest hits (most-recent first if `limit` < total; otherwise chronological)
pub fn get_hits(limit: usize) -> Vec<PlaybookHit> {
    let s = store().lock().unwrap();
    if limit == 0 || limit >= s.hits.len() {
        return s.hits.iter().cloned().collect();
    }
    s.hits
        .iter()
        .rev()
        .take(limit)
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

/// Current pending entries (unordered).
pub fn get_pending() -> Vec<PendingEvidence> {
    let s = store().lock().unwrap();
    s.pending.values().cloned().collect()
}

#[derive(Debug, Clone, Serialize)]
pub struct PbMetrics {
    pub pb_hits_total: u64,
    pub pb_pending_current: usize,
}

pub fn metrics() -> PbMetrics {
    let s = store().lock().unwrap();
    PbMetrics {
        pb_hits_total: s.hits_total,
        pb_pending_current: s.pending.len(),
    }
}

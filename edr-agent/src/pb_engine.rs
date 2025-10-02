// src/pb_engine.rs
// A pragmatic playbook engine for Linux ransomware & more.
// Loads YAML playbooks, tracks slot fulfillment in sliding windows, performs
// simple predicate evaluation (eq, ne, in, contains, regex, gte, lte), and
// derives 'file_burst' aggregates for mass FileIO write/rename activity.
//
// Required deps in Cargo.toml:
//   serde = { version = "1", features = ["derive"] }
//   serde_yaml = "0.9"
//   regex = "1"
//   anyhow = "1"
//   parking_lot = "0.12"

use anyhow::{anyhow, Result};
use regex::Regex;
use serde::Deserialize;
use serde_yaml::Value as Yaml;
use std::collections::{HashMap, VecDeque};
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Mirror a subset of your DecisionEngine to create Signals without circular deps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetectorKind {
    Playbook,
}

#[derive(Debug, Clone)]
pub struct Signal {
    pub detector: DetectorKind,
    pub score: f32,
    pub ts: u64,
    pub exe: Option<String>,
    pub user: Option<String>,
    pub host: Option<String>,
    pub cmd_prefix: Option<String>,
    pub parent: Option<String>,
    pub signer: Option<String>,
    pub hash: Option<String>,
    pub pid: Option<i32>,
    pub tags: Vec<String>,
}

// ---- Fact model (normalized telemetry snippets) ----

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FactKind {
    Exec,
    FileIO,
    NetConn,
    Auth,
    Kernel,
    VendorAlert,
    Agg, // derived aggregates (file_burst)
}

#[derive(Debug, Clone)]
pub struct Fact {
    pub ts: u64,
    pub kind: FactKind,
    pub host: String,
    pub user: Option<String>,
    pub exe: Option<String>,
    pub cmd: Option<String>,
    pub fields: HashMap<String, String>,
}

impl Fact {
    pub fn get_str(&self, k: &str) -> Option<&str> {
        if k == "exe" {
            return self.exe.as_deref();
        }
        if k == "cmd" {
            return self.cmd.as_deref();
        }
        if k == "user" {
            return self.user.as_deref();
        }
        if k == "host" {
            return Some(&self.host);
        }
        self.fields.get(k).map(|s| s.as_str())
    }
    pub fn get_f64(&self, k: &str) -> Option<f64> {
        self.get_str(k).and_then(|v| v.parse::<f64>().ok())
    }
}

// ---- YAML types ----

#[derive(Debug, Clone, Deserialize)]
pub struct PredicateSpec {
    pub field: String,
    pub op: String,
    pub value: Yaml,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SlotSpec {
    pub slot_id: String,
    pub slot_ttl_sec: u64,
    pub fact: String,
    // YAML key is "where" → map to field `where_`
    #[serde(default, rename = "where")]
    pub where_: Vec<PredicateSpec>,
    #[serde(default)]
    pub derive_tags: Vec<String>, // applied when this slot matches
}

#[derive(Debug, Clone, Deserialize)]
pub struct VendorMapEntry {
    pub product: String,
    pub alert_type: String,
    pub fills_slot: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExplainSpec {
    pub primary_ttp: Option<String>,
    pub narrative: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlaybookSpec {
    pub id: String,
    pub version: Option<String>,
    pub name: String,
    pub severity: String,
    pub description: Option<String>,
    pub window_sec: u64,
    pub cooldown_sec: u64,
    pub required: Vec<SlotSpec>,
    #[serde(default)]
    pub optional: Vec<SlotSpec>,
    #[serde(default)]
    pub forbidden: Vec<SlotSpec>,
    #[serde(default)]
    pub vendor_map: Vec<VendorMapEntry>,
    #[serde(default)]
    pub explain: Option<ExplainSpec>,
}

// ---- Runtime state ----

#[derive(Debug, Clone)]
struct SlotHit {
    ts: u64,
    tags: Vec<String>,
    fact: Fact,
}

#[derive(Debug)]
struct Hypothesis {
    created: u64,
    hits: HashMap<String, SlotHit>, // slot_id -> hit
    last_updated: u64,
}

impl Hypothesis {
    fn new(now: u64) -> Self {
        Self {
            created: now,
            hits: HashMap::new(),
            last_updated: now,
        }
    }
    fn is_complete(&self, pb: &PlaybookSpec) -> bool {
        pb.required
            .iter()
            .all(|s| self.hits.contains_key(&s.slot_id))
    }
    fn violates_forbidden(&self, pb: &PlaybookSpec, eval: &Evaluator) -> bool {
        pb.forbidden.iter().any(|slot| {
            self.hits
                .get(&slot.slot_id)
                .map(|h| eval.match_slot(slot, &h.fact))
                .unwrap_or(false)
        })
    }
    fn age(&self, now: u64) -> u64 {
        now.saturating_sub(self.created)
    }
}

// Basic predicate evaluator
struct Evaluator {
    regex_cache: parking_lot::RwLock<HashMap<String, Regex>>,
}

impl Evaluator {
    fn new() -> Self {
        Self {
            regex_cache: parking_lot::RwLock::new(HashMap::new()),
        }
    }
    fn re(&self, pat: &str) -> Option<Regex> {
        if let Some(r) = self.regex_cache.read().get(pat) {
            return Some(r.clone());
        }
        if let Ok(r) = Regex::new(pat) {
            self.regex_cache.write().insert(pat.to_string(), r.clone());
            Some(r)
        } else {
            None
        }
    }
    fn match_slot(&self, spec: &SlotSpec, f: &Fact) -> bool {
        // fact kind
        if !fact_kind_matches(&spec.fact, &f.kind) {
            return false;
        }
        // where predicates
        for p in &spec.where_ {
            if !self.predicate(p, f) {
                return false;
            }
        }
        true
    }
    fn predicate(&self, p: &PredicateSpec, f: &Fact) -> bool {
        let field = p.field.as_str();
        let op = p.op.as_str();
        let val_yaml = &p.value;

        // helpers to coerce YAML -> String/Vec<String> or f64
        let val_str = || -> Option<String> {
            match val_yaml {
                Yaml::String(s) => Some(s.clone()),
                Yaml::Number(n) => Some(n.to_string()),
                Yaml::Bool(b) => Some(if *b { "true".into() } else { "false".into() }),
                _ => None,
            }
        };
        let val_list = || -> Option<Vec<String>> {
            match val_yaml {
                Yaml::Sequence(seq) => {
                    let mut out = Vec::new();
                    for v in seq {
                        if let Yaml::String(s) = v {
                            out.push(s.clone());
                        } else if let Yaml::Number(n) = v {
                            out.push(n.to_string());
                        } else if let Yaml::Bool(b) = v {
                            out.push(if *b { "true".into() } else { "false".into() });
                        }
                    }
                    Some(out)
                }
                _ => None,
            }
        };
        let val_f64 = || -> Option<f64> {
            match val_yaml {
                Yaml::Number(n) => n
                    .as_f64()
                    .or_else(|| n.as_i64().map(|x| x as f64))
                    .or_else(|| n.as_u64().map(|x| x as f64)),
                Yaml::String(s) => s.parse::<f64>().ok(),
                _ => None,
            }
        };

        // Special aggregate fields: "__agg.kind", "__agg.files", "__agg.distinct_topdirs", "__agg.window_sec"
        let lookup = |k: &str| -> Option<String> { f.get_str(k).map(|s| s.to_string()) };

        match op {
            "eq" => {
                if let (Some(lhs), Some(rhs)) = (lookup(field), val_str()) {
                    lhs == rhs
                } else {
                    false
                }
            }
            "ne" => {
                if let (Some(lhs), Some(rhs)) = (lookup(field), val_str()) {
                    lhs != rhs
                } else {
                    false
                }
            }
            "contains" => {
                if let (Some(lhs), Some(rhs)) = (lookup(field), val_str()) {
                    lhs.contains(&rhs)
                } else {
                    false
                }
            }
            "regex" => {
                if let (Some(lhs), Some(pat)) = (lookup(field), val_str()) {
                    if let Some(re) = self.re(&pat) {
                        re.is_match(&lhs)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            "in" => {
                if let (Some(lhs), Some(list)) = (lookup(field), val_list()) {
                    list.iter().any(|v| v == &lhs)
                } else {
                    false
                }
            }
            "gte" => {
                if let (Some(lhs), Some(rhs)) = (f.get_f64(field), val_f64()) {
                    lhs >= rhs
                } else {
                    false
                }
            }
            "lte" => {
                if let (Some(lhs), Some(rhs)) = (f.get_f64(field), val_f64()) {
                    lhs <= rhs
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

// ---- Aggregation: file_burst (mass writes/renames across topdirs) ----

#[derive(Default)]
struct FileBurstState {
    // windowed activity per topdir
    by_topdir: HashMap<String, VecDeque<(u64, String)>>, // (ts, path)
    window_sec: u64,
}

impl FileBurstState {
    fn new(window_sec: u64) -> Self {
        Self {
            by_topdir: HashMap::new(),
            window_sec,
        }
    }
    fn topdir_of(path: &str) -> String {
        // naive: /home/user/file -> /home ; /var/www/x -> /var/www
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            return "/".into();
        }
        if parts.len() >= 2 {
            format!("/{}/{}", parts[0], parts[1])
        } else {
            format!("/{}", parts[0])
        }
    }
    fn observe(&mut self, ts: u64, op: &str, path: &str) {
        if !(op == "write" || op == "rename") {
            return;
        }
        let key = Self::topdir_of(path);
        self.by_topdir
            .entry(key)
            .or_default()
            .push_back((ts, path.to_string()));
    }
    fn evict_old(&mut self, now: u64) {
        let win = self.window_sec;
        for q in self.by_topdir.values_mut() {
            while let Some((ts, _)) = q.front().cloned() {
                if now > ts && now - ts > win {
                    q.pop_front();
                } else {
                    break;
                }
            }
        }
        self.by_topdir.retain(|_, q| !q.is_empty());
    }
    fn derive_aggregate(&mut self, now: u64) -> Option<Fact> {
        self.evict_old(now);
        let mut total_files = 0usize;
        let mut topdirs = 0usize;
        for (_k, q) in &self.by_topdir {
            if !q.is_empty() {
                topdirs += 1;
                total_files += q.len();
            }
        }
        if topdirs == 0 || total_files == 0 {
            return None;
        }
        // Build aggregate Fact
        let mut fields = HashMap::new();
        fields.insert("__agg.kind".into(), "file_burst".into());
        fields.insert("__agg.files".into(), format!("{}", total_files));
        fields.insert("__agg.distinct_topdirs".into(), format!("{}", topdirs));
        fields.insert("__agg.window_sec".into(), format!("{}", self.window_sec));
        Some(Fact {
            ts: now,
            kind: FactKind::Agg,
            host: "-".into(),
            user: None,
            exe: None,
            cmd: None,
            fields,
        })
    }
}

// ---- Engine ----

pub struct PlaybookEngine {
    eval: Evaluator,
    // registry
    pub playbooks: HashMap<String, PlaybookSpec>, // id -> spec
    // per-host incidents (simplified by host; could be per (exe,user,host) later)
    hypotheses: HashMap<(String, String), Hypothesis>, // (pb_id, host)
    // slot TTLs per playbook (quick lookup)
    slot_ttls: HashMap<(String, String), u64>, // (pb_id, slot_id) -> ttl
    // Aggregators (per host)
    fileburst: HashMap<String, FileBurstState>, // host -> state
    // cooldown memory: (pb,host) -> last_emit_ts
    cooldown: HashMap<(String, String), u64>,
}

impl PlaybookEngine {
    pub fn empty() -> Self {
        Self {
            eval: Evaluator::new(),
            playbooks: HashMap::new(),
            hypotheses: HashMap::new(),
            slot_ttls: HashMap::new(),
            fileburst: HashMap::new(),
            cooldown: HashMap::new(),
        }
    }

    pub fn load_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let mut playbooks = HashMap::new();
        for entry in std::fs::read_dir(dir.as_ref())? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let s = std::fs::read_to_string(&path)?;
                let spec: PlaybookSpec = serde_yaml::from_str(&s)
                    .map_err(|e| anyhow!("Failed to parse {:?}: {e}", path))?;
                playbooks.insert(spec.id.clone(), spec);
            }
        }
        // build TTL map
        let mut slot_ttls = HashMap::new();
        for (pb_id, pb) in &playbooks {
            for s in pb
                .required
                .iter()
                .chain(pb.optional.iter())
                .chain(pb.forbidden.iter())
            {
                slot_ttls.insert((pb_id.clone(), s.slot_id.clone()), s.slot_ttl_sec);
            }
        }
        Ok(Self {
            eval: Evaluator::new(),
            playbooks,
            hypotheses: HashMap::new(),
            slot_ttls,
            fileburst: HashMap::new(),
            cooldown: HashMap::new(),
        })
    }

    fn now_sec() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs()
    }

    fn host_key(f: &Fact) -> String {
        f.host.clone()
    }

    pub fn tick(&mut self, now: u64) {
        // run aggregators
        let keys: Vec<_> = self.fileburst.keys().cloned().collect();
        for h in keys {
            if let Some(agg) = self
                .fileburst
                .get_mut(&h)
                .and_then(|s| s.derive_aggregate(now))
            {
                let _ = self.ingest_fact(agg); // derived facts may fulfill slots
            }
        }
        // expire stale hypotheses and slot hits
        let pb_ids: Vec<_> = self.playbooks.keys().cloned().collect();
        for pb_id in pb_ids {
            // collect hypothesis keys for this playbook
            let keys: Vec<_> = self
                .hypotheses
                .keys()
                .filter(|(id, _)| id == &pb_id)
                .cloned()
                .collect();
            for key in keys {
                if let Some(h) = self.hypotheses.get_mut(&key) {
                    // drop expired slot hits
                    let (pbid, _host) = &key;
                    let mut to_remove = Vec::new();
                    for (slot_id, hit) in &h.hits {
                        let ttl = *self
                            .slot_ttls
                            .get(&(pbid.clone(), slot_id.clone()))
                            .unwrap_or(&600);
                        if now > hit.ts && now - hit.ts > ttl {
                            to_remove.push(slot_id.clone());
                        }
                    }
                    for s in to_remove {
                        h.hits.remove(&s);
                    }
                    // drop hypothesis if empty and old
                    if h.hits.is_empty() && now.saturating_sub(h.created) > 2 * 60 {
                        self.hypotheses.remove(&key);
                    }
                }
            }
        }
    }

    pub fn ingest_fact(&mut self, fact: Fact) -> Option<PlaybookHit> {
        let now = if fact.ts == 0 {
            Self::now_sec()
        } else {
            fact.ts
        };
        // Observe for aggregators
        if let FactKind::FileIO = fact.kind {
            if let (Some(op), Some(path)) = (fact.get_str("op"), fact.get_str("path")) {
                let host = Self::host_key(&fact);
                let window = 420; // default 7m for fileburst; individual playbooks can still constrain via predicates
                self.fileburst
                    .entry(host.clone())
                    .or_insert_with(|| FileBurstState::new(window))
                    .observe(fact.ts, op, path);
            }
        }

        // Attempt to match slots in all playbooks relevant to this fact kind
        // We keep hypothesis per (playbook, host)
        let host = Self::host_key(&fact);
        let mut completed: Option<PlaybookHit> = None;

        for (pb_id, pb) in &self.playbooks {
            // match required/optional slots on this fact
            let mut matched_slots: Vec<&SlotSpec> = Vec::new();
            for slot in pb.required.iter().chain(pb.optional.iter()) {
                if self.eval.match_slot(slot, &fact) {
                    matched_slots.push(slot);
                }
            }
            if matched_slots.is_empty() {
                continue;
            }
            let key = (pb_id.clone(), host.clone());
            let hyp = self
                .hypotheses
                .entry(key.clone())
                .or_insert_with(|| Hypothesis::new(now));
            hyp.last_updated = now;

            for slot in matched_slots {
                hyp.hits.insert(
                    slot.slot_id.clone(),
                    SlotHit {
                        ts: fact.ts,
                        tags: slot.derive_tags.clone(),
                        fact: fact.clone(),
                    },
                );
            }

            // After updating, check completion (all required slots present and no forbidden)
            let window_ok = hyp.age(now) <= pb.window_sec;
            if window_ok && hyp.is_complete(pb) && !hyp.violates_forbidden(pb, &self.eval) {
                // Cooldown gate
                let cd_key = (pb_id.clone(), host.clone());
                let last = self.cooldown.get(&cd_key).cloned().unwrap_or(0);
                if last == 0 || now.saturating_sub(last) >= pb.cooldown_sec {
                    self.cooldown.insert(cd_key, now);
                    completed = Some(PlaybookHit::from_pb(pb, host.clone(), hyp));
                    // reset hypothesis to allow new chain
                    *hyp = Hypothesis::new(now);
                }
            }
        }

        completed
    }

    pub fn satisfy_slot_from_vendor(
        &mut self,
        product: &str,
        alert_type: &str,
        host: &str,
        ts: u64,
    ) -> Option<PlaybookHit> {
        let now = if ts == 0 { Self::now_sec() } else { ts };
        let mut out: Option<PlaybookHit> = None;
        for (pb_id, pb) in &self.playbooks {
            for vm in &pb.vendor_map {
                if vm.product.eq_ignore_ascii_case(product)
                    && vm.alert_type.eq_ignore_ascii_case(alert_type)
                {
                    // Fill the mapped slot with a synthetic fact
                    let fact = Fact {
                        ts: now,
                        kind: FactKind::VendorAlert,
                        host: host.to_string(),
                        user: None,
                        exe: None,
                        cmd: None,
                        fields: HashMap::new(),
                    };
                    // find slot spec by id
                    if let Some(slot) = pb
                        .required
                        .iter()
                        .chain(pb.optional.iter())
                        .find(|s| s.slot_id == vm.fills_slot)
                    {
                        let key = (pb_id.clone(), host.to_string());
                        let hyp = self
                            .hypotheses
                            .entry(key.clone())
                            .or_insert_with(|| Hypothesis::new(now));
                        hyp.hits.insert(
                            slot.slot_id.clone(),
                            SlotHit {
                                ts: now,
                                tags: slot.derive_tags.clone(),
                                fact: fact.clone(),
                            },
                        );
                        // check completion
                        if hyp.is_complete(pb)
                            && !hyp.violates_forbidden(pb, &self.eval)
                            && hyp.age(now) <= pb.window_sec
                        {
                            let cd_key = (pb_id.clone(), host.to_string());
                            let last = self.cooldown.get(&cd_key).cloned().unwrap_or(0);
                            if last == 0 || now.saturating_sub(last) >= pb.cooldown_sec {
                                self.cooldown.insert(cd_key, now);
                                out = Some(PlaybookHit::from_pb(pb, host.to_string(), hyp));
                                *hyp = Hypothesis::new(now);
                            }
                        }
                    }
                }
            }
        }
        out
    }
}

// ---- Hit materialization ----

#[derive(Debug, Clone)]
pub struct PlaybookHit {
    pub playbook_id: String,
    pub playbook_name: String,
    pub severity: String,
    pub host: String,
    pub ts: u64,
    pub tags: Vec<String>,
    pub rationale: Vec<String>,
    pub slots: HashMap<String, SlotHitSummary>,
    pub primary_ttp: Option<String>,
    pub narrative: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SlotHitSummary {
    pub ts: u64,
    pub tags: Vec<String>,
    pub fields: HashMap<String, String>,
}

impl PlaybookHit {
    fn from_pb(pb: &PlaybookSpec, host: String, hyp: &Hypothesis) -> Self {
        // merge tags and prepare rationale
        let mut tags = Vec::new();
        let mut slots: HashMap<String, SlotHitSummary> = HashMap::new();
        for (slot_id, hit) in &hyp.hits {
            for t in &hit.tags {
                tags.push(t.clone());
            }
            let mut fields = HashMap::new();
            for (k, v) in &hit.fact.fields {
                // include a few likely useful keys
                if k == "op"
                    || k == "path"
                    || k == "topdir"
                    || k.starts_with("__agg")
                    || k == "exe"
                    || k == "cmd"
                {
                    fields.insert(k.clone(), v.clone());
                }
            }
            if let Some(exe) = &hit.fact.exe {
                fields.insert("exe".into(), exe.clone());
            }
            if let Some(cmd) = &hit.fact.cmd {
                fields.insert("cmd".into(), cmd.clone());
            }
            slots.insert(
                slot_id.clone(),
                SlotHitSummary {
                    ts: hit.ts,
                    tags: hit.tags.clone(),
                    fields,
                },
            );
        }
        tags.sort();
        tags.dedup();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        let rationale = vec![format!(
            "Playbook '{}' satisfied (required slots complete within window).",
            pb.name
        )];
        Self {
            playbook_id: pb.id.clone(),
            playbook_name: pb.name.clone(),
            severity: pb.severity.clone(),
            host,
            ts: now,
            tags,
            rationale,
            slots,
            primary_ttp: pb.explain.as_ref().and_then(|e| e.primary_ttp.clone()),
            narrative: pb.explain.as_ref().and_then(|e| e.narrative.clone()),
        }
    }
    pub fn to_signal(&self) -> Signal {
        // risk heuristic: required size + optional contribution
        let req = self.slots.len() as f32;
        let base = 0.6_f32.min(0.3 + 0.1 * req);
        let score = (base + 0.1 * (self.tags.len() as f32).min(4.0)).min(0.95);
        let mut tagset = self.tags.clone();
        if let Some(ttp) = &self.primary_ttp {
            tagset.push(ttp.clone());
        }
        Signal {
            detector: DetectorKind::Playbook,
            score,
            ts: self.ts,
            exe: None,
            user: None,
            host: Some(self.host.clone()),
            cmd_prefix: None,
            parent: None,
            signer: None,
            hash: None,
            pid: None,
            tags: tagset,
        }
    }
}

// Helper to map fact kind strings used in YAML
fn fact_kind_matches(spec: &str, k: &FactKind) -> bool {
    match (spec, k) {
        ("Exec", FactKind::Exec) => true,
        ("FileIO", FactKind::FileIO) => true,
        ("NetConn", FactKind::NetConn) => true,
        ("Auth", FactKind::Auth) => true,
        ("Kernel", FactKind::Kernel) => true,
        ("VendorAlert", FactKind::VendorAlert) => true,
        // Aggregate 'file_burst' is represented as FactKind::Agg with __agg.kind=file_burst
        ("Agg", FactKind::Agg) => true,
        _ => false,
    }
}

// quick test helpers
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_pb() {
        let y = r#"
id: pb_demo
version: 0.1
name: Demo
severity: high
window_sec: 600
cooldown_sec: 60
required:
  - slot_id: file_burst
    slot_ttl_sec: 420
    fact: Agg
    where:
      - { field: __agg.kind, op: eq, value: file_burst }
"#;
        let pb: PlaybookSpec = serde_yaml::from_str(y).unwrap();
        assert_eq!(pb.id, "pb_demo");
        assert_eq!(pb.required.len(), 1);
        assert_eq!(pb.required[0].where_[0].field, "__agg.kind");
    }
}

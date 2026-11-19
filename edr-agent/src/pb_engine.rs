// src/pb_engine.rs
// Playbook engine. Loads YAML playbooks, matches facts against their slots, and fires a
// playbook when every required slot has a live hit inside the playbook window and no
// forbidden slot has one.
//
// Slot conditions use eq, ne, contains, regex, in, gte and lte. A slot whose conditions
// test `__agg.*` fields is a counting slot: its other conditions pick which events count,
// and the counts over the aggregate window are attached to the event that just arrived
// before the slot is checked (see "Aggregates" below).
//
// Time is Unix seconds everywhere (pb_adapter::clock converts kernel stamps). Facts carry
// their event time. tick(now) runs from the batch loop every 5 s and drops slot hits and
// counted events that aged out, even when no new event arrives.

use anyhow::{anyhow, Result};
use regex::Regex;
use serde::Deserialize;
use serde_yaml::Value as Yaml;
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Mirror a subset of the DecisionEngine to create Signals without circular deps.
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
    // Defaults to empty so the loader can say why a file has none.
    // validate_playbook rejects an empty list.
    #[serde(default)]
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

/// Evidence for one playbook on one host: the newest hit per slot. Forbidden slots are
/// kept here too, so a live forbidden hit can veto the playbook.
#[derive(Debug, Default)]
struct Hypothesis {
    hits: HashMap<String, SlotHit>, // slot_id -> newest hit
}

impl Hypothesis {
    /// Drop hits older than their slot TTL.
    fn expire(&mut self, now: u64, ttls: Option<&HashMap<String, u64>>) {
        self.hits.retain(|slot_id, hit| {
            let ttl = ttls.and_then(|t| t.get(slot_id)).copied().unwrap_or(600);
            now.saturating_sub(hit.ts) <= ttl
        });
    }

    /// Every required slot has a live hit and the oldest one is inside the window.
    /// The window is measured over the evidence, not from when the hypothesis started,
    /// so an early hit that aged out does not block a later chain.
    fn complete(&self, pb: &PlaybookSpec, now: u64) -> bool {
        if pb.required.is_empty() {
            return false;
        }
        let mut oldest = now;
        for slot in &pb.required {
            match self.hits.get(&slot.slot_id) {
                Some(hit) => oldest = oldest.min(hit.ts),
                None => return false,
            }
        }
        now.saturating_sub(oldest) <= pb.window_sec
    }

    fn vetoed(&self, pb: &PlaybookSpec) -> bool {
        pb.forbidden
            .iter()
            .any(|slot| self.hits.contains_key(&slot.slot_id))
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

        // tags_any: true when any of the fact's tags (comma-joined in `tags`) is in the
        // list for op in, or equals the value for op eq.
        if field == "tags_any" {
            let tags = f.get_str("tags").unwrap_or("");
            let mut each = tags.split(',').filter(|t| !t.is_empty());
            return match op {
                "in" => val_list()
                    .map(|list| each.any(|t| list.iter().any(|v| v == t)))
                    .unwrap_or(false),
                "eq" => val_str().map(|v| each.any(|t| t == v)).unwrap_or(false),
                _ => false,
            };
        }

        // `__agg.*` fields are ordinary fields here. Counting slots get them from
        // agg_fields before the slot is checked.
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

// ---- Aggregates: counting slots ----
//
// A slot that tests `__agg.*` fields counts events. Example, fileburst_crypto:
//   op in [write, rename], topdir in [/home, ...], __agg.files gte 500, __agg.window_sec lte 600
// Every event that passes the slot's other conditions is kept for the aggregate window
// (the `__agg.window_sec lte N` bound, else the slot TTL), per playbook, slot and host.
// When an event arrives it is added, events older than the window are dropped, and the
// counts are copied onto a clone of the event as `__agg.*` fields. Then the whole slot,
// counts included, is checked on that clone. So `__agg.files gte 500` reads as "500
// distinct files passed this slot's other conditions in the last 600 s".
//
// File bursts keep every write, rename and delete in the place the slot names, even when
// the slot only asks for writes, so the rename ratio compares renames with all changes
// there. `files` and the distinct counts use only the ops the slot asks for, or write and
// rename when it names none. A plain open never counts as a change.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AggKind {
    FileBurst,
    PartialEncryptBurst,
    ErrnoSpike,
    ExecBurst,
    ProcBurstWorkers,
}

impl AggKind {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "file_burst" => Some(Self::FileBurst),
            "partial_encrypt_burst" => Some(Self::PartialEncryptBurst),
            "errno_spike" => Some(Self::ErrnoSpike),
            "exec_burst" => Some(Self::ExecBurst),
            "proc_burst_workers" => Some(Self::ProcBurstWorkers),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::FileBurst => "file_burst",
            Self::PartialEncryptBurst => "partial_encrypt_burst",
            Self::ErrnoSpike => "errno_spike",
            Self::ExecBurst => "exec_burst",
            Self::ProcBurstWorkers => "proc_burst_workers",
        }
    }

    fn fact_kind(self) -> FactKind {
        match self {
            Self::FileBurst | Self::PartialEncryptBurst | Self::ErrnoSpike => FactKind::FileIO,
            Self::ExecBurst | Self::ProcBurstWorkers => FactKind::Exec,
        }
    }

    fn is_file_burst(self) -> bool {
        matches!(self, Self::FileBurst | Self::PartialEncryptBurst)
    }

    /// `__agg.<name>` fields this kind sets.
    fn fields(self) -> &'static [&'static str] {
        match self {
            // partial_encrypt_burst is the same window. The playbook adds the
            // short-write and rename-ratio conditions that make it partial encryption.
            Self::FileBurst | Self::PartialEncryptBurst => &[
                "kind",
                "files",
                "distinct_topdirs",
                "distinct_dirs",
                "distinct_mounts",
                "rename_ratio",
                "total_bytes",
                "avg_write_kib",
                "window_sec",
            ],
            Self::ErrnoSpike => &["kind", "count", "window_sec"],
            Self::ExecBurst => &["kind", "count", "distinct_topdirs", "window_sec"],
            Self::ProcBurstWorkers => &["kind", "children", "window_sec"],
        }
    }
}

/// How one counting slot builds its window, worked out once at load.
#[derive(Debug, Clone)]
struct AggPlan {
    kind: AggKind,
    window_sec: u64,
    /// Conditions an event must pass to be counted.
    entry: Vec<PredicateSpec>,
    /// File bursts: the slot's conditions on `op`, used for `files`.
    ops: Vec<PredicateSpec>,
}

/// The counting plan for a slot, None for a plain slot, or why the slot can never match.
fn agg_plan(slot: &SlotSpec) -> std::result::Result<Option<AggPlan>, String> {
    let agg: Vec<&PredicateSpec> = slot
        .where_
        .iter()
        .filter(|p| p.field.starts_with("__agg."))
        .collect();
    if agg.is_empty() {
        return Ok(None);
    }
    let fact = parse_fact_kind(&slot.fact);

    let mut kind = None;
    for p in &agg {
        if p.field == "__agg.kind" {
            if p.op != "eq" {
                return Err("`__agg.kind` must use op eq".into());
            }
            let name = match &p.value {
                Yaml::String(s) => s.clone(),
                _ => return Err("`__agg.kind` must be a string".into()),
            };
            kind = Some(
                AggKind::parse(&name).ok_or_else(|| format!("unknown aggregate kind `{name}`"))?,
            );
        }
    }
    // A slot with counts but no kind gets the plain burst for its fact kind.
    let kind = match (kind, &fact) {
        (Some(k), _) => k,
        (None, Some(FactKind::FileIO)) => AggKind::FileBurst,
        (None, Some(FactKind::Exec)) => AggKind::ExecBurst,
        (None, _) => {
            return Err(format!(
                "`__agg` fields on a {} slot, counts exist only for FileIO and Exec",
                slot.fact
            ))
        }
    };
    if fact.as_ref() != Some(&kind.fact_kind()) {
        return Err(format!(
            "aggregate `{}` counts {:?} facts, not {}",
            kind.name(),
            kind.fact_kind(),
            slot.fact
        ));
    }
    for p in &agg {
        let name = &p.field["__agg.".len()..];
        if !kind.fields().contains(&name) {
            return Err(format!("aggregate `{}` has no field `{}`", kind.name(), p.field));
        }
    }

    let window_sec = agg
        .iter()
        .find(|p| p.field == "__agg.window_sec" && p.op == "lte")
        .and_then(|p| yaml_f64(&p.value))
        .map(|v| v as u64)
        .unwrap_or(slot.slot_ttl_sec);

    let mut entry = Vec::new();
    let mut ops = Vec::new();
    for p in &slot.where_ {
        if p.field.starts_with("__agg.") {
            continue;
        }
        if kind.is_file_burst() && p.field == "op" {
            ops.push(p.clone());
        } else {
            entry.push(p.clone());
        }
    }
    Ok(Some(AggPlan {
        kind,
        window_sec,
        entry,
        ops,
    }))
}

// Distinct-value counters kept per window.
const DIM_FILES: usize = 0; // paths changed by an op the slot asks for
const DIM_PATHS: usize = 1; // every changed path, the rename ratio denominator
const DIM_TOPDIRS: usize = 2;
const DIM_DIRS: usize = 3;
const DIM_MOUNTS: usize = 4;
const DIM_ARG_TOPDIRS: usize = 5; // exec_burst: top dirs of the path arguments
const DIM_CHILDREN: usize = 6; // proc_burst_workers: child pids
const DIMS: usize = 7;

/// Oldest events are dropped past this many, so one window cannot grow without bound.
const AGG_WINDOW_CAP: usize = 200_000;

#[derive(Debug)]
struct AggEntry {
    ts: u64,
    keys: Vec<(usize, String)>,
    rename: bool,
    bytes: Option<u64>,
}

#[derive(Debug, Default)]
struct AggWindow {
    entries: VecDeque<AggEntry>,
    distinct: [HashMap<String, u32>; DIMS],
    renames: u64,
    bytes: u64,
    sized_writes: u64,
}

impl AggWindow {
    fn push(&mut self, e: AggEntry) {
        for (dim, key) in &e.keys {
            *self.distinct[*dim].entry(key.clone()).or_insert(0) += 1;
        }
        if e.rename {
            self.renames += 1;
        }
        if let Some(b) = e.bytes {
            self.bytes += b;
            self.sized_writes += 1;
        }
        self.entries.push_back(e);
        if self.entries.len() > AGG_WINDOW_CAP {
            self.pop_front();
        }
    }

    fn pop_front(&mut self) {
        let Some(e) = self.entries.pop_front() else {
            return;
        };
        for (dim, key) in e.keys {
            if let Some(n) = self.distinct[dim].get_mut(&key) {
                *n -= 1;
                if *n == 0 {
                    self.distinct[dim].remove(&key);
                }
            }
        }
        if e.rename {
            self.renames -= 1;
        }
        if let Some(b) = e.bytes {
            self.bytes -= b;
            self.sized_writes -= 1;
        }
    }

    fn evict(&mut self, now: u64, window_sec: u64) {
        while let Some(front) = self.entries.front() {
            if now.saturating_sub(front.ts) > window_sec {
                self.pop_front();
            } else {
                break;
            }
        }
    }

    fn n(&self, dim: usize) -> usize {
        self.distinct[dim].len()
    }
}

/// The window entry for `f`, or None when the slot does not count this event.
fn agg_entry(plan: &AggPlan, f: &Fact, eval: &Evaluator) -> Option<AggEntry> {
    if f.kind != plan.kind.fact_kind() {
        return None;
    }
    // Most file facts are plain opens. Checking the op first skips the location regexes.
    if plan.kind.is_file_burst() && !matches!(f.get_str("op"), Some("write" | "rename" | "delete")) {
        return None;
    }
    if !plan.entry.iter().all(|p| eval.predicate(p, f)) {
        return None;
    }
    let mut keys = Vec::new();
    let mut rename = false;
    let mut bytes = None;
    match plan.kind {
        AggKind::FileBurst | AggKind::PartialEncryptBurst => {
            let op = f.get_str("op")?;
            let path = f.get_str("path")?;
            let counts = if plan.ops.is_empty() {
                op == "write" || op == "rename"
            } else {
                plan.ops.iter().all(|p| eval.predicate(p, f))
            };
            keys.push((DIM_PATHS, path.to_string()));
            if counts {
                keys.push((DIM_FILES, path.to_string()));
                keys.push((DIM_TOPDIRS, topdir_of(path)));
                keys.push((DIM_DIRS, parent_dir(path)));
                if let Some(m) = f.get_str("mount_point") {
                    keys.push((DIM_MOUNTS, m.to_string()));
                }
            }
            rename = op == "rename";
            if op == "write" {
                bytes = f.get_str("bytes").and_then(|b| b.parse::<u64>().ok());
            }
        }
        AggKind::ErrnoSpike => {}
        AggKind::ExecBurst => {
            // argv[0] is the tool itself, so only later absolute paths are targets.
            if let Some(cmd) = f.get_str("cmd") {
                for arg in cmd.split_whitespace().skip(1) {
                    if arg.starts_with('/') {
                        keys.push((DIM_ARG_TOPDIRS, topdir_of(arg)));
                    }
                }
            }
        }
        AggKind::ProcBurstWorkers => {
            keys.push((DIM_CHILDREN, f.get_str("pid").unwrap_or("-").to_string()));
        }
    }
    Some(AggEntry {
        ts: f.ts,
        keys,
        rename,
        bytes,
    })
}

/// The `__agg.*` fields for the current state of a window.
fn agg_fields(plan: &AggPlan, w: &AggWindow) -> Vec<(String, String)> {
    let mut out = vec![
        ("__agg.kind".to_string(), plan.kind.name().to_string()),
        ("__agg.window_sec".to_string(), plan.window_sec.to_string()),
    ];
    let mut put = |k: &str, v: String| out.push((format!("__agg.{k}"), v));
    match plan.kind {
        AggKind::FileBurst | AggKind::PartialEncryptBurst => {
            put("files", w.n(DIM_FILES).to_string());
            put("distinct_topdirs", w.n(DIM_TOPDIRS).to_string());
            put("distinct_dirs", w.n(DIM_DIRS).to_string());
            put("distinct_mounts", w.n(DIM_MOUNTS).to_string());
            let changed = w.n(DIM_PATHS).max(1) as f64;
            put("rename_ratio", format!("{:.3}", (w.renames as f64 / changed).min(1.0)));
            // Only write syscalls carry a size. Without them the byte fields stay unset,
            // so a slot that needs them does not match.
            if w.sized_writes > 0 {
                put("total_bytes", w.bytes.to_string());
                put(
                    "avg_write_kib",
                    format!("{:.1}", w.bytes as f64 / w.sized_writes as f64 / 1024.0),
                );
            }
        }
        AggKind::ErrnoSpike => put("count", w.entries.len().to_string()),
        AggKind::ExecBurst => {
            put("count", w.entries.len().to_string());
            put("distinct_topdirs", w.n(DIM_ARG_TOPDIRS).to_string());
        }
        AggKind::ProcBurstWorkers => put("children", w.n(DIM_CHILDREN).to_string()),
    }
    out
}

/// Top directory of a path: the first component, or the first two under /var, /run
/// and /usr, where the first level is a system area and the second names the data.
/// /home/alice/x is /home, /var/www/x is /var/www, /run/user/1000 is /run/user.
pub fn topdir_of(path: &str) -> String {
    let mut parts = path.split('/').filter(|s| !s.is_empty());
    match (path.starts_with('/'), parts.next(), parts.next()) {
        (false, _, _) | (true, None, _) => "-".to_string(),
        (true, Some(a), Some(b)) if matches!(a, "var" | "run" | "usr") => format!("/{a}/{b}"),
        (true, Some(a), _) => format!("/{a}"),
    }
}

/// Parent directory of an absolute path.
pub fn parent_dir(path: &str) -> String {
    match path.rfind('/') {
        Some(0) => "/".to_string(),
        Some(i) => path[..i].to_string(),
        None => "-".to_string(),
    }
}

// ---- Loading and validation ----

/// Playbook directory: $PB_DIR, else ./playbooks (same default as main.rs).
pub fn playbook_dir() -> String {
    std::env::var("PB_DIR").unwrap_or_else(|_| "playbooks".to_string())
}

/// What one directory load did, file by file.
#[derive(Debug, Default)]
pub struct LoadReport {
    /// Ids of playbooks that loaded.
    pub loaded: Vec<String>,
    /// (file name, reason) for YAML files that were rejected.
    pub skipped: Vec<(String, String)>,
    /// (playbook id, warning) for loaded playbooks with parts that can never match.
    pub warnings: Vec<(String, String)>,
    /// Files without a .yaml or .yml extension.
    pub ignored: Vec<String>,
}

impl LoadReport {
    pub fn log(&self, dir: &Path) {
        for (file, reason) in &self.skipped {
            eprintln!("playbooks: skipped {file}: {reason}");
        }
        for (id, w) in &self.warnings {
            eprintln!("playbooks: {id}: {w}");
        }
        for file in &self.ignored {
            eprintln!("playbooks: ignored {file} (not .yaml)");
        }
        eprintln!(
            "playbooks: loaded {} of {} YAML files from {} ({} skipped)",
            self.loaded.len(),
            self.loaded.len() + self.skipped.len(),
            dir.display(),
            self.skipped.len()
        );
    }
}

// Top-level keys PlaybookSpec reads.
const SPEC_KEYS: &[&str] = &[
    "id",
    "version",
    "name",
    "severity",
    "description",
    "window_sec",
    "cooldown_sec",
    "required",
    "optional",
    "forbidden",
    "vendor_map",
    "explain",
];
// Labels the engine does not read and that carry no matching logic.
const LABEL_KEYS: &[&str] = &["title", "reason_code", "rules"];
// Ops handled by Evaluator::predicate. Keep the two in sync.
const PREDICATE_OPS: &[&str] = &["eq", "ne", "contains", "regex", "in", "gte", "lte"];

fn load_playbook(path: &Path) -> std::result::Result<(PlaybookSpec, Vec<String>), String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read: {e}"))?;
    let spec: PlaybookSpec = serde_yaml::from_str(&text).map_err(|e| format!("{e}"))?;
    // Parsed again as a plain map to find keys the engine would silently ignore.
    let keys: Vec<String> = match serde_yaml::from_str::<Yaml>(&text) {
        Ok(Yaml::Mapping(m)) => m
            .keys()
            .filter_map(|k| k.as_str().map(str::to_string))
            .collect(),
        _ => Vec::new(),
    };
    let warnings = validate_playbook(&spec, &keys)?;
    Ok((spec, warnings))
}

/// Reject a playbook that can never fire, and warn about parts that can never match.
fn validate_playbook(pb: &PlaybookSpec, keys: &[String]) -> std::result::Result<Vec<String>, String> {
    let unknown: Vec<&str> = keys
        .iter()
        .map(String::as_str)
        .filter(|k| !SPEC_KEYS.contains(k) && !LABEL_KEYS.contains(k))
        .collect();

    // is_complete() is true for an empty list, so a playbook with no required
    // slots would fire on its first optional match.
    if pb.required.is_empty() {
        let mut why = "no required slots".to_string();
        if !unknown.is_empty() {
            why.push_str(&format!(
                " (it uses keys this engine does not read: {})",
                unknown.join(", ")
            ));
        }
        return Err(why);
    }
    for slot in &pb.required {
        if let Some(why) = slot_problem(slot) {
            return Err(format!("required slot `{}` can never match: {why}", slot.slot_id));
        }
    }

    let mut warnings = Vec::new();
    for slot in &pb.optional {
        if let Some(why) = slot_problem(slot) {
            warnings.push(format!("optional slot `{}` can never match: {why}", slot.slot_id));
        }
    }
    for slot in &pb.forbidden {
        if let Some(why) = slot_problem(slot) {
            warnings.push(format!(
                "forbidden slot `{}` can never match, so it never vetoes: {why}",
                slot.slot_id
            ));
        }
    }
    for vm in &pb.vendor_map {
        let known = pb
            .required
            .iter()
            .chain(pb.optional.iter())
            .any(|s| s.slot_id == vm.fills_slot);
        if !known {
            warnings.push(format!("vendor_map fills unknown slot `{}`", vm.fills_slot));
        }
    }
    if !unknown.is_empty() {
        warnings.push(format!(
            "ignoring keys this engine does not implement: {}",
            unknown.join(", ")
        ));
    }
    Ok(warnings)
}

/// Why a slot can never match a fact, or None if it can.
fn slot_problem(slot: &SlotSpec) -> Option<String> {
    if parse_fact_kind(&slot.fact).is_none() {
        return Some(format!("fact kind `{}` is not one the engine produces", slot.fact));
    }
    for p in &slot.where_ {
        let op = p.op.as_str();
        if !PREDICATE_OPS.contains(&op) {
            return Some(format!("op `{op}` on `{}` is not supported", p.field));
        }
        let value_ok = match op {
            "in" => matches!(p.value, Yaml::Sequence(_)),
            "gte" | "lte" => yaml_f64(&p.value).is_some(),
            _ => matches!(p.value, Yaml::String(_) | Yaml::Number(_) | Yaml::Bool(_)),
        };
        if !value_ok {
            return Some(format!("op `{op}` on `{}` has the wrong value type", p.field));
        }
        if op == "regex" {
            if let Yaml::String(pat) = &p.value {
                if let Err(e) = Regex::new(pat) {
                    let why = e.to_string();
                    let last = why.lines().last().unwrap_or("").trim().to_string();
                    return Some(format!("bad regex on `{}`: {last}", p.field));
                }
            }
        }
    }
    // Counting slots: the aggregate kind must exist, fit the fact kind and set
    // every `__agg` field the slot tests.
    if let Err(why) = agg_plan(slot) {
        return Some(why);
    }
    None
}

// Same coercion as the gte/lte arm of Evaluator::predicate.
fn yaml_f64(v: &Yaml) -> Option<f64> {
    match v {
        Yaml::Number(n) => n
            .as_f64()
            .or_else(|| n.as_i64().map(|x| x as f64))
            .or_else(|| n.as_u64().map(|x| x as f64)),
        Yaml::String(s) => s.parse::<f64>().ok(),
        _ => None,
    }
}


/// Load `dir` and swap the result into `engine`. Files are parsed before the lock is
/// taken. In-flight hypotheses and counts start over, as after a restart. Cooldowns of
/// playbooks that are still loaded are kept, so a reload cannot make a playbook that
/// just fired fire again at once. If the directory cannot be read, the current set stays.
pub fn reload_into(engine: &std::sync::Mutex<PlaybookEngine>, dir: &Path) -> Result<LoadReport> {
    let (mut fresh, report) = PlaybookEngine::load_dir_report(dir)?;
    let mut guard = engine
        .lock()
        .map_err(|_| anyhow!("playbook engine lock poisoned"))?;
    fresh.cooldown = guard
        .cooldown
        .drain()
        .filter(|((id, _), _)| fresh.playbooks.contains_key(id))
        .collect();
    *guard = fresh;
    Ok(report)
}

// ---- Engine ----

/// (playbook id, slot id, host, group). The group is the parent pid for
/// proc_burst_workers and empty for every other kind.
type WindowKey = (String, String, String, String);

pub struct PlaybookEngine {
    eval: Evaluator,
    pub playbooks: HashMap<String, PlaybookSpec>, // id -> spec
    // One hypothesis per (playbook, host).
    hypotheses: HashMap<(String, String), Hypothesis>,
    // playbook id -> slot id -> ttl
    slot_ttls: HashMap<String, HashMap<String, u64>>,
    // playbook id -> slot id -> counting plan, for slots that test __agg fields
    agg_plans: HashMap<String, HashMap<String, AggPlan>>,
    windows: HashMap<WindowKey, AggWindow>,
    // (playbook, host) -> time it last fired
    cooldown: HashMap<(String, String), u64>,
}

impl PlaybookEngine {
    pub fn empty() -> Self {
        Self {
            eval: Evaluator::new(),
            playbooks: HashMap::new(),
            hypotheses: HashMap::new(),
            slot_ttls: HashMap::new(),
            agg_plans: HashMap::new(),
            windows: HashMap::new(),
            cooldown: HashMap::new(),
        }
    }

    /// Load every playbook in `dir` and log what was loaded and what was skipped.
    /// A bad file is skipped. Only an unreadable directory is an error.
    pub fn load_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let (engine, report) = Self::load_dir_report(dir.as_ref())?;
        report.log(dir.as_ref());
        Ok(engine)
    }

    /// Same as load_dir, but returns the report instead of logging it.
    pub fn load_dir_report<P: AsRef<Path>>(dir: P) -> Result<(Self, LoadReport)> {
        let dir = dir.as_ref();
        let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
            .map_err(|e| anyhow!("cannot read playbook dir {}: {e}", dir.display()))?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<std::io::Result<_>>()?;
        // Sorted so a duplicate id is resolved the same way on every host.
        paths.sort();

        let mut report = LoadReport::default();
        let mut playbooks = HashMap::new();
        for path in paths {
            if !path.is_file() {
                continue;
            }
            let file = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            if !matches!(
                path.extension().and_then(|s| s.to_str()),
                Some("yaml") | Some("yml")
            ) {
                report.ignored.push(file);
                continue;
            }
            match load_playbook(&path) {
                Ok((spec, warnings)) => {
                    if playbooks.contains_key(&spec.id) {
                        report
                            .skipped
                            .push((file, format!("duplicate playbook id `{}`", spec.id)));
                        continue;
                    }
                    for w in warnings {
                        report.warnings.push((spec.id.clone(), w));
                    }
                    report.loaded.push(spec.id.clone());
                    playbooks.insert(spec.id.clone(), spec);
                }
                Err(reason) => report.skipped.push((file, reason)),
            }
        }

        let mut engine = Self::empty();
        for (pb_id, pb) in &playbooks {
            let ttls = engine.slot_ttls.entry(pb_id.clone()).or_default();
            let mut plans = HashMap::new();
            for s in pb.required.iter().chain(&pb.optional).chain(&pb.forbidden) {
                ttls.insert(s.slot_id.clone(), s.slot_ttl_sec);
                // A bad plan was already reported by validate_playbook (an error for a
                // required slot, a warning otherwise), so it is skipped here.
                if let Ok(Some(plan)) = agg_plan(s) {
                    plans.insert(s.slot_id.clone(), plan);
                }
            }
            if !plans.is_empty() {
                engine.agg_plans.insert(pb_id.clone(), plans);
            }
        }
        engine.playbooks = playbooks;
        Ok((engine, report))
    }

    fn now_sec() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs()
    }

    /// Advance time with no new event: drop counted events older than their window,
    /// slot hits older than their TTL, empty hypotheses, and cooldowns that ended.
    pub fn tick(&mut self, now: u64) {
        let Self {
            playbooks,
            hypotheses,
            slot_ttls,
            agg_plans,
            windows,
            cooldown,
            ..
        } = self;
        windows.retain(|(pb_id, slot_id, _, _), w| {
            match agg_plans.get(pb_id).and_then(|m| m.get(slot_id)) {
                Some(plan) => {
                    w.evict(now, plan.window_sec);
                    !w.entries.is_empty()
                }
                None => false,
            }
        });
        hypotheses.retain(|(pb_id, _), h| {
            h.expire(now, slot_ttls.get(pb_id));
            !h.hits.is_empty()
        });
        cooldown.retain(|(pb_id, _), last| {
            playbooks
                .get(pb_id)
                .map(|pb| now.saturating_sub(*last) < pb.cooldown_sec)
                .unwrap_or(false)
        });
    }

    /// Match one fact against every playbook. Returns the playbooks it completed.
    pub fn ingest_fact(&mut self, mut fact: Fact) -> Vec<PlaybookHit> {
        if fact.ts == 0 {
            fact.ts = Self::now_sec();
        }
        let now = fact.ts;
        let host = fact.host.clone();
        let Self {
            eval,
            playbooks,
            hypotheses,
            slot_ttls,
            agg_plans,
            windows,
            cooldown,
        } = self;

        let mut fired = Vec::new();
        for (pb_id, pb) in playbooks.iter() {
            let plans = agg_plans.get(pb_id);
            let mut matched: Vec<(&SlotSpec, Fact)> = Vec::new();
            for slot in pb.required.iter().chain(&pb.optional).chain(&pb.forbidden) {
                match plans.and_then(|m| m.get(&slot.slot_id)) {
                    None => {
                        if eval.match_slot(slot, &fact) {
                            matched.push((slot, fact.clone()));
                        }
                    }
                    Some(plan) => {
                        let Some(entry) = agg_entry(plan, &fact, eval) else {
                            continue;
                        };
                        let group = if plan.kind == AggKind::ProcBurstWorkers {
                            fact.get_str("ppid").unwrap_or("-").to_string()
                        } else {
                            String::new()
                        };
                        let key = (pb_id.clone(), slot.slot_id.clone(), host.clone(), group);
                        let w = windows.entry(key).or_default();
                        w.evict(now, plan.window_sec);
                        w.push(entry);
                        let mut counted = fact.clone();
                        for (k, v) in agg_fields(plan, w) {
                            counted.fields.insert(k, v);
                        }
                        if eval.match_slot(slot, &counted) {
                            matched.push((slot, counted));
                        }
                    }
                }
            }
            if matched.is_empty() {
                continue;
            }

            let key = (pb_id.clone(), host.clone());
            let hyp = hypotheses.entry(key.clone()).or_default();
            hyp.expire(now, slot_ttls.get(pb_id));
            for (slot, f) in matched {
                hyp.hits.insert(
                    slot.slot_id.clone(),
                    SlotHit {
                        ts: now,
                        tags: slot.derive_tags.clone(),
                        fact: f,
                    },
                );
            }

            if hyp.complete(pb, now) && !hyp.vetoed(pb) {
                let cooled = cooldown
                    .get(&key)
                    .map(|last| now.saturating_sub(*last) >= pb.cooldown_sec)
                    .unwrap_or(true);
                if cooled {
                    cooldown.insert(key, now);
                    fired.push(PlaybookHit::from_pb(pb, host.clone(), hyp, now));
                    // Start a new chain. The counting windows keep their events.
                    hyp.hits.clear();
                }
            }
        }
        fired
    }

    /// Fill the slot a vendor_map entry names with a vendor alert. No caller in the agent
    /// today: vendor_fusion::fuse wraps this, and nothing calls fuse.
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
                if !(vm.product.eq_ignore_ascii_case(product)
                    && vm.alert_type.eq_ignore_ascii_case(alert_type))
                {
                    continue;
                }
                let Some(slot) = pb
                    .required
                    .iter()
                    .chain(pb.optional.iter())
                    .find(|s| s.slot_id == vm.fills_slot)
                else {
                    continue;
                };
                let mut fields = HashMap::new();
                fields.insert("product".to_string(), product.to_string());
                fields.insert("alert_type".to_string(), alert_type.to_string());
                let fact = Fact {
                    ts: now,
                    kind: FactKind::VendorAlert,
                    host: host.to_string(),
                    user: None,
                    exe: None,
                    cmd: None,
                    fields,
                };
                let key = (pb_id.clone(), host.to_string());
                let hyp = self.hypotheses.entry(key.clone()).or_default();
                hyp.expire(now, self.slot_ttls.get(pb_id));
                hyp.hits.insert(
                    slot.slot_id.clone(),
                    SlotHit {
                        ts: now,
                        tags: slot.derive_tags.clone(),
                        fact,
                    },
                );
                if hyp.complete(pb, now) && !hyp.vetoed(pb) {
                    let cooled = self
                        .cooldown
                        .get(&key)
                        .map(|last| now.saturating_sub(*last) >= pb.cooldown_sec)
                        .unwrap_or(true);
                    if cooled {
                        self.cooldown.insert(key, now);
                        out = Some(PlaybookHit::from_pb(pb, host.to_string(), hyp, now));
                        hyp.hits.clear();
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
    /// Event time of the fact that completed the playbook, Unix seconds.
    pub ts: u64,
    pub tags: Vec<String>,
    pub rationale: Vec<String>,
    pub slots: HashMap<String, SlotHitSummary>,
    /// Slot ids that had a hit, sorted.
    pub filled_slots: Vec<String>,
    /// Optional slots with no hit, sorted.
    pub missing_optional: Vec<String>,
    pub primary_ttp: Option<String>,
    pub narrative: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SlotHitSummary {
    pub ts: u64,
    pub tags: Vec<String>,
    pub fields: HashMap<String, String>,
}

/// One readable line of evidence for a slot hit.
fn evidence_line(slot_id: &str, f: &Fact) -> String {
    let mut parts = Vec::new();
    for k in ["exe", "cmd", "op", "path", "path2", "errno", "mount_fstype", "tags"] {
        if let Some(v) = f.get_str(k) {
            if !v.is_empty() {
                parts.push(format!("{k}={v}"));
            }
        }
    }
    let mut agg: Vec<_> = f
        .fields
        .iter()
        .filter(|(k, _)| k.starts_with("__agg.") && k.as_str() != "__agg.kind")
        .map(|(k, v)| format!("{}={v}", &k["__agg.".len()..]))
        .collect();
    agg.sort();
    if let Some(kind) = f.fields.get("__agg.kind") {
        parts.push(format!("{kind}({})", agg.join(" ")));
    }
    format!("{slot_id}: {}", parts.join(" "))
}

impl PlaybookHit {
    fn from_pb(pb: &PlaybookSpec, host: String, hyp: &Hypothesis, now: u64) -> Self {
        let mut tags = Vec::new();
        let mut slots: HashMap<String, SlotHitSummary> = HashMap::new();
        for (slot_id, hit) in &hyp.hits {
            tags.extend(hit.tags.iter().cloned());
            let mut fields = hit.fact.fields.clone();
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

        let mut filled_slots: Vec<String> = hyp.hits.keys().cloned().collect();
        filled_slots.sort();
        let mut missing_optional: Vec<String> = pb
            .optional
            .iter()
            .filter(|s| !hyp.hits.contains_key(&s.slot_id))
            .map(|s| s.slot_id.clone())
            .collect();
        missing_optional.sort();

        let mut rationale = vec![format!(
            "Playbook '{}' satisfied: every required slot hit within {} s.",
            pb.name, pb.window_sec
        )];
        for slot_id in &filled_slots {
            rationale.push(evidence_line(slot_id, &hyp.hits[slot_id].fact));
        }

        Self {
            playbook_id: pb.id.clone(),
            playbook_name: pb.name.clone(),
            severity: pb.severity.clone(),
            host,
            ts: now,
            tags,
            rationale,
            slots,
            filled_slots,
            missing_optional,
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

// Map a fact kind string used in YAML to the engine's kind.
fn parse_fact_kind(spec: &str) -> Option<FactKind> {
    match spec {
        "Exec" => Some(FactKind::Exec),
        "FileIO" => Some(FactKind::FileIO),
        "NetConn" => Some(FactKind::NetConn),
        "Auth" => Some(FactKind::Auth),
        "Kernel" => Some(FactKind::Kernel),
        "VendorAlert" => Some(FactKind::VendorAlert),
        // Counts are attached to FileIO and Exec facts (see agg_plan), so no
        // separate aggregate kind is produced.
        _ => None,
    }
}

fn fact_kind_matches(spec: &str, k: &FactKind) -> bool {
    parse_fact_kind(spec).as_ref() == Some(k)
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

    use std::fs;

    // Ids of the playbooks in playbooks/ that pass validation. The other 18 YAML
    // files are skipped with a logged reason.
    const SHIPPED_LOADABLE: &[&str] = &[
        "pb_container_breakout_precursors_linux",
        "pb_outage_ransomware_linux_attr_locker",
        "pb_outage_ransomware_linux_backup_wipe_first",
        "pb_outage_ransomware_linux_container_hostpath",
        "pb_outage_ransomware_linux_db_impact",
        "pb_outage_ransomware_linux_fileburst_crypto",
        "pb_outage_ransomware_linux_fuse_overlay",
        "pb_outage_ransomware_linux_intermit_encrypt",
        "pb_outage_ransomware_linux_luks_locker",
        "pb_outage_ransomware_linux_memfd_inplace",
        "pb_outage_ransomware_linux_netshare_only",
        "pb_outage_ransomware_linux_vm_image_datastore",
        "pb_outage_wiper_linux_service_kill",
    ];

    const GOOD: &str = r#"
id: pb_good
name: Good
severity: high
window_sec: 600
cooldown_sec: 60
required:
  - slot_id: miner
    slot_ttl_sec: 60
    fact: Exec
    where:
      - { field: exe, op: regex, value: "xmrig$" }
"#;

    fn temp_dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("edr_pb_{tag}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn exec_fact(exe: &str) -> Fact {
        Fact {
            ts: 100,
            kind: FactKind::Exec,
            host: "h".into(),
            user: None,
            exe: Some(exe.into()),
            cmd: None,
            fields: HashMap::new(),
        }
    }

    #[test]
    fn shipped_playbooks_load_or_are_reported() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("playbooks");
        let (engine, report) =
            PlaybookEngine::load_dir_report(&dir).expect("a bad file must not abort the load");
        report.log(&dir);

        let yaml_files = fs::read_dir(&dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yaml"))
            .count();
        // Every YAML file is either loaded or skipped with a reason. None vanish.
        assert_eq!(report.loaded.len() + report.skipped.len(), yaml_files);
        assert!(report.skipped.iter().all(|(_, why)| !why.is_empty()));
        // The only file the loader ignores is the empty engine.rs stub.
        assert_eq!(report.ignored, vec!["engine.rs".to_string()]);

        let mut loaded = report.loaded.clone();
        loaded.sort();
        assert_eq!(loaded, SHIPPED_LOADABLE);
        assert_eq!(engine.playbooks.len(), SHIPPED_LOADABLE.len());
    }

    #[test]
    fn bad_files_are_skipped_and_the_rest_load() {
        let d = temp_dir("bad");
        fs::write(d.join("a_good.yaml"), GOOD).unwrap();
        fs::write(d.join("b_broken.yaml"), "id: b\nname: Broken: colon\n").unwrap();
        fs::write(
            d.join("c_steps.yaml"),
            "id: c\nname: c\nseverity: low\nwindow_sec: 1\ncooldown_sec: 1\nsteps: []\n",
        )
        .unwrap();
        fs::write(
            d.join("d_netflow.yaml"),
            GOOD.replace("pb_good", "pb_d").replace("fact: Exec", "fact: NetFlow"),
        )
        .unwrap();
        fs::write(d.join("e_dup.yaml"), GOOD).unwrap();
        fs::write(d.join("notes.txt"), "not a playbook").unwrap();

        let (engine, report) =
            PlaybookEngine::load_dir_report(&d).expect("bad files must not abort the load");
        assert_eq!(report.loaded, vec!["pb_good".to_string()]);
        assert_eq!(engine.playbooks.len(), 1);
        assert_eq!(report.ignored, vec!["notes.txt".to_string()]);

        let want = [
            ("b_broken.yaml", "mapping values are not allowed"),
            ("c_steps.yaml", "no required slots (it uses keys this engine does not read: steps)"),
            ("d_netflow.yaml", "fact kind `NetFlow`"),
            ("e_dup.yaml", "duplicate playbook id `pb_good`"),
        ];
        assert_eq!(report.skipped.len(), want.len());
        for ((file, why), (want_file, want_why)) in report.skipped.iter().zip(want) {
            assert_eq!(file, want_file);
            assert!(why.contains(want_why), "{file}: {why}");
        }
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn dead_optional_and_forbidden_slots_are_warnings() {
        let d = temp_dir("warn");
        let pb = format!(
            "{GOOD}optional:
  - slot_id: gpu
    slot_ttl_sec: 60
    fact: HostPerf
forbidden:
  - slot_id: allow
    slot_ttl_sec: 60
    fact: Exec
    where:
      - {{ field: exe, op: not_in, value: [a] }}
links: []
"
        );
        fs::write(d.join("w.yaml"), pb).unwrap();
        let (_, report) = PlaybookEngine::load_dir_report(&d).unwrap();
        assert_eq!(report.loaded, vec!["pb_good".to_string()]);
        let warnings: Vec<&str> = report.warnings.iter().map(|(_, w)| w.as_str()).collect();
        assert!(warnings[0].starts_with("optional slot `gpu` can never match"), "{warnings:?}");
        assert!(warnings[1].starts_with("forbidden slot `allow` can never match"), "{warnings:?}");
        assert!(warnings[2].contains("links"), "{warnings:?}");
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn loaded_playbook_fires_on_a_matching_fact() {
        let d = temp_dir("fire");
        fs::write(d.join("a.yaml"), GOOD).unwrap();
        let mut eng = PlaybookEngine::load_dir(&d).unwrap();
        assert!(eng.ingest_fact(exec_fact("/usr/bin/ls")).is_empty());
        let hits = eng.ingest_fact(exec_fact("/usr/bin/xmrig"));
        assert_eq!(hits.len(), 1, "required slot filled");
        assert_eq!(hits[0].playbook_id, "pb_good");
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn reload_into_swaps_the_set_and_keeps_it_on_error() {
        let d = temp_dir("reload");
        fs::write(d.join("a.yaml"), GOOD).unwrap();
        let engine = std::sync::Mutex::new(PlaybookEngine::load_dir(&d).unwrap());
        assert_eq!(engine.lock().unwrap().playbooks.len(), 1);

        fs::write(d.join("b.yaml"), GOOD.replace("pb_good", "pb_second")).unwrap();
        let report = reload_into(&engine, &d).unwrap();
        assert_eq!(report.loaded.len(), 2);
        assert!(engine.lock().unwrap().playbooks.contains_key("pb_second"));

        // An unreadable directory is an error and the current set stays.
        assert!(reload_into(&engine, &d.join("missing")).is_err());
        assert_eq!(engine.lock().unwrap().playbooks.len(), 2);
        let _ = fs::remove_dir_all(&d);
    }

    // Two required steps 100 s apart at most, a long TTL, and a forbidden step.
    const TWO_STEP: &str = r#"
id: pb_two
name: Two
severity: high
window_sec: 100
cooldown_sec: 1000
required:
  - slot_id: a
    slot_ttl_sec: 1000
    fact: Exec
    where:
      - { field: exe, op: eq, value: /usr/bin/a }
  - slot_id: b
    slot_ttl_sec: 30
    fact: Exec
    where:
      - { field: exe, op: eq, value: /usr/bin/b }
forbidden:
  - slot_id: quiet
    slot_ttl_sec: 50
    fact: Exec
    where:
      - { field: exe, op: eq, value: /usr/bin/maint }
"#;

    fn engine_from(tag: &str, yaml: &str) -> PlaybookEngine {
        let d = temp_dir(tag);
        fs::write(d.join("p.yaml"), yaml).unwrap();
        let (eng, report) = PlaybookEngine::load_dir_report(&d).unwrap();
        assert_eq!(report.loaded.len(), 1, "{:?}", report.skipped);
        let _ = fs::remove_dir_all(&d);
        eng
    }

    fn exec_at(exe: &str, ts: u64) -> Fact {
        let mut f = exec_fact(exe);
        f.ts = ts;
        f
    }

    #[test]
    fn window_is_measured_over_the_evidence() {
        let mut eng = engine_from("span", TWO_STEP);
        let t0 = 1_800_000_000;
        assert!(eng.ingest_fact(exec_at("/usr/bin/a", t0)).is_empty());
        // b 150 s after a: outside the 100 s window.
        assert!(eng.ingest_fact(exec_at("/usr/bin/b", t0 + 150)).is_empty());
        // a again: a and b are now 20 s apart. The old engine timed the window from the
        // first hit of the hypothesis (t0), so this chain could never complete.
        let hits = eng.ingest_fact(exec_at("/usr/bin/a", t0 + 170));
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].ts, t0 + 170);
        assert_eq!(hits[0].filled_slots, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn tick_expires_hits_without_new_events() {
        let mut eng = engine_from("tick", TWO_STEP);
        let t0 = 1_800_000_000;
        assert!(eng.ingest_fact(exec_at("/usr/bin/b", t0)).is_empty());
        assert_eq!(eng.hypotheses.len(), 1);
        // b has a 30 s TTL. With no event in between, tick drops it and the empty hypothesis.
        eng.tick(t0 + 31);
        assert!(eng.hypotheses.is_empty());
        assert!(eng.ingest_fact(exec_at("/usr/bin/a", t0 + 40)).is_empty());
        // The TTL is also checked at completion, so this holds without tick too.
        let mut eng = engine_from("tick2", TWO_STEP);
        eng.ingest_fact(exec_at("/usr/bin/b", t0));
        assert!(eng.ingest_fact(exec_at("/usr/bin/a", t0 + 40)).is_empty());
    }

    #[test]
    fn live_forbidden_hit_vetoes_until_it_expires() {
        let mut eng = engine_from("veto", TWO_STEP);
        let t0 = 1_800_000_000;
        eng.ingest_fact(exec_at("/usr/bin/maint", t0));
        eng.ingest_fact(exec_at("/usr/bin/a", t0 + 1));
        assert!(eng.ingest_fact(exec_at("/usr/bin/b", t0 + 2)).is_empty(), "vetoed");
        // The forbidden hit has a 50 s TTL. After it, the same chain fires.
        eng.ingest_fact(exec_at("/usr/bin/a", t0 + 60));
        assert_eq!(eng.ingest_fact(exec_at("/usr/bin/b", t0 + 61)).len(), 1);
    }

    #[test]
    fn cooldown_holds_and_survives_reload() {
        let d = temp_dir("cool");
        fs::write(d.join("p.yaml"), TWO_STEP).unwrap();
        let engine = std::sync::Mutex::new(PlaybookEngine::load_dir(&d).unwrap());
        let t0 = 1_800_000_000;
        let run = |t: u64| {
            let mut eng = engine.lock().unwrap();
            eng.ingest_fact(exec_at("/usr/bin/a", t));
            eng.ingest_fact(exec_at("/usr/bin/b", t + 1)).len()
        };
        assert_eq!(run(t0), 1);
        assert_eq!(run(t0 + 500), 0, "inside the 1000 s cooldown");
        reload_into(&engine, &d).unwrap();
        assert_eq!(run(t0 + 600), 0, "a reload keeps the cooldown");
        assert_eq!(run(t0 + 1001), 1, "cooldown over");
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn tags_any_matches_any_tag() {
        let y = GOOD.replace(
            r#"- { field: exe, op: regex, value: "xmrig$" }"#,
            "- { field: tags_any, op: in, value: [RWX_MAP, MEMFD_EXEC] }",
        );
        let mut eng = engine_from("tags", &y);
        let mut f = exec_fact("/memfd:x (deleted)");
        f.fields.insert("tags".into(), "memory::w_then_x".into());
        assert!(eng.ingest_fact(f.clone()).is_empty());
        f.fields.insert("tags".into(), "memory::memfd_exec,MEMFD_EXEC".into());
        assert_eq!(eng.ingest_fact(f).len(), 1);
    }

    #[test]
    fn counting_slots_are_validated_at_load() {
        let slot = |fact: &str, preds: &str| {
            format!(
                "id: pb_c\nname: c\nseverity: high\nwindow_sec: 60\ncooldown_sec: 60\nrequired:\n  - slot_id: s\n    slot_ttl_sec: 60\n    fact: {fact}\n    where:\n{preds}"
            )
        };
        let cases = [
            (slot("FileIO", "      - { field: __agg.kind, op: eq, value: mystery }\n"), "unknown aggregate kind `mystery`"),
            (slot("Exec", "      - { field: __agg.kind, op: eq, value: file_burst }\n"), "counts FileIO facts, not Exec"),
            (slot("FileIO", "      - { field: __agg.kind, op: eq, value: errno_spike }\n      - { field: __agg.files, op: gte, value: 3 }\n"), "has no field `__agg.files`"),
            (slot("FileIO", "      - { field: __agg.kind, op: ne, value: file_burst }\n"), "must use op eq"),
            (slot("Agg", "      - { field: __agg.kind, op: eq, value: file_burst }\n"), "fact kind `Agg`"),
        ];
        for (i, (yaml, want)) in cases.iter().enumerate() {
            let d = temp_dir(&format!("agg{i}"));
            fs::write(d.join("p.yaml"), yaml).unwrap();
            let (_, report) = PlaybookEngine::load_dir_report(&d).unwrap();
            assert!(report.loaded.is_empty(), "{yaml}");
            assert!(report.skipped[0].1.contains(want), "{}", report.skipped[0].1);
            let _ = fs::remove_dir_all(&d);
        }
    }

    fn file_fact(op: &str, path: &str, ts: u64) -> Fact {
        let mut fields = HashMap::new();
        fields.insert("op".to_string(), op.to_string());
        fields.insert("path".to_string(), path.to_string());
        fields.insert("topdir".to_string(), topdir_of(path));
        Fact {
            ts,
            kind: FactKind::FileIO,
            host: "h".into(),
            user: None,
            exe: None,
            cmd: None,
            fields,
        }
    }

    #[test]
    fn file_burst_counts_distinct_files_in_the_window() {
        let y = r#"
id: pb_burst
name: Burst
severity: high
window_sec: 600
cooldown_sec: 600
required:
  - slot_id: burst
    slot_ttl_sec: 600
    fact: FileIO
    where:
      - { field: op, op: eq, value: write }
      - { field: topdir, op: in, value: [/home, /srv] }
      - { field: __agg.kind, op: eq, value: file_burst }
      - { field: __agg.files, op: gte, value: 3 }
      - { field: __agg.rename_ratio, op: lte, value: 0.5 }
      - { field: __agg.window_sec, op: lte, value: 60 }
"#;
        let mut eng = engine_from("burst", y);
        let t0 = 1_800_000_000;
        // The same file three times is one file. A file outside /home and /srv does not count.
        for i in 0..3 {
            assert!(eng.ingest_fact(file_fact("write", "/home/a/1", t0 + i)).is_empty());
        }
        assert!(eng.ingest_fact(file_fact("write", "/etc/x", t0 + 3)).is_empty());
        assert!(eng.ingest_fact(file_fact("write", "/srv/2", t0 + 4)).is_empty());
        // A plain open is not a change.
        assert!(eng.ingest_fact(file_fact("open", "/home/a/9", t0 + 5)).is_empty());
        let hits = eng.ingest_fact(file_fact("write", "/home/b/3", t0 + 6));
        assert_eq!(hits.len(), 1);
        let f = &hits[0].slots["burst"].fields;
        assert_eq!(f["__agg.files"], "3");
        assert_eq!(f["__agg.distinct_topdirs"], "2");
        assert_eq!(f["__agg.window_sec"], "60");
        assert_eq!(f["__agg.rename_ratio"], "0.000");

        // Renames in the same place count toward the ratio even though the slot asks
        // for writes: 3 renames of 6 changed paths is 0.5, a fourth file pushes it over.
        let mut eng = engine_from("burst2", y);
        for (i, p) in ["/home/r1", "/home/r2", "/home/r3"].iter().enumerate() {
            eng.ingest_fact(file_fact("rename", p, t0 + i as u64));
        }
        eng.ingest_fact(file_fact("write", "/home/w1", t0 + 4));
        eng.ingest_fact(file_fact("write", "/home/w2", t0 + 5));
        assert_eq!(eng.ingest_fact(file_fact("write", "/home/w3", t0 + 6)).len(), 1);
        let mut eng = engine_from("burst3", y);
        for (i, p) in ["/home/r1", "/home/r2", "/home/r3", "/home/r4"].iter().enumerate() {
            eng.ingest_fact(file_fact("rename", p, t0 + i as u64));
        }
        eng.ingest_fact(file_fact("write", "/home/w1", t0 + 5));
        eng.ingest_fact(file_fact("write", "/home/w2", t0 + 6));
        assert!(eng.ingest_fact(file_fact("write", "/home/w3", t0 + 7)).is_empty());

        // Events older than the 60 s window drop out of the count.
        let mut eng = engine_from("burst4", y);
        eng.ingest_fact(file_fact("write", "/home/1", t0));
        eng.ingest_fact(file_fact("write", "/home/2", t0 + 1));
        assert!(eng.ingest_fact(file_fact("write", "/home/3", t0 + 100)).is_empty());
        eng.tick(t0 + 200);
        assert!(eng.windows.is_empty(), "tick drops empty windows");
    }

    #[test]
    fn top_and_parent_dirs() {
        assert_eq!(topdir_of("/home/alice/doc.txt"), "/home");
        assert_eq!(topdir_of("/var/www/html/index.php"), "/var/www");
        assert_eq!(topdir_of("/run/user/1000/x"), "/run/user");
        assert_eq!(topdir_of("/usr/local/bin/x"), "/usr/local");
        assert_eq!(topdir_of("/srv"), "/srv");
        assert_eq!(topdir_of("/var"), "/var");
        assert_eq!(topdir_of("relative/x"), "-");
        assert_eq!(parent_dir("/home/a/b.txt"), "/home/a");
        assert_eq!(parent_dir("/b.txt"), "/");
    }
}


// src/services/trust_vector.rs
//! TrustVector: 10-D per-endpoint trust with dimension-targeted penalties,
//! damped penalties, decay toward baseline, causal history, and an extensible
//! tag→dimension mapping. Values are in [0.0, 1.0] where 1.0 = fully trusted.
//!
//! Improvements in this version:
//! - Clear, centralized tag→dimension mapping table with strengths
//! - Support for benign/meta tag filtering (prevents accidental penalties)
//! - Optional severity scaling (e.g., "dns_tunnel:high")
//! - Per-dimension time-based decay using `last_updated` (apply_decay_auto)
//! - Diagnostics helpers (explain_tag, top_deficits)
//! - Safer global helpers for updating/reading TRUST_VECTOR_GLOBAL

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub const TRUST_DIM_CT: usize = 10;

/// Global map (optional) for per-endpoint TrustVectors keyed by endpoint_id/host.
pub static TRUST_VECTOR_GLOBAL: Lazy<Mutex<HashMap<String, TrustVector>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Canonical dimensions (stable order/index).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrustDim {
    Memory = 0,      // RWX flips, injection, memfd, hollowing
    Network = 1,     // connect/accept, jitter, exfil, beacon signals
    Privilege = 2,   // setuid/capset/ptrace/seccomp/bpf/module tamper
    Persistence = 3, // autoruns, scheduler abuse, tamper
    FileSys = 4,     // unlink/rename/xattr/tamper, destructive ops
    Process = 5,     // exec/parent anomalies, fork storms, retained FDs
    Container = 6,   // setns/unshare/pivot_root
    Lateral = 7,     // cred dump, remote exec, pivots
    Beacon = 8,      // cadence/ASN/JA3 novelty, retrans spikes
    Stealth = 9,     // suppression/visibility gaps, anti-forensics
}

impl TrustDim {
    #[inline]
    pub fn idx(self) -> usize {
        self as usize
    }

    pub fn all() -> &'static [TrustDim; TRUST_DIM_CT] {
        use TrustDim::*;
        static ALL: [TrustDim; TRUST_DIM_CT] = [
            Memory,
            Network,
            Privilege,
            Persistence,
            FileSys,
            Process,
            Container,
            Lateral,
            Beacon,
            Stealth,
        ];
        &ALL
    }

    pub fn name(self) -> &'static str {
        use TrustDim::*;
        match self {
            Memory => "memory",
            Network => "network",
            Privilege => "privilege",
            Persistence => "persistence",
            FileSys => "filesystem",
            Process => "process",
            Container => "container",
            Lateral => "lateral",
            Beacon => "beacon",
            Stealth => "stealth",
        }
    }
}

impl fmt::Display for TrustDim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Per-endpoint trust vector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustVector {
    /// Per-dimension trust values in [0,1].
    pub v: [f32; TRUST_DIM_CT],
    /// Optional tag echoes for diagnostics/snapshots.
    pub tags: Vec<String>,
    /// Per-dimension last-updated timestamps (secs since epoch).
    #[serde(skip)]
    pub last_updated: [u64; TRUST_DIM_CT],
    /// Light causal history (bounded length; human-facing).
    #[serde(skip)]
    pub causal_history: HashMap<String, Vec<String>>,
    /// Cache of total deficit sum Σ(1 - v[i]) for quick overall score.
    #[serde(skip)]
    cached_deficit_sum: f32,
}

impl Default for TrustVector {
    fn default() -> Self {
        Self::new()
    }
}

impl TrustVector {
    /// Start fully trusted.
    pub fn new() -> Self {
        Self {
            v: [1.0; TRUST_DIM_CT],
            tags: Vec::new(),
            last_updated: [now_ts(); TRUST_DIM_CT],
            causal_history: HashMap::new(),
            cached_deficit_sum: 0.0,
        }
    }

    /// Construct from a list of tags (applies small, targeted penalties).
    pub fn from_tag_list(tags: &[String]) -> Self {
        let mut tv = Self::new();
        tv.apply_tags(tags);
        tv
    }

    /// Apply a batch of tags at once (uses mapping with benign filtering).
    pub fn apply_tags(&mut self, tags: &[String]) {
        for t in tags {
            self.apply_tag(t);
        }
        self.tags.extend_from_slice(tags);
        self.recompute_cache();
    }

    /// Direct penalty: subtract `strength` (≥0) from dimension trust.
    pub fn penalty(&mut self, dim: TrustDim, strength: f32) {
        if strength <= 0.0 {
            return;
        }
        let i = dim.idx();
        let before = self.v[i];
        self.v[i] = (self.v[i] - strength).clamp(0.0, 1.0);
        self.cached_deficit_sum += (before - self.v[i]).max(0.0);
        self.last_updated[i] = now_ts();
        self.push_history(dim, format!("penalty:{:.3}", strength));
    }

    /// Damped penalty (e.g., pass anomaly "damp" factor here).
    pub fn soft_penalty(&mut self, dim: TrustDim, base_strength: f32, damp: f32) {
        self.penalty(dim, (base_strength * damp).min(1.0));
    }

    /// Heal a dimension by some amount.
    pub fn heal(&mut self, dim: TrustDim, amount: f32) {
        if amount <= 0.0 {
            return;
        }
        let i = dim.idx();
        let before_def = 1.0 - self.v[i];
        self.v[i] = (self.v[i] + amount).clamp(0.0, 1.0);
        let after_def = 1.0 - self.v[i];
        self.cached_deficit_sum -= (before_def - after_def).max(0.0);
        self.last_updated[i] = now_ts();
        self.push_history(dim, format!("heal:{:.3}", amount));
    }

    /// Time decay: deficits (1 - v[i]) shrink by half every `half_life_s`.
    /// This applies a single dt across all dimensions (legacy behavior).
    pub fn apply_decay(&mut self, dt_s: f32, half_life_s: f32) {
        if half_life_s <= 0.0 || dt_s <= 0.0 {
            return;
        }
        let df = (0.5f32).powf(dt_s / half_life_s);
        self.cached_deficit_sum = 0.0;
        for i in 0..TRUST_DIM_CT {
            let deficit = (1.0 - self.v[i]) * df;
            self.v[i] = 1.0 - deficit;
            self.cached_deficit_sum += deficit;
        }
    }

    /// Improved decay: uses each dimension's `last_updated` to compute dt individually,
    /// then updates last_updated to `now_s`. Call this periodically.
    pub fn apply_decay_auto(&mut self, now_s: u64, half_life_s: f32) {
        if half_life_s <= 0.0 {
            return;
        }
        self.cached_deficit_sum = 0.0;
        for i in 0..TRUST_DIM_CT {
            let dt_s = (now_s.saturating_sub(self.last_updated[i])) as f32;
            if dt_s > 0.0 {
                let df = (0.5f32).powf(dt_s / half_life_s);
                let deficit = (1.0 - self.v[i]) * df;
                self.v[i] = 1.0 - deficit;
                self.last_updated[i] = now_s;
            }
            self.cached_deficit_sum += 1.0 - self.v[i];
        }
    }

    /// Conservative merge (min per dimension).
    pub fn merge_min(&mut self, other: &TrustVector) {
        for i in 0..TRUST_DIM_CT {
            let before = self.v[i];
            self.v[i] = self.v[i].min(other.v[i]);
            self.cached_deficit_sum += (before - self.v[i]).max(0.0);
        }
        if self.tags.len() < 128 {
            for t in &other.tags {
                if self.tags.len() >= 128 {
                    break;
                }
                if !self.tags.iter().any(|x| x == t) {
                    self.tags.push(t.clone());
                }
            }
        }
    }

    /// Weighted merge: linear interpolate each dimension by `w` (0..=1).
    pub fn merge_weighted(&mut self, other: &TrustVector, w: f32) {
        let w = w.clamp(0.0, 1.0);
        for i in 0..TRUST_DIM_CT {
            let before = self.v[i];
            self.v[i] = (1.0 - w) * self.v[i] + w * other.v[i];
            self.cached_deficit_sum += (before - self.v[i]).max(0.0);
        }
    }

    /// Overall trust = mean across dimensions.
    pub fn score_overall(&self) -> f32 {
        1.0 - (self.cached_deficit_sum / (TRUST_DIM_CT as f32))
    }

    /// Deficit (1 - trust) for a dimension.
    #[inline]
    pub fn deficit_by(&self, dim: TrustDim) -> f32 {
        1.0 - self.v[dim.idx()]
    }

    /// Convenience: apply penalty by dimension string (case-insensitive).
    pub fn penalty_by_name(&mut self, dim: &str, strength: f32) {
        if let Some(d) = dim_from_str(dim) {
            self.penalty(d, strength);
        }
    }

    /// Export as a map of "trust.<name>" → value (0.0–1.0).
    pub fn to_map(&self) -> HashMap<String, f64> {
        let mut m = HashMap::with_capacity(TRUST_DIM_CT);
        for d in TrustDim::all() {
            m.insert(format!("trust.{}", d.name()), self.v[d.idx()] as f64);
        }
        m
    }

    /// Top-K dimensions by deficit (largest risk first).
    pub fn top_deficits(&self, k: usize) -> Vec<(TrustDim, f32)> {
        let mut pairs: Vec<(TrustDim, f32)> = TrustDim::all()
            .iter()
            .map(|&d| (d, 1.0 - self.v[d.idx()]))
            .collect();
        pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        pairs.truncate(k);
        pairs
    }

    /// Keeps a small human-readable history per dimension.
    fn push_history(&mut self, dim: TrustDim, note: String) {
        let key = dim.name().to_string();
        let ent = self.causal_history.entry(key).or_insert_with(Vec::new);
        if ent.len() >= 64 {
            ent.remove(0);
        }
        ent.push(format!("{note}@{}", now_ts()));
    }

    /// Recompute cached deficit (call after batch updates).
    pub fn recompute_cache(&mut self) {
        self.cached_deficit_sum = self.v.iter().map(|&x| 1.0 - x).sum::<f32>();
    }

    // ----------------- Tag → dimension mapping (extensible, severity-aware) -----------------

    /// Apply a single tag using the mapping table. Benign/meta tags are ignored.
    pub fn apply_tag(&mut self, tag: &str) {
        let t = normalize_tag(tag);

        // 1) Drop obviously benign/meta tags
        if is_meta_tag(&t) {
            self.tags.push(tag.to_string());
            return;
        }

        // 2) Extract explicit severity if any (e.g., "dns_tunnel:high")
        let (core, sev_scale) = parse_severity(&t);

        // 3) Try the structured rule table first
        if let Some(rule) = MATCH_TABLE.iter().find(|r| r.matches(&core)) {
            // FIX: iterate the slice directly (avoid &&[...])
            for &(dim, base) in rule.dims {
                self.penalty(dim, (base * sev_scale).min(1.0));
            }
            self.tags.push(tag.to_string());
            return;
        }

        // 4) Fallback: legacy heuristic mapping (broad substr checks)
        legacy_apply_tag(self, &core, sev_scale);

        self.tags.push(tag.to_string());
    }

    /// Explain how a tag would map to dimensions (for diagnostics/UI).
    pub fn explain_tag(tag: &str) -> Vec<(TrustDim, f32, &'static str)> {
        let t = normalize_tag(tag);
        if is_meta_tag(&t) {
            return vec![];
        }
        let (core, sev_scale) = parse_severity(&t);
        if let Some(rule) = MATCH_TABLE.iter().find(|r| r.matches(&core)) {
            return rule
                .dims
                .iter()
                .map(|(d, base)| (*d, base * sev_scale, rule.name))
                .collect();
        }
        // Fallback explanation (heuristic, label as "legacy")
        legacy_explain(&core, sev_scale)
            .into_iter()
            .map(|(d, s)| (d, s, "legacy"))
            .collect()
    }
}

// --------------------- mapping & helpers ---------------------

/// Mapping rule: if any `patterns` is a substring of the tag, apply `dims` penalties.
struct TagRule {
    name: &'static str,
    patterns: &'static [&'static str],
    dims: &'static [(TrustDim, f32)],
}

impl TagRule {
    fn matches(&self, tag_lower: &str) -> bool {
        self.patterns.iter().any(|p| tag_lower.contains(p))
    }
}

/// Centralized mapping table (keep patterns all lowercase).
/// Strengths are conservative; many signals already compute risk elsewhere.
/// Add/adjust entries as new modules/signals arrive.
const MATCH_TABLE: &[TagRule] = &[
    // Memory / injection
    TagRule {
        name: "process_injection",
        patterns: &[
            "process_injection",
            "ptrace_injection",
            "inject",
            "hollow",
            "memfd",
            "mprotect_exec",
        ],
        dims: &[
            (TrustDim::Memory, 0.12),
            (TrustDim::Process, 0.08),
            (TrustDim::Stealth, 0.05),
        ],
    },
    TagRule {
        name: "cred_dump",
        patterns: &["cred_dump", "lsass", "dumpcred", "hashdump"],
        dims: &[
            (TrustDim::Lateral, 0.12),
            (TrustDim::Memory, 0.08),
            (TrustDim::Stealth, 0.06),
        ],
    },
    // Network / beacon / tunnel / proxy
    TagRule {
        name: "dns_tunnel",
        patterns: &["dns_tunnel", "dnstunnel"],
        dims: &[
            (TrustDim::Network, 0.10),
            (TrustDim::Beacon, 0.12),
            (TrustDim::Stealth, 0.04),
        ],
    },
    TagRule {
        name: "proxy_tools",
        patterns: &[
            "proxy_binary",
            "proxy_listener",
            "socks",
            "tor",
            "v2ray",
            "shadowsocks",
        ],
        dims: &[
            (TrustDim::Network, 0.08),
            (TrustDim::Beacon, 0.06),
            (TrustDim::Stealth, 0.05),
        ],
    },
    TagRule {
        name: "net_anomaly",
        patterns: &["network_anomaly", "suspicious_socket", "ja3", "beacon"],
        dims: &[(TrustDim::Network, 0.07), (TrustDim::Beacon, 0.05)],
    },
    // Privilege & kernel tamper
    TagRule {
        name: "priv_escalation",
        patterns: &[
            "privilege_escalation",
            "setuid_abuse",
            "capset",
            "seccomp",
            "bpf_usage",
            "kernel_module",
        ],
        dims: &[(TrustDim::Privilege, 0.15), (TrustDim::Stealth, 0.03)],
    },
    // Persistence / filesystem
    TagRule {
        name: "persistence",
        patterns: &[
            "suspicious_persistence",
            "persistence_mechanism",
            "autorun",
            "scheduler",
        ],
        dims: &[
            (TrustDim::Persistence, 0.12),
            (TrustDim::FileSys, 0.05),
            (TrustDim::Stealth, 0.03),
        ],
    },
    TagRule {
        name: "script_drop_tmp",
        patterns: &["script_drop", "tmp_script", "suspicious_tmp_script"],
        dims: &[
            (TrustDim::Persistence, 0.05),
            (TrustDim::FileSys, 0.04),
            (TrustDim::Stealth, 0.03),
        ],
    },
    // Lateral / auth abuse
    TagRule {
        name: "password_spray",
        patterns: &["password_spray", "brute_force"],
        dims: &[
            (TrustDim::Lateral, 0.10),
            (TrustDim::Beacon, 0.04),
            (TrustDim::Stealth, 0.02),
        ],
    },
    TagRule {
        name: "mfa_bypass",
        patterns: &["mfa_bypass", "push_fatigue"],
        dims: &[
            (TrustDim::Privilege, 0.12),
            (TrustDim::Lateral, 0.06),
            (TrustDim::Stealth, 0.04),
        ],
    },
    // IPC / process abuse
    TagRule {
        name: "ipc_abuse",
        patterns: &["ipc_abuse", "suspicious_ipc"],
        dims: &[
            (TrustDim::Process, 0.06),
            (TrustDim::Memory, 0.06),
            (TrustDim::Privilege, 0.04),
        ],
    },
    // Container / namespaces
    TagRule {
        name: "container_ns",
        patterns: &["setns", "unshare", "pivot_root", "container_escape"],
        dims: &[(TrustDim::Container, 0.08), (TrustDim::Privilege, 0.04)],
    },
    // USB peripheral (low severity, potential lateral)
    TagRule {
        name: "usb",
        patterns: &["usb_inserted", "usb_device_seen"],
        dims: &[(TrustDim::FileSys, 0.02), (TrustDim::Lateral, 0.02)],
    },
    // Global anomaly gates / outliers
    TagRule {
        name: "global_outlier",
        patterns: &["mahalanobis_outlier", "elliptic_outlier", "krim_alert"],
        dims: &[
            (TrustDim::Memory, 0.02),
            (TrustDim::Network, 0.02),
            (TrustDim::Privilege, 0.02),
            (TrustDim::Persistence, 0.02),
            (TrustDim::FileSys, 0.02),
            (TrustDim::Process, 0.02),
            (TrustDim::Container, 0.02),
            (TrustDim::Lateral, 0.02),
            (TrustDim::Beacon, 0.02),
            (TrustDim::Stealth, 0.02),
        ],
    },
];

/// Legacy heuristic mapping retained for compatibility; used if no rule matches.
fn legacy_apply_tag(tv: &mut TrustVector, t: &str, sev_scale: f32) {
    use TrustDim::*;
    let mut penalized = false;

    // Memory & injection signatures
    if t.contains("mprotect_exec")
        || t.contains("dllinject")
        || t.contains("proc_hollow")
        || t.contains("memfd")
        || t.contains("memory_anomaly")
    {
        tv.penalty(Memory, 0.12 * sev_scale);
        tv.penalty(Process, 0.05 * sev_scale);
        penalized = true;
    }

    // Network / beacon
    if t.starts_with("net_") || t.starts_with("tcp_") || t.contains("beacon") || t.contains("ja3") {
        tv.penalty(Network, 0.08 * sev_scale);
        if t.contains("state") || t.contains("retrans") || t.contains("beacon") {
            tv.penalty(Beacon, 0.08 * sev_scale);
        }
        penalized = true;
    }

    // Privilege / kernel tamper
    if t.contains("priv_")
        || t == "ptrace"
        || t == "seccomp"
        || t.contains("kernel_module")
        || t == "bpf_usage"
    {
        tv.penalty(Privilege, 0.12 * sev_scale);
        tv.penalty(Stealth, 0.04 * sev_scale);
        penalized = true;
    }

    // Filesystem / persistence
    if t.starts_with("file_") || t.starts_with("fs_") || t.contains("xattr") || t.contains("tamper")
    {
        tv.penalty(FileSys, 0.06 * sev_scale);
        penalized = true;
    }
    if t.contains("autorun") || t.contains("persistence") || t.contains("scheduler") {
        tv.penalty(Persistence, 0.08 * sev_scale);
        penalized = true;
    }

    // Container / namespaces
    if t.starts_with("ns_")
        || t.contains("setns")
        || t.contains("unshare")
        || t.contains("pivot_root")
    {
        tv.penalty(Container, 0.06 * sev_scale);
        penalized = true;
    }

    // Lateral movement cues
    if t.contains("cred_dump")
        || t.contains("lateral_move")
        || t.contains("psexec")
        || t.contains("winrm")
    {
        tv.penalty(Lateral, 0.12 * sev_scale);
        penalized = true;
    }

    // Global batch outlier gates
    if t == "mahalanobis_outlier" || t == "elliptic_outlier" || t == "krim_alert" {
        for d in TrustDim::all() {
            tv.penalty(*d, 0.02 * sev_scale);
        }
        penalized = true;
    }

    // Process churn
    if t == "exec" || t.contains("proc_fork") || t.contains("proc_exit") {
        tv.penalty(Process, 0.02 * sev_scale);
        penalized = true;
    }

    if !penalized {
        // Unknown tags are recorded but do not affect trust by default.
    }
}

/// Legacy explanation fallback used by `explain_tag`.
fn legacy_explain(t: &str, sev_scale: f32) -> Vec<(TrustDim, f32)> {
    use TrustDim::*;
    let mut v = Vec::new();
    if t.contains("mprotect_exec")
        || t.contains("dllinject")
        || t.contains("proc_hollow")
        || t.contains("memfd")
        || t.contains("memory_anomaly")
    {
        v.push((Memory, 0.12 * sev_scale));
        v.push((Process, 0.05 * sev_scale));
    }
    if t.starts_with("net_") || t.starts_with("tcp_") || t.contains("beacon") || t.contains("ja3") {
        v.push((Network, 0.08 * sev_scale));
        if t.contains("state") || t.contains("retrans") || t.contains("beacon") {
            v.push((Beacon, 0.08 * sev_scale));
        }
    }
    if t.contains("priv_")
        || t == "ptrace"
        || t == "seccomp"
        || t.contains("kernel_module")
        || t == "bpf_usage"
    {
        v.push((Privilege, 0.12 * sev_scale));
        v.push((Stealth, 0.04 * sev_scale));
    }
    if t.starts_with("file_") || t.starts_with("fs_") || t.contains("xattr") || t.contains("tamper")
    {
        v.push((FileSys, 0.06 * sev_scale));
    }
    if t.contains("autorun") || t.contains("persistence") || t.contains("scheduler") {
        v.push((Persistence, 0.08 * sev_scale));
    }
    if t.starts_with("ns_")
        || t.contains("setns")
        || t.contains("unshare")
        || t.contains("pivot_root")
    {
        v.push((Container, 0.06 * sev_scale));
    }
    if t.contains("cred_dump")
        || t.contains("lateral_move")
        || t.contains("psexec")
        || t.contains("winrm")
    {
        v.push((Lateral, 0.12 * sev_scale));
    }
    if t == "mahalanobis_outlier" || t == "elliptic_outlier" || t == "krim_alert" {
        for d in TrustDim::all() {
            v.push((*d, 0.02 * sev_scale));
        }
    }
    if t == "exec" || t.contains("proc_fork") || t.contains("proc_exit") {
        v.push((Process, 0.02 * sev_scale));
    }
    v
}

/// Normalize and lowercase an input tag.
fn normalize_tag(tag: &str) -> String {
    tag.trim().to_ascii_lowercase()
}

/// Simple severity parser: accepts suffixes ":low" (0.5x), ":med" (1.0x), ":high" (1.5x)
/// If numeric suffix like ":1.2" exists, multiplies by that factor (clamped to [0.1, 3.0]).
fn parse_severity(tag_lower: &str) -> (String, f32) {
    if let Some((core, sev)) = tag_lower.rsplit_once(':') {
        let scale = match sev {
            "low" => 0.5,
            "med" | "medium" => 1.0,
            "high" => 1.5,
            _ => sev
                .parse::<f32>()
                .ok()
                .map(|x| x.clamp(0.1, 3.0))
                .unwrap_or(1.0),
        };
        (core.to_string(), scale)
    } else {
        (tag_lower.to_string(), 1.0)
    }
}

/// Heuristic filter for meta/benign tags commonly found in telemetry payloads.
/// We do not want these to influence trust directly.
fn is_meta_tag(t: &str) -> bool {
    const META_PREFIXES: &[&str] = &[
        "replay_", "soc_", "env_", "trust.", "feature", "cpu", "mem", "host",
    ];
    const META_EXACT: &[&str] = &[
        "gnn_escalate",
        "timestamp",
        "category",
        "signal",
        "telemetry_batch",
        "forensic_replay",
        "passive",
        "fallback",
        "snapshot",
        "process_snapshot",
        "benign_suppressed",
        "note",
        "summary",
    ];
    if META_EXACT.iter().any(|&x| t == x) {
        return true;
    }
    META_PREFIXES.iter().any(|p| t.starts_with(p))
}

// --------------------- helpers ---------------------

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Parse various names/aliases to a TrustDim.
pub fn dim_from_str(s: &str) -> Option<TrustDim> {
    let t = s.to_ascii_lowercase();
    use TrustDim::*;
    Some(match t.as_str() {
        "memory" | "mem" => Memory,
        "network" | "net" => Network,
        "privilege" | "priv" | "kernel" => Privilege,
        "persistence" | "persist" => Persistence,
        "filesystem" | "file" | "fs" => FileSys,
        "process" | "proc" => Process,
        "container" | "cont" | "ns" => Container,
        "lateral" | "lat" => Lateral,
        "beacon" | "c2" => Beacon,
        "stealth" | "evade" => Stealth,
        _ => return None,
    })
}

/// Convenience: mutate an endpoint's TrustVector in the global map.
pub fn with_endpoint_tv<F, T>(endpoint_id: &str, f: F) -> T
where
    F: FnOnce(&mut TrustVector) -> T,
{
    let mut guard = TRUST_VECTOR_GLOBAL.lock().unwrap();
    let tv = guard
        .entry(endpoint_id.to_string())
        .or_insert_with(TrustVector::new);
    f(tv)
}

/// Public helper to update a TrustVector for an endpoint (penalties/heals + tags).
/// - `dims`: list of (`dimension_name`, `Some(penalty)`), or (`dimension_name`, `None`) to heal a bit.
/// - `tags`: optional tags to apply via the mapping table.
/// - `now_s`: if provided, run per-dimension auto-decay to this timestamp first.
pub fn update_trust_vector(
    endpoint_id: &str,
    dims: &[(&str, Option<f32>)],
    tags: &[String],
    now_s: Option<u64>,
) -> TrustVector {
    with_endpoint_tv(endpoint_id, |tv| {
        if let Some(ts) = now_s {
            // default half-life 1 hour; adjust by env in your caller if needed
            tv.apply_decay_auto(ts, 3600.0);
        }
        for (name, maybe_penalty) in dims {
            if let Some(dim) = dim_from_str(name) {
                match maybe_penalty {
                    Some(p) => tv.penalty(dim, *p),
                    None => tv.heal(dim, 0.05),
                }
            }
        }
        if !tags.is_empty() {
            tv.apply_tags(tags);
        }
        tv.clone()
    })
}

// --------------------- optional tests ---------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn penalties_and_decay() {
        let mut tv = TrustVector::new();
        tv.penalty(TrustDim::Memory, 0.3);
        assert!((tv.deficit_by(TrustDim::Memory) - 0.3).abs() < 1e-6);
        let pre = tv.score_overall();
        tv.apply_decay(3600.0, 3600.0); // one half-life (legacy)
        let post = tv.score_overall();
        assert!(post > pre);
    }

    #[test]
    fn from_tags() {
        let tv = TrustVector::from_tag_list(&vec!["mprotect_exec".into(), "net_connect".into()]);
        assert!(tv.deficit_by(TrustDim::Memory) > 0.0);
        assert!(tv.deficit_by(TrustDim::Network) > 0.0);
        assert!(tv.score_overall() < 1.0);
    }

    #[test]
    fn explain_and_meta() {
        // Meta should not map
        assert!(TrustVector::explain_tag("gnn_escalate").is_empty());
        // Known rule should map
        let e = TrustVector::explain_tag("dns_tunnel:high");
        assert!(!e.is_empty());
        assert!(e.iter().any(|(d, _, _)| *d == TrustDim::Beacon));
    }

    #[test]
    fn auto_decay_updates_each_dim() {
        let mut tv = TrustVector::new();
        tv.penalty(TrustDim::Process, 0.4);
        let then = tv.last_updated[TrustDim::Process.idx()];
        let now = then + 3600;
        let before = tv.v[TrustDim::Process.idx()];
        tv.apply_decay_auto(now, 3600.0);
        assert!(tv.v[TrustDim::Process.idx()] > before); // healed toward 1.0
    }
}

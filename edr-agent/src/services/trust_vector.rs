// src/trust_vector.rs
//! TrustVector: 10-D per-endpoint trust with dimension-targeted penalties,
//! soft penalties (with "damp"), decay toward baseline, causal history, and
//! tag→dimension mapping. Values are in [0.0, 1.0] where 1.0 = fully trusted.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Global map (optional) for per-endpoint TrustVectors keyed by endpoint_id/host.
pub static TRUST_VECTOR_GLOBAL: Lazy<Mutex<HashMap<String, TrustVector>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub const TRUST_DIM_CT: usize = 10;

/// Canonical dimensions (stable order/index).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrustDim {
    Memory = 0,     // RWX flips, injection, memfd, hollowing
    Network = 1,    // connect/accept, jitter, exfil, beacon signals
    Privilege = 2,  // setuid/capset/ptrace/seccomp/bpf/module tamper
    Persistence = 3,// autoruns, scheduler abuse, tamper
    FileSys = 4,    // unlink/rename/xattr/tamper, destructive ops
    Process = 5,    // exec/parent anomalies, fork storms, retained FDs
    Container = 6,  // setns/unshare/pivot_root
    Lateral = 7,    // cred dump, remote exec, pivots
    Beacon = 8,     // cadence/ASN/JA3 novelty, retrans spikes
    Stealth = 9,    // suppression/visibility gaps, anti-forensics
}

impl TrustDim {
    #[inline] pub fn idx(self) -> usize { self as usize }
    pub fn all() -> &'static [TrustDim; TRUST_DIM_CT] {
        use TrustDim::*;
        static ALL: [TrustDim; TRUST_DIM_CT] = [
            Memory, Network, Privilege, Persistence, FileSys,
            Process, Container, Lateral, Beacon, Stealth,
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.name()) }
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
    fn default() -> Self { Self::new() }
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
        for t in tags { tv.apply_tag(t); }
        tv.tags.extend_from_slice(tags);
        tv.recompute_cache();
        tv
    }

    /// Direct penalty: subtract `strength` (≥0) from dimension trust.
    pub fn penalty(&mut self, dim: TrustDim, strength: f32) {
        if strength <= 0.0 { return; }
        let i = dim.idx();
        let before = self.v[i];
        self.v[i] = (self.v[i] - strength).clamp(0.0, 1.0);
        self.cached_deficit_sum += (before - self.v[i]).max(0.0);
        self.last_updated[i] = now_ts();
        self.push_history(dim, format!("penalty:{:.3}", strength));
    }

    /// Damped penalty (e.g., pass Mahalanobis/Envelope "damp" here).
    pub fn soft_penalty(&mut self, dim: TrustDim, base_strength: f32, damp: f32) {
        self.penalty(dim, (base_strength * damp).min(1.0));
    }

    /// Heal a dimension by some amount.
    pub fn heal(&mut self, dim: TrustDim, amount: f32) {
        if amount <= 0.0 { return; }
        let i = dim.idx();
        let before_def = 1.0 - self.v[i];
        self.v[i] = (self.v[i] + amount).clamp(0.0, 1.0);
        let after_def = 1.0 - self.v[i];
        self.cached_deficit_sum -= (before_def - after_def).max(0.0);
        self.last_updated[i] = now_ts();
        self.push_history(dim, format!("heal:{:.3}", amount));
    }

    /// Time decay: deficits (1 - v[i]) shrink by half every `half_life_s`.
    pub fn apply_decay(&mut self, dt_s: f32, half_life_s: f32) {
        if half_life_s <= 0.0 || dt_s <= 0.0 { return; }
        let df = (0.5f32).powf(dt_s / half_life_s);
        self.cached_deficit_sum = 0.0;
        for i in 0..TRUST_DIM_CT {
            let deficit = (1.0 - self.v[i]) * df;
            self.v[i] = 1.0 - deficit;
            self.cached_deficit_sum += deficit;
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
                if self.tags.len() >= 128 { break; }
                if !self.tags.iter().any(|x| x == t) { self.tags.push(t.clone()); }
            }
        }
    }

    /// Overall trust = mean across dimensions.
    pub fn score_overall(&self) -> f32 {
        1.0 - (self.cached_deficit_sum / (TRUST_DIM_CT as f32))
    }

    /// Deficit (1 - trust) for a dimension.
    #[inline] pub fn deficit_by(&self, dim: TrustDim) -> f32 {
        1.0 - self.v[dim.idx()]
    }

    /// Convenience: apply penalty by dimension string (case-insensitive).
    pub fn penalty_by_name(&mut self, dim: &str, strength: f32) {
        if let Some(d) = dim_from_str(dim) { self.penalty(d, strength); }
    }

    /// Export as a map of "trust.<name>" → value (0.0–1.0).
    pub fn to_map(&self) -> HashMap<String, f64> {
        let mut m = HashMap::with_capacity(TRUST_DIM_CT);
        for d in TrustDim::all() {
            m.insert(format!("trust.{}", d.name()), self.v[d.idx()] as f64);
        }
        m
    }

    /// Keeps a small human-readable history per dimension.
    fn push_history(&mut self, dim: TrustDim, note: String) {
        let key = dim.name().to_string();
        let ent = self.causal_history.entry(key).or_insert_with(Vec::new);
        if ent.len() >= 64 { ent.remove(0); }
        ent.push(format!("{note}@{}", now_ts()));
    }

    /// Recompute cached deficit (call after batch updates).
    pub fn recompute_cache(&mut self) {
        self.cached_deficit_sum = self.v.iter().map(|&x| 1.0 - x).sum::<f32>();
    }

    // ----------------- Tag → dimension mapping (small targeted nudges) -----------------

    pub fn apply_tag(&mut self, tag: &str) {
        use TrustDim::*;
        let t = tag.to_ascii_lowercase();

        // Memory & injection signatures
        if t.contains("mprotect_exec") || t.contains("dllinject") || t.contains("proc_hollow")
            || t.contains("memfd") || t.contains("memory_anomaly")
        {
            self.penalty(Memory, 0.12);
            self.penalty(Process, 0.05);
        }

        // Network / beacon
        if t.starts_with("net_") || t.starts_with("tcp_") || t.contains("beacon") || t.contains("ja3") {
            self.penalty(Network, 0.08);
            if t.contains("state") || t.contains("retrans") || t.contains("beacon") {
                self.penalty(Beacon, 0.08);
            }
        }

        // Privilege / kernel tamper
        if t.contains("priv_") || t == "ptrace" || t == "seccomp" || t.contains("kernel_module") || t == "bpf_usage" {
            self.penalty(Privilege, 0.12);
            self.penalty(Stealth, 0.04);
        }

        // Filesystem / persistence
        if t.starts_with("file_") || t.starts_with("fs_") || t.contains("xattr") || t.contains("tamper") {
            self.penalty(FileSys, 0.06);
        }
        if t.contains("autorun") || t.contains("persistence") || t.contains("scheduler") {
            self.penalty(Persistence, 0.08);
        }

        // Container / namespaces
        if t.starts_with("ns_") || t.contains("setns") || t.contains("unshare") || t.contains("pivot_root") {
            self.penalty(Container, 0.06);
        }

        // Lateral movement cues
        if t.contains("cred_dump") || t.contains("lateral_move") || t.contains("psexec") || t.contains("winrm") {
            self.penalty(Lateral, 0.12);
        }

        // Global batch outlier gates
        if t == "mahalanobis_outlier" || t == "elliptic_outlier" || t == "krim_alert" {
            for d in TrustDim::all() { self.penalty(*d, 0.02); }
        }

        // Process churn
        if t == "exec" || t.contains("proc_fork") || t.contains("proc_exit") {
            self.penalty(Process, 0.02);
        }

        self.tags.push(tag.to_string());
    }
}

// --------------------- helpers ---------------------

fn now_ts() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
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
        tv.apply_decay(3600.0, 3600.0); // one half-life
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
}

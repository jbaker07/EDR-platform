// src/services/trust_kernel.rs
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;

use crate::services::trust_vector::TrustVector;

// --------------------------- tuning knobs ---------------------------

/// Max number of historical TrustVector snapshots retained by the kernel.
const KERNEL_HISTORY_CAP: usize = 200;
/// Minimum snapshots required before we compute distances.
const MIN_HISTORY_FOR_ANOMALY: usize = 8;
/// Numerical floor for std-dev to avoid division by zero.
const Z_EPS: f64 = 1e-6;
/// Default anomaly threshold for RMS z-score (≈ diagonal Mahalanobis).
const DEFAULT_MAHAL_THRESHOLD: f64 = 3.0;

/// Per-key duplicate suppression window (seconds).
const FREQ_WINDOW_SECS: u64 = 300; // 5 minutes
/// If the key appeared at least this many times inside the window,
/// treat it as "frequent" (return true).
const FREQ_MIN_HITS_SUPPRESS: usize = 2;

/// Maximum per-dimension history length for low-level z-scoring API.
const MAX_DIM_HISTORY: usize = 25;

// --------------------------- kernel struct ---------------------------

/// Main TrustKernel tracker for per-endpoint trust dynamics.
/// Stores a rolling window of TrustVector snapshots and exposes
/// diagonal Mahalanobis (RMS z-score) anomaly scoring.
#[derive(Debug)]
pub struct TrustKernel {
    history: VecDeque<HashMap<String, f64>>,
    last_anomaly: Option<(HashMap<String, f64>, f64)>,
    mahal_threshold: f64,
}

impl Default for TrustKernel {
    fn default() -> Self {
        Self::new()
    }
}

impl TrustKernel {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(KERNEL_HISTORY_CAP),
            last_anomaly: None,
            mahal_threshold: DEFAULT_MAHAL_THRESHOLD,
        }
    }

    /// Optionally override the anomaly threshold at runtime.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.mahal_threshold = threshold.max(0.0);
        self
    }

    /// Push the latest TrustVector snapshot into the kernel, prune history,
    /// and compute the diagonal Mahalanobis/RMS z-score distance.
    ///
    /// Returns Some(distance) when enough history exists, otherwise None.
    /// If the distance exceeds the configured threshold, `last_anomaly` is updated.
    pub fn update(&mut self, vector: &TrustVector) -> Option<f64> {
        let snapshot = vector.to_map();

        if self.history.len() == KERNEL_HISTORY_CAP {
            self.history.pop_front();
        }
        self.history.push_back(snapshot.clone());

        let distance = self.compute_mahalanobis(&snapshot)?;

        if distance > self.mahal_threshold {
            self.last_anomaly = Some((snapshot, distance));
        }

        Some(distance)
    }

    /// Compute a diagonal-covariance Mahalanobis proxy:
    /// 1) compute per-dimension mean/std from history
    /// 2) convert current value → z = (x - μ)/σ (σ floored by Z_EPS)
    /// 3) return RMS(z) across all dimensions
    pub fn compute_mahalanobis(&self, vector: &HashMap<String, f64>) -> Option<f64> {
        if self.history.len() < MIN_HISTORY_FOR_ANOMALY {
            return None;
        }

        // We assume TrustVector::to_map returns the same dimension set every time.
        // Still, be defensive and only operate over the intersection of keys present now.
        let mut dims: Vec<&str> = vector.keys().map(|k| k.as_str()).collect();
        dims.sort_unstable();

        // Compute per-dimension mean and std from history.
        let mut means: HashMap<&str, f64> = HashMap::with_capacity(dims.len());
        let mut stds: HashMap<&str, f64> = HashMap::with_capacity(dims.len());

        for &dim in &dims {
            // Gather history values (default to 1.0 for missing to bias toward "fully trusted")
            let mut vals = Vec::with_capacity(self.history.len());
            for snap in &self.history {
                vals.push(*snap.get(dim).unwrap_or(&1.0));
            }
            let mean = mean(&vals);
            let std = stddev(&vals, mean).max(Z_EPS);
            means.insert(dim, mean);
            stds.insert(dim, std);
        }

        // Compute RMS z-score
        let mut sum_sq = 0.0;
        let mut n = 0.0;
        for &dim in &dims {
            if let Some(&x) = vector.get(dim) {
                let mu = *means.get(dim).unwrap_or(&1.0);
                let sd = *stds.get(dim).unwrap_or(&1.0);
                let z = (x - mu) / sd;
                sum_sq += z * z;
                n += 1.0;
            }
        }
        if n == 0.0 {
            return None;
        }
        Some((sum_sq / n).sqrt())
    }

    /// True if the last update exceeded the anomaly threshold.
    pub fn is_anomalous(&self) -> bool {
        self.last_anomaly.is_some()
    }

    /// Returns the last anomalous snapshot and its distance, if any.
    pub fn last_anomaly(&self) -> Option<(HashMap<String, f64>, f64)> {
        self.last_anomaly.clone()
    }

    /// Number of snapshots currently retained.
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Clear history and last anomaly state.
    pub fn reset(&mut self) {
        self.history.clear();
        self.last_anomaly = None;
    }
}

// --------------------------- global caches ---------------------------

/// Historical cache for individual dimensions (used by the per-dimension API).
static TRUST_HISTORY: Lazy<Mutex<HashMap<String, VecDeque<f64>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Time-bounded frequency memory to avoid repeated triggers.
/// Key → list of recent timestamps (secs).
static FREQUENT_ESC_MEM: Lazy<Mutex<HashMap<String, VecDeque<u64>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Legitimate binaries that often escalate (canonical paths or filename match).
static LEGIT_ESCALATORS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| {
    let mut s = HashSet::new();
    // Common escalators
    for p in [
        "/usr/bin/sudo",
        "/usr/bin/pkexec",
        "/bin/su",
        "/usr/sbin/service",
        "/usr/bin/systemctl",
        // Add a few more frequently-seen helpers:
        "/usr/bin/doas",
        "/usr/bin/newgrp",
    ] {
        s.insert(p.to_string());
    }
    Mutex::new(s)
});

// --------------------------- public helpers (used by modules) ---------------------------

/// Low-dimensional z-score for a single "dimension" stream.
/// Maintains a bounded history and returns |z| if enough samples exist.
pub fn calculate_mahalanobis_distance(dim: &str, current_score: f64) -> Option<f64> {
    let mut history_map = TRUST_HISTORY.lock().ok()?;
    let history = history_map
        .entry(dim.to_string())
        .or_insert_with(VecDeque::new);

    if history.len() >= MAX_DIM_HISTORY {
        history.pop_front();
    }
    history.push_back(current_score);

    if history.len() < (MAX_DIM_HISTORY / 2) {
        return None;
    }

    let mean = mean_deque(history);
    let std_dev = stddev_deque(history, mean).max(Z_EPS);
    Some(((current_score - mean).abs()) / std_dev)
}

/// Record-and-check frequency: returns true if `key` has been seen
/// at least `FREQ_MIN_HITS_SUPPRESS` times within the last `FREQ_WINDOW_SECS`.
///
/// This function both records the current hit and performs windowed suppression.
/// (Callers that merely want to "record" can ignore the return value.)
pub fn check_frequent_escalation(key: &str) -> bool {
    let now = now_ts();
    let mut mem = FREQUENT_ESC_MEM.lock().unwrap();
    let dq = mem.entry(key.to_string()).or_insert_with(VecDeque::new);

    // prune stale
    while let Some(&ts) = dq.front() {
        if now.saturating_sub(ts) > FREQ_WINDOW_SECS {
            dq.pop_front();
        } else {
            break;
        }
    }

    // push current
    dq.push_back(now);

    dq.len() >= FREQ_MIN_HITS_SUPPRESS
}

/// Returns true if `path` is a known legitimate escalator.
/// We canonicalize when possible and also allow basename matches.
pub fn is_known_legit_escalator(path: &str) -> bool {
    let canon = canonicalize_lossy(path);
    let fname = Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let ok = LEGIT_ESCALATORS
        .lock()
        .map(|set| {
            set.contains(&canon)
                || set.contains(path)
                || set.contains(fname) // allow entries like "sudo"
                || set.iter().any(|p| p.ends_with('/') && canon.starts_with(p)) // prefix dir rule
        })
        .unwrap_or(false);

    // Allow very common filename-only match (e.g., chrooted sudo)
    ok || fname == "sudo" || fname == "doas" || fname == "pkexec"
}

/// Allow runtime registration of additional known-good escalators.
pub fn register_legit_escalator<P: AsRef<str>>(p: P) {
    if let Ok(mut set) = LEGIT_ESCALATORS.lock() {
        set.insert(p.as_ref().to_string());
    }
}

// --------------------------- small utils ---------------------------

/// Unix timestamp (seconds)
fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn mean(nums: &[f64]) -> f64 {
    if nums.is_empty() {
        return 0.0;
    }
    nums.iter().sum::<f64>() / (nums.len() as f64)
}

fn stddev(nums: &[f64], mean: f64) -> f64 {
    if nums.len() < 2 {
        return 0.0;
    }
    let var = nums.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (nums.len() as f64);
    var.sqrt()
}

fn mean_deque(dq: &VecDeque<f64>) -> f64 {
    if dq.is_empty() {
        return 0.0;
    }
    dq.iter().sum::<f64>() / (dq.len() as f64)
}

fn stddev_deque(dq: &VecDeque<f64>, mean: f64) -> f64 {
    if dq.len() < 2 {
        return 0.0;
    }
    let var = dq.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (dq.len() as f64);
    var.sqrt()
}

/// Best-effort canonicalization that falls back gracefully and returns a String.
fn canonicalize_lossy<P: AsRef<Path>>(p: P) -> String {
    fs::canonicalize(&p)
        .ok()
        .unwrap_or_else(|| PathBuf::from(p.as_ref()))
        .to_string_lossy()
        .to_string()
}

// --------------------------- optional tests ---------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::trust_vector::{TrustDim, TrustVector};

    #[test]
    fn freq_window_basic() {
        let key = "pid:123";
        // ensure clean slate
        {
            let mut mem = FREQUENT_ESC_MEM.lock().unwrap();
            mem.remove(key);
        }
        assert_eq!(check_frequent_escalation(key), false);
        // second hit within window should flag as frequent
        assert_eq!(check_frequent_escalation(key), true);
    }

    #[test]
    fn mahalanobis_rms_zscore_grows() {
        let mut kernel = TrustKernel::new();
        // feed stable history
        for _ in 0..MIN_HISTORY_FOR_ANOMALY {
            let tv = TrustVector::new();
            kernel.update(&tv);
        }
        // create a deviation on one dim
        let mut tv = TrustVector::new();
        tv.penalty(TrustDim::Network, 0.5);
        let d = kernel.update(&tv).unwrap();
        assert!(d > 0.0);
    }

    #[test]
    fn known_escalator_matches_basename() {
        assert!(is_known_legit_escalator("/weird/chroot/bin/sudo"));
    }
}

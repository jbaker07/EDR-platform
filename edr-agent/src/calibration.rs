// src/calibration.rs
//! Rolling, robust per-feature calibrator.
//! - Keeps a bounded window (default 4096) per (host,family,key)
//! - Computes median, MAD, and empirical CDF rank (p-emp)
//! - Persists to JSON so calibration travels with the host
//!
//! Use: see main.rs wiring—observe() then calibrate().

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct CalibKey {
    pub host: String,
    pub family: String,
    pub key: String,
}
impl PartialEq for CalibKey {
    fn eq(&self, o: &Self) -> bool {
        self.host == o.host && self.family == o.family && self.key == o.key
    }
}
impl Hash for CalibKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.host.hash(state);
        self.family.hash(state);
        self.key.hash(state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollingSeries {
    cap: usize,
    window: VecDeque<f64>,
}
impl RollingSeries {
    pub fn with_cap(cap: usize) -> Self {
        Self { cap, window: VecDeque::with_capacity(cap) }
    }
    pub fn push(&mut self, x: f64) {
        if self.window.len() == self.cap {
            self.window.pop_front();
        }
        self.window.push_back(x);
    }
    fn sorted(&self) -> Vec<f64> {
        let mut v: Vec<f64> = self.window.iter().copied().collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        v
    }
    pub fn len(&self) -> usize { self.window.len() }
    pub fn is_ready(&self) -> bool { self.window.len() >= 32 }

    pub fn median(&self) -> f64 {
        let v = self.sorted();
        if v.is_empty() { return 0.0; }
        let n = v.len();
        if n % 2 == 1 { v[n/2] } else { 0.5*(v[n/2 - 1] + v[n/2]) }
    }
    pub fn mad(&self, med: f64) -> f64 {
        if self.window.is_empty() { return 1e-6; }
        let mut devs: Vec<f64> = self.window.iter().map(|x| (x - med).abs()).collect();
        devs.sort_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let n = devs.len();
        if n % 2 == 1 { devs[n/2] } else { 0.5*(devs[n/2 - 1] + devs[n/2]) }
    }
    pub fn quantile(&self, q: f64) -> f64 {
        let v = self.sorted();
        if v.is_empty() { return 0.0; }
        let n = v.len();
        let idx = ((n as f64 - 1.0) * q.clamp(0.0,1.0)).round() as usize;
        v[idx]
    }
    pub fn p_emp(&self, x: f64) -> f64 {
        if self.window.is_empty() { return 0.5; }
        let v = self.sorted();
        let n = v.len() as f64;
        // Rank-based ECDF with 0.5/n smoothing
        let k = match v.binary_search_by(|y| y.partial_cmp(&x).unwrap()) {
            Ok(i) => i as f64,
            Err(i) => i as f64,
        };
        ((k + 0.5) / (n + 1.0)).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistImage {
    cap: usize,
    series: HashMap<CalibKey, RollingSeries>,
}

pub struct RollingCalibrator {
    cap: usize,
    series: HashMap<CalibKey, RollingSeries>,
    root: PathBuf,
    dirty: bool,
}
impl Default for RollingCalibrator {
    fn default() -> Self {
        Self::new(4096, PathBuf::from("state/calibration"))
    }
}
impl RollingCalibrator {
    pub fn new(cap: usize, root: PathBuf) -> Self {
        let mut me = Self { cap, series: HashMap::new(), root, dirty: false };
        let _ = me.load(); // best-effort
        me
    }

    fn series_mut(&mut self, key: &CalibKey) -> &mut RollingSeries {
        self.series.entry(key.clone()).or_insert_with(|| RollingSeries::with_cap(self.cap))
    }

    pub fn observe(&mut self, host: &str, family: &str, key: &str, value: f64) {
        let k = CalibKey { host: host.into(), family: family.into(), key: key.into() };
        self.series_mut(&k).push(value);
        self.dirty = true;
    }

    pub fn calibrate(&self, host: &str, family: &str, key: &str, value: f64) -> CalibResult {
        let k = CalibKey { host: host.into(), family: family.into(), key: key.into() };
        if let Some(s) = self.series.get(&k) {
            let med = s.median();
            let mad = s.mad(med).max(1e-6);
            let z_robust = 0.6745 * (value - med) / mad;
            let p_emp = s.p_emp(value);
            CalibResult {
                ready: s.is_ready(),
                count: s.len(),
                median: med,
                mad,
                z_robust,
                p_emp,
            }
        } else {
            CalibResult { ready: false, count: 0, median: 0.0, mad: 1.0, z_robust: 0.0, p_emp: 0.5 }
        }
    }

    pub fn flush(&mut self) {
        if !self.dirty { return; }
        let _ = fs::create_dir_all(&self.root);
        let path = self.root.join("rolling_calibration.json");
        let img = PersistImage { cap: self.cap, series: self.series.clone() };
        if let Ok(txt) = serde_json::to_string_pretty(&img) {
            let _ = fs::write(path, txt);
        }
        self.dirty = false;
    }

    fn load(&mut self) {
        let path = self.root.join("rolling_calibration.json");
        if let Ok(txt) = fs::read_to_string(&path) {
            if let Ok(img) = serde_json::from_str::<PersistImage>(&txt) {
                self.cap = img.cap;
                self.series = img.series;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CalibResult {
    pub ready: bool,
    pub count: usize,
    pub median: f64,
    pub mad: f64,
    pub z_robust: f64,
    pub p_emp: f64,
}

/// Softplus for smooth positive mapping.
pub fn softplus(x: f64) -> f64 { (1.0 + x.exp()).ln() }

/// Logit on (0,1); clamps to avoid infinities.
pub fn logit(p: f64) -> f64 {
    let q = p.clamp(1e-6, 1.0 - 1e-6);
    (q / (1.0 - q)).ln()
}

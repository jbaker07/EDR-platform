use std::collections::{HashMap, VecDeque};
use std::f64::EPSILON;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static TRUST_VECTOR_GLOBAL: Lazy<Mutex<HashMap<String, TrustVector>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

use lazy_static::lazy_static;

lazy_static! {
    pub static ref BASELINES: HashMap<String, TrustVector> = {
        let mut m = HashMap::new();
        // Example: insert some default roles
        m.insert("default".to_string(), TrustVector::new());
        m
    };

    pub static ref DEFAULT_BASELINE: TrustVector = TrustVector::new();
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrustVector {
    pub dimensions: HashMap<String, f64>,
    pub last_updated: HashMap<String, u64>,
    pub causal_history: HashMap<String, Vec<String>>,
    pub history: HashMap<String, Vec<f64>>,
    pub dimension_stats: HashMap<String, DimensionStats>,
}

impl TrustVector {
    pub fn new() -> Self {
        let mut dimensions = HashMap::new();
        let mut last_updated = HashMap::new();
        let history = HashMap::new();
        let dimension_stats = HashMap::new();

        for dim in [
            "process", "memory", "network", "file", "auth",
            "geo", "usb", "script", "container", "graph"
        ] {
            dimensions.insert(dim.to_string(), 100.0);
            last_updated.insert(dim.to_string(), now_ts());
        }

        Self {
            dimensions,
            last_updated,
            causal_history: HashMap::new(),
            history,
            dimension_stats,
        }
    }
     pub fn has_valid_stats(&self) -> bool {
        for (dim, _) in &self.dimensions {
            if let Some(stats) = self.get_stats(dim) {
                if stats.variance.sqrt() == 0.0 || stats.count < 5 {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
    pub fn get(&self, dim: &str) -> f64 {
        *self.dimensions.get(dim).unwrap_or(&100.0)
    }

    pub fn set(&mut self, dim: &str, score: f64) {
        self.dimensions.insert(dim.to_string(), score.clamp(0.0, 100.0));
        self.last_updated.insert(dim.to_string(), now_ts());
    }

    pub fn apply_penalty(&mut self, dim: &str, amount: f64, reason_tag: &str) {
        let current = self.get(dim);
        self.set(dim, current - amount);

        self.causal_history
            .entry(dim.to_string())
            .or_insert_with(Vec::new)
            .push(format!("{}@{}", reason_tag, now_ts()));
    }

    pub fn apply_tagged_penalty(&mut self, tag: &str) {
        let penalty_map = get_tag_penalty_map(tag);
        for (dim, penalty) in penalty_map {
            self.apply_penalty(&dim, penalty, tag);
        }
    }

    pub fn get_stats(&self, dim: &str) -> Option<&DimensionStats> {
        self.dimension_stats.get(dim)
    }
    pub fn apply_decay(&mut self, decay_rate: f64, exponent: f64) {
        let now = now_ts();

        for (dim, score) in self.dimensions.clone() {
            let last = self.last_updated.get(&dim).cloned().unwrap_or(now);
            let dt = ((now - last) as f64) / 60.0; // minutes
            if dt > 0.0 {
                let decay_amount = decay_rate * dt.powf(exponent);
                self.set(&dim, score - decay_amount);
            }
        }
    }

    pub fn compute_weighted_score(&self) -> f64 {
        let weights = [
            ("process", 1.2),
            ("memory", 1.2),
            ("network", 1.1),
            ("file", 1.1),
            ("auth", 1.0),
            ("geo", 0.8),
            ("usb", 0.7),
            ("script", 1.0),
            ("container", 1.0),
            ("graph", 1.3),
        ];

        let mut total = 0.0;
        let mut weight_sum = 0.0;

        for (dim, weight) in weights.iter() {
            let val = self.get(dim);
            total += val * weight;
            weight_sum += weight;
        }

        total / weight_sum
    }

    pub fn compute_average(&self) -> f64 {
        let total: f64 = self.dimensions.values().sum();
        total / (self.dimensions.len() as f64)
    }

    pub fn get_history(&self, dim: &str) -> Vec<String> {
        self.causal_history.get(dim).cloned().unwrap_or_default()
    }

    pub fn to_map(&self) -> HashMap<String, f64> {
        self.dimensions.clone()
    }

    pub fn set_dimension_score(&mut self, key: &str, value: f64) {
        self.dimensions.insert(key.to_string(), value);
        self.history.entry(key.to_string()).or_default().push(value);

        self.dimension_stats
            .entry(key.to_string())
            .or_insert_with(|| DimensionStats::new(50))
            .update(value);
    }

    pub fn get_mahalanobis_for(&self, dim: &str) -> Option<f64> {
        if let (Some(stats), Some(current)) = (self.dimension_stats.get(dim), self.dimensions.get(dim)) {
            Some(stats.mahalanobis(*current))
        } else {
            None
        }
    }
}

pub fn mahalanobis_distance(current: &TrustVector, baseline: &TrustVector) -> f64 {
    let mut sum_sq = 0.0;
    let mut count = 0;

    for (dim, score) in &current.dimensions {
        if let Some(base_score) = baseline.dimensions.get(dim) {
            let diff = score - base_score;
            sum_sq += diff * diff;
            count += 1;
        }
    }

    if count == 0 {
        return 0.0;
    }

    (sum_sq / count as f64).sqrt()
}





#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionStats {
    pub mean: f64,
    pub variance: f64,
    pub count: usize,

    // Add these fields for sliding window stats
    pub recent_values: VecDeque<f64>,
    pub max_len: usize,
}

impl DimensionStats {
    pub fn new(max_len: usize) -> Self {
        Self {
            count: 0,
            mean: 0.0,
            variance: 0.0,
            recent_values: VecDeque::with_capacity(max_len),
            max_len,
        }
    }

    pub fn update(&mut self, value: f64) {
        if self.recent_values.len() >= self.max_len {
            self.recent_values.pop_front();
        }
        self.recent_values.push_back(value);
    }

    pub fn mean(&self) -> f64 {
        if self.recent_values.is_empty() {
            return 0.0;
        }
        self.recent_values.iter().copied().sum::<f64>() / self.recent_values.len() as f64
    }

    pub fn variance(&self) -> f64 {
        let mean = self.mean();
        self.recent_values
            .iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>()
            / (self.recent_values.len() as f64 + EPSILON)
    }

     pub fn mahalanobis(&self, current: f64) -> f64 {
        let var = if self.variance <= EPSILON { EPSILON } else { self.variance };
        ((current - self.mean).powi(2) / var).sqrt()
    }
}

pub fn update_trust_vector(vector: &mut TrustVector, category: &str, delta: f64) {
    let current = vector.get(category);
    vector.set(category, current + delta);
    vector
        .causal_history
        .entry(category.to_string())
        .or_insert_with(Vec::new)
        .push(format!("delta:{}@{}", delta, now_ts()));
}

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn get_tag_penalty_map(tag: &str) -> HashMap<String, f64> {
    let mut map = HashMap::new();
    match tag {
        "evasion::reflective_injection" => {
            map.insert("memory".to_string(), 40.0);
            map.insert("process".to_string(), 20.0);
        }
        "insider::script_exfil" => {
            map.insert("script".to_string(), 25.0);
            map.insert("file".to_string(), 30.0);
            map.insert("auth".to_string(), 10.0);
        }
        "network::beaconing" => {
            map.insert("network".to_string(), 35.0);
        }
        "gnn::cluster_shift" => {
            map.insert("graph".to_string(), 20.0);
        }
        _ => {
            map.insert("process".to_string(), 10.0);
        }
    }
    map
}

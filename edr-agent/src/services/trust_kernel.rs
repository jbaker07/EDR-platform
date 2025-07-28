use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;

use crate::services::trust_vector::TrustVector;

const HISTORY_SIZE: usize = 100;
const MAX_HISTORY: usize = 25;

/// Main TrustKernel tracker for high-dimensional Mahalanobis scoring
#[derive(Debug)]
pub struct TrustKernel {
    pub history: VecDeque<HashMap<String, f64>>,
    pub last_anomaly: Option<(HashMap<String, f64>, f64)>,
}

/// Historical cache for individual dimensions
static TRUST_HISTORY: Lazy<Mutex<HashMap<String, VecDeque<f64>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

/// Frequency tracker to avoid repeated triggers
static FREQUENT_ESCALATIONS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| {
    Mutex::new(HashSet::new())
});

/// Legitimate binaries that often escalate
static LEGIT_ESCALATORS: Lazy<HashSet<String>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert("/usr/bin/sudo".to_string());
    s.insert("/usr/bin/pkexec".to_string());
    s.insert("/bin/su".to_string());
    s.insert("/usr/sbin/service".to_string());
    s.insert("/usr/bin/systemctl".to_string());
    s
});

impl TrustKernel {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(HISTORY_SIZE),
            last_anomaly: None,
        }
    }

    pub fn update(&mut self, vector: &TrustVector) {
        let snapshot = vector.to_map();

        if self.history.len() == HISTORY_SIZE {
            self.history.pop_front();
        }
        self.history.push_back(snapshot.clone());

        if let Some(distance) = self.compute_mahalanobis(&snapshot) {
            if distance > 3.0 {
                self.last_anomaly = Some((snapshot, distance));
                // Optional: push TrustEvent here
            }
        }
    }

    pub fn compute_mahalanobis(&self, vector: &HashMap<String, f64>) -> Option<f64> {
        if self.history.len() < 5 {
            return None;
        }

        let dims: Vec<String> = vector.keys().cloned().collect();
        let mut means = HashMap::new();

        for dim in &dims {
            let avg = self.history.iter()
                .map(|v| v.get(dim).unwrap_or(&100.0))
                .sum::<f64>() / self.history.len() as f64;
            means.insert(dim.clone(), avg);
        }

        let mut variance_sum = 0.0;
        for dim in &dims {
            let mean = means.get(dim).unwrap_or(&100.0);
            if let Some(val) = vector.get(dim) {
                variance_sum += (val - mean).powi(2);
            }
        }

        let mahalanobis_distance = (variance_sum.sqrt()) / (dims.len() as f64).sqrt();
        Some(mahalanobis_distance)
    }

    pub fn is_anomalous(&self) -> bool {
        self.last_anomaly.is_some()
    }

    pub fn last_anomaly(&self) -> Option<(HashMap<String, f64>, f64)> {
        self.last_anomaly.clone()
    }
}

/// Unix timestamp
fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Low-dimensional Mahalanobis scoring (per dimension)
pub fn calculate_mahalanobis_distance(dim: &str, current_score: f64) -> Option<f64> {
    let mut history_map = TRUST_HISTORY.lock().ok()?;
    let history = history_map.entry(dim.to_string()).or_insert_with(VecDeque::new);

    if history.len() >= MAX_HISTORY {
        history.pop_front();
    }
    history.push_back(current_score);

    if history.len() < 10 {
        return None;
    }

    let mean = history.iter().copied().sum::<f64>() / history.len() as f64;
    let variance = history.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / history.len() as f64;
    let std_dev = variance.sqrt();

    if std_dev == 0.0 {
        return None;
    }

    Some((current_score - mean).abs() / std_dev)
}

/// Prevent redundant escalations from same key
pub fn check_frequent_escalation(key: &str) -> bool {
    let mut mem = FREQUENT_ESCALATIONS.lock().unwrap();
    let was_already_seen = !mem.insert(key.to_string());
    was_already_seen
}

/// Check if binary path is commonly used for escalation
pub fn is_known_legit_escalator(path: &str) -> bool {
    LEGIT_ESCALATORS.contains(path)
}

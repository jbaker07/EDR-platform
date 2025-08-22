// trust_diff_explainer.rs

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustSnapshot {
    pub timestamp: DateTime<Utc>,
    pub trust_vector: HashMap<String, f64>, // dimension -> score
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustDiffExplanation {
    pub dimension: String,
    pub old_score: f64,
    pub new_score: f64,
    pub delta: f64,
    pub interpretation: String,
}

pub fn compare_trust_snapshots(old: &TrustSnapshot, new: &TrustSnapshot) -> Vec<TrustDiffExplanation> {
    let mut diffs = Vec::new();

    for (dimension, new_score) in &new.trust_vector {
        let old_score = old.trust_vector.get(dimension).cloned().unwrap_or(0.0);
        let delta = new_score - old_score;

        let interpretation = if delta.abs() < 0.01 {
            "No meaningful change".to_string()
        } else if delta < -10.0 {
            format!("Sharp drop in {} trust", dimension)
        } else if delta < 0.0 {
            format!("Mild decay in {} trust", dimension)
        } else if delta > 10.0 {
            format!("Significant gain in {} trust", dimension)
        } else {
            format!("Slight increase in {} trust", dimension)
        };

        diffs.push(TrustDiffExplanation {
            dimension: dimension.clone(),
            old_score,
            new_score: *new_score,
            delta,
            interpretation,
        });
    }

    diffs
}

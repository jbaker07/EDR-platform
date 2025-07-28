// trust_score_breakdown.rs
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustScoreImpact {
    pub module: String,            // e.g., "auth_monitor"
    pub dimension: String,         // e.g., "auth", "net", "process", "memory"
    pub impact: f64,               // score delta (positive or negative)
    pub reason: String,            // e.g., "mfa_bypass_detected"
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustBreakdown {
    pub endpoint_id: String,
    pub role: String,
    pub impacts: Vec<TrustScoreImpact>,
}

impl TrustBreakdown {
    pub fn new(endpoint_id: &str, role: &str) -> Self {
        Self {
            endpoint_id: endpoint_id.to_string(),
            role: role.to_string(),
            impacts: Vec::new(),
        }
    }

    pub fn add_impact(&mut self, module: &str, dimension: &str, impact: f64, reason: &str) {
        self.impacts.push(TrustScoreImpact {
            module: module.to_string(),
            dimension: dimension.to_string(),
            impact,
            reason: reason.to_string(),
            timestamp: Utc::now(),
        });
    }

    pub fn summarize(&self) -> HashMap<String, f64> {
        let mut summary = HashMap::new();
        for impact in &self.impacts {
            let entry = summary.entry(impact.dimension.clone()).or_insert(0.0);
            *entry += impact.impact;
        }
        summary
    }

    pub fn export_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or_else(|_| "{}".to_string())
    }
}

// edr-agent/risk_alert_writer.rs

use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
pub struct RiskAlert {
    pub timestamp: u64,
    pub endpoint_id: String,
    pub trust_score: f64,
    pub risk_level: String,
    pub summary: String,
}

pub fn emit_risk_alert(
    endpoint_id: &str,
    trust_score: f64,
    risk_level: &str,
    summary: &str,
) -> RiskAlert {
    let alert = RiskAlert {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        endpoint_id: endpoint_id.to_string(),
        trust_score,
        risk_level: risk_level.to_string(),
        summary: summary.to_string(),
    };

    // You could send this alert to a remote server or local queue here.
    println!("🚨 RISK ALERT: {:?}", alert);
    alert
}

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::detect::Finding;
use crate::telemetry::Telemetry;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity { None, Low, Medium, High }

#[derive(Debug, Serialize)]
pub struct Alert {
    pub ts: DateTime<Utc>,
    pub risk: f32,                // 0..1
    pub severity: Severity,
    pub event: Telemetry,         // original event snapshot
    pub findings: Vec<Finding>,   // contributing signals (rules/anomaly/anchors)
    pub explanation: String,      // compact human string
}

impl Alert {
    /// Default constructor builds an explanation from findings.
    pub fn new(ts: DateTime<Utc>, event: Telemetry, risk: f32, severity: Severity, findings: Vec<Finding>) -> Self {
        let explanation = findings.iter()
            .map(|f| format!("{}:{}{}", f.source, f.score,
                f.label.as_ref().map(|l| format!("({})", l)).unwrap_or_default()))
            .collect::<Vec<_>>()
            .join(", ");
        Self { ts, risk, severity, event, findings, explanation }
    }

    /// Explicit explanation override (used for benign audit).
    pub fn new_with_explanation(
        ts: DateTime<Utc>,
        event: Telemetry,
        risk: f32,
        severity: Severity,
        findings: Vec<Finding>,
        explanation: String,
    ) -> Self {
        Self { ts, risk, severity, event, findings, explanation }
    }
}

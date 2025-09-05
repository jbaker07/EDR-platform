// src/detectors/mod.rs
use serde::Serialize;

use crate::telemetry::Telemetry; // expects your Telemetry struct

pub mod rules;
pub mod registry; // add when you drop in a registry

/// A detection result from any detector (rules/anomaly/anchors/etc.)
#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub source: String,           // e.g., "rules", "mahalanobis"
    pub score: f32,               // [0,1]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,    // rule id / model label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Common interface for detectors.
/// `features` is the shared numeric vector extracted from the Telemetry event.
/// Pure rules engines can ignore it.
pub trait Detector: Send + Sync {
    fn name(&self) -> &'static str;
    fn supports(&self, _t: &Telemetry) -> bool { true }
    fn score(&self, t: &Telemetry, features: &[f64]) -> Vec<Finding>;
}

pub mod rules_hot;
pub mod tag_flags;        // added below

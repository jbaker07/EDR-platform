// src/detectors/tag_flags.rs
use serde_json::json;
use serde_json::Value;

use crate::detect::types::NormalizedAlert;
use crate::detect::{Detector, Finding};

pub struct TagFlagsDetector;

fn truthy(v: &Value) -> bool {
    match v {
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().map(|x| x != 0.0).unwrap_or(false),
        Value::String(s) => {
            let s = s.trim();
            s == "1" || s.eq_ignore_ascii_case("true") || s.eq_ignore_ascii_case("yes")
        }
        _ => false,
    }
}

fn has_flag(attrs: &Value, key: &str) -> bool {
    // 1) direct attribute, e.g. attributes["mahalanobis_outlier"] == true/"true"/"1"
    if let Some(v) = attrs.get(key) {
        if truthy(v) {
            return true;
        }
    }
    // 2) nested attributes.tags[key] == true/"true"/"1"
    if let Some(tags) = attrs.get("tags") {
        if let Some(v) = tags.get(key) {
            if truthy(v) {
                return true;
            }
        }
    }
    false
}

impl Detector for TagFlagsDetector {
    fn name(&self) -> &'static str {
        "tag_flags"
    }

    // NormalizedAlert is the event type the Detector trait expects in your codebase
    fn score(&self, e: &NormalizedAlert, _features: &[f64]) -> Vec<Finding> {
        let attrs = &e.attributes;
        let mut out = Vec::new();

        if has_flag(attrs, "mahalanobis_outlier") {
            out.push(Finding {
                source: "mahalanobis".into(),
                score: 0.75,
                label: Some("mahal_outlier".into()),
                details: Some(json!({ "reason": "batch χ²" })),
            });
        }
        if has_flag(attrs, "elliptic_outlier") {
            out.push(Finding {
                source: "elliptic_envelope".into(),
                score: 0.70,
                label: Some("elliptic_outlier".into()),
                details: Some(json!({ "reason": "margin<0" })),
            });
        }
        if has_flag(attrs, "krim_alert") {
            out.push(Finding {
                source: "krim_lite".into(),
                score: 0.80,
                label: Some("krim_graph".into()),
                details: Some(json!({ "reason": "graph anomaly" })),
            });
        }

        out
    }
}

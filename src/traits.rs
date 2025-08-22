// src/traits.rs
//! Feature emission interface: turn an `Episode` into structured observations
//! that can be z-scored, tagged, and shipped to the scoring pipeline.
//!
//! Goals:
//! - Keep your existing numeric `value: f64` (no breaking change).
//! - Add metadata (ts, unit, confidence, desc, tags) for richer sinks and UIs.
//! - Provide small builder helpers so emitters stay terse and readable.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::episode::Episode;

/// A single feature observation produced by an emitter.
///
/// - `key`: stable identifier (e.g., "mem.wtox_latency_sec").
/// - `value`: the raw numeric statistic (before z-scaling).
/// - `z`: standardized score relative to baselines (positive or negative).
/// - `family`: grouping key for UI/routing (e.g., "mem", "net", "priv").
/// - `tags`: optional extra labels (e.g., ["wtox", "memfd"]).
/// - `unit`: human hint for `value` (e.g., "sec", "bytes", "count").
/// - `confidence`: 0..1; how confident the emitter is in this observation.
/// - `desc`: short human-readable description for explainers/sinks.
/// - `ts`: capture time (UTC).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureObservation {
    pub key: String,
    pub value: f64,
    pub z: f64,
    pub family: String,
    pub tags: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,

    pub ts: DateTime<Utc>,
}

impl Default for FeatureObservation {
    fn default() -> Self {
        Self {
            key: String::new(),
            value: 0.0,
            z: 0.0,
            family: "misc".to_string(),
            tags: Vec::new(),
            unit: None,
            confidence: None,
            desc: None,
            ts: Utc::now(),
        }
    }
}

impl FeatureObservation {
    /// Quick constructor for numeric features.
    pub fn numeric<K: Into<String>, F: Into<String>>(family: F, key: K, value: f64, z: f64) -> Self {
        Self {
            key: key.into(),
            value,
            z,
            family: family.into(),
            ..Default::default()
        }
    }

    /// Add a tag.
    pub fn tag<S: Into<String>>(mut self, t: S) -> Self {
        self.tags.push(t.into());
        self
    }

    /// Add multiple tags.
    pub fn tags<I, S>(mut self, it: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags.extend(it.into_iter().map(Into::into));
        self
    }

    /// Attach a unit label (e.g., "sec", "bytes").
    pub fn unit(mut self, u: &'static str) -> Self {
        self.unit = Some(u);
        self
    }

    /// Set confidence (0..1).
    pub fn confidence(mut self, c: f32) -> Self {
        self.confidence = Some(c.clamp(0.0, 1.0));
        self
    }

    /// Human description.
    pub fn desc<S: Into<String>>(mut self, d: S) -> Self {
        self.desc = Some(d.into());
        self
    }

    /// Override timestamp (defaults to now).
    pub fn with_ts(mut self, ts: DateTime<Utc>) -> Self {
        self.ts = ts;
        self
    }

    /// Is this feature anomalous beyond a |z|-threshold?
    pub fn is_anomalous(&self, z_threshold: f64) -> bool {
        self.z.abs() > z_threshold
    }
}

/// Emitters convert an `Episode` into a list of `FeatureObservation`s.
/// They may compute z-scores using `crate::baselines::BaselineStore`.
pub trait FeatureEmitter {
    /// Optional human-readable name (for logs/metrics).
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Produce features from an episode, using the baselines store for shaping.
    fn emit(
        &self,
        ep: &Episode,
        baselines: &mut crate::baselines::BaselineStore,
    ) -> Vec<FeatureObservation>;
}

// ---------- Small utilities for emitters ----------

/// Keep only features with |z| above `threshold`.
pub fn filter_anomalies<'a>(
    feats: impl IntoIterator<Item = &'a FeatureObservation>,
    threshold: f64,
) -> Vec<&'a FeatureObservation> {
    feats
        .into_iter()
        .filter(|f| f.is_anomalous(threshold))
        .collect()
}

/// Return a terse summary helpful in logs or debug UIs.
pub fn summarize_features(feats: &[FeatureObservation]) -> String {
    if feats.is_empty() {
        return "0 feats".into();
    }
    let n = feats.len();
    let max = feats
        .iter()
        .max_by(|a, b| a.z.abs().partial_cmp(&b.z.abs()).unwrap())
        .map(|f| format!("max|z| {:.2} @ {}:{}", f.z, f.family, f.key))
        .unwrap_or_default();
    format!("{n} feats; {max}")
}

/// Convenience macro to build a numeric feature in one line:
/// ```ignore
/// let f = feat!("mem", "wtox_latency_sec", 1.23, 3.1).unit("sec").tag("wtox");
/// ```
#[macro_export]
macro_rules! feat {
    ($family:expr, $key:expr, $val:expr, $z:expr) => {
        $crate::traits::FeatureObservation::numeric($family, $key, $val as f64, $z as f64)
    };
}

use std::path::Path;

use anyhow::Result;

use crate::telemetry::Telemetry;
use super::{Detector, Finding};
use super::rules::RulesDetector;

/// Holds all detectors (rules/anomaly/anchors/etc.) and runs them for each event.
pub struct DetectorRegistry {
    detectors: Vec<Box<dyn Detector>>,
}

impl DetectorRegistry {
    pub fn new() -> Self {
        Self { detectors: Vec::new() }
    }

    /// Add any detector instance.
    pub fn add(&mut self, detector: Box<dyn Detector>) {
        self.detectors.push(detector);
    }

    /// Convenience: load rules from a JSON file (tolerates extra top-level keys).
    pub fn add_rules_from(&mut self, path: &Path) -> Result<()> {
        if path.exists() {
            let det = RulesDetector::from_file(path)?;
            self.add(Box::new(det));
        }
        Ok(())
    }

    /// Run all detectors that support this event and collect findings.
    pub fn run_all(&self, t: &Telemetry, features: &[f64]) -> Vec<Finding> {
        let mut out = Vec::new();
        for d in &self.detectors {
            if d.supports(t) {
                out.extend(d.score(t, features));
            }
        }
        out
    }
}

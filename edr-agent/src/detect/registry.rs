// src/detect/registry.rs
// DetectorRegistry (rules / anomaly / ML detectors registry) used by pipeline.rs
use crate::detect::rules::RulesDetector;
use crate::detect::{Detector, Finding, NormalizedEvent}; // alias exported by detect::mod
use anyhow::Result;
use std::path::Path;

pub struct DetectorRegistry {
    detectors: Vec<Box<dyn Detector>>,
}

impl Default for DetectorRegistry {
    fn default() -> Self {
        Self {
            detectors: Vec::new(),
        }
    }
}

impl DetectorRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, detector: Box<dyn Detector>) {
        self.detectors.push(detector);
    }

    pub fn add_rules_from(&mut self, path: &Path) -> Result<()> {
        let det = RulesDetector::from_file(path)?;
        self.add(Box::new(det));
        Ok(())
    }

    /// Back-compat alias for older pipeline code.
    pub fn run_all(&self, event: &NormalizedEvent, features: &[f64]) -> Vec<Finding> {
        self.score_all(event, features)
    }

    /// Score an event with all registered detectors and flatten results.
    pub fn score_all(&self, event: &NormalizedEvent, features: &[f64]) -> Vec<Finding> {
        let mut out = Vec::new();
        for det in &self.detectors {
            if det.supports(event) {
                out.extend(det.score(event, features));
            }
        }
        out
    }
}

// src/pipeline.rs
use std::fs::File;
use std::io::Write;

use anyhow::Result;
use chrono::Utc;
use serde_json::{to_string, json};

use crate::alert::{Alert, Severity};
use crate::detect::{Finding, registry::DetectorRegistry, NormalizedEvent}; // alias
use crate::detect::types::NormalizedAlert; // concrete struct we construct
use crate::features::extract_features;
use crate::telemetry::Telemetry;

/// Simple thresholds for mapping risk -> severity.
#[derive(Clone, Debug)]
pub struct Thresholds {
    pub low: f32,
    pub medium: f32,
    pub high: f32,
}

/// Minimal evaluator that runs registered detectors and optionally writes JSONL alerts.
pub struct Evaluator {
    pub registry: DetectorRegistry,
    pub thresholds: Thresholds,
    out: Option<File>,
    audit_benign: bool,
}

impl Evaluator {
    pub fn new(thresholds: Thresholds) -> Self {
        Self { registry: DetectorRegistry::new(), thresholds, out: None, audit_benign: false }
    }

    pub fn set_audit_benign(&mut self, yes: bool) { self.audit_benign = yes; }

    pub fn set_output(&mut self, path: &std::path::Path) -> Result<()> {
        if let Some(dir) = path.parent() { std::fs::create_dir_all(dir)?; }
        let f = File::options().create(true).append(true).open(path)?;
        self.out = Some(f);
        Ok(())
    }

    pub fn evaluate(&mut self, event: &Telemetry) -> Result<Option<Alert>> {
        let features = extract_features(event);

        // Build a NormalizedAlert and pass it as &NormalizedEvent (alias)
        let norm = to_normalized_alert(event);
        let findings = self.registry.run_all(&norm as &NormalizedEvent, &features);

        let (risk, sev) = grade(&findings, &self.thresholds);

        let explanation = if findings.is_empty() {
            format!("benign: no detectors fired; risk={:.2} (< low {:.2})", risk, self.thresholds.low)
        } else {
            let parts = findings.iter().map(|f| {
                let lbl = f.label.as_deref().unwrap_or("-");
                format!("{}:{}({})", f.source, f.score, lbl)
            }).collect::<Vec<_>>().join(", ");

            match sev {
                Severity::None =>
                    format!("benign: risk={:.2} (< low {:.2}); contributions: {}", risk, self.thresholds.low, parts),
                _ => parts,
            }
        };

        let alert = match sev {
            Severity::None if !self.audit_benign => return Ok(None),
            _ => Alert::new_with_explanation(Utc::now(), event.clone(), risk, sev, findings, explanation),
        };

        let line = to_string(&alert)?;
        println!("{}", line);
        if let Some(f) = self.out.as_mut() {
            let _ = writeln!(f, "{}", line);
        }

        Ok(Some(alert))
    }
}

fn grade(findings: &[Finding], th: &Thresholds) -> (f32, Severity) {
    if findings.is_empty() { return (0.0, Severity::None); }
    let mut sum = 0.0_f32;
    for f in findings { sum += f.score; }
    let risk = (sum / (findings.len() as f32)).clamp(0.0, 1.0);

    let sev = if risk >= th.high { Severity::High }
        else if risk >= th.medium { Severity::Medium }
        else if risk >= th.low { Severity::Low }
        else { Severity::None };

    (risk, sev)
}

/// Minimal conversion so rules can read from `attributes`/`entities`.
fn to_normalized_alert(event: &Telemetry) -> NormalizedAlert {
    let attrs = serde_json::to_value(event).unwrap_or(json!({}));
    NormalizedAlert {
        id: format!("telemetry:{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)),
        ts: Utc::now(),
        source: "agent".to_string(),
        connector_id: "edr-agent".to_string(),
        severity: 0.0,
        title: "telemetry".to_string(),
        short_why: "local telemetry event".to_string(),
        entities: json!({}),
        attributes: attrs,
        mitre: vec![],
    }
}

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct TrustHardness {
    pub volatility_score: f64,      // System stability (0–1, higher = more unstable)
    pub tamper_signals: usize,      // Count of tampering indicators
    pub log_gap_detected: bool,     // True if telemetry gaps observed
    pub source_module: String,      // E.g., "process_monitor", "network", etc.
    pub timestamp: u64,             // UNIX timestamp of when hardness was computed
    pub audit_note: Option<String>, // Optional notes for audit trail
}

impl TrustHardness {
    /// Constructs a new TrustHardness with current timestamp
    pub fn new(
        volatility_score: f64,
        tamper_signals: usize,
        log_gap_detected: bool,
        source_module: &str,
        audit_note: Option<String>,
    ) -> Self {
        Self {
            volatility_score,
            tamper_signals,
            log_gap_detected,
            source_module: source_module.into(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            audit_note,
        }
    }

    /// Final computed trust hardness score
    pub fn calculate(&self) -> f64 {
        let mut score = 1.0;

        // Volatility penalty
        score -= match self.volatility_score {
            v if v > 0.9 => 0.3,
            v if v > 0.7 => 0.2,
            v if v > 0.5 => 0.1,
            _ => 0.0,
        };

        // Tamper signal penalty
        score -= 0.08 * self.tamper_signals as f64;

        // Log gap penalty
        if self.log_gap_detected {
            score -= 0.35;
        }

        score.clamp(0.0, 1.0)
    }

    /// Export as map (for GNN, logging, or SHAP explanations)
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("volatility_score".into(), format!("{:.2}", self.volatility_score));
        map.insert("tamper_signals".into(), self.tamper_signals.to_string());
        map.insert("log_gap_detected".into(), self.log_gap_detected.to_string());
        map.insert("source_module".into(), self.source_module.clone());
        map.insert("timestamp".into(), self.timestamp.to_string());
        map.insert("trust_hardness".into(), format!("{:.2}", self.calculate()));
        if let Some(note) = &self.audit_note {
            map.insert("audit_note".into(), note.clone());
        }
        map
    }

    /// Should this event trigger an escalation alert?
    pub fn triggers_escalation(&self) -> bool {
        self.calculate() < 0.4 || self.log_gap_detected || self.tamper_signals >= 3
    }
}

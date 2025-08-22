use crate::telemetry::TelemetryRecord;
use std::collections::HashMap;
use serde::Serialize;

/// Max/min bounds for scores
const MAX_RISK: f32 = 100.0;
const MIN_RISK: f32 = 0.0;

/// Core event emitted to the trust pipeline / logger.
#[derive(Debug, Clone, Serialize)]
pub struct TrustEvent {
    pub timestamp: u64,
    pub pid: i32,
    pub ppid: i32,
    pub uid: u32,
    pub binary_path: String,
    pub command_line: String,
    pub cwd: String,
    pub anomaly_type: String,
    pub component: String,
    pub metadata: HashMap<String, String>,
    /// Primary risk value in [0, 100]
    pub risk_score: f32,
    pub source_module: String,
    pub decay_context: Option<String>,
    // Optional fields for full coverage and backward compatibility
    pub module: Option<String>,
    pub signal: Option<String>,
    pub signal_type: Option<String>,
    pub score: Option<f32>,     // mirrors risk_score unless caller overrides
    pub raw_score: Option<f32>, // mirrors risk_score unless caller overrides
    pub tags: Option<Vec<String>>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TrustVerdict {
    Normal,
    Monitor,
    IsolateImmediately { score: f64, reason: String },
}

// ------------------------- Constructors -------------------------

impl TrustEvent {
    /// Minimal constructor: everything else is defaulted / empty.
    pub fn new_minimal(
        timestamp: u64,
        pid: i32,
        ppid: i32,
        uid: u32,
        binary_path: String,
        command_line: String,
        cwd: String,
        anomaly_type: String,
        component: String,
        source_module: String,
    ) -> Self {
        let mut ev = Self {
            timestamp,
            pid,
            ppid,
            uid,
            binary_path,
            command_line,
            cwd,
            anomaly_type,
            component: component.clone(),
            metadata: HashMap::new(),
            risk_score: 0.0,
            source_module: source_module.clone(),
            decay_context: Some(format!("{}_behavior", component)),
            module: Some(source_module),
            signal: None,
            signal_type: None,
            score: None,
            raw_score: None,
            tags: None,
            description: None,
        };
        ev.sanitize_scores();
        ev
    }

    /// Rich constructor used widely across modules.
    /// `score` is interpreted as a risk value in [0, 100].
    pub fn new_full(
        timestamp: u64,
        pid: i32,
        ppid: i32,
        uid: u32,
        binary_path: String,
        command_line: String,
        cwd: String,
        anomaly_type: String,
        component: String,
        source_module: String,
        description: Option<String>,
        signal_type: Option<String>,
        tags: Option<Vec<String>>,
        score: Option<f32>,
    ) -> Self {
        let component_clone = component.clone();
        let mut ev = Self {
            timestamp,
            pid,
            ppid,
            uid,
            binary_path,
            command_line,
            cwd,
            anomaly_type,
            component,
            metadata: HashMap::new(),
            risk_score: score.unwrap_or(0.0),
            source_module: source_module.clone(),
            decay_context: Some(format!("{}_behavior", component_clone)),
            module: Some(source_module),
            raw_score: score,
            score,
            tags: tags.clone(),
            // heuristic: first tag becomes signal if present
            signal: tags.as_ref().and_then(|t| t.get(0).cloned()),
            signal_type,
            description,
        };
        ev.sanitize_scores();
        ev
    }

    /// Constructor used when assembling from scattered parts.
    /// `risk_score` is treated as a risk in [0, 100].
    pub fn from_parts(
        timestamp: u64,
        pid: i32,
        ppid: i32,
        uid: u32,
        binary_path: String,
        anomaly_type: &str,
        component: &str,
        metadata: Option<HashMap<String, String>>,
        risk_score: f32,
        source_module: &str,
        description: Option<String>,
        tags: Option<Vec<String>>,
        signal: Option<String>,
        signal_type: Option<String>,
        decay_context: Option<String>,
        module: Option<String>,
    ) -> Self {
        let metadata_map = metadata.unwrap_or_default();
        let mut ev = Self {
            timestamp,
            pid,
            ppid,
            uid,
            binary_path,
            command_line: metadata_map
                .get("command_line")
                .cloned()
                .unwrap_or_else(|| "n/a".into()),
            cwd: metadata_map
                .get("cwd")
                .cloned()
                .unwrap_or_else(|| "n/a".into()),
            anomaly_type: anomaly_type.into(),
            component: component.into(),
            metadata: metadata_map,
            risk_score,
            source_module: source_module.into(),
            description,
            tags,
            signal,
            signal_type,
            score: Some(risk_score),
            raw_score: Some(risk_score),
            decay_context,
            module,
        };
        ev.sanitize_scores();
        ev
    }

    /// Fluent helper to add/overwrite metadata.
    #[inline]
    pub fn with_meta(mut self, k: &str, v: impl ToString) -> Self {
        self.metadata.insert(k.to_string(), v.to_string());
        self
    }

    /// Ensure scores are sane and consistent.
    fn sanitize_scores(&mut self) {
        // Clamp primary risk to [0, 100]
        self.risk_score = self.risk_score.clamp(MIN_RISK, MAX_RISK);
        // Mirror into optional fields if they weren't set
        if self.score.is_none() {
            self.score = Some(self.risk_score);
        }
        if self.raw_score.is_none() {
            self.raw_score = Some(self.risk_score);
        }
    }
}

// ------------------------- Utilities -------------------------

/// Small SHA-256 based short fingerprint.
pub fn hash_str(input: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(&result[..6]) // short stable fingerprint
}

/// Build a compact payload for downstream logs/ML.
pub fn build_trust_payload(record: &TelemetryRecord) -> HashMap<String, String> {
    let mut payload = HashMap::new();

    payload.insert("pid".to_string(), record.pid.to_string());
    payload.insert("ppid".to_string(), record.ppid.to_string());
    payload.insert("uid".to_string(), record.uid.to_string());
    payload.insert("binary_path".to_string(), record.binary_path.clone());
    payload.insert("command_line".to_string(), record.command_line.clone());
    payload.insert("start_time".to_string(), record.timestamp.to_string());

    // risk is in [0,100]; if missing, assume 0
    let risk: f32 = record.risk_score.unwrap_or(0) as f32;
    let risk = risk.clamp(MIN_RISK, MAX_RISK);
    let trust = trust_from_risk(risk);

    payload.insert("risk_score".to_string(), format!("{:.2}", risk));
    payload.insert("trust_score".to_string(), format!("{:.2}", trust));

    payload
}

/// Convert a risk in [0,100] to a trust score in [0,100].
#[inline]
pub fn trust_from_risk(risk: f32) -> f32 {
    (100.0 - risk).clamp(0.0, 100.0)
}

/// Assign a **risk** score to a TelemetryRecord (0–100).
/// Previously this function set a "trust score" into `risk_score` (inverted) — fixed.
pub fn assign_trust_score(record: &mut TelemetryRecord) {
    // Heuristic risk builder (start at 0 = safe)
    let mut risk: f32 = 0.0;

    // Simple indicators — tune as needed
    if record.binary_path.contains("/tmp") {
        risk += 40.0;
    }
    if record.command_line.contains("curl") || record.command_line.contains("wget") {
        risk += 25.0;
    }
    if record.command_line.contains("base64") || record.command_line.contains("eval") {
        risk += 20.0;
    }

    // Clamp and set
    risk = risk.clamp(MIN_RISK, MAX_RISK);
    record.risk_score = Some(risk as u32);
}

/// Emit a TrustEvent (stdout here; plug into your bus where needed).
pub fn submit_trust_event(event: TrustEvent) {
    // Ensure consistency even if constructed externally
    let mut ev = event.clone();
    ev.sanitize_scores();

    println!(
        "[🛡 TrustEvent] Module={:?} | Component={} | SignalType={:?} | Signal={:?} | Risk={:.2} | Score={:?} | Raw={:?} | Tags={:?}",
        ev.module,
        ev.component,
        ev.signal_type,
        ev.signal,
        ev.risk_score,
        ev.score,
        ev.raw_score,
        ev.tags
    );

    println!("  └── Description     : {:?}", ev.description);
    println!("  └── Anomaly Type    : {}", ev.anomaly_type);
    println!("  └── Source Module   : {}", ev.source_module);
    println!("  └── Timestamp       : {}", ev.timestamp);
    println!("  └── PID/PPID/UID    : {}/{}/{}", ev.pid, ev.ppid, ev.uid);
    println!("  └── Binary          : {}", ev.binary_path);
    println!("  └── Command Line    : {}", ev.command_line);
    println!("  └── CWD             : {}", ev.cwd);
    println!("  └── Metadata        : {:#?}", ev.metadata);
}

/// Generate a simple feature vector and payload-like map values.
/// - `cpu`: typically a 0–1 or % value; we keep as given
/// - `mem`: bytes or KB; we keep as given to avoid breaking callers
/// - `risk`: risk in [0,100]
pub fn generate_trust_payload(hostname: &str, cpu: f64, mem: u64, risk: f64) -> HashMap<String, String> {
    let risk = (risk as f32).clamp(MIN_RISK, MAX_RISK);
    let trust = trust_from_risk(risk);

    let mut map = HashMap::new();
    map.insert("host".to_string(), hostname.to_string());
    map.insert("cpu".to_string(), format!("{:.4}", cpu));
    map.insert("mem".to_string(), mem.to_string());
    map.insert("risk_score".to_string(), format!("{:.2}", risk));
    map.insert("trust_score".to_string(), format!("{:.2}", trust));
    map
}

/// Minimal, stable 3-dimensional feature vector used across modules.
/// Kept backward-compatible: [cpu, mem_as_f64, risk]
pub fn generate_feature_vector(cpu: f64, mem: u64, risk: f64) -> Vec<f64> {
    let risk = (risk as f32).clamp(MIN_RISK, MAX_RISK) as f64;
    vec![cpu, mem as f64, risk]
}

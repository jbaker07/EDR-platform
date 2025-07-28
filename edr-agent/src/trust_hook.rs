use crate::telemetry::TelemetryRecord;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Clone)]
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
    pub risk_score: f32,
    pub source_module: String,
    pub decay_context: Option<String>,
    // Optional fields for full coverage and backward compatibility
    pub module: Option<String>,
    pub signal: Option<String>,
    pub signal_type: Option<String>,
    pub score: Option<f32>,
    pub raw_score: Option<f32>,
    pub tags: Option<Vec<String>>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TrustVerdict {
    Normal,
    Monitor,
    IsolateImmediately {
        score: f64,
        reason: String,
    },
}

impl TrustEvent {
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
        Self {
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
            risk_score: 0.0,
            source_module: source_module.clone(),
            decay_context: None,
            module: Some(source_module),
            signal: None,
            signal_type: None,
            score: None,
            raw_score: None,
            tags: None,
            description: None,
        }
    }
}

impl TrustEvent {
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
        let tags_clone = tags.clone();

        Self {
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
            module: Some(source_module.clone()),
            raw_score: score,
            score,
            tags: tags_clone.clone(),
            signal: tags_clone.as_ref().and_then(|t| t.get(0).cloned()),
            signal_type,
            description,
        }
    }
}impl TrustEvent {
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

        Self {
            timestamp,
            pid,
            ppid,
            uid, // now uses correct `u32` type
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
        }
    }
}

pub fn hash_str(input: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(&result[..6]) // short stable fingerprint
}

pub fn build_trust_payload(record: &TelemetryRecord) -> HashMap<String, String> {
    let mut payload = HashMap::new();

    payload.insert("pid".to_string(), record.pid.to_string());
    payload.insert("ppid".to_string(), record.ppid.to_string());
    payload.insert("uid".to_string(), record.uid.to_string());
    payload.insert("binary_path".to_string(), record.binary_path.clone());
    payload.insert("command_line".to_string(), record.command_line.clone());
    payload.insert("start_time".to_string(), record.timestamp.to_string());

    let score: f32 = record.risk_score.unwrap_or(0) as f32;
    payload.insert("risk_score".to_string(), format!("{:.2}", score)); // consistent f32 formatting

    payload
}

pub fn assign_trust_score(record: &mut TelemetryRecord) {
    let mut trust_score: f32 = 100.0;

    if record.binary_path.contains("tmp") || record.command_line.contains("curl") {
        trust_score -= 40.0;
    }

    record.risk_score = Some(trust_score as u32);
}

pub fn submit_trust_event(event: TrustEvent) {
    println!(
        "[🛡 TrustEvent] Module={:?} | Component={} | SignalType={:?} | Signal={:?} | Score={:?} | Raw={:?} | Tags={:?}",
        event.module,
        event.component,
        event.signal_type,
        event.signal,
        event.score,
        event.raw_score,
        event.tags
    );

    println!("  └── Description     : {:?}", event.description);
    println!("  └── Anomaly Type    : {}", event.anomaly_type);
    println!("  └── Source Module   : {}", event.source_module);
    println!("  └── Timestamp       : {}", event.timestamp);
    println!("  └── PID/PPID/UID    : {}/{}/{}", event.pid, event.ppid, event.uid);
    println!("  └── Binary          : {}", event.binary_path);
    println!("  └── Command Line    : {}", event.command_line);
    println!("  └── CWD             : {}", event.cwd);
    println!("  └── Metadata        : {:#?}", event.metadata);
}

pub fn generate_trust_payload(hostname: &str, cpu: f64, mem: u64, risk: f64) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("trust_score".to_string(), format!("{:.2}", 100.0 - risk * 10.0));
    map
}

pub fn generate_feature_vector(cpu: f64, mem: u64, risk: f64) -> Vec<f64> {
    vec![cpu, mem as f64, risk]
}

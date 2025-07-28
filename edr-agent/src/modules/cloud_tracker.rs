use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::trust_hook::{generate_trust_payload, generate_feature_vector};
use crate::gnn_hook::push_to_gnn_vector_log;
use crate::relay::transmit_or_buffer;
use hostname;
use crate::telemetry::MemorySnapshot



#[derive(Debug, Serialize, Clone)]
pub struct CloudSession {
    pub timestamp: u64,
    pub user: String,
    pub provider: String,
    pub ip_address: String,
    pub role: String,
    pub event: String, // e.g., "AssumeRole", "ConsoleLogin", "APIKeyAccess"
    pub anomaly_score: f32,
    pub semantic_tags: Vec<String>,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// ✅ Simulated scan of cloud access telemetry
pub fn get_cloud_sessions() -> Vec<CloudSession> {
    vec![
        CloudSession {
            timestamp: now(),
            user: "alice@corp.com".into(),
            provider: "AWS".into(),
            ip_address: "3.92.123.88".into(),
            role: "DevOps".into(),
            event: "AssumeRole".into(),
            anomaly_score: 0.1,
            semantic_tags: vec!["cloud_role_change".into()],
        },
        CloudSession {
            timestamp: now(),
            user: "bob@corp.com".into(),
            provider: "Azure".into(),
            ip_address: "41.121.199.11".into(),
            role: "GlobalAdmin".into(),
            event: "ConsoleLogin".into(),
            anomaly_score: 0.7,
            semantic_tags: vec!["cloud_console_login".into(), "privilege_risk".into()],
        },
        CloudSession {
            timestamp: now(),
            user: "bot-3@corp.com".into(),
            provider: "GCP".into(),
            ip_address: "10.0.0.5".into(),
            role: "ServiceAccount".into(),
            event: "APIKeyAccess".into(),
            anomaly_score: 0.4,
            semantic_tags: vec!["cloud_api_key_usage".into()],
        },
    ]
}

/// 🔁 Sends enriched sessions to Trust Engine, GNN, and Relay
pub fn handle_cloud_sessions() {
    let sessions = get_cloud_sessions();
    let hostname = hostname::get().unwrap_or_default().to_string_lossy().to_string();

    for session in sessions {
        let cpu = 0.1;
        let mem = 250.0;
        let score = session.anomaly_score;

        let snapshot = MemorySnapshot {
        id: session.user.clone(),
        path: session.provider.clone(),
        hash: session.ip_address.clone(),
        risk_score: (score * 100.0).round() as u32,
        reason: session.event.clone(),
        timestamp: std::time::SystemTime::now(),
        behavior_deviation: Some(score > 0.5),
        cpu_usage: Some(cpu),
        memory_usage: Some(mem),
        pid: 0,
};

let trust_payload = generate_trust_payload(&snapshot);
let features = generate_feature_vector(cpu, mem, score as f64);
push_to_gnn_vector_log(features);

        println!(
            "[☁️ Cloud Trust] User: {} | Risk: {:.2} | Tags: {:?}",
            session.user, score, session.semantic_tags
        );
    }
}

pub fn cloud_telemetry() -> Vec<String> {
    // your implementation
    vec!["cloud telemetry event".into()]
}

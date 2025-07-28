
use crate::adaptive_threshold_engine::{AdaptiveThresholdEngine, evaluate_role_thresholds, load_role_profile_map};
use crate::gnn_hook::export_trust_vector_to_gnn;
use crate::services::replay_trigger::trigger_replay_if_needed;
use crate::trust_digest_engine::TrustDigestEngine;
use crate::services::trust_state_writer::persist_trust_state;
use crate::score_reason::ScoreReason;
use crate::services::trust_config::TrustConfig;
use crate::services::trust_vector::{TrustVector, TRUST_VECTOR_GLOBAL, BASELINES, DEFAULT_BASELINE, mahalanobis_distance};

use crate::gnn_hook::stream_to_gnn_fifo;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::BufWriter;
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use libc;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use chrono::Utc;
use crate::services::trust_anchor_logger::{AnchorDropEvent, log_anchor_drop};
use crate::services::graph_snapshot_writer::store_graph_snapshot;
use crate::telemetry::TelemetryRecord;
use crate::logger::log;

const MAHALANOBIS_THRESHOLD: f64 = 4.0; // You can tune this threshold later


/// Returns the Mahalanobis distance threshold per trust dimension.
/// Later this can be adaptive based on endpoint role or history.
fn get_critical_threshold(dim: &str) -> f64 {
    match dim {
        "memory" => 4.5,
        "network" => 4.0,
        "file" => 3.8,
        "auth" => 4.2,
        "geo" => 4.0,
        _ => 5.0, // default catch-all
    }
}

// --- Data Structures ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
    pub endpoint_id: String,
    pub endpoint_role: String,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub risk_score: f64,
    pub tags: Vec<String>,
}
impl TelemetryData {
    pub fn from_record(record: &TelemetryRecord) -> Self {
        let risk_score = record.risk_score.unwrap_or(0) as f64;

        Self {
            endpoint_id: format!("endpoint_{}", record.pid),
            endpoint_role: "default".to_string(),
            cpu_usage: 0.0,
            memory_usage: 0.0,
            risk_score,
            tags: record.tags.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustResult {
    pub endpoint_id: String,
    pub score: f64,
    pub escalated: bool,
    pub reasons: Vec<ScoreReason>,
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

// --- Global Engines ---

lazy_static! {
    static ref DIGEST_ENGINE: Mutex<TrustDigestEngine> = Mutex::new(TrustDigestEngine::new());
    static ref THRESHOLD_ENGINE: Mutex<AdaptiveThresholdEngine> = Mutex::new(AdaptiveThresholdEngine::new(70.0, 0.15));
    static ref ESCALATION_THRESHOLD: f64 = 120.0;
}

// --- Core Scoring Logic ---

fn calculate_base_score(telemetry: &TelemetryData) -> f64 {
    (telemetry.cpu_usage + telemetry.memory_usage + telemetry.risk_score) / 3.0
}

// --- Main Evaluation Flow ---
fn apply_mahalanobis_gate(trust_vector: &TrustVector, role: &str) -> bool {
    let baseline = BASELINES.get(role).unwrap_or(&DEFAULT_BASELINE);
    let distance = mahalanobis_distance(trust_vector, baseline);
    distance >= MAHALANOBIS_THRESHOLD
}


pub fn evaluate_and_dispatch_trust_score(telemetry: &TelemetryData) -> TrustResult {
    let mut reasons: Vec<ScoreReason> = Vec::new();
    let mut global_vector_map = TRUST_VECTOR_GLOBAL.lock().unwrap();
    let vector = global_vector_map.entry(telemetry.endpoint_id.clone()).or_insert_with(TrustVector::new);
    if let Some(role) = Some(&telemetry.endpoint_role) {
    let baseline = BASELINES.get(role).unwrap_or(&DEFAULT_BASELINE);
    let distance = mahalanobis_distance(vector, baseline);

    if distance.is_nan() {
        log("⚠️ Mahalanobis scoring skipped: distance is NaN");
    } else if distance >= MAHALANOBIS_THRESHOLD {
        log(&format!(
            "🚨 Mahalanobis anomaly: distance = {:.2} (threshold = {:.2}) for role: {}",
            distance, MAHALANOBIS_THRESHOLD, role
        ));

        // Optional: Add trust penalty or trigger escalation here
    }
}

    if telemetry.cpu_usage > 90.0 {
        reasons.push(ScoreReason::HighCpuUsage);
    }

    // Apply decay and compute scores
    vector.apply_decay(0.25, 1.2);
    let weighted_score = vector.compute_weighted_score();
    reasons.push(ScoreReason::Custom(format!("Weighted trust vector score: {:.2}", weighted_score)));

    // Mahalanobis gating check — ensures stats exist before checking
    if !vector.has_valid_stats() {
        reasons.push(ScoreReason::Custom("Skipping Mahalanobis scoring due to missing stats".into()));
        log("⚠️ Mahalanobis scoring skipped: no baseline vector or stats exist.");
    }

    let mut dimension_drops = vec![];
    let mut dimension_escalated = false;
    let mut digest = DIGEST_ENGINE.lock().unwrap();

    let cumulative_score = digest.get_score(&telemetry.endpoint_id);
    for (dim, score) in &vector.dimensions {
        if *score < 70.0 {
            let history = vector.get_history(dim);
            dimension_drops.push(format!("{} dropped to {:.1} due to {:?}", dim, score, history));

            log_anchor_drop(AnchorDropEvent {
                timestamp: Utc::now().to_rfc3339(),
                endpoint_id: telemetry.endpoint_id.clone(),
                endpoint_role: telemetry.endpoint_role.clone(),
                dimension: dim.clone(),
                score: *score,
                reason: format!("{:?}", history),
            });
        }

        if vector.has_valid_stats() {
            if let Some(m_score) = vector.get_mahalanobis_for(dim) {
                let threshold = get_critical_threshold(dim);

                if m_score >= threshold {
                    dimension_escalated = true;
                    reasons.push(ScoreReason::Custom(format!(
                        "{} dimension is critically anomalous (Mahalanobis = {:.2} exceeds {:.2})",
                        dim, m_score, threshold,
                    )));
                    trigger_replay_if_needed(&telemetry.endpoint_id, weighted_score, &[format!("dimension::{}", dim)], &vec![], &telemetry.endpoint_role);
                    store_graph_snapshot(&telemetry.endpoint_id, &reasons.iter().map(|r| format!("{:?}", r)).collect::<Vec<_>>(), weighted_score);
                } else if m_score >= 3.0 {
                    reasons.push(ScoreReason::Custom(format!(
                        "{} dimension is an outlier (Mahalanobis = {:.2})", dim, m_score
                    )));
                }
            }
        }
    }
     // 🚨 Check if any critical semantic tags are present
    for tag in &telemetry.tags {
        if is_critical_tag(tag) {
            reasons.push(ScoreReason::CriticalTag(tag.to_string()));
            trigger_replay_if_needed(
                &telemetry.endpoint_id,
                cumulative_score,
                &[format!("tag::{}", tag)],
                &vec![],
                &telemetry.endpoint_role,
            );

            store_graph_snapshot(
                &telemetry.endpoint_id,
                &reasons.iter().map(|r| format!("{:?}", r)).collect::<Vec<_>>(),
                cumulative_score,
            );


            return TrustResult {
                endpoint_id: telemetry.endpoint_id.clone(),
                score: cumulative_score,
                escalated: true,
                reasons,
            };


        }
    }

    if !dimension_drops.is_empty() {
        reasons.push(ScoreReason::Custom(format!(
            "Drop details: [{}]", dimension_drops.join("; ")
        )));
    }

    for tag in &telemetry.tags {
        vector.apply_tagged_penalty(tag);
        reasons.push(ScoreReason::Tagged(tag.clone()));
    }

    let mut anomalies = vec![];
    for (dim, score) in &vector.dimensions {
        if let Some(stats) = vector.get_stats(dim) {
            let distance = stats.mahalanobis(*score);
            if distance > 2.0 {
                anomalies.push(format!("Anomaly in {} (M-Distance: {:.2})", dim, distance));
            }
            if distance > 3.5 {
                reasons.push(ScoreReason::Custom(format!(
                    "Dimension '{}' triggered hard escalation (M-Distance: {:.2})", dim, distance
                )));
                dimension_escalated = true;
            }
        }
    }

    if !anomalies.is_empty() {
        reasons.push(ScoreReason::Custom(format!(
            "Mahalanobis anomalies detected: [{}]", anomalies.join("; ")
        )));
    }
    
    let average_score = vector.compute_average();
    stream_to_gnn_fifo(&vector.to_map());


    digest.ingest_score(&telemetry.endpoint_id, average_score);

    if cumulative_score >= *ESCALATION_THRESHOLD {
        reasons.push(ScoreReason::TrustDecayExceeded);
    }

    let mut threshold = THRESHOLD_ENGINE.lock().unwrap();
    threshold.update(&telemetry.endpoint_id, average_score);
    let adaptive_decision = threshold.should_escalate(&telemetry.endpoint_id, average_score);
    if adaptive_decision {
        reasons.push(ScoreReason::Custom("Adaptive threshold exceeded".into()));
    }

    let digest_decision = cumulative_score >= *ESCALATION_THRESHOLD || dimension_escalated;


    let mut final_score = cumulative_score;
    let mut escalated = false;

    if adaptive_decision && digest_decision {
        escalated = true;
        final_score += 10.0;
    }

    for (dim, score) in &vector.dimensions {
        if *score < 30.0 {
            reasons.push(ScoreReason::Custom(format!(
                "Critical drop in {}: {:.1} — triggering escalation", dim, score
            )));
            escalated = true;
            final_score += 10.0;
        }
    }

    let role_map = load_role_profile_map("json_files/role_risk_profile_map.json");
    let adjusted_score = evaluate_role_thresholds(&telemetry.endpoint_role, final_score, &role_map);

    persist_trust_state(&telemetry.endpoint_id, adjusted_score);
    export_trust_vector_to_gnn(
        &telemetry.endpoint_id,
        &telemetry.endpoint_role,
        adjusted_score,
        vector.to_map(),
        telemetry.tags.clone(),
    );

    if escalated {
        trigger_replay_if_needed(
            &telemetry.endpoint_id,
            adjusted_score,
            &telemetry.tags,
            &vec![],
            &telemetry.endpoint_role,
        );
    }

    TrustResult {
        endpoint_id: telemetry.endpoint_id.clone(),
        score: adjusted_score.clamp(0.0, 100.0),
        escalated,
        reasons,
    }
}

fn is_critical_tag(tag: &str) -> bool {
    matches!(tag,
        "kernel::exploit"
        | "memory::injection"
        | "auth::spray"
        | "credential::dump"
        | "process::hollow"
        | "persistence::stealth"
        | "signal::tamper"
        | "geo::suspicious"
    )
}

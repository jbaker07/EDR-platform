
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
use std::fs::File;
use std::io::Read;
use std::sync::OnceLock;
use crate::modules::telemetry_fingerprint::{load_fingerprint_db, is_known_good};

static BASELINE_STATS: OnceLock<(Vec<f32>, Vec<Vec<f32>>)> = OnceLock::new();

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
    // 🔐 Load fingerprint DB once (lazy-static or cached globally in future)
    let fingerprint_db = load_fingerprint_db("src/modules/telemetry_fingerprint.json");
    vector.apply_decay_and_analyze(0.25, 1.2);
    let weighted_score = vector.compute_weighted_score();
    let anomalies = vector.collect_mahalanobis_anomalies();

    for a in anomalies {
        reasons.push(ScoreReason::Custom(a));
    }

    if is_known_good(telemetry, &fingerprint_db) {
        log("trust_suppress", &format!("🔕 Suppressed known-good telemetry: {:?}", telemetry));
        return TrustResult {
            endpoint_id: telemetry.endpoint_id.clone(),
            score: 100.0,
            escalated: false,
            reasons: vec![ScoreReason::Custom("Suppressed due to fingerprint match".into())],
        };
    

    // ... rest of function continues ...
    vector.apply_decay_and_analyze(0.25, 1.2);
    reasons.extend(vector.emit_score_reasons());

    for tag in &telemetry.tags {
        vector.apply_tagged_penalty(tag);
    }
    reasons.extend(vector.emit_tag_penalties(&telemetry.tags));

    let weighted_score = vector.compute_weighted_score();
    reasons.push(ScoreReason::Custom(format!("Weighted trust vector score: {:.2}", weighted_score)));

    // Mahalanobis or adaptive anomalies are already added above
// Adaptive escalation
    if adaptive_decision {
        reasons.push(ScoreReason::Custom("Adaptive threshold exceeded".into()));
    }

    pub fn load_mahalanobis_baseline() {
    if BASELINE_STATS.get().is_some() {
        return;
    }

    let mut file = match File::open("json_files/mahalanobis_baseline.json") {
        Ok(f) => f,
        Err(_) => return,
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return;
    }

    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap_or_default();
    let mean = parsed["mean"].as_array().unwrap_or(&vec![])
        .iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect::<Vec<_>>();

    let cov = parsed["cov"].as_array().unwrap_or(&vec![])
        .iter().map(|row| {
            row.as_array().unwrap_or(&vec![])
                .iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    BASELINE_STATS.set((mean, cov)).ok();
}

pub fn calculate_mahalanobis_distance(vec: &Vec<f32>) -> Option<f32> {
    let (mean, cov) = BASELINE_STATS.get()?;

    if vec.len() != mean.len() || cov.len() != vec.len() {
        return None;
    }

    let mut diff = vec.iter().zip(mean.iter()).map(|(a, b)| a - b).collect::<Vec<_>>();

    // Naive inverse of diagonal covariance (no full inversion)
    let mut distance = 0.0;
    for i in 0..vec.len() {
        let var = cov[i][i];
        if var > 0.0 {
            distance += (diff[i] * diff[i]) / var;
        }
    }

    Some(distance.sqrt())
}

use crate::config::get_config;
let config = get_config();

if config.bootstrap_mode.unwrap_or(false) {
    log("🚧 Bootstrap mode enabled — skipping Mahalanobis and adaptive scoring");
    
    return TrustResult {
        endpoint_id: telemetry.endpoint_id.clone(),
        score: 100.0,
        escalated: false,
        reasons: vec![ScoreReason::Custom("Bootstrap mode: no scoring".into())],
    };
}



pub fn to_serialized_json(&self) -> String {
    match serde_json::to_string_pretty(&self) {
        Ok(s) => s,
        Err(_) => "{}".to_string(),
    }
}

pub fn evaluate_and_dispatch_trust_score(telemetry: &TelemetryData) -> TrustResult {
    let mut reasons: Vec<ScoreReason> = Vec::new();
    let mut global_vector_map = TRUST_VECTOR_GLOBAL.lock().unwrap();
    let vector = global_vector_map.entry(telemetry.endpoint_id.clone()).or_insert_with(TrustVector::new);
    let mut dimension_escalated = false;
    let mut dimension_drops = vec![];

    // Apply decay early
    vector.apply_decay(0.25, 1.2);

    // Mahalanobis: central distance from full vector (baseline-wide)
    if let Some(role) = Some(&telemetry.endpoint_role) {
        let baseline = match load_baseline(role) {
            Some(v) => v,
            None => DEFAULT_BASELINE.clone(),
        };
        
        let distance = mahalanobis_distance(vector, baseline);

        if distance.is_nan() {
            log("⚠️ Mahalanobis scoring skipped: distance is NaN");
            reasons.push(ScoreReason::Custom("Skipping Mahalanobis scoring due to missing stats".into()));
        } else {
            reasons.push(ScoreReason::Custom(format!("Global Mahalanobis distance: {:.2}", distance)));
            if distance >= MAHALANOBIS_THRESHOLD {
                reasons.push(ScoreReason::Custom(format!(
                    "🚨 Global Mahalanobis anomaly (dist={:.2} ≥ {:.2})", distance, MAHALANOBIS_THRESHOLD
                )));
            }
        }
    }

    // Example CPU usage hook
    if telemetry.cpu_usage > 90.0 {
        reasons.push(ScoreReason::HighCpuUsage);
    }

    // Score computation
    let weighted_score = vector.compute_weighted_score();
    reasons.push(ScoreReason::Custom(format!("Weighted trust vector score: {:.2}", weighted_score)));

    // Per-dimension Mahalanobis checks
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

        if let Some(stats) = vector.get_stats(dim) {
            let dist = stats.mahalanobis(*score);
            if dist > 2.0 {
                reasons.push(ScoreReason::Custom(format!(
                    "{} dimension is an outlier (M-Distance: {:.2})", dim, dist
                )));
            }
            if dist > 3.5 {
                reasons.push(ScoreReason::Custom(format!(
                    "{} dimension triggered escalation (M-Distance: {:.2})", dim, dist
                )));
                dimension_escalated = true;
                trigger_replay_if_needed(
                    &telemetry.endpoint_id,
                    weighted_score,
                    &[format!("dimension::{}", dim)],
                    &vec![],
                    &telemetry.endpoint_role,
                );
                store_graph_snapshot(
                    &telemetry.endpoint_id,
                    &reasons.iter().map(|r| format!("{:?}", r)).collect::<Vec<_>>(),
                    weighted_score,
                );
            }
        }
    }

    // Log drop summary if any
    if !dimension_drops.is_empty() {
        reasons.push(ScoreReason::Custom(format!(
            "Drop details: [{}]", dimension_drops.join("; ")
        )));
    }

    // Tag logic
    for tag in &telemetry.tags {
        if is_critical_tag(tag) {
            reasons.push(ScoreReason::CriticalTag(tag.to_string()));
            trigger_replay_if_needed(
                &telemetry.endpoint_id,
                weighted_score,
                &[format!("tag::{}", tag)],
                &vec![],
                &telemetry.endpoint_role,
            );
            store_graph_snapshot(
                &telemetry.endpoint_id,
                &reasons.iter().map(|r| format!("{:?}", r)).collect::<Vec<_>>(),
                weighted_score,
            );
            return TrustResult {
                endpoint_id: telemetry.endpoint_id.clone(),
                score: weighted_score,
                escalated: true,
                reasons,
            };
        }

        vector.apply_tagged_penalty(tag);
        reasons.push(ScoreReason::Tagged(tag.clone()));
    }

    // Stream and finalize
    stream_to_gnn_fifo(&vector.to_map());

    let average_score = vector.compute_average();
    let mut digest = DIGEST_ENGINE.lock().unwrap();
    digest.ingest_score(&telemetry.endpoint_id, average_score);

    // Adaptive threshold logic
    let mut threshold = THRESHOLD_ENGINE.lock().unwrap();
    threshold.update(&telemetry.endpoint_id, average_score);
    let adaptive_decision = threshold.should_escalate(&telemetry.endpoint_id, average_score);

    let cumulative_score = digest.get_score(&telemetry.endpoint_id);
    let mut final_score = cumulative_score;
    let mut escalated = false;

    if cumulative_score >= *ESCALATION_THRESHOLD {
        reasons.push(ScoreReason::TrustDecayExceeded);
    }

    if adaptive_decision {
        reasons.push(ScoreReason::Custom("Adaptive threshold exceeded".into()));
    }

    if adaptive_decision && (cumulative_score >= *ESCALATION_THRESHOLD || dimension_escalated) {
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

    // Role-based thresholding
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

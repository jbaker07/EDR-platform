use crate::adaptive_threshold_engine::{AdaptiveThresholdEngine, evaluate_role_thresholds, load_role_profile_map};
use crate::gnn_hook::{export_trust_vector_to_gnn, stream_to_gnn_fifo};
use crate::services::replay_trigger::trigger_replay_if_needed;
use crate::services::trust_state_writer::persist_trust_state;
use crate::score_reason::ScoreReason;
use crate::services::trust_config::TrustConfig;

use crate::logger::log;
use crate::telemetry::TelemetryRecord;
use crate::services::graph_snapshot_writer::store_graph_snapshot;
use crate::services::trust_anchor_logger::{AnchorDropEvent, log_anchor_drop};
use crate::modules::telemetry_fingerprint::{load_fingerprint_db, is_known_good};

use crate::services::trust_vector::{TrustVector, TrustDim, TRUST_DIM_CT};
use crate::services::trust_vector::TRUST_VECTOR_GLOBAL; // global per-endpoint store

use chrono::Utc;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

// ─────────────────────────────────────────────────────────────────────────────
// Env helpers
// ─────────────────────────────────────────────────────────────────────────────
#[inline]
fn env_f32(name: &str, default: f32) -> f32 {
    std::env::var(name).ok().and_then(|s| s.parse::<f32>().ok()).unwrap_or(default)
}
#[inline]
fn env_f64(name: &str, default: f64) -> f64 {
    std::env::var(name).ok().and_then(|s| s.parse::<f64>().ok()).unwrap_or(default)
}

// Outlier weights are in TRUST units (0..1). Tune via env if needed.
#[derive(Debug, Clone, Copy)]
struct OutlierWeights { mahal: f32, elliptic: f32, krim: f32 }
impl Default for OutlierWeights {
    fn default() -> Self {
        Self {
            mahal:   env_f32("EDR_W_MAHAL",   0.15),
            elliptic:env_f32("EDR_W_ELLIP",   0.20),
            krim:    env_f32("EDR_W_KRIM",    0.25),
        }
    }
}

// Risk thresholds (risk = 100 - trust)
lazy_static! {
    static ref DIGEST_ENGINE: Mutex<crate::trust_digest_engine::TrustDigestEngine> =
        Mutex::new(crate::trust_digest_engine::TrustDigestEngine::new());
    static ref THRESHOLD_ENGINE: Mutex<AdaptiveThresholdEngine> =
        Mutex::new(AdaptiveThresholdEngine::new(env_f64("EDR_ADAPT_BASE", 60.0), env_f64("EDR_ADAPT_SLOPE", 0.15)));
    // When rolling risk (from digest) exceeds this, we consider escalation
    static ref ESCALATION_RISK_THRESHOLD: f64 = env_f64("EDR_RISK_THRESHOLD", 60.0);
    // Decay half-life for trust deficits (seconds)
    static ref TRUST_HALFLIFE_S: f32 = env_f32("EDR_TRUST_HALFLIFE_S", 3600.0);
}

// ─────────────────────────────────────────────────────────────────────────────
// Public types (kept compatible)
// ─────────────────────────────────────────────────────────────────────────────
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
    /// Returned as TRUST (0..100, higher is better).
    pub score: f64,
    pub escalated: bool,
    pub reasons: Vec<ScoreReason>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn apply_tagged_outlier_penalties_dimmed(
    tv: &mut TrustVector,
    data: &TelemetryData,
    reasons: &mut Vec<ScoreReason>,
    w: OutlierWeights,
) {
    let tags: HashSet<&str> = data.tags.iter().map(|s| s.as_str()).collect();

    // Mahalanobis gate → Process/Memory/Network
    if tags.contains("mahalanobis_outlier") {
        tv.penalty_by_name("process",  w.mahal * 0.50);
        tv.penalty_by_name("memory",   w.mahal * 0.30);
        tv.penalty_by_name("network",  w.mahal * 0.20);
        reasons.push(ScoreReason::outlier("mahalanobis", "χ² gate exceeded".into()));
    }

    // Robust envelope → Memory/Privilege/Process
    if tags.contains("elliptic_outlier") {
        tv.penalty_by_name("memory",     w.elliptic * 0.40);
        tv.penalty_by_name("privilege",  w.elliptic * 0.30);
        tv.penalty_by_name("process",    w.elliptic * 0.30);
        reasons.push(ScoreReason::outlier("elliptic_envelope", "robust envelope margin < 0".into()));
    }

    // KRIM causal subgraph → Network/Privilege/Persistence
    if tags.contains("krim_alert") {
        tv.penalty_by_name("network",     w.krim * 0.40);
        tv.penalty_by_name("privilege",   w.krim * 0.30);
        tv.penalty_by_name("persistence", w.krim * 0.30);
        reasons.push(ScoreReason::CausalSubgraph);
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

// ─────────────────────────────────────────────────────────────────────────────
// Main evaluation flow
// ─────────────────────────────────────────────────────────────────────────────

pub fn evaluate_and_dispatch_trust_score(telemetry: &TelemetryData) -> TrustResult {
    // Optional bootstrap bypass
    if let Ok(cfg) = std::env::var("EDR_BOOTSTRAP") {
        if cfg == "1" || cfg.eq_ignore_ascii_case("true") {
            log("trust 🚧 Bootstrap mode — skipping scoring");
            return TrustResult {
                endpoint_id: telemetry.endpoint_id.clone(),
                score: 100.0,
                escalated: false,
                reasons: vec![ScoreReason::Custom("Bootstrap mode: no scoring".into())],
            };
        }
    }

    let mut reasons: Vec<ScoreReason> = Vec::new();

    // Load fingerprint DB and suppress known-good if matched
    let fingerprint_db = load_fingerprint_db("src/modules/telemetry_fingerprint.json");

    // Build a simple probe map from TelemetryData so types match is_known_good(&HashMap<..>, ..)
    let mut probe: HashMap<String, String> = HashMap::new();
    probe.insert("endpoint_id".into(), telemetry.endpoint_id.clone());
    probe.insert("endpoint_role".into(), telemetry.endpoint_role.clone());
    probe.insert("risk_score".into(), format!("{:.2}", telemetry.risk_score));
    probe.insert("cpu_usage".into(), format!("{:.2}", telemetry.cpu_usage));
    probe.insert("memory_usage".into(), format!("{:.2}", telemetry.memory_usage));
    if !telemetry.tags.is_empty() {
        probe.insert("tags".into(), telemetry.tags.join(","));
    }
    if is_known_good(&probe, &fingerprint_db) {
        log(&format!(
            "trust_suppress 🔕 Suppressed known-good telemetry: {:?}",
            telemetry
        ));
        return TrustResult {
            endpoint_id: telemetry.endpoint_id.clone(),
            score: 100.0,
            escalated: false,
            reasons: vec![ScoreReason::Custom("Suppressed due to fingerprint match".into())],
        };
    }

    // Get or create per-endpoint TrustVector
    let mut map = TRUST_VECTOR_GLOBAL.lock().unwrap();
    let tv = map.entry(telemetry.endpoint_id.clone()).or_insert_with(TrustVector::new);

    // 1) Time-based decay toward baseline (half-life)
    // Main loop period is ~30s; if different, pass actual dt here.
    tv.apply_decay(30.0, *TRUST_HALFLIFE_S);

    // 2) Apply tag-level nudges immediately (dimension-targeted)
    for tag in &telemetry.tags {
        tv.apply_tag(tag);
        reasons.push(ScoreReason::Tagged(tag.clone()));
    }

    // 3) Apply outlier gates (Mahalanobis, EllipticEnvelope, KRIM) with splits
    apply_tagged_outlier_penalties_dimmed(tv, telemetry, &mut reasons, OutlierWeights::default());

    // 4) CPU heuristic (kept)
    if telemetry.cpu_usage > 90.0 {
        reasons.push(ScoreReason::HighCpuUsage);
        tv.penalty_by_name("process", 0.03); // small process penalty for pegged CPU
    }

    // 5) Per-dimension checks for sharp drops (trust < 0.70)
    let mut dimension_drops = Vec::new();
    for d in TrustDim::all() {
        let trust = tv.v[d.idx()];
        if trust < 0.70 {
            let dim_name = d.to_string();
            dimension_drops.push(format!("{dim_name} dropped to {:.2}", trust));
            log_anchor_drop(AnchorDropEvent {
                timestamp: Utc::now().to_rfc3339(),
                endpoint_id: telemetry.endpoint_id.clone(),
                endpoint_role: telemetry.endpoint_role.clone(),
                dimension: dim_name.clone(),
                score: (trust * 100.0) as f64,
                reason: "below 0.70".into(),
            });
        }
    }
    if !dimension_drops.is_empty() {
        reasons.push(ScoreReason::Custom(format!("Drop details: [{}]", dimension_drops.join("; "))));
    }

    // 6) Critical tags: immediate escalation path
    for tag in &telemetry.tags {
        if is_critical_tag(tag) {
            reasons.push(ScoreReason::CriticalTag(tag.clone()));
            let trust_now = (tv.score_overall() * 100.0) as f64;
            trigger_replay_if_needed(
                &telemetry.endpoint_id,
                trust_now,
                &[format!("tag::{}", tag)],
                &vec![],
                &telemetry.endpoint_role,
            );
            store_graph_snapshot(
                &telemetry.endpoint_id,
                &reasons.iter().map(|r| format!("{:?}", r)).collect::<Vec<_>>(),
                trust_now,
            );
            // Continue to sinks/export below; mark escalated
        }
    }

    // 7) Stream current vector to GNN FIFO (map of trust.* -> value)
    stream_to_gnn_fifo(&tv.to_map());

    // 8) Compute TRUST (0..100) and RISK (0..100), feed digest/adaptive threshold
    let trust_score = (tv.score_overall() * 100.0) as f64;    // higher is better
    let risk_score  = 100.0 - trust_score;                    // higher is worse

    let mut digest = DIGEST_ENGINE.lock().unwrap();
    digest.ingest_score(&telemetry.endpoint_id, risk_score);

    let mut threshold = THRESHOLD_ENGINE.lock().unwrap();
    threshold.update(&telemetry.endpoint_id, risk_score);
    let adaptive_escalate = threshold.should_escalate(&telemetry.endpoint_id, risk_score);

    let rolling_risk = digest.get_score(&telemetry.endpoint_id);

    // 9) Role-aware thresholding (keep your existing flow)
    let role_map = load_role_profile_map("json_files/role_risk_profile_map.json");
    let adjusted_trust = evaluate_role_thresholds(&telemetry.endpoint_role, trust_score, &role_map);

    // 10) Persist state & export to GNN
    persist_trust_state(&telemetry.endpoint_id, adjusted_trust);
    export_trust_vector_to_gnn(
        &telemetry.endpoint_id,
        &telemetry.endpoint_role,
        adjusted_trust,
        tv.to_map(),
        telemetry.tags.clone(),
    );

    // 11) Decide escalation (adaptive + rolling risk, plus any critical per-dim)
    let mut escalated = false;
    if rolling_risk >= *ESCALATION_RISK_THRESHOLD || adaptive_escalate {
        escalated = true;
        reasons.push(ScoreReason::Custom(format!(
            "Escalation: risk {:.1} (thr {:.1}) adaptive={}",
            rolling_risk, *ESCALATION_RISK_THRESHOLD, adaptive_escalate
        )));
        trigger_replay_if_needed(
            &telemetry.endpoint_id,
            adjusted_trust,
            &telemetry.tags,
            &vec![],
            &telemetry.endpoint_role,
        );
    }

    TrustResult {
        endpoint_id: telemetry.endpoint_id.clone(),
        score: adjusted_trust.clamp(0.0, 100.0),
        escalated,
        reasons,
    }
}

// services/trust_vote_engine.rs

use std::collections::HashMap;
use crate::session_trust_curve::{SessionTrustCurve};
use crate::trust_engine_final::calculate_final_score;

pub fn vote_with_decay_adjustment(
    gnn_score: f64,
    rule_score: f64,
    digest_score: f64,
    session_id: &str
) -> f64 {
    let base_score = calculate_final_score(gnn_score, rule_score, digest_score);

    // Optionally pull from session trust curve
    let decay_penalty = match crate::replay_trigger::SESSION_CURVES.lock().unwrap().get(session_id) {
        Some(curve) if curve.is_escalating(0.25) => 0.15,
        _ => 0.0,
    };

    (base_score - decay_penalty).clamp(0.0, 1.0)
}

#[derive(Debug, Clone)]
pub struct TrustVoteResult {
    pub final_score: f64,
    pub missing_inputs: Vec<String>,
    pub explanation: String,
}

/// Computes a weighted trust score based on available sub-engines.
/// If all inputs are missing, uses a fallback score.
pub fn vote_trust_score(
    gnn_score: Option<f64>,
    rule_score: Option<f64>,
    digest_score: Option<f64>,
    weights: Option<HashMap<&str, f64>>,
    fallback: f64,
) -> TrustVoteResult {
    let mut total_weight = 0.0;
    let mut weighted_sum = 0.0;
    let mut missing = Vec::new();
    let mut used = Vec::new();

    // Default weights if none provided
    let w = weights.unwrap_or_else(|| {
        HashMap::from([
            ("gnn", 0.4),
            ("rule", 0.4),
            ("digest", 0.2),
        ])
    });

    let sources = vec![
        ("gnn", gnn_score),
        ("rule", rule_score),
        ("digest", digest_score),
    ];

    for (label, score_opt) in sources {
        if let Some(score) = score_opt {
            let weight = *w.get(label).unwrap_or(&0.0);
            weighted_sum += score * weight;
            total_weight += weight;
            used.push(label.to_string());
        } else {
            missing.push(label.to_string());
        }
    }

    let (final_score, explanation) = if total_weight == 0.0 {
        (
            fallback,
            format!(
                "Fallback score used ({}) — missing inputs: {}",
                fallback,
                missing.join(", ")
            ),
        )
    } else {
        let score = (weighted_sum / total_weight).clamp(0.0, 100.0);
        (
            (score * 100.0).round() / 100.0,
            format!(
                "Score computed using weighted inputs from: {}",
                used.join(", ")
            ),
        )
    };

    TrustVoteResult {
        final_score,
        missing_inputs: missing,
        explanation,
    }
}

// src/gnn_models.rs
use serde::{Deserialize, Serialize};

/// Input format for GNN scoring pipelines.
/// This mirrors what `gnn_hook.rs` is trying to construct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnnScoringInput {
    /// Node identifiers (e.g., "pid:1234")
    pub nodes: Vec<String>,
    /// Directed edges (source, target)
    pub edges: Vec<(String, String)>,
    /// Flat feature vector (embedding, telemetry-derived features, etc.)
    pub features: Vec<f32>,

    /// Extra metadata fields referenced by gnn_hook.rs
    pub session_id: String,
    pub timestamp: i64,
    pub trust_vector: Option<crate::services::trust_vector::TrustVector>,
    pub semantic_tags: Vec<String>,
    pub anchor_hits: Vec<String>,
    pub suppressed_by_fingerprint: Vec<String>,
    pub decay_score: f32,
    pub graph_snapshot_path: Option<String>,
    pub notes: Option<String>,
}

impl Default for GnnScoringInput {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            features: Vec::new(),
            session_id: String::new(),
            timestamp: 0,
            trust_vector: None,
            semantic_tags: Vec::new(),
            anchor_hits: Vec::new(),
            suppressed_by_fingerprint: Vec::new(),
            decay_score: 0.0,
            graph_snapshot_path: None,
            notes: None,
        }
    }
}

impl GnnScoringInput {
    /// Construct an empty scoring input.
    pub fn new() -> Self {
        Self::default()
    }

    /// Convenience builder for common case.
    pub fn from_graph(
        nodes: Vec<String>,
        edges: Vec<(String, String)>,
        features: Vec<f32>,
    ) -> Self {
        Self {
            nodes,
            edges,
            features,
            ..Default::default()
        }
    }

    /// Convenience builder with metadata.
    pub fn with_meta(
        nodes: Vec<String>,
        edges: Vec<(String, String)>,
        features: Vec<f32>,
        session_id: String,
        timestamp: i64,
        trust_vector: Option<crate::services::trust_vector::TrustVector>,
        semantic_tags: Vec<String>,
        anchor_hits: Vec<String>,
        suppressed_by_fingerprint: Vec<String>,
        decay_score: f32,
        graph_snapshot_path: Option<String>,
        notes: Option<String>,
    ) -> Self {
        Self {
            nodes,
            edges,
            features,
            session_id,
            timestamp,
            trust_vector,
            semantic_tags,
            anchor_hits,
            suppressed_by_fingerprint,
            decay_score,
            graph_snapshot_path,
            notes,
        }
    }
}

// src/graph_builder.rs
use serde::{Deserialize, Serialize};

/// Lightweight node representation used for causal / process graphs.
/// Keep this in sync with what `episode.rs` expects.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GraphNode {
    /// Stable string ID (e.g., "pid:1234")
    pub id: String,

    /// Trust score (0–1, higher == more trusted). KRIM may interpret deltas.
    pub trust_score: f32,

    /// Optional OS/user/process context
    pub uid: Option<u32>,
    pub pid: Option<u32>,
    pub ppid: Option<u32>,

    /// Process details
    pub binary_path: Option<String>,
    pub command_line: Option<String>,
    pub cwd: Option<String>,

    /// Free-form tags (beacon, dll_inject, etc.)
    pub tags: Vec<String>,

    /// App-specific anchors (ids of detections/anchors that touched this node)
    pub anchor_ids: Vec<String>,
}

/// Directed edge (source -> target).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    // Space for additional attributes if you need later (weight, label, ts, …)
}

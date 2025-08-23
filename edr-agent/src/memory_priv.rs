//! Memory/Privilege feature emitter
//!
//! Scans the episode-derived process graph to summarize indicators of memory
//! abuse and privilege changes. Metrics are computed but not converted to
//! FeatureObservation to avoid guessing your struct ctor—see the TODO below.

use std::collections::HashSet;

use crate::episode::Episode;
use crate::graph_builder::GraphNode;
use crate::traits::{FeatureEmitter, FeatureObservation};
use crate::baselines::BaselineStore;

/// Tunables & heuristics for memory/privilege features.
#[derive(Debug, Clone)]
pub struct MemoryPrivEmitter {
    /// Minimum trust-score drop along a node to consider “interesting”.
    pub min_trust_drop: f32,
    /// Max number of tags to consider from a node (defensive cap).
    pub max_tags_per_node: usize,
}

impl Default for MemoryPrivEmitter {
    fn default() -> Self {
        Self {
            min_trust_drop: 0.20,
            max_tags_per_node: 32,
        }
    }
}

impl MemoryPrivEmitter {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn risky_tags() -> &'static [&'static str] {
        &[
            "memory_anomaly",
            "cred_dump",
            "priv_escalation",
            "w_x_transition",
            "memfd_exec",
            "persistence_artifact",
        ]
    }

    fn analyze_nodes(&self, nodes: &[GraphNode]) -> MemoryPrivStats {
        let risky: HashSet<&str> = Self::risky_tags().iter().copied().collect();

        let mut privileged_proc_count = 0usize;
        let mut risky_tag_hits = 0usize;
        let mut unique_risky_tags: HashSet<String> = HashSet::new();
        let mut anchor_hits = 0usize;

        let mut max_drop = 0f32;

        for n in nodes {
            // Privilege: uid==0 or explicit priv_escalation tag.
            if n.uid.unwrap_or_default() == 0
                || n.tags.iter().any(|t| t == "priv_escalation")
            {
                privileged_proc_count += 1;
            }

            // Risky tags related to memory/priv.
            for t in n.tags.iter().take(self.max_tags_per_node) {
                if risky.contains(t.as_str()) {
                    risky_tag_hits += 1;
                    unique_risky_tags.insert(t.clone());
                }
            }

            // Anchors suggest causal signals (e.g., exploit start/end).
            if !n.anchor_ids.is_empty() {
                anchor_hits += n.anchor_ids.len();
            }

            // Track largest trust-score drop seen (relative to 1.0 default).
            let drop = (1.0f32 - n.trust_score).max(0.0);
            if drop > max_drop {
                max_drop = drop;
            }
        }

        MemoryPrivStats {
            privileged_proc_count,
            risky_tag_hits,
            unique_risky_tags_count: unique_risky_tags.len(),
            anchor_hits,
            max_trust_drop: max_drop,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryPrivStats {
    pub privileged_proc_count: usize,
    pub risky_tag_hits: usize,
    pub unique_risky_tags_count: usize,
    pub anchor_hits: usize,
    pub max_trust_drop: f32,
}

impl FeatureEmitter for MemoryPrivEmitter {
    fn emit(
        &self,
        ep: &Episode,
        _baselines: &mut BaselineStore,
    ) -> Vec<FeatureObservation> {
        let (nodes, _edges) = ep.to_graph();
        let stats = self.analyze_nodes(&nodes);

        // TODO: Map `stats` to your FeatureObservation(s).
        //
        // Example (pseudocode; adjust to your struct API):
        //
        // let mut out = Vec::new();
        // out.push(FeatureObservation::new("mempriv.privileged_proc_count", stats.privileged_proc_count as f64));
        // out.push(FeatureObservation::new("mempriv.risky_tag_hits", stats.risky_tag_hits as f64));
        // out.push(FeatureObservation::new("mempriv.unique_risky_tags", stats.unique_risky_tags_count as f64));
        // out.push(FeatureObservation::new("mempriv.anchor_hits", stats.anchor_hits as f64));
        // out.push(FeatureObservation::new("mempriv.max_trust_drop", stats.max_trust_drop as f64));
        // return out;
        //
        // To keep compilation robust without assuming constructors:
        let _ = stats;
        Vec::new()
    }
}

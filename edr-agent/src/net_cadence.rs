//! Network cadence feature emitter
//!
//! Summarizes novelty/changes in network indicators from the episode graph
//! (e.g., ASN changes, JA3 novelty). Metrics are prepped for conversion into
//! FeatureObservation—see TODO.

use std::collections::HashSet;

use crate::baselines::BaselineStore;
use crate::episode::Episode;
use crate::graph_builder::GraphNode;
use crate::traits::{FeatureEmitter, FeatureObservation};

#[derive(Debug, Clone)]
pub struct NetCadenceEmitter {
    /// Require at least this many hits to consider it notable.
    pub min_hits_threshold: usize,
    /// Defensive cap on per-node tags processed.
    pub max_tags_per_node: usize,
}

impl Default for NetCadenceEmitter {
    fn default() -> Self {
        Self {
            min_hits_threshold: 1,
            max_tags_per_node: 32,
        }
    }
}

impl NetCadenceEmitter {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn net_tags() -> &'static [&'static str] {
        &[
            "new_asn",
            "ja3_novelty",
            "dns_burst",
            "beacon_like",
            "cnc_suspect",
            "sni_novelty",
        ]
    }

    fn analyze_nodes(&self, nodes: &[GraphNode]) -> NetCadenceStats {
        let nety: HashSet<&str> = Self::net_tags().iter().copied().collect();

        let mut total_hits = 0usize;
        let mut unique_tags: HashSet<String> = HashSet::new();
        let mut nodes_with_nety = 0usize;

        let mut max_drop = 0f32;

        for n in nodes {
            let mut node_hit = false;

            for t in n.tags.iter().take(self.max_tags_per_node) {
                if nety.contains(t.as_str()) {
                    total_hits += 1;
                    node_hit = true;
                    unique_tags.insert(t.clone());
                }
            }

            if node_hit {
                nodes_with_nety += 1;
            }

            let drop = (1.0f32 - n.trust_score).max(0.0);
            if drop > max_drop {
                max_drop = drop;
            }
        }

        NetCadenceStats {
            total_hits,
            unique_net_tags_count: unique_tags.len(),
            nodes_with_net_cadence: nodes_with_nety,
            max_trust_drop: max_drop,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetCadenceStats {
    pub total_hits: usize,
    pub unique_net_tags_count: usize,
    pub nodes_with_net_cadence: usize,
    pub max_trust_drop: f32,
}

impl FeatureEmitter for NetCadenceEmitter {
    fn emit(&self, ep: &Episode, _baselines: &mut BaselineStore) -> Vec<FeatureObservation> {
        let (nodes, _edges) = ep.to_graph();
        let stats = self.analyze_nodes(&nodes);

        // TODO: Convert `stats` into FeatureObservation(s).
        //
        // Example (pseudocode; adjust to your struct API):
        //
        // let mut out = Vec::new();
        // out.push(FeatureObservation::new("netcad.total_hits", stats.total_hits as f64));
        // out.push(FeatureObservation::new("netcad.unique_tags", stats.unique_net_tags_count as f64));
        // out.push(FeatureObservation::new("netcad.nodes_with_net_cadence", stats.nodes_with_net_cadence as f64));
        // out.push(FeatureObservation::new("netcad.max_trust_drop", stats.max_trust_drop as f64));
        // return out;
        //
        // Keep it compile-proof for now:
        let _ = stats;
        Vec::new()
    }
}

use std::collections::HashMap;

/// Structure for holding node-level GNN scores and metadata.
#[derive(Debug, Clone)]
pub struct GnnNodeAnalysis {
    pub node_id: String,
    pub score: f64,
    pub centrality: f64,
    pub tags: Vec<String>,
    pub triggered: bool,
}

/// Computes weighted replay relevance based on node attributes.
pub fn compute_replay_priority(node: &GnnNodeAnalysis) -> f64 {
    let mut weight = 1.0;

    if node.score > 0.75 {
        weight += 1.5;
    }
    if node.centrality > 0.6 {
        weight += 1.0;
    }
    if node.tags.iter().any(|t| t.contains("escalation") || t.contains("persistence")) {
        weight += 1.2;
    }
    if node.triggered {
        weight += 1.5;
    }

    weight
}

/// Performs full node analysis from raw GNN output.
pub fn analyze_gnn_scores(
    raw_scores: &HashMap<String, f64>,
    centrality_map: &HashMap<String, f64>,
    tag_map: &HashMap<String, Vec<String>>,
    trigger_flags: &HashMap<String, bool>,
) -> Vec<GnnNodeAnalysis> {
    raw_scores
        .iter()
        .map(|(node_id, &score)| {
            let centrality = *centrality_map.get(node_id).unwrap_or(&0.0);
            let tags = tag_map.get(node_id).cloned().unwrap_or_else(Vec::new);
            let triggered = *trigger_flags.get(node_id).unwrap_or(&false);

            GnnNodeAnalysis {
                node_id: node_id.clone(),
                score,
                centrality,
                tags,
                triggered,
            }
        })
        .collect()
}

/// Returns nodes with replay-worthy impact, sorted by descending priority.
pub fn extract_replay_targets(analyses: &[GnnNodeAnalysis]) -> Vec<(String, f64)> {
    let mut targets: Vec<(String, f64)> = analyses
        .iter()
        .map(|node| (node.node_id.clone(), compute_replay_priority(node)))
        .collect();

    targets.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    targets
}

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::graph_builder::{GraphEdge, GraphNode};
use crate::krim_sequence_models::{SequencePath as BaselineSequence, save_sequences_to_disk};
use crate::score_reason::ScoreReason;
use crate::trust_hook::{TrustCategory, TrustEvent, TrustSource};
use crate::trust_vector::TrustVector;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SequencePath {
    pub start_node: String,
    pub path: Vec<String>,
    pub trust_drop: Option<f32>,
    pub tags: Vec<String>,
    pub anchor_hits: Vec<String>,
}

fn load_baseline_sequences(path: &str) -> Vec<BaselineSequence> {
    let raw = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str::<Vec<BaselineSequence>>(&raw).unwrap_or_default()
}

fn sequence_matches_baseline(seq: &SequencePath, baseline: &[BaselineSequence]) -> bool {
    for base in baseline {
        if base.node_sequence == seq.path || base.tag_trace == seq.tags {
            return true;
        }
    }
    false
}

pub fn analyze_graph_and_score(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    max_depth: usize,
    baseline_path: &str,
) -> Vec<TrustEvent> {
    let node_lookup: HashMap<String, &GraphNode> =
        nodes.iter().map(|n| (n.id.clone(), n)).collect();

    let baseline_sequences = load_baseline_sequences(baseline_path);
    let sequences = extract_behavior_sequences(&nodes.to_vec(), &edges.to_vec(), max_depth);
    score_sequences(&sequences, &node_lookup, &baseline_sequences)
}

pub fn extract_behavior_sequences(
    nodes: &Vec<GraphNode>,
    edges: &Vec<GraphEdge>,
    max_depth: usize,
) -> Vec<SequencePath> {
    let mut graph: HashMap<String, Vec<&GraphEdge>> = HashMap::new();
    let mut node_lookup: HashMap<String, &GraphNode> = HashMap::new();

    for node in nodes {
        node_lookup.insert(node.id.clone(), node);
    }

    for edge in edges {
        graph.entry(edge.source.clone()).or_default().push(edge);
    }

    let mut sequences = Vec::new();

    for start_node in nodes {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_node.id.clone(), vec![start_node.id.clone()]));

        while let Some((current, path)) = queue.pop_front() {
            if path.len() > max_depth {
                continue;
            }
            visited.insert(current.clone());

            if let Some(neighbors) = graph.get(&current) {
                for edge in neighbors {
                    if visited.contains(&edge.target) {
                        continue;
                    }
                    let mut new_path = path.clone();
                    new_path.push(edge.target.clone());
                    queue.push_back((edge.target.clone(), new_path));
                }
            }

            if path.len() > 1 {
                let full_nodes: Vec<&GraphNode> = path
                    .iter()
                    .filter_map(|id| node_lookup.get(id))
                    .cloned()
                    .collect();

                let trust_drop = if full_nodes.len() > 1 {
                    let start_trust = full_nodes.first().unwrap().trust_score;
                    let end_trust = full_nodes.last().unwrap().trust_score;
                    Some(start_trust - end_trust)
                } else {
                    None
                };

                let tags: HashSet<String> =
                    full_nodes.iter().flat_map(|n| n.tags.clone()).collect();

                let anchors: HashSet<String> =
                    full_nodes.iter().flat_map(|n| n.anchor_ids.clone()).collect();

                sequences.push(SequencePath {
                    start_node: start_node.id.clone(),
                    path: path.clone(),
                    trust_drop,
                    tags: tags.into_iter().collect(),
                    anchor_hits: anchors.into_iter().collect(),
                });
            }
        }
    }

    sequences
}

pub fn score_sequences(
    sequences: &[SequencePath],
    node_lookup: &HashMap<String, &GraphNode>,
    baseline: &[BaselineSequence],
) -> Vec<TrustEvent> {
    let mut events = Vec::new();

    for seq in sequences {
        // Suppress known benign patterns
        if sequence_matches_baseline(seq, baseline) {
            continue;
        }

        if seq.path.len() < 3 && seq.trust_drop.unwrap_or(0.0) < 0.2 {
            continue;
        }

        let has_anchor = !seq.anchor_hits.is_empty();
        let risky_tags = ["exec_from_tmp", "memory_anomaly", "cred_dump", "lateral_move"];
        let has_risky_tag = seq.tags.iter().any(|t| risky_tags.contains(&t.as_str()));
        let drop = seq.trust_drop.unwrap_or(0.0);

        if has_anchor && has_risky_tag && drop > 0.4 {
            if let Some(last_id) = seq.path.last() {
                if let Some(last_node) = node_lookup.get(last_id) {
                    let event = TrustEvent {
                        timestamp: Utc::now().timestamp(),
                        source: TrustSource::KrimLite,
                        category: TrustCategory::Graph,
                        score: drop,
                        reason: ScoreReason::CausalSubgraph,
                        uid: last_node.uid.unwrap_or(0),
                        pid: last_node.pid.unwrap_or(0),
                        ppid: last_node.ppid.unwrap_or(0),
                        binary_path: last_node.binary_path.clone().unwrap_or_default(),
                        command_line: last_node.command_line.clone().unwrap_or_default(),
                        cwd: last_node.cwd.clone().unwrap_or_default(),
                        metadata: Some(format!(
                            "KRIM triggered on anchor sequence: {:?} with tags: {:?}",
                            seq.anchor_hits, seq.tags
                        )),
                        trust_vector: TrustVector::from_tag_list(&seq.tags),
                    };
                    events.push(event);
                }
            }
        }
    }

    events
}

pub fn save_sequence_patterns(sequences: &Vec<SequencePath>, path: &str) {
    if let Ok(json) = serde_json::to_string_pretty(sequences) {
        if let Err(e) = std::fs::write(path, json) {
            eprintln!("[krim_lite] Failed to write sequence patterns: {}", e);
        } else {
            println!(
                "[krim_lite] Wrote {} sequence patterns to {}",
                sequences.len(),
                path
            );
        }
    } else {
        eprintln!("[krim_lite] Failed to serialize sequence patterns");
    }
}

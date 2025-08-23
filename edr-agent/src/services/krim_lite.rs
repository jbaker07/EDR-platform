// src/services/krim_lite.rs
// KRIM-lite: build short behavior paths over a process graph, suppress
// known-good baselines, and emit TrustEvents scored by anchors, risky tags,
// and trust-score drop along the path.

use std::collections::{HashMap, HashSet, VecDeque};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::graph_builder::{GraphEdge, GraphNode};
use crate::episode::Episode;

use crate::telemetry::TelemetryRecord;
use crate::trust_hook::TrustEvent;

// ---------- Config / weights ----------

#[derive(Debug, Clone, Copy)]
struct Weights {
    w_anchor: f32,
    w_risky: f32,
    w_drop: f32,
}
impl Default for Weights {
    fn default() -> Self {
        // Calibrated mild bias toward causal anchors; drop is strong
        Self {
            w_anchor: 0.6,
            w_risky: 0.4,
            w_drop: 1.0,
        }
    }
}

#[inline]
fn risky_tag_set() -> &'static [&'static str] {
    &[
        "exec_from_tmp",
        "memory_anomaly",
        "cred_dump",
        "lateral_move",
        "w_x_transition",
        "memfd_exec",
        "new_asn",
        "ja3_novelty",
        "priv_escalation",
        "persistence_artifact",
    ]
}

// ---------- Baseline format ----------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaselineSequence {
    pub node_sequence: Vec<String>,
    pub tag_trace: Vec<String>,
}

fn load_baseline_sequences(path: &str) -> Vec<BaselineSequence> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Vec<BaselineSequence>>(&raw).ok())
        .unwrap_or_default()
}

// ---------- Internal sequence type ----------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SequencePath {
    pub start_node: String,
    pub path: Vec<String>,
    pub trust_drop: Option<f32>,
    pub tags: Vec<String>,
    pub anchor_hits: Vec<String>,
}

fn sequence_matches_baseline(seq: &SequencePath, baseline: &[BaselineSequence]) -> bool {
    // Exact node sequence OR exact tag trace match → suppress
    baseline
        .iter()
        .any(|b| b.node_sequence == seq.path || b.tag_trace == seq.tags)
}

fn compact_tag_trace(tags: &[String], max: usize) -> Vec<String> {
    if tags.len() <= max {
        return tags.to_vec();
    }
    let mut uniq = Vec::<String>::with_capacity(max);
    let mut seen = HashSet::<&str>::new();
    for t in tags {
        if seen.insert(t.as_str()) {
            uniq.push(t.clone());
        }
        if uniq.len() == max {
            break;
        }
    }
    uniq
}

// ---------- Public entry points ----------

/// Primary API: analyze an already-built graph
pub fn analyze_graph_and_score(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    max_depth: usize,
    baseline_path: &str,
) -> Vec<TrustEvent> {
    let node_lookup: HashMap<String, &GraphNode> = nodes.iter().map(|n| (n.id.clone(), n)).collect();

    let baseline_sequences = load_baseline_sequences(baseline_path);
    let sequences = extract_behavior_sequences(nodes, edges, max_depth);
    score_sequences(
        &sequences,
        &node_lookup,
        &baseline_sequences,
        Weights::default(),
    )
}

/// Convenience: build graph from an Episode and score
pub fn analyze_episode_and_score(
    ep: &Episode,
    baseline_path: &str,
    max_depth: usize,
) -> Vec<TrustEvent> {
    let (nodes, edges) = ep.to_graph();
    analyze_graph_and_score(&nodes, &edges, max_depth, baseline_path)
}

/// Convenience: build Episode from records, then score
pub fn analyze_records_and_score(
    records: &[TelemetryRecord],
    baseline_path: &str,
    max_depth: usize,
) -> Vec<TrustEvent> {
    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into());
    let ep = Episode::from_records(host, records);
    analyze_episode_and_score(&ep, baseline_path, max_depth)
}

// ---------- Path extraction ----------

pub fn extract_behavior_sequences(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    max_depth: usize,
) -> Vec<SequencePath> {
    // Build adjacency
    let mut graph: HashMap<String, Vec<&GraphEdge>> = HashMap::new();
    let mut node_lookup: HashMap<String, &GraphNode> = HashMap::new();

    for node in nodes {
        node_lookup.insert(node.id.clone(), node);
    }
    for edge in edges {
        graph.entry(edge.source.clone()).or_default().push(edge);
    }

    let mut sequences: Vec<SequencePath> = Vec::new();
    let mut dedup: HashSet<String> = HashSet::new(); // path signature dedup

    for start in nodes {
        // BFS with per-path visited to avoid cycles
        let mut queue: VecDeque<(String, Vec<String>, HashSet<String>)> = VecDeque::new();
        queue.push_back((
            start.id.clone(),
            vec![start.id.clone()],
            {
                let mut s = HashSet::new();
                s.insert(start.id.clone());
                s
            },
        ));

        while let Some((cur, path, visited)) = queue.pop_front() {
            if path.len() >= max_depth {
                consider_path(&path, &node_lookup, &mut sequences, &mut dedup);
                continue;
            }

            if let Some(neigh) = graph.get(&cur) {
                for edge in neigh {
                    let tgt = edge.target.clone();
                    if visited.contains(&tgt) {
                        continue;
                    }
                    let mut p2 = path.clone();
                    p2.push(tgt.clone());

                    let mut v2 = visited.clone();
                    v2.insert(tgt);

                    // enqueue
                    queue.push_back((edge.target.clone(), p2.clone(), v2));

                    // also consider the partial path now (progressive sequences)
                    consider_path(&p2, &node_lookup, &mut sequences, &mut dedup);
                }
            } else {
                // leaf → finalize this path
                consider_path(&path, &node_lookup, &mut sequences, &mut dedup);
            }
        }
    }

    sequences
}

fn consider_path(
    path: &[String],
    node_lookup: &HashMap<String, &GraphNode>,
    out: &mut Vec<SequencePath>,
    dedup: &mut HashSet<String>,
) {
    if path.len() < 2 {
        return;
    }

    // Build signature for dedup (node path + last-node tags reduced)
    let last = path.last().unwrap();
    let tags_tail = node_lookup
        .get(last)
        .map(|n| {
            let mut t = n.tags.clone();
            t.sort();
            t.truncate(8);
            t.join("|")
        })
        .unwrap_or_default();

    let sig = format!("{}::{}", path.join("->"), tags_tail);
    if !dedup.insert(sig) {
        return;
    }

    // Trust drop: prefer explicit node trust_score, else neutral
    let mut start_trust = 1.0f32;
    let mut end_trust = 1.0f32;
    if let Some(n0) = node_lookup.get(&path[0]) {
        start_trust = n0.trust_score;
    }
    if let Some(nz) = node_lookup.get(path.last().unwrap()) {
        end_trust = nz.trust_score;
    }
    let drop = Some((start_trust - end_trust).max(0.0));

    // Aggregate tags and anchors across path
    let mut tags: HashSet<String> = HashSet::new();
    let mut anchors: HashSet<String> = HashSet::new();
    for nid in path {
        if let Some(n) = node_lookup.get(nid) {
            for t in &n.tags {
                tags.insert(t.clone());
            }
            for a in &n.anchor_ids {
                anchors.insert(a.clone());
            }
        }
    }

    out.push(SequencePath {
        start_node: path[0].clone(),
        path: path.to_vec(),
        trust_drop: drop,
        tags: compact_tag_trace(&tags.into_iter().collect::<Vec<_>>(), 16),
        anchor_hits: anchors.into_iter().collect(),
    });
}

// ---------- Scoring ----------

pub fn score_sequences(
    sequences: &[SequencePath],
    node_lookup: &HashMap<String, &GraphNode>,
    baseline: &[BaselineSequence],
    w: Weights,
) -> Vec<TrustEvent> {
    let mut events = Vec::new();
    let risky = risky_tag_set();

    for seq in sequences {
        // Baseline suppression (exact)
        if sequence_matches_baseline(seq, baseline) {
            continue;
        }

        // Quick filters
        let drop = seq.trust_drop.unwrap_or(0.0);
        if seq.path.len() < 2 {
            continue;
        }

        // Calculate a composite score
        let anchor_score = if seq.anchor_hits.is_empty() { 0.0 } else { 1.0 };
        let risky_hits = seq
            .tags
            .iter()
            .filter(|t| risky.contains(&t.as_str()))
            .count() as f32;
        let score = w.w_anchor * anchor_score + w.w_risky * risky_hits + w.w_drop * drop;

        // Thresholds (tunable)
        let pass = (anchor_score > 0.0 && drop > 0.25) || (risky_hits >= 2.0) || (score >= 1.2);
        if !pass {
            continue;
        }

        // Build a TrustEvent grounded in the last node on the path
        if let Some(last_id) = seq.path.last() {
            if let Some(last_node) = node_lookup.get(last_id) {
                // Construct metadata explaining *why* this fired
                let mut meta = HashMap::new();
                meta.insert("path".into(), format!("{:?}", seq.path));
                meta.insert("anchors".into(), format!("{:?}", seq.anchor_hits));
                meta.insert("tags".into(), format!("{:?}", seq.tags));
                meta.insert("drop".into(), format!("{:.2}", drop));
                meta.insert("score".into(), format!("{:.2}", score));

                // Assemble event compatible with your TrustEvent struct
                let ts_u64 = Utc::now().timestamp() as u64;
                let ev = TrustEvent {
                    timestamp: ts_u64,
                    pid: last_node.pid.unwrap_or(0) as i32,
                    ppid: last_node.ppid.unwrap_or(0) as i32,
                    uid: last_node.uid.unwrap_or(0),
                    binary_path: last_node.binary_path.clone().unwrap_or_default(),
                    command_line: last_node.command_line.clone().unwrap_or_default(),
                    cwd: last_node.cwd.clone().unwrap_or_default(),
                    anomaly_type: "krim_path_anomaly".into(),
                    component: "graph".into(),
                    metadata: meta.clone(),
                    risk_score: score, // treat the composite as a risk-like score
                    source_module: "krim_lite".into(),
                    decay_context: Some("krim_sequence".into()),
                    module: Some("krim_lite".into()),
                    signal: Some("krim_alert".into()),
                    signal_type: Some("graph".into()),
                    score: Some(score),
                    raw_score: Some(score),
                    tags: Some(seq.tags.clone()),
                    description: Some(format!(
                        "KRIM path anomaly: anchors={:?}, risky_hits={:.0}, drop={:.2}, score={:.2}",
                        seq.anchor_hits, risky_hits, drop, score
                    )),
                };
                events.push(ev);
            }
        }
    }

    events
}

// ---------- Utility ----------

pub fn save_sequence_patterns(sequences: &Vec<SequencePath>, path: &str) {
    if let Ok(json) = serde_json::to_string_pretty(sequences) {
        if let Err(e) = std::fs::write(path, json) {
            eprintln!("[krim] Failed to write sequence patterns: {}", e);
        } else {
            println!(
                "[krim] Wrote {} sequence patterns to {}",
                sequences.len(),
                path
            );
        }
    } else {
        eprintln!("[krim] Failed to serialize sequence patterns");
    }
}

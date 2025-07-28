// File: edr-agent/src/gnn_bridge.rs

use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Debug)]
pub struct GnnNode {
    pub node_id: String,
    pub node_type: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Serialize, Debug)]
pub struct GnnEdge {
    pub source: String,
    pub target: String,
    pub relation: String,
}

#[derive(Serialize, Debug)]
pub struct GnnGraphDelta {
    pub timestamp: u64,
    pub nodes: Vec<GnnNode>,
    pub edges: Vec<GnnEdge>,
}

fn get_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn update_graph(hostname: &str, event_type: &str, metadata: HashMap<String, String>) {
    let node_id = format!("{}-event-{}", hostname, event_type);

    let node = GnnNode {
        node_id: node_id.clone(),
        node_type: "event".into(),
        attributes: metadata.clone(),
    };

    let edge = GnnEdge {
        source: hostname.into(),
        target: node_id.clone(),
        relation: event_type.into(),
    };

    let graph_delta = GnnGraphDelta {
        timestamp: get_unix_timestamp(),
        nodes: vec![node],
        edges: vec![edge],
    };

    let serialized = serde_json::to_string(&graph_delta).unwrap();
    let output_path = Path::new("/tmp/gnn_delta.json");
    let mut file = File::create(output_path).expect("Unable to create delta file");
    file.write_all(serialized.as_bytes()).expect("Unable to write delta file");

    println!("[GNN] Delta written to /tmp/gnn_delta.json");
}

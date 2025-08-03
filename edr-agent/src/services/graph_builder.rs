use crate::graph_builder::{GraphNode, GraphEdge};

pub fn extract_causal_subgraph(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    trigger_node_id: &str,
    max_depth: usize,
) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    let mut node_map: HashMap<String, &GraphNode> = nodes.iter().map(|n| (n.id.clone(), n)).collect();
    let mut edge_map: HashMap<String, Vec<&GraphEdge>> = HashMap::new();

    for edge in edges {
        edge_map.entry(edge.target.clone()).or_default().push(edge); // reverse walk
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((trigger_node_id.to_string(), 0));

    let mut subgraph_nodes = HashSet::new();
    let mut subgraph_edges = HashSet::new();

    while let Some((current_id, depth)) = queue.pop_front() {
        if depth > max_depth || !visited.insert(current_id.clone()) {
            continue;
        }

        if let Some(node) = node_map.get(&current_id) {
            subgraph_nodes.insert((*node).clone());
        }

        if let Some(parents) = edge_map.get(&current_id) {
            for edge in parents {
                subgraph_edges.insert((*edge).clone());
                queue.push_back((edge.source.clone(), depth + 1));
            }
        }
    }

    let node_vec: Vec<GraphNode> = subgraph_nodes.into_iter().collect();
    let edge_vec: Vec<GraphEdge> = subgraph_edges.into_iter().collect();

    (node_vec, edge_vec)
}

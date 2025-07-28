import os
import json
import logging
import networkx as nx
from networkx.readwrite import json_graph
from typing import Any, Dict, List
import torch
from torch_geometric.data import Data

# === Setup Logging ===
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def load_graph(filepath: str) -> nx.Graph:
    if not os.path.exists(filepath):
        logger.warning(f"Graph file not found: {filepath}")
        return nx.Graph()
    try:
        with open(filepath, "r") as f:
            data = json.load(f)
        return json_graph.node_link_graph(data)
    except Exception as e:
        logger.error(f"Failed to load graph: {e}")
        return nx.Graph()

def save_graph(graph: nx.Graph, filepath: str):
    try:
        with open(filepath, "w") as f:
            json.dump(json_graph.node_link_data(graph), f)
        logger.info(f"Graph saved: {filepath}")
    except Exception as e:
        logger.error(f"Failed to save graph: {e}")

def update_node(graph: nx.Graph, node_id: Any, attributes: Dict[str, Any]):
    if node_id not in graph:
        logger.warning(f"Node {node_id} not found.")
        return
    for k, v in attributes.items():
        if isinstance(v, (int, float, str, list, dict)):
            graph.nodes[node_id][k] = v
            logger.debug(f"Node {node_id} updated: {k} -> {v}")
        else:
            logger.warning(f"Non-serializable attribute skipped: {k}")

def batch_update_nodes(graph: nx.Graph, updates: List[Dict[str, Any]]):
    for update in updates:
        node_id = update.get("node_id")
        if not node_id:
            logger.warning("Missing node_id in update.")
            continue
        attrs = {k: v for k, v in update.items() if k != "node_id"}
        update_node(graph, node_id, attrs)

def convert_graph_to_data(graph: nx.Graph) -> Data:
    try:
        node_features = []
        node_map = {n: i for i, n in enumerate(graph.nodes)}
        for node in graph.nodes:
            vec = graph.nodes[node].get("feature_vector", [0.0])
            node_features.append(torch.tensor(vec, dtype=torch.float))
        edges = [(node_map[u], node_map[v]) for u, v in graph.edges]
        edge_index = torch.tensor(edges, dtype=torch.long).t().contiguous() if edges else torch.empty((2, 0), dtype=torch.long)
        return Data(x=torch.stack(node_features), edge_index=edge_index)
    except Exception as e:
        logger.error(f"Graph conversion failed: {e}")
        return Data()

import networkx as nx
import torch
from torch_geometric.data import Data
from gnn_model import GNN, extract_node_features
import os

# === Config Paths ===
GRAPH_PATH = "backend/gnn_pipeline/data/processed/telemetry_graph.gpickle"
MODEL_PATH = "backend/gnn_pipeline/models/gnn_trust_model.pt"

# === GNN Model Wrapper ===
class GNNScorer:
    def __init__(self):
        self.graph = None
        self.model = None
        self.device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
        self._load_graph()
        self._load_model()

    def _load_graph(self):
        if os.path.exists(GRAPH_PATH):
            self.graph = nx.read_gpickle(GRAPH_PATH)
        else:
            raise FileNotFoundError(f"Graph not found at: {GRAPH_PATH}")

    def _load_model(self):
        self.model = GNN(
            in_channels=64,  # Adjust based on extract_node_features
            hidden_channels=128,
            out_channels=1
        ).to(self.device)
        if os.path.exists(MODEL_PATH):
            self.model.load_state_dict(torch.load(MODEL_PATH, map_location=self.device))
            self.model.eval()
        else:
            raise FileNotFoundError(f"GNN model not found at: {MODEL_PATH}")

    def score_node(self, node_id: str) -> float:
        if node_id not in self.graph.nodes:
            raise ValueError(f"Node {node_id} not found in graph.")

        # Convert NetworkX graph to PyG Data object
        node_features, edge_index = extract_node_features(self.graph)

        data = Data(
            x=node_features,
            edge_index=edge_index
        ).to(self.device)

        with torch.no_grad():
            output = self.model(data.x, data.edge_index)
            node_idx = list(self.graph.nodes).index(node_id)
            score = output[node_idx].item()

        return round(score, 4)

# === Interface ===
scorer_instance = GNNScorer()

def get_gnn_score(node_id: str) -> float:
    try:
        return scorer_instance.score_node(node_id)
    except Exception as e:
        print(f"[GNN Score Error] {e}")
        return 0.0

import torch
import networkx as nx
import pickle
import csv
from torch_geometric.nn import GCNConv
from torch_geometric.utils import from_networkx
from torch_geometric.data import Data

MODEL_PATH = "backend/gnn_pipeline/models/gnn_model_wave10.pt"
GRAPH_PATH = "backend/gnn_pipeline/data/processed/telemetry_graph_wave10.gpickle"
OUTPUT_CSV = "backend/gnn_pipeline/results/trust_scores_wave10.csv"

class GCN(torch.nn.Module):
    def __init__(self, in_channels, hidden_channels=16, out_channels=2):
        super().__init__()
        self.conv1 = GCNConv(in_channels, hidden_channels)
        self.conv2 = GCNConv(hidden_channels, out_channels)

    def forward(self, x, edge_index):
        x = self.conv1(x, edge_index)
        x = torch.relu(x)
        x = self.conv2(x, edge_index)
        return x

def load_gnn_model(model_path: str, in_channels: int) -> GCN:
    model = GCN(in_channels=in_channels)
    model.load_state_dict(torch.load(model_path, map_location=torch.device('cpu')))
    model.eval()
    return model

def load_graph(graph_path: str) -> nx.Graph:
    with open(graph_path, "rb") as f:
        return pickle.load(f)

def build_data(graph: nx.Graph) -> tuple[Data, list]:
    valid_nodes = []
    for n, d in graph.nodes(data=True):
        f = d.get("feature")
        if isinstance(f, list) and all(isinstance(v, (int, float)) for v in f):
            valid_nodes.append(n)
        else:
            print(f"[!] Skipping invalid feature on node {n}: {f}")

    if not valid_nodes:
        raise ValueError("❌ No valid nodes with clean 'feature' vectors found.")

    subgraph = graph.subgraph(valid_nodes).copy()

    for n in subgraph.nodes():
        subgraph.nodes[n]["x"] = subgraph.nodes[n]["feature"]

    data = from_networkx(subgraph)
    return data, list(subgraph.nodes)

@torch.no_grad()
def compute_all_trust_scores(model: GCN, data: Data, node_ids: list):
    x = data.x.float()
    edge_index = data.edge_index
    logits = model(x, edge_index)
    probs = torch.softmax(logits, dim=1)

    hard_label_count = 0
    results = []
    for idx, prob in enumerate(probs):
        label = torch.argmax(prob).item()
        confidence = prob[label].item()
        node_id = node_ids[idx]

        if confidence == 1.0:
            hard_label_count += 1
            print(f"[!] {node_id} → HARD prediction: prob={prob.tolist()}")

        trust = 100.0 - (1 - confidence) * 50.0 if label == 1 else confidence * 100.0

        results.append({
            "node_id": node_id,
            "predicted_label": "benign" if label == 1 else "malicious",
            "confidence": round(confidence, 4),
            "trust_score": round(trust, 1),
            "wave_version": "wave10"
        })

    if hard_label_count == len(node_ids):
        print(f"\n⚠️ WARNING: All {hard_label_count} predictions are confidence=1.0. Your model may be overfitting or only seeing one class.")

    return results

def save_results_to_csv(results: list, output_path: str):
    fieldnames = ["node_id", "predicted_label", "confidence", "trust_score", "wave_version"]
    with open(output_path, "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(results)
    print(f"[+] Saved trust scores to: {output_path}")

if __name__ == "__main__":
    try:
        graph = load_graph(GRAPH_PATH)
        data, node_ids = build_data(graph)
        in_channels = data.num_node_features
        model = load_gnn_model(MODEL_PATH, in_channels)
        results = compute_all_trust_scores(model, data, node_ids)
        save_results_to_csv(results, OUTPUT_CSV)
    except Exception as e:
        print(f"[!] Error during batch trust scoring: {e}")

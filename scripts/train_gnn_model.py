import torch
import torch.nn.functional as F
import pandas as pd
import pickle
import numpy as np
from sklearn.model_selection import train_test_split
from torch_geometric.data import Data
from torch_geometric.nn import GCNConv

# Paths
GRAPH_PATH = "backend/gnn_pipeline/data/processed/telemetry_graph_unified_enriched.gpickle"
LABEL_PATH = "backend/gnn_pipeline/data/processed/training_ready_unified_enriched.csv"
MODEL_SAVE_PATH = "backend/gnn_pipeline/models/gnn_model_final_enriched.pt"

# === GCN Model ===
class GCN(torch.nn.Module):
    def __init__(self, in_channels, hidden_channels, out_channels):
        super().__init__()
        self.conv1 = GCNConv(in_channels, hidden_channels)
        self.conv2 = GCNConv(hidden_channels, out_channels)

    def forward(self, x, edge_index):
        x = self.conv1(x, edge_index)
        x = F.relu(x)
        x = self.conv2(x, edge_index)
        return x

# === Build Torch Geometric Graph Data ===
def build_data(graph_path, label_path):
    G = pickle.load(open(graph_path, "rb"))
    label_df = pd.read_csv(label_path, dtype={"pid": str, "label": int})

    node_list = list(G.nodes)
    node_index_map = {node: i for i, node in enumerate(node_list)}
    label_map = {str(row["pid"]): int(row["label"]) for _, row in label_df.iterrows()}

    features = []
    labels = []
    for node in node_list:
        attrs = G.nodes[node]
        pid = str(node)
        features.append([
            G.degree(node),
            float(attrs.get("cpu", 0)),
            float(attrs.get("memory", 0)),
            float(attrs.get("risk_score", 0)),
            1 if attrs.get("risk_level", "").lower() in ["high", "critical"] else 0,
            1 if str(attrs.get("source", "")).startswith("simulated_malicious") else 0,
            float(attrs.get("syscalls", 0)),
            float(attrs.get("anomaly_score", 0)),
            float(attrs.get("parent_risk", 0)),
            1 if "admin" in str(attrs.get("user", "")).lower() else 0
        ])
        labels.append(label_map.get(pid, 0))

    x = torch.tensor(features, dtype=torch.float32)
    y = torch.tensor(labels, dtype=torch.long)
    edge_index = torch.tensor([
        [node_index_map[u], node_index_map[v]] for u, v in G.edges
    ], dtype=torch.long).t().contiguous()

    return Data(x=x, edge_index=edge_index, y=y)

# === Train and Evaluate ===
def train_model(data):
    # Split indices
    num_nodes = data.num_nodes
    indices = list(range(num_nodes))
    y_np = data.y.numpy()
    train_idx, val_idx = train_test_split(indices, test_size=0.2, stratify=y_np)

    train_mask = torch.zeros(num_nodes, dtype=torch.bool)
    val_mask = torch.zeros(num_nodes, dtype=torch.bool)
    train_mask[train_idx] = True
    val_mask[val_idx] = True

    model = GCN(in_channels=data.num_node_features, hidden_channels=32, out_channels=2)
    optimizer = torch.optim.Adam(model.parameters(), lr=0.01, weight_decay=5e-4)
    criterion = torch.nn.CrossEntropyLoss()

    best_val_acc = 0
    patience, wait = 10, 0
    best_state = None

    for epoch in range(1, 201):
        model.train()
        optimizer.zero_grad()
        out = model(data.x, data.edge_index)
        loss = criterion(out[train_mask], data.y[train_mask])
        loss.backward()
        optimizer.step()

        model.eval()
        _, pred = model(data.x, data.edge_index).max(dim=1)
        correct = pred[val_mask] == data.y[val_mask]
        val_acc = int(correct.sum()) / int(val_mask.sum())

        print(f"[Epoch {epoch:03d}] Loss: {loss.item():.4f} | Val Acc: {val_acc * 100:.2f}%")

        if val_acc > best_val_acc:
            best_val_acc = val_acc
            wait = 0
            best_state = model.state_dict()
        else:
            wait += 1
            if wait >= patience:
                print("[!] Early stopping.")
                break

    model.load_state_dict(best_state)
    torch.save(model.state_dict(), MODEL_SAVE_PATH)
    print(f"[✓] Trained robust model saved to: {MODEL_SAVE_PATH}")

# === Run Training ===
if __name__ == "__main__":
    print("[+] Loading telemetry graph and labels...")
    data = build_data(GRAPH_PATH, LABEL_PATH)
    print("[+] Beginning aggressive supervised GNN training...")
    train_model(data)

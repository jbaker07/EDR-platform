import argparse
import torch
import pickle
import numpy as np
import networkx as nx

from app.services.trust_gnn_engine import model

parser = argparse.ArgumentParser()
parser.add_argument("--node", type=str, required=True, help="Node ID (e.g., '15229_156804')")
parser.add_argument("--version", type=str, default="wave10", help="GNN model version")
args = parser.parse_args()

graph_path = f"backend/gnn_pipeline/data/processed/telemetry_graph_{args.version}.gpickle"

try:
    with open(graph_path, "rb") as f:
        G = pickle.load(f)
except Exception as e:
    print(f"[!] Failed to load graph: {e}")
    exit(1)

if args.node not in G:
    print(f"[!] Node '{args.node}' not found in graph.")
    exit(1)

node_data = G.nodes[args.node]
if "feature" not in node_data:
    print(f"[!] Node '{args.node}' missing 'feature' data.")
    exit(1)

# Build full feature matrix and edge index for graph
node_list = list(G.nodes)
node_index = {node: i for i, node in enumerate(node_list)}

x = []
for node in node_list:
    feature = G.nodes[node].get("feature")
    if feature is None:
        print(f"[!] Node '{node}' missing 'feature' data.")
        exit(1)
    x.append(feature)

x = torch.tensor(x, dtype=torch.float32)
edges = [(node_index[u], node_index[v]) for u, v in G.edges if u in node_index and v in node_index]
edge_index = torch.tensor(edges, dtype=torch.long).t().contiguous()

# Run inference
with torch.no_grad():
    out = model(x, edge_index)
    node_idx = node_index[args.node]
    logits = out[node_idx]
    prob = torch.softmax(logits, dim=0).tolist()
    label = "malicious" if np.argmax(prob) == 1 else "benign"
    confidence = prob[1]

print("\n🔍 GNN Trust Evaluation Result")
print("-------------------------------")
print(f"  🧩 Node ID      : {args.node}")
print(f"  🌊 Model Version: {args.version}")
print(f"  🧠 Prediction   : {label}")
print(f"  🎯 Confidence   : {round(confidence, 4)}")
print("-------------------------------\n")

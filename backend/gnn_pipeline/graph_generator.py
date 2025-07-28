import os
import json
import hashlib
import pickle
import networkx as nx
from pathlib import Path

# Adjust paths as needed
BASE_DIR = Path(__file__).resolve().parents[2]  # Move up two levels to EDR-platform/
INPUT_PATH = BASE_DIR / "graph_output" / "enriched_telemetry.json"
OUTPUT_PATH = BASE_DIR / "graph_output" / "telemetry_graph.gpickle2"

def build_graph():
    with open(INPUT_PATH, "r") as f:
        telemetry_data = json.load(f)

    G = nx.DiGraph()

    # Add graph nodes
    for entry in telemetry_data:
        if not entry.get("is_graph_node", False):
            continue

        node_id = entry.get("event_id")
        if not node_id:
            node_id = hashlib.sha256(json.dumps(entry, sort_keys=True).encode()).hexdigest()

        G.add_node(node_id, **{
            "exe": entry.get("exe"),
            "cmdline": entry.get("cmdline"),
            "user": entry.get("user"),
            "pid": entry.get("pid"),
            "ppid": entry.get("ppid"),
            "risk_score": entry.get("risk_score"),
            "risk_level": entry.get("risk_level"),
            "tags": entry.get("tags", []),
            "timestamp": entry.get("timestamp"),
            "is_graph_node": True
        })

    # Add edges based on shared tags
    node_list = list(G.nodes(data=True))
    for i in range(len(node_list)):
        id_i, data_i = node_list[i]
        tags_i = set(data_i.get("tags", []))
        for j in range(i + 1, len(node_list)):
            id_j, data_j = node_list[j]
            tags_j = set(data_j.get("tags", []))
            shared = tags_i & tags_j
            if shared:
                G.add_edge(id_i, id_j, shared_tags=list(shared))

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    
    # Save using pickle to avoid compatibility issues
    with open(OUTPUT_PATH, "wb") as f:
        pickle.dump(G, f)

    print(f"✅ Graph saved to {OUTPUT_PATH} with {len(G.nodes())} nodes and {len(G.edges())} edges.")

if __name__ == "__main__":
    build_graph()

# backend/app/services/inspect_graph.py

import pickle
import networkx as nx

GRAPH_PATH = "backend/gnn_pipeline/data/processed/telemetry_graph.gpickle"

def inspect_graph():
    try:
        with open(GRAPH_PATH, "rb") as f:
            G = pickle.load(f)

        print(f"✅ Loaded graph with {G.number_of_nodes()} nodes and {G.number_of_edges()} edges.\n")

        for node, data in G.nodes(data=True):
            print(f"🟢 Node: {node}")
            for k, v in data.items():
                print(f"    {k}: {v}")
            print("-" * 50)

    except Exception as e:
        print(f"[!] Failed to load or inspect graph: {e}")

if __name__ == "__main__":
    inspect_graph()

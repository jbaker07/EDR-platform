import pickle
import networkx as nx

path = "backend/gnn_pipeline/data/processed/telemetry_graph_wave10.gpickle"
G = pickle.load(open(path, "rb"))

sample = list(G.nodes())[:10]
print("[*] Example node IDs in the graph:")
for node in sample:
    print("   →", node)

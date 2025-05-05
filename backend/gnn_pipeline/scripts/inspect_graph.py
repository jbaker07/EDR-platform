import pickle
import networkx as nx

with open("backend/gnn_pipeline/data/processed/telemetry_graph.gpickle", "rb") as f:
    G = pickle.load(f)

print("✅ Graph loaded.")
print(f"🧠 Total nodes: {G.number_of_nodes()}")
print(f"🔗 Total edges: {G.number_of_edges()}")

print("\n🎯 Sample nodes:")
for node, attrs in list(G.nodes(data=True))[:5]:
    print(f"{node}: {attrs}")

print("\n🔗 Sample edges:")
for u, v, attrs in list(G.edges(data=True))[:5]:
    print(f"{u} → {v} | {attrs}")

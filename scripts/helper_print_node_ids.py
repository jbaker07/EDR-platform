import pickle
with open("backend/gnn_pipeline/data/processed/telemetry_graph_wave10.gpickle", "rb") as f:
    G = pickle.load(f)

label_counts = {}
for n, d in G.nodes(data=True):
    y = d.get("y")
    label_counts[y] = label_counts.get(y, 0) + 1
print(label_counts)

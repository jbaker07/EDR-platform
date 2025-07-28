import time
import networkx as nx
import requests
from datetime import datetime

GNN_PATH = "backend/gnn_pipeline/data/processed/telemetry_graph.gpickle"
SWEEP_ENDPOINT = "http://localhost:8000/api/forensics/run"

def detect_gnn_anomalies(graph: nx.Graph):
    # Simple heuristic: cluster growth or edge delta over threshold
    changed_clusters = []
    for component in nx.connected_components(graph):
        subgraph = graph.subgraph(component)
        if len(subgraph.edges) > 50:  # Arbitrary growth threshold
            changed_clusters.append(subgraph)
    return changed_clusters

def main():
    while True:
        try:
            G = nx.read_gpickle(GNN_PATH)
            anomalies = detect_gnn_anomalies(G)
            if anomalies:
                print(f"[GNN] Triggering sweep for {len(anomalies)} suspect clusters")
                requests.post(SWEEP_ENDPOINT, json={"reason": "GNN_anomaly", "timestamp": str(datetime.utcnow())})
        except Exception as e:
            print(f"[!] GNN sweep error: {e}")
        time.sleep(1800)  # Run every 30 min

if __name__ == "__main__":
    main()

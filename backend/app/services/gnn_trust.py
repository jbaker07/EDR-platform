# backend/app/services/gnn_trust.py
import networkx as nx

# Load GNN graph once (you may want to move this to an init function if memory is a concern)
G = nx.read_gpickle("backend/gnn_pipeline/data/processed/telemetry_graph.gpickle")

def compute_gnn_trust_score(hostname: str) -> float:
    if hostname not in G:
        return 0.0
    centrality = nx.degree_centrality(G)
    score = centrality.get(hostname, 0.0) * 100
    return round(score, 2)

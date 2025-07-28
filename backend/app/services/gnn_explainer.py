import networkx as nx

def get_gnn_trace(event: dict) -> dict:
    G = nx.read_gpickle("backend/gnn_pipeline/data/processed/telemetry_graph.gpickle")
    node_id = event.get("gnn_node_id")
    if not node_id or node_id not in G:
        return {}

    # Root cause scoring using degree centrality
    centrality = nx.degree_centrality(G)
    ranked_nodes = sorted(centrality.items(), key=lambda x: x[1], reverse=True)
    likely_root = ranked_nodes[0][0] if ranked_nodes else node_id
    
    return {
        "node": node_id,
        "likely_root": likely_root,
        "causal_parents": G.nodes[node_id].get("causes", []),
        "related_nodes": list(G.neighbors(node_id))
    }

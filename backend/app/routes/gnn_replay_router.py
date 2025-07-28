from fastapi import APIRouter
import networkx as nx

router = APIRouter()

@router.get("/api/gnn/replay")
def get_replay_path(event_id: str):
    G = nx.read_gpickle("backend/gnn_pipeline/data/processed/telemetry_graph.gpickle")

    if event_id not in G:
        return {"error": "Event node not found."}

    # Get a linearized trace path (MVP version)
    trace = []
    current = event_id
    visited = set()

    while current and current not in visited:
        node_data = G.nodes[current]
        trace.append({
            "node_id": current,
            "timestamp": node_data.get("timestamp", ""),
            "event_type": node_data.get("type", "unknown"),
            "shap_top": node_data.get("shap_feature", "N/A"),
            "trust_delta": node_data.get("trust_delta", 0)
        })
        visited.add(current)
        causes = node_data.get("causes", [])
        current = causes[0] if causes else None

    trace.reverse()  # show earliest first
    return {"replay": trace}

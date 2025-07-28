from fastapi import APIRouter
import pickle
import networkx as nx
from gnn_pipeline.utils.graph_archiver import (
    archive_gnn_fragment,
    list_archived_snapshots,
    load_archived_snapshot,
)

router = APIRouter()

@router.post("/api/gnn/archive")
def archive_graph(incident_id: str):
    try:
        with open("backend/gnn_pipeline/data/processed/telemetry_graph.gpickle", "rb") as f:
            G = pickle.load(f)
    except Exception as e:
        return {"error": f"Failed to load graph: {str(e)}"}

    path = archive_gnn_fragment(G, incident_id)
    return {"status": "archived", "path": path}


@router.get("/api/gnn/archive/list")
def list_archives():
    try:
        files = list_archived_snapshots()
        return {"files": files}
    except Exception as e:
        return {"error": f"Failed to list archives: {str(e)}"}


@router.get("/api/gnn/archive/load")
def load_archive(filename: str):
    try:
        G = load_archived_snapshot(filename)
        return {
            "nodes": list(G.nodes(data=True)),
            "edges": list(G.edges(data=True)),
        }
    except Exception as e:
        return {"error": f"Failed to load archive: {str(e)}"}

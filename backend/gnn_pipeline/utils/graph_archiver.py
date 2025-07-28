import os
import pickle
import networkx as nx

ARCHIVE_DIR = "backend/gnn_pipeline/data/archives"

os.makedirs(ARCHIVE_DIR, exist_ok=True)

def archive_gnn_fragment(graph: nx.Graph, incident_id: str) -> str:
    """Save a GNN fragment to disk with incident ID in filename."""
    path = os.path.join(ARCHIVE_DIR, f"{incident_id}.gpickle")
    with open(path, "wb") as f:
        pickle.dump(graph, f)
    return path

def list_archived_snapshots() -> list:
    """Return list of all saved .gpickle archive files."""
    return [f for f in os.listdir(ARCHIVE_DIR) if f.endswith(".gpickle")]

def load_archived_snapshot(filename: str) -> nx.Graph:
    """Load a previously archived graph snapshot."""
    path = os.path.join(ARCHIVE_DIR, filename)
    with open(path, "rb") as f:
        return pickle.load(f)

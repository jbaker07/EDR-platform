import os
import json
import pickle
from typing import List, Dict
import networkx as nx
from pathlib import Path

RAW_DATA_PATH = Path("backend/gnn_pipeline/data/raw")
PROCESSED_DATA_PATH = Path("backend/gnn_pipeline/data/processed")
GRAPH_OUTPUT_PATH = PROCESSED_DATA_PATH / "telemetry_graph.gpickle"

def build_graph_from_telemetry() -> nx.Graph:
    G = nx.Graph()

    for file in RAW_DATA_PATH.glob("*.json"):
        with open(file) as f:
            data = json.load(f)

        hostname = data.get("hostname", "unknown")
        timestamp = data.get("timestamp", 0)

        G.add_node(hostname, type="host", timestamp=timestamp)

        # --- Processes ---
        for proc in data.get("processes", []):
            pid_node = f"{hostname}-pid-{proc['pid']}"
            G.add_node(pid_node, type="process", **proc)
            G.add_edge(pid_node, hostname, relation="runs_on")

            if "cmd" in proc and "/tmp" in proc["cmd"]:
                tmp_file = f"{hostname}-file-/tmp"
                G.add_node(tmp_file, type="file", path="/tmp")
                G.add_edge(pid_node, tmp_file, relation="accesses")

            if "user" in proc:
                user_node = f"{hostname}-user-{proc['user']}"
                G.add_node(user_node, type="user", user=proc["user"])
                G.add_edge(pid_node, user_node, relation="owned_by")

        # --- File Events ---
        for file_event in data.get("file_events", []):
            file_path = file_event.get("path", "unknown")
            proc_id = f"{hostname}-pid-{file_event.get('pid', 'na')}"
            file_node = f"{hostname}-file-{file_path}"
            G.add_node(file_node, type="file", **file_event)
            G.add_edge(proc_id, file_node, relation="accessed")

        # --- Auth Events ---
        for auth in data.get("auth_events", []):
            user = auth.get("user", "unknown")
            auth_node = f"{hostname}-auth-{user}"
            G.add_node(auth_node, type="auth", **auth)
            G.add_edge(auth_node, hostname, relation="auth_attempt")

        # --- Registry Events ---
        for reg in data.get("registry_events", []):
            key = reg.get("key", "unknown")
            reg_node = f"{hostname}-reg-{key}"
            G.add_node(reg_node, type="registry", **reg)
            G.add_edge(reg_node, hostname, relation="registry_access")

        # --- Cloud Metadata ---
        if "cloud_metadata" in data:
            cloud = data["cloud_metadata"]
            cloud_node = f"{hostname}-cloud-{cloud.get('provider', 'unknown')}"
            G.add_node(cloud_node, type="cloud", **cloud)
            G.add_edge(hostname, cloud_node, relation="cloud_hosted")

        # --- Identity ---
        if "identity" in data:
            id_data = data["identity"]
            id_node = f"{hostname}-identity-{id_data.get('username', 'unknown')}"
            G.add_node(id_node, type="identity", **id_data)
            G.add_edge(id_node, hostname, relation="belongs_to")

    return G

def save_graph(G: nx.Graph):
    PROCESSED_DATA_PATH.mkdir(parents=True, exist_ok=True)
    with open(GRAPH_OUTPUT_PATH, "wb") as f:
        pickle.dump(G, f)
    print(f"[✓] Graph saved to: {GRAPH_OUTPUT_PATH}")

if __name__ == "__main__":
    graph = build_graph_from_telemetry()
    save_graph(graph)

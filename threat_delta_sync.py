import json
import os
import hashlib
from typing import Dict, List
from datetime import datetime

SYNC_LOG = "fleet/synced_deltas.json"
DELTA_INPUT_DIR = "replays/annotated_deltas/"
SYNC_TOKEN = os.getenv("DELTA_SYNC_TOKEN", "dev-mode-token")

def hash_delta(delta: Dict) -> str:
    return hashlib.sha256(json.dumps(delta, sort_keys=True).encode()).hexdigest()

def load_deltas(directory: str) -> List[Dict]:
    deltas = []
    for file in os.listdir(directory):
        if file.endswith(".json"):
            with open(os.path.join(directory, file), "r") as f:
                try:
                    delta = json.load(f)
                    deltas.append(delta)
                except Exception:
                    continue
    return deltas

def filter_minimal_delta(delta: Dict) -> Dict:
    """Strip non-behavioral fields — privacy hardened"""
    return {
        "semantic_tags": delta.get("semantic_tags", []),
        "anomalies": delta.get("anomalies", []),
        "gnn_nodes": delta.get("gnn_nodes", []),
        "gnn_edges": delta.get("gnn_edges", []),
        "risk_score": delta.get("risk_score", 0.0),
        "trust_score": delta.get("trust_score", 0.0),
        "timestamp": delta.get("timestamp"),
    }

def sync_deltas():
    deltas = load_deltas(DELTA_INPUT_DIR)
    synced_log = []

    if os.path.exists(SYNC_LOG):
        with open(SYNC_LOG, "r") as f:
            synced_log = json.load(f)

    known_hashes = {entry["hash"] for entry in synced_log}
    new_entries = []

    for delta in deltas:
        minimal = filter_minimal_delta(delta)
        delta_hash = hash_delta(minimal)
        if delta_hash not in known_hashes:
            new_entries.append({
                "hash": delta_hash,
                "synced_at": datetime.utcnow().isoformat(),
                "payload": minimal,
                "signature": f"signed::{SYNC_TOKEN[:8]}::{delta_hash[:10]}"
            })

    if new_entries:
        synced_log.extend(new_entries)
        with open(SYNC_LOG, "w") as f:
            json.dump(synced_log, f, indent=4)

        print(f"[✅] Synced {len(new_entries)} new behavior deltas.")
    else:
        print("[ℹ️] No new deltas to sync.")

if __name__ == "__main__":
    sync_deltas()

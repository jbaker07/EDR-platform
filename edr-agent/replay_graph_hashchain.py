import os
import json
import hashlib
from datetime import datetime

REPLAY_DIR = "replay/vault"
HASHCHAIN_LOG = "replay/chain/hash_log.json"

def compute_hash(data):
    return hashlib.sha256(json.dumps(data, sort_keys=True).encode()).hexdigest()

def load_existing_chain():
    if not os.path.exists(HASHCHAIN_LOG):
        return []
    with open(HASHCHAIN_LOG, "r") as f:
        return json.load(f)

def append_to_chain(session_data, previous_hash):
    block = {
        "session_id": session_data.get("session_id"),
        "timestamp": datetime.utcnow().isoformat(),
        "data_hash": compute_hash(session_data),
        "previous_hash": previous_hash,
    }
    block["block_hash"] = compute_hash(block)
    return block

def build_hashchain():
    os.makedirs(os.path.dirname(HASHCHAIN_LOG), exist_ok=True)
    chain = load_existing_chain()
    prev_hash = chain[-1]["block_hash"] if chain else "GENESIS"

    for file in sorted(os.listdir(REPLAY_DIR)):
        if not file.endswith(".replay.json"):
            continue
        path = os.path.join(REPLAY_DIR, file)
        with open(path, "r") as f:
            session = json.load(f)
        block = append_to_chain(session, prev_hash)
        chain.append(block)
        prev_hash = block["block_hash"]

    with open(HASHCHAIN_LOG, "w") as f:
        json.dump(chain, f, indent=2)

    print(f"[HashChain] {len(chain)} replay blocks chained.")

if __name__ == "__main__":
    build_hashchain()

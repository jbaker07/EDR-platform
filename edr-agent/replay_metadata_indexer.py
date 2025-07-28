import os
import json
import hashlib
from datetime import datetime
from pathlib import Path

REPLAY_DIR = Path("replays/")
METADATA_INDEX_FILE = Path("replay_index/metadata_index.json")

# Thresholds for triggering escalation
TRUST_DECAY_THRESHOLD = -30
DANGEROUS_TAGS = {"persistence", "lateral_movement", "credential_access"}

def hash_file(file_path):
    sha256 = hashlib.sha256()
    with open(file_path, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            sha256.update(chunk)
    return sha256.hexdigest()

def determine_escalation(trust_delta, tags):
    if trust_delta <= TRUST_DECAY_THRESHOLD:
        return True
    if any(tag in DANGEROUS_TAGS for tag in tags):
        return True
    return False

def extract_metadata_from_replay(replay_data, replay_path):
    trust_score_start = replay_data.get("start_trust", 100)
    trust_score_end = replay_data.get("end_trust", 100)
    tags = replay_data.get("tags", [])
    uid = replay_data.get("session_id") or replay_data.get("replay_id") or hash_file(replay_path)

    trust_delta = trust_score_end - trust_score_start

    metadata = {
        "id": uid,
        "filename": os.path.basename(replay_path),
        "path": str(replay_path),
        "timestamp": replay_data.get("timestamp") or datetime.fromtimestamp(os.path.getmtime(replay_path)).isoformat(),
        "trust_score_start": trust_score_start,
        "trust_score_end": trust_score_end,
        "trust_delta": trust_delta,
        "tags": tags,
        "sha256": hash_file(replay_path),
        "user_id": replay_data.get("user_id", "unknown"),
        "endpoint_id": replay_data.get("endpoint_id", "unknown"),
        "kill_chain_stage": replay_data.get("kill_chain_stage", []),
        "threat_profile": replay_data.get("threat_profile", None),
        "escalation_required": determine_escalation(trust_delta, tags)
    }

    return metadata

def build_index():
    index = []

    if not REPLAY_DIR.exists():
        print(f"[ERROR] Replay directory {REPLAY_DIR} does not exist.")
        return

    for file in REPLAY_DIR.glob("*.json"):
        try:
            with open(file, "r") as f:
                replay_data = json.load(f)
            metadata = extract_metadata_from_replay(replay_data, file)
            index.append(metadata)
        except Exception as e:
            print(f"[ERROR] Failed to process {file}: {e}")

    METADATA_INDEX_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(METADATA_INDEX_FILE, "w") as out_file:
        json.dump(index, out_file, indent=4)

    print(f"[+] Indexed {len(index)} replay files into {METADATA_INDEX_FILE}")

if __name__ == "__main__":
    build_index()

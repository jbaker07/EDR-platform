import os
import json
import time
from datetime import datetime, timedelta

REPLAY_VAULT_DIR = "replay/vault"
ESCALATION_TAGS = {"critical", "mfa_bypass", "exfil_detected", "priv_esc", "unknown_payload"}
TTL_HOURS = 6  # default time-to-live

def load_replay_metadata(file_path):
    with open(file_path, "r") as f:
        data = json.load(f)
    return data

def is_eligible_for_deletion(replay_data):
    tags = set(replay_data.get("tags", []))
    flagged = replay_data.get("flagged", False)
    timestamp_str = replay_data.get("timestamp")
    if not timestamp_str:
        return False
    replay_time = datetime.fromisoformat(timestamp_str)
    expired = datetime.utcnow() > (replay_time + timedelta(hours=TTL_HOURS))
    escalated = tags & ESCALATION_TAGS or flagged
    return expired and not escalated

def sweep_vault():
    files_deleted = 0
    for file in os.listdir(REPLAY_VAULT_DIR):
        if not file.endswith(".replay.json"):
            continue
        path = os.path.join(REPLAY_VAULT_DIR, file)
        try:
            replay_data = load_replay_metadata(path)
            if is_eligible_for_deletion(replay_data):
                os.remove(path)
                files_deleted += 1
        except Exception as e:
            print(f"[Vault] Error processing {file}: {e}")
    print(f"[Vault] Swept vault. Deleted {files_deleted} expired replays.")

if __name__ == "__main__":
    sweep_vault()

import os
import json
from datetime import datetime
from collections import defaultdict

class ReplaySyncValidator:
    def __init__(self, replay_dir="replay_archive", anchor_dir="anchor_hits"):
        self.replay_dir = replay_dir
        self.anchor_dir = anchor_dir
        self.validation_log = []

    def load_json(self, path):
        try:
            with open(path, 'r') as f:
                return json.load(f)
        except Exception as e:
            print(f"Failed to load {path}: {e}")
            return {}

    def validate(self):
        anchor_hits = self.load_all_anchor_hits()
        for session_file in os.listdir(self.replay_dir):
            session_path = os.path.join(self.replay_dir, session_file)
            session_data = self.load_json(session_path)

            session_id = session_data.get("session_id", session_file)
            anchor_ids = session_data.get("anchors_triggered", [])

            for anchor_id in anchor_ids:
                if anchor_id not in anchor_hits:
                    self.validation_log.append((session_id, anchor_id, "MISSING"))
                else:
                    self.validation_log.append((session_id, anchor_id, "FOUND"))

        return self.validation_log

    def load_all_anchor_hits(self):
        hit_map = defaultdict(dict)
        for file in os.listdir(self.anchor_dir):
            file_path = os.path.join(self.anchor_dir, file)
            anchor_data = self.load_json(file_path)
            for anchor in anchor_data.get("anchors", []):
                hit_map[anchor.get("anchor_id", "unknown")] = anchor
        return hit_map

    def print_report(self):
        print("\n=== Replay Sync Validation Report ===")
        for session_id, anchor_id, status in self.validation_log:
            print(f"Session {session_id} -> Anchor {anchor_id}: {status}")
        print("=== End of Report ===\n")

if __name__ == "__main__":
    validator = ReplaySyncValidator()
    validator.validate()
    validator.print_report()

def validate_and_sync_replay(session_data, replay_config):
    """
    Validate whether a session meets criteria for forensic replay, and sync if so.

    Args:
        session_data (dict): The full session dictionary including telemetry and trust info.
        replay_config (dict): Replay configuration such as trigger thresholds.

    Returns:
        bool: True if replay sync was triggered, False otherwise.
    """
    trust_score = session_data.get("trust_score", 100)
    triggers = replay_config.get("triggers", {})
    threshold = triggers.get("trust_threshold", 60)

    if trust_score <= threshold:
        print(f"[Replay Sync] Trust score {trust_score} below threshold {threshold}. Sync triggered.")
        # Simulate syncing replay to external forensic archive
        return True

    print(f"[Replay Sync] Trust score {trust_score} above threshold. No sync.")
    return False
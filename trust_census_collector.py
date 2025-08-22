import json
import random
import time
import subprocess
import sys
from datetime import datetime

def generate_trust_snapshot():
    return {
        "hostname": f"host-{random.randint(1000, 9999)}",
        "timestamp": datetime.utcnow().isoformat(),
        "trust_score": round(random.uniform(0, 1), 4),
        "trust_level": random.choice(["High", "Medium", "Low"]),
        "decay_factors": {
            "cpu_spike": random.choice([True, False]),
            "tag_privilege_escalation": random.choice([True, False]),
            "multiple_logins": random.choice([True, False])
        },
        "ontology_tags": [
            "lolbin_usage",
            "off_hours_login",
            "file_drop_exec"
        ],
        "escalation_flags": {
            "mode_switch_triggered": random.choice([True, False]),
            "replay_required": random.choice([True, False])
        },
        "session_replay_id": f"replay-{random.randint(100000, 999999)}"
    }

def write_trust_census(path: str):
    snapshot = generate_trust_snapshot()
    with open(path, "w") as f:
        json.dump(snapshot, f, indent=4)
    print(f"[✓] Trust snapshot written to {path}")
    return path

def sign_and_export(snapshot_path):
    try:
        subprocess.run(["python3", "intel_pack_signer.py", snapshot_path], check=True)
        subprocess.run(["python3", "vault_exporter.py", snapshot_path], check=True)
        print("[✓] Snapshot signed and exported to vault.")
    except subprocess.CalledProcessError as e:
        print(f"[!] Signing/export failed: {e}")

if __name__ == "__main__":
    if len(sys.argv) == 2:
        output_path = sys.argv[1]
    else:
        output_path = "/mnt/data/trust_census_snapshot.json"

    path = write_trust_census(output_path)
    sign_and_export(path)

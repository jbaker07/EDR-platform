import os
import json
import hashlib
import time
from datetime import datetime

# Path where individual endpoint fingerprints are dropped
FINGERPRINT_DIR = "trust_fingerprints/"
FLEET_LOG = "fleet_fingerprint_log.json"
os.makedirs(FINGERPRINT_DIR, exist_ok=True)

# Central fleet-wide map
fleet_fingerprint_db = {}

def hash_behavior_profile(profile_dict):
    """Creates a deterministic hash of a behavior fingerprint"""
    profile_str = json.dumps(profile_dict, sort_keys=True)
    return hashlib.sha256(profile_str.encode()).hexdigest()

def load_fingerprints():
    """Load new or updated endpoint-level fingerprint files"""
    global fleet_fingerprint_db

    for fname in os.listdir(FINGERPRINT_DIR):
        if fname.endswith(".json"):
            path = os.path.join(FINGERPRINT_DIR, fname)
            with open(path, "r") as f:
                profile = json.load(f)
                hash_id = hash_behavior_profile(profile)
                endpoint = profile.get("endpoint_id")

                if endpoint:
                    fleet_fingerprint_db[endpoint] = {
                        "fingerprint_hash": hash_id,
                        "behavior_profile": profile,
                        "last_seen": datetime.utcnow().isoformat()
                    }

def detect_cluster_anomalies():
    """Find matching fingerprints across distinct endpoints"""
    clusters = {}
    for ep, data in fleet_fingerprint_db.items():
        key = data["fingerprint_hash"]
        if key not in clusters:
            clusters[key] = []
        clusters[key].append(ep)

    alerts = []
    for hash_id, endpoints in clusters.items():
        if len(endpoints) > 3:
            alerts.append({
                "pattern_id": hash_id,
                "affected_endpoints": endpoints,
                "timestamp": datetime.utcnow().isoformat(),
                "reason": "Matching fingerprint detected across multiple nodes"
            })
    return alerts

def save_fleet_log(alerts):
    with open(FLEET_LOG, "w") as f:
        json.dump({
            "timestamp": datetime.utcnow().isoformat(),
            "alerts": alerts,
            "fleet_size": len(fleet_fingerprint_db)
        }, f, indent=2)

def track_distributed_fingerprints():
    print("[🔎] Scanning endpoint behavior fingerprints...")
    load_fingerprints()
    alerts = detect_cluster_anomalies()
    save_fleet_log(alerts)
    print(f"[✔] Scan complete. {len(alerts)} alert(s) written to {FLEET_LOG}")

if __name__ == "__main__":
    while True:
        track_distributed_fingerprints()
        time.sleep(300)  # every 5 minutes

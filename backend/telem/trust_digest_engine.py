
import hashlib
import json

def generate_behavior_digest(entry):
    keys = [
        entry.get("process_name", ""),
        entry.get("cmdline", ""),
        entry.get("user", ""),
        str(entry.get("entropy", "")),
        str(entry.get("num_children", "")),
        str(entry.get("network_volume", ""))
    ]
    raw = "|".join(keys).lower().strip()
    return hashlib.sha256(raw.encode()).hexdigest()

def build_digest_log(telemetry_stream):
    digest_map = {}
    for line in telemetry_stream:
        entry = json.loads(line)
        digest = generate_behavior_digest(entry)
        if digest not in digest_map:
            digest_map[digest] = {
                "first_seen": entry.get("timestamp"),
                "tags": entry.get("tags", []),
                "trust_mode": entry.get("mode", "minimal"),
                "count": 1
            }
        else:
            digest_map[digest]["count"] += 1
    return digest_map

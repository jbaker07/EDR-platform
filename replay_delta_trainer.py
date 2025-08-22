import os
import json
from datetime import datetime
from collections import defaultdict

REPLAY_DIR = "/opt/replays"
DELTA_LOG = "/opt/replays/replay_delta_findings.json"
RETRAIN_QUEUE = "/opt/queues/retrain_queue.json"
SUGGESTED_TAGS = "/opt/suggestions/new_tag_candidates.json"
DASHBOARD_OUTPUT = "/opt/reports/trust_dwell_metrics.json"

def load_json(path):
    with open(path, "r") as f:
        return json.load(f)

def save_json(data, path):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump(data, f, indent=2)

def analyze_trust_decay_vs_alert(session):
    trust_curve = session.get("trust_curve", [])
    alerts = session.get("alerts", [])
    session_id = session.get("session_id", "unknown")

    decay_findings = []
    tag_counter = defaultdict(int)

    for alert in alerts:
        alert_time = alert.get("timestamp")
        decay_points = [pt for pt in trust_curve if pt["score"] < 50 and pt["timestamp"] < alert_time]

        if decay_points:
            decay_point = decay_points[0]
            delay = alert_time - decay_point["timestamp"]
            tags = alert.get("tags", [])

            decay_findings.append({
                "session_id": session_id,
                "should_have_flagged_at": decay_point["timestamp"],
                "detected_at": alert_time,
                "detection_delay": delay,
                "original_trust_score": decay_point["score"],
                "affected_tags": tags
            })

            for tag in tags:
                tag_counter[tag] += 1

    return decay_findings, tag_counter

def run_replay_delta_training():
    delta_results = []
    all_tag_counts = defaultdict(int)
    retrain_samples = []
    new_tag_candidates = set()
    dashboard_data = []

    for fname in os.listdir(REPLAY_DIR):
        if not fname.endswith(".json"):
            continue

        session_path = os.path.join(REPLAY_DIR, fname)
        session = load_json(session_path)

        decay_results, tag_counts = analyze_trust_decay_vs_alert(session)
        delta_results.extend(decay_results)

        retrain_samples.append(session) if decay_results else None

        for tag, count in tag_counts.items():
            all_tag_counts[tag] += count
            if count >= 2:  # configurable threshold
                new_tag_candidates.add(tag)

        for result in decay_results:
            dashboard_data.append({
                "session_id": result["session_id"],
                "delay": result["detection_delay"],
                "affected_tags": result["affected_tags"],
                "timestamp": result["detected_at"]
            })

    save_json(delta_results, DELTA_LOG)
    save_json([s["session_id"] for s in retrain_samples], RETRAIN_QUEUE)
    save_json(list(new_tag_candidates), SUGGESTED_TAGS)
    save_json(dashboard_data, DASHBOARD_OUTPUT)

    print(f"[{datetime.now()}] ✅ Delta training completed: {len(delta_results)} findings.")
    print(f"→ {len(retrain_samples)} sessions queued for retraining")
    print(f"→ {len(new_tag_candidates)} new tags suggested")

if __name__ == "__main__":
    run_replay_delta_training()

# trust_retrainer.py

import json
from datetime import datetime
from pathlib import Path

TRUST_HISTORY_PATH = Path("data/trust/trust_history.json")
RETRAIN_SIGNAL_PATH = Path("signals/retrain_trigger.json")

def check_for_drift(trust_data, threshold=0.2):
    recent_trust = [entry["score"] for entry in trust_data[-50:]]
    baseline = sum([entry["score"] for entry in trust_data[:100]]) / 100.0
    avg_recent = sum(recent_trust) / len(recent_trust)
    drift = baseline - avg_recent
    return drift > threshold, baseline, avg_recent

def trigger_retrain(baseline, current):
    signal = {
        "timestamp": datetime.utcnow().isoformat(),
        "reason": "Trust baseline drifted",
        "baseline_score": baseline,
        "current_score": current
    }
    RETRAIN_SIGNAL_PATH.parent.mkdir(parents=True, exist_ok=True)
    with open(RETRAIN_SIGNAL_PATH, "w") as f:
        json.dump(signal, f, indent=2)
    print(f"🚨 Retrain triggered: Baseline {baseline:.2f} → Current {current:.2f}")

def run_trust_retrainer():
    if not TRUST_HISTORY_PATH.exists():
        print("No trust history found.")
        return

    with open(TRUST_HISTORY_PATH, "r") as f:
        trust_data = json.load(f)

    if len(trust_data) < 150:
        print("Not enough data to evaluate drift.")
        return

    drift_detected, baseline, current = check_for_drift(trust_data)
    if drift_detected:
        trigger_retrain(baseline, current)
    else:
        print(f"✅ No retrain needed. Baseline: {baseline:.2f}, Current: {current:.2f}")

if __name__ == "__main__":
    run_trust_retrainer()

def retrain_if_needed(trust_history: list, current_score: float, threshold: float = 0.5) -> bool:
    """
    Determines whether the trust engine needs retraining based on historical trust scores.
    This can help in adapting to drift or evolving behaviors.
    """
    if not trust_history:
        return False

    decay_trend = sum(trust_history[-5:]) / min(5, len(trust_history))
    if current_score < threshold and decay_trend < threshold:
        print("Triggering retraining logic due to sustained low trust scores.")
        return True

    return False
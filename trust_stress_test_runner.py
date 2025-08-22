import os
import json
from datetime import datetime
from trust_engine_final import evaluate_trust_from_input
from trust_digest_engine import update_digest_log
from semantic_gap_analyzer import flag_missing_tags

REPLAY_DIR = "replay/generated_attack_sims/"
FAILURE_LOG = "stress_test_output/failures.json"
SUMMARY_LOG = "stress_test_output/summary_report.json"

os.makedirs("stress_test_output", exist_ok=True)

def load_replays():
    return [
        os.path.join(REPLAY_DIR, f)
        for f in os.listdir(REPLAY_DIR)
        if f.endswith(".json")
    ]

def run_stress_test_on_replay(replay_path):
    with open(replay_path, "r") as f:
        replay_data = json.load(f)

    failures = []
    trust_curve = []
    current_score = 1.0

    for step in replay_data["steps"]:
        mock_input = {
            "process_name": step["mock_process"],
            "command_line": step["mock_command"],
            "semantic_tags": [step["semantic_tag"]],
            "timestamp": step["timestamp"]
        }

        trust_result = evaluate_trust_from_input(mock_input)
        current_score = trust_result["trust_score"]
        trust_curve.append(current_score)

        # Optional enhancement: record digest
        update_digest_log(mock_input)

        # Fail condition: trust doesn’t decay or increases
        if trust_result["trust_score"] > current_score or trust_result["trust_score"] > 0.9:
            failures.append({
                "step": step,
                "reason": "Trust score remained high or increased during adversarial chain",
                "trust_score": trust_result["trust_score"]
            })

    return {
        "attack_id": replay_data["attack_id"],
        "total_steps": len(replay_data["steps"]),
        "failures": failures,
        "final_trust_score": current_score,
        "trust_curve": trust_curve,
        "timestamp": datetime.utcnow().isoformat()
    }

def generate_summary(results):
    total = len(results)
    failed = sum(1 for r in results if r["failures"])
    summary = {
        "total_runs": total,
        "failures_detected": failed,
        "pass_rate": f"{(total - failed) / total * 100:.2f}%",
        "timestamp": datetime.utcnow().isoformat()
    }
    with open(SUMMARY_LOG, "w") as f:
        json.dump(summary, f, indent=2)

def run_batch_stress_test():
    results = []
    for replay_path in load_replays():
        print(f"Running stress test on: {replay_path}")
        result = run_stress_test_on_replay(replay_path)
        results.append(result)

        if result["failures"]:
            with open(FAILURE_LOG, "a") as f:
                json.dump(result, f, indent=2)
                f.write(",\n")

            # Optional enhancement: flag semantic gaps
            flag_missing_tags(result)

    generate_summary(results)
    print("[✔] Trust stress test batch complete.")

if __name__ == "__main__":
    run_batch_stress_test()

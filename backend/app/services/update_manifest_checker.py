import os
import json
from typing import Any, Dict, Tuple

def load_json(file_path: str) -> Dict[str, Any]:
    try:
        with open(file_path, "r") as f:
            return json.load(f)
    except Exception as e:
        return {}

def diff_dicts(baseline: Dict[str, Any], current: Dict[str, Any]) -> Dict[str, Any]:
    added = {k: v for k, v in current.items() if k not in baseline}
    removed = {k: v for k, v in baseline.items() if k not in current}
    changed = {
        k: {"from": baseline[k], "to": current[k]}
        for k in baseline.keys() & current.keys()
        if baseline[k] != current[k]
    }
    return {"added": added, "removed": removed, "changed": changed}

def compare_json_files(baseline_dir: str, current_dir: str) -> Dict[str, Dict[str, Any]]:
    diff_report = {}

    baseline_files = {f for f in os.listdir(baseline_dir) if f.endswith(".json")}
    current_files = {f for f in os.listdir(current_dir) if f.endswith(".json")}
    all_files = baseline_files | current_files

    for filename in all_files:
        baseline_file = os.path.join(baseline_dir, filename)
        current_file = os.path.join(current_dir, filename)

        baseline_data = load_json(baseline_file) if os.path.exists(baseline_file) else {}
        current_data = load_json(current_file) if os.path.exists(current_file) else {}

        file_diff = diff_dicts(baseline_data, current_data)
        if any(file_diff.values()):
            diff_report[filename] = file_diff

    return diff_report

def save_diff_report(report: Dict[str, Any], output_path: str):
    with open(output_path, "w") as f:
        json.dump(report, f, indent=2)

if __name__ == "__main__":
    baseline_folder = "baseline_configs"
    current_folder = "current_configs"
    output_file = "manifest_diff_report.json"

    diff = compare_json_files(baseline_folder, current_folder)
    save_diff_report(diff, output_file)
    print(f"[✓] Manifest diff written to {output_file}")

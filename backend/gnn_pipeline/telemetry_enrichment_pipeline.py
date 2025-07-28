import json
import os
import time
from datetime import datetime

# === Load Ontology Files ===
BASE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), '../../'))
ONTOLOGY_PATH = os.path.join(BASE_DIR, "ontology/core_ontology.json")
TAGS_PATH = os.path.join(BASE_DIR, "ontology/semantic_tags.json")
RULES_PATH = os.path.join(BASE_DIR, "ontology/detection_rules.json")
INPUT_PATH = os.path.join(BASE_DIR, "graph_output/sample_telemetry.json")
OUTPUT_PATH = os.path.join(BASE_DIR, "graph_output/enriched_telemetry.json")


def load_json(path):
    with open(path, 'r') as f:
        return json.load(f)


def normalize_timestamp(ts_str):
    try:
        return int(datetime.strptime(ts_str, "%Y-%m-%d %H:%M:%S").timestamp())
    except Exception:
        return int(time.time())


def match_tags(entry, tag_defs):
    matched = []
    for tag, conditions in tag_defs.items():
        for cond in conditions.get("match_contains", []):
            for field in ["exe", "cmdline"]:
                if cond.lower() in entry.get(field, "").lower():
                    matched.append(tag)
                    break
    return list(set(matched))


def evaluate_rules(entry, rules):
    hits = []
    for rule in rules:
        if rule.get("field") in entry:
            val = entry[rule["field"]]
            if rule.get("contains") and rule["contains"] in val:
                hits.append(rule["id"])
            elif rule.get("equals") and rule["equals"] == val:
                hits.append(rule["id"])
    return hits


def calculate_risk_score(tags, rules):
    score = 0
    score += len(tags) * 5
    score += len(rules) * 10
    return min(score, 100)


def get_risk_level(score):
    if score >= 90:
        return "Critical"
    elif score >= 70:
        return "High"
    elif score >= 40:
        return "Medium"
    else:
        return "Low"


def enrich_telemetry():
    tag_defs = load_json(TAGS_PATH)
    rules_raw = load_json(RULES_PATH)
    rules = []
    for rule in rules_raw.values():
        if isinstance(rule, list):
            rules.extend([r for r in rule if isinstance(r, dict)])
        elif isinstance(rule, dict):
            rules.append(rule)

    entries = load_json(INPUT_PATH)

    enriched = []
    for entry in entries:
        entry["timestamp"] = normalize_timestamp(entry.get("timestamp", ""))
        entry["tags"] = match_tags(entry, tag_defs)
        entry["rule_hits"] = evaluate_rules(entry, rules)
        entry["risk_score"] = calculate_risk_score(entry["tags"], entry["rule_hits"])
        entry["risk_level"] = get_risk_level(entry["risk_score"])
        entry["is_graph_node"] = True if entry["tags"] or entry["rule_hits"] else False
        entry["linked_tags"] = entry["tags"]
        enriched.append(entry)

    with open(OUTPUT_PATH, 'w') as f:
        json.dump(enriched, f, indent=2)

    print(f"✅ Enriched {len(enriched)} entries and saved to {OUTPUT_PATH}")


if __name__ == "__main__":
    enrich_telemetry()

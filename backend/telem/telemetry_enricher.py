
import json
import argparse
from pathlib import Path

def load_json_file(path):
    with open(path, 'r') as f:
        return json.load(f)

def match_tags(entry, tag_data, fallback_data):
    tags = []
    reasons = []
    mode_scores = {'minimal': 0, 'forensic': 0, 'cloud': 0}
    for tag_name, tag in tag_data.get("tags", {}).items():
        if all(k in entry for k in tag["mapped_fields"]):
            if "entropy" in entry and "entropy" in tag["mapped_fields"] and entry["entropy"] < 0.9:
                continue
            tags.append(tag_name)
            reasons.append(tag["trust_reason"])
            mode_scores[tag["mode"]] += 1

    # Fallback if nothing matched
    if not tags:
        for tag_name, tag in fallback_data.get("tags", {}).items():
            if all(k in entry for k in tag["mapped_fields"]):
                tags.append(tag_name)
                reasons.append(tag["trust_reason"])
                mode_scores[tag["mode"]] += 1

    mode = max(mode_scores, key=mode_scores.get) if mode_scores else "minimal"
    return tags, reasons, mode

def match_mitre(entry, mitre_data):
    ttp_matches = []
    process = entry.get("process_name", "").lower()
    cmd = entry.get("cmdline", "").lower()

    if process in mitre_data.get("names", {}):
        ttp_matches.append(mitre_data["names"][process])

    for flag, tid in mitre_data.get("flags", {}).items():
        if flag.lower() in cmd:
            ttp_matches.append(tid)

    return list(set(ttp_matches)) if ttp_matches else ["unclassified"]

def enrich_telemetry(input_path, output_path, tag_path, mitre_path, fallback_path):
    tags = load_json_file(tag_path)
    mitre = load_json_file(mitre_path)
    fallback = load_json_file(fallback_path)

    enriched = []
    with open(input_path, 'r') as infile:
        for line in infile:
            try:
                entry = json.loads(line)
                matched_tags, reasons, mode = match_tags(entry, tags, fallback)
                mitres = match_mitre(entry, mitre)
                entry.update({
                    "tags": matched_tags,
                    "trust_reason": "; ".join(reasons) if reasons else "unlabeled",
                    "mode": mode,
                    "mitre_techniques": mitres
                })
                enriched.append(entry)
            except Exception as e:
                print(f"Error parsing line: {e}")

    with open(output_path, 'w') as out:
        for e in enriched:
            out.write(json.dumps(e) + '\n')

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Enrich raw telemetry with semantic tags and MITRE TTPs")
    parser.add_argument("--input", required=True, help="Path to input .jsonl telemetry file")
    parser.add_argument("--output", required=True, help="Path to output .jsonl enriched file")
    parser.add_argument("--tags", required=True, help="Path to semantic_tags.json")
    parser.add_argument("--mitre", required=True, help="Path to threat_patterns.json")
    parser.add_argument("--fallback", required=True, help="Path to unknown_risk_profiles.json")
    args = parser.parse_args()

    enrich_telemetry(args.input, args.output, args.tags, args.mitre, args.fallback)
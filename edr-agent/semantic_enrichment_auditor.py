import json
from pathlib import Path

SEMANTIC_TAGS_FILE = Path("ontology/semantic_tags.json")
REPLAY_INDEX_FILE = Path("replay_index/metadata_index.json")
AUDIT_LOG_FILE = Path("replay_index/semantic_audit_log.json")


def load_semantic_tags():
    with open(SEMANTIC_TAGS_FILE, "r") as f:
        return json.load(f)


def load_replay_index():
    with open(REPLAY_INDEX_FILE, "r") as f:
        return json.load(f)


def audit_semantic_coverage(replays, tag_definitions):
    missing_tag_reports = []

    for replay in replays:
        tags_seen = set(replay.get("tags", []))
        trust_delta = replay.get("trust_delta", 0)
        log_entry = {
            "id": replay.get("id"),
            "missing_tags": [],
            "trust_delta": trust_delta,
            "flagged_for_review": False
        }

        for tag, definition in tag_definitions.items():
            triggers = definition.get("trigger_fields", [])
            risk = definition.get("risk_weight", 0)

            if risk >= 7 and any(field in replay for field in triggers):
                if tag not in tags_seen:
                    log_entry["missing_tags"].append(tag)

        if log_entry["missing_tags"]:
            log_entry["flagged_for_review"] = True
            missing_tag_reports.append(log_entry)

    return missing_tag_reports


def run_audit():
    tag_definitions = load_semantic_tags()
    replays = load_replay_index()
    audit_report = audit_semantic_coverage(replays, tag_definitions)

    with open(AUDIT_LOG_FILE, "w") as out_file:
        json.dump(audit_report, out_file, indent=4)

    print(f"[✓] Semantic audit complete: {len(audit_report)} replays flagged with missing tags.")


if __name__ == "__main__":
    run_audit()

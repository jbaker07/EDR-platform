import json
from collections import defaultdict
from typing import List, Dict

REPLAY_PATH = "replays/confirmed_sessions.json"
RULE_HITS_PATH = "reports/detection_rule_hits.json"
TAG_MAP_PATH = "config/semantic_tags.json"
OUTPUT_PATH = "reports/semantic_gaps.json"

MIN_TAG_IMPORTANCE = 0.3  # Tags below this weight are considered underperforming

def load_json(path: str) -> Dict:
    with open(path, "r") as f:
        return json.load(f)

def find_gaps(replays: List[Dict], rules: Dict, tag_map: Dict) -> Dict:
    missing_tags = []
    underweighted_tags = defaultdict(int)
    sessions_missing_detection = []

    for session in replays:
        tags = set(session.get("tags", []))
        gnn_score = session.get("gnn_score", 0)
        trust = session.get("trust_score", 0)
        sid = session.get("session_id", "unknown")

        rule_hit = rules.get(sid, {}).get("rules_triggered", [])

        # Check for high-score but no rule triggers
        if gnn_score > 0.7 and trust < 0.4 and not rule_hit:
            sessions_missing_detection.append(sid)

        # Check for tag weight gaps
        for tag in tags:
            tag_info = tag_map.get(tag, {})
            weight = tag_info.get("risk_weight", 0)
            if weight < MIN_TAG_IMPORTANCE:
                underweighted_tags[tag] += 1

        # Identify suspicious sessions with zero or one tag
        if len(tags) <= 1 and gnn_score > 0.6:
            missing_tags.append({
                "session_id": sid,
                "score": gnn_score,
                "existing_tags": list(tags)
            })

    return {
        "sessions_missing_tags": missing_tags,
        "sessions_missing_detection": sessions_missing_detection,
        "underweighted_tags": sorted(underweighted_tags.items(), key=lambda x: -x[1])
    }

def run_gap_analysis():
    replays = load_json(REPLAY_PATH)
    rule_hits = load_json(RULE_HITS_PATH)
    tag_map = load_json(TAG_MAP_PATH)

    gap_report = find_gaps(replays, rule_hits, tag_map)

    print(f"[📊] Found {len(gap_report['sessions_missing_tags'])} sessions with low tag coverage.")
    print(f"[⚠️] Found {len(gap_report['sessions_missing_detection'])} high-risk sessions with no rule hits.")
    print(f"[📉] Underweighted tags identified: {len(gap_report['underweighted_tags'])}")

    with open(OUTPUT_PATH, "w") as f:
        json.dump(gap_report, f, indent=4)

if __name__ == "__main__":
    run_gap_analysis()

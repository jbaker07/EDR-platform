import json
from typing import List, Dict
from datetime import datetime, timedelta
import hashlib

SESSION_LOG = "replays/confirmed_sessions.json"
ESCAPE_REPORT = "reports/trust_escape_attempts.json"
SEMANTIC_TAG_OUTPUT = "tags/generated_escape_tags.json"

SUSPECT_BEHAVIORS = [
    "benign_padding",
    "sleep_interval_injection",
    "scheduled_fragmentation",
    "whitelisted_tool_abuse",
    "decoy_task_flooding",
    "temporal_anomaly",
    "gnn_discrepancy"
]

def hash_session(session: Dict) -> str:
    return hashlib.sha256(json.dumps(session, sort_keys=True).encode()).hexdigest()

def temporal_anomaly_score(timestamp: str, recent_sessions: List[Dict]) -> bool:
    try:
        current_time = datetime.fromisoformat(timestamp)
        times = [datetime.fromisoformat(s["timestamp"]) for s in recent_sessions if "timestamp" in s]
        if not times:
            return False
        avg_time = sum([(current_time - t).total_seconds() for t in times], 0.0) / len(times)
        return abs(avg_time) > 600  # >10 min deviation
    except Exception:
        return False

def gnn_discrepancy_check(session: Dict) -> bool:
    return session.get("gnn_score", 1.0) - session.get("risk_score", 0.0) > 0.6

def looks_like_escape_pattern(session: Dict, history: List[Dict]) -> List[str]:
    flags = []

    if session.get("duration_seconds", 0) < 60 and session.get("risk_score", 0) < 0.2:
        recent_similar = [s for s in history[-10:] if s.get("process_name") == session.get("process_name")]
        if len(recent_similar) >= 5:
            flags.append("benign_padding")

    if "sleep" in session.get("command_line", "").lower():
        flags.append("sleep_interval_injection")

    if session.get("parent_session_id") and session["parent_session_id"] not in [
        s["session_id"] for s in history
    ]:
        flags.append("scheduled_fragmentation")

    if session.get("process_name") in ["powershell", "wmic", "certutil"] and session.get("risk_score", 0) < 0.4:
        flags.append("whitelisted_tool_abuse")

    if len(history) >= 3:
        prev = history[-3:]
        pattern = [s["risk_score"] for s in prev] + [session["risk_score"]]
        if pattern == sorted(pattern[:2]) + sorted(pattern[2:], reverse=True):
            flags.append("decoy_task_flooding")

    if temporal_anomaly_score(session.get("timestamp", ""), history[-10:]):
        flags.append("temporal_anomaly")

    if gnn_discrepancy_check(session):
        flags.append("gnn_discrepancy")

    return flags

def run_escape_detection():
    with open(SESSION_LOG, "r") as f:
        sessions = json.load(f)

    escape_attempts = []
    generated_tags = {}
    history = []

    for session in sessions:
        flags = looks_like_escape_pattern(session, history)
        if flags:
            tag_id = f"escape.{hash_session(session)[:10]}"
            escape_attempts.append({
                "session_id": session["session_id"],
                "flags": flags,
                "timestamp": session.get("timestamp"),
                "summary": f"Trust evasion: {', '.join(flags)}",
                "semantic_tag": tag_id
            })
            generated_tags[tag_id] = {
                "tag": tag_id,
                "description": f"Escape attempt: {', '.join(flags)}",
                "confidence": 0.95,
                "source": "trust_escape_detector",
                "related_flags": flags
            }
        history.append(session)

    with open(ESCAPE_REPORT, "w") as f:
        json.dump(escape_attempts, f, indent=4)

    with open(SEMANTIC_TAG_OUTPUT, "w") as f:
        json.dump(generated_tags, f, indent=4)

    print(f"[🚨] Escape attempts: {len(escape_attempts)} | Tags Generated: {len(generated_tags)}")

if __name__ == "__main__":
    run_escape_detection()

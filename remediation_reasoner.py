import json
from datetime import datetime
from typing import List, Dict, Optional

from replay_engine.replay_utils import load_replay_by_id
from ontology.semantic_tags import load_tag_metadata
from shap_explainer import generate_shap_summary
from trust_engine.trust_digest_engine import load_fingerprint
from trust_engine.adaptive_threshold_engine import get_role_thresholds


def reason_about_remediation(session_id: str, role: str = "default") -> Dict:
    replay = load_replay_by_id(session_id)
    if not replay:
        return {"error": f"Replay session {session_id} not found."}

    tag_meta = load_tag_metadata()
    shap_summary = generate_shap_summary(replay)
    fingerprint = load_fingerprint(session_id)
    thresholds = get_role_thresholds(role)

    score_drop = replay.get("score_drop", 0.0)
    final_score = replay.get("final_score", 1.0)
    decay_source = replay.get("decay_causes", [])
    tags = replay.get("semantic_tags", [])

    critical_tags = [tag for tag in tags if tag_meta.get(tag, {}).get("severity", 0) > 7]

    reasoning = {
        "session_id": session_id,
        "timestamp": datetime.utcnow().isoformat(),
        "final_trust_score": final_score,
        "score_drop": score_drop,
        "primary_causes": decay_source,
        "critical_tags": critical_tags,
        "recommended_actions": [],
        "why": "",
        "how_to_fix": "",
        "recurrence_prevention": ""
    }

    if final_score < thresholds["escalation"]:
        reasoning["why"] = (
            f"Trust score dropped to {final_score:.2f}, below the escalation threshold "
            f"for role '{role}' ({thresholds['escalation']:.2f})."
        )

        if "privilege_escalation" in tags:
            reasoning["recommended_actions"].append("Force password reset and revalidate user sessions")
            reasoning["how_to_fix"] = "Revoke elevated privileges and audit system calls"
            reasoning["recurrence_prevention"] = "Monitor future privilege assignment with decay penalties"

        if "data_exfil" in tags:
            reasoning["recommended_actions"].append("Audit outbound traffic and block unknown endpoints")
            reasoning["how_to_fix"] = "Kill suspected processes or disable exfil vector"
            reasoning["recurrence_prevention"] = "Enable anomaly-based alerting on data transfer volumes"

        if not reasoning["recommended_actions"]:
            reasoning["recommended_actions"].append("Perform full session forensic replay and initiate containment")

    return reasoning


# Optional JSON output for integration with SOC panel
def explain_and_export(session_id: str, outfile: Optional[str] = None):
    result = reason_about_remediation(session_id)
    if outfile:
        with open(outfile, "w") as f:
            json.dump(result, f, indent=2)
    return result


if __name__ == "__main__":
    print(json.dumps(reason_about_remediation("session_abc123"), indent=2))

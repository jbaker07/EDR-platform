import os
import json
import uuid
from datetime import datetime
from typing import Dict, Any

REPLAY_QUEUE_PATH = os.path.join(os.path.dirname(__file__), "queues", "replay_queue.jsonl")
os.makedirs(os.path.dirname(REPLAY_QUEUE_PATH), exist_ok=True)

def write_to_replay_queue(event: Dict[str, Any], reason: str = "escalation_triggered") -> None:
    """
    Appends a structured replay ticket to the replay queue.
    """
    
    replay_ticket = {
        "ticket_id": str(uuid.uuid4()),
        "timestamp": datetime.utcnow().isoformat(),
        "hostname": event.get("hostname", "unknown"),
        "session_id": event.get("session_id", str(uuid.uuid4())),
        "trigger_reason": reason,
        "replay_data": event
    }

    with open(REPLAY_QUEUE_PATH, "a") as f:
        f.write(json.dumps(replay_ticket) + "\n")

import json
from typing import Dict, Any, List


def load_anchor_definitions(file_path: str) -> List[Dict[str, Any]]:
    with open(file_path, 'r') as f:
        return json.load(f)


def evaluate_anchor_impact(anchor: Dict[str, Any], telemetry: Dict[str, Any]) -> float:
    confidence = telemetry.get("confidence", 0)
    risk_score = telemetry.get("risk_score", 0)
    category = anchor.get("category", "")
    decay_multiplier = {
        "Privilege Escalation": 1.25,
        "Lateral Movement": 1.1,
        "Credential Abuse": 1.4,
        "Living off the Land": 1.0,
        "Cloud Misuse": 1.5,
        "Insider Threat": 1.2,
        "Recon / Enumeration": 0.75,
        "Anomalous Sequences": 1.35
    }.get(category, 1.0)

    if (confidence >= anchor.get("confidence_threshold", 0.8) and
            risk_score >= anchor.get("min_risk", 70)):
        base_decay = (confidence * risk_score) / 100
        return base_decay * decay_multiplier
    return 0.0


def compute_total_decay(telemetry: Dict[str, Any], anchor_definitions: List[Dict[str, Any]]) -> float:
    decay_score = 0.0
    for anchor in anchor_definitions:
        decay_score += evaluate_anchor_impact(anchor, telemetry)
    return round(decay_score, 2)


def assess_trust_decay(telemetry: Dict[str, Any], anchor_path: str) -> Dict[str, Any]:
    anchor_definitions = load_anchor_definitions(anchor_path)
    decay_score = compute_total_decay(telemetry, anchor_definitions)

    updated_trust = max(0.0, telemetry.get("initial_trust", 100) - decay_score)
    result = {
        "session_id": telemetry.get("session_id"),
        "initial_trust": telemetry.get("initial_trust", 100),
        "decay_score": decay_score,
        "final_trust": updated_trust,
        "triggered": updated_trust < 40,  # Flag for potential escalation
    }
    return result

import math
import time

def apply_decay(current_trust_score: float, last_event_timestamp: float, category: str = None) -> dict:
    """
    Applies trust decay based on time elapsed since last high-confidence event and optional behavior category.

    Args:
        current_trust_score (float): The current trust score for the process/session/user.
        last_event_timestamp (float): Unix timestamp of the last meaningful event.
        category (str): Optional behavioral category (e.g., 'privilege_escalation', 'insider_threat').

    Returns:
        dict: {
            'decayed_trust': float,
            'decay_factor': float,
            'justification': str
        }
    """

    now = time.time()
    elapsed_hours = (now - last_event_timestamp) / 3600.0

    decay_rate_map = {
        'privilege_escalation': 0.15,
        'lateral_movement': 0.12,
        'insider_threat': 0.18,
        'living_off_the_land': 0.10,
        'credential_abuse': 0.20,
        'cloud_misuse': 0.16,
        'anomalous_sequences': 0.17,
        'recon': 0.14,
        None: 0.13
    }

    decay_rate = decay_rate_map.get(category, 0.13)

    decay_factor = math.exp(-decay_rate * elapsed_hours)
    decayed_trust = max(0, min(100, current_trust_score * decay_factor))

    return {
        "decayed_trust": round(decayed_trust, 2),
        "decay_factor": round(decay_factor, 3),
        "justification": f"Applied exponential decay with rate {decay_rate} over {round(elapsed_hours, 2)} hours"
    }

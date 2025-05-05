from typing import Optional
import numpy as np
from datetime import datetime, timedelta

from app.memory.memory_manager import memory_manager
from app.services.db import get_db_session
from app.models.agent import Agent

# Constants (tunable)
MAX_TRUST = 100.0
RISK_WEIGHT_MULTIPLIER = 1.25
ATTESTATION_PENALTY = 25.0
MISSING_HASH_PENALTY = 15.0
DECAY_WINDOW_SECONDS = 3600

def calculate_trust_score(hostname: str, window_seconds: int = DECAY_WINDOW_SECONDS) -> float:
    """
    Calculate a trust score (0–100) for an endpoint based on:
    - Recent behavioral risk (decay-weighted)
    - Agent attestation status
    - Binary and policy hash presence
    """
    trust_score = MAX_TRUST
    now = datetime.utcnow()

    # Step 1: Time-decayed risk analysis
    events = memory_manager.get_events_for_host(hostname, window_seconds)
    if events:
        decayed_risks = []

        for e in events:
            raw_risk = e.get("risk_score", 0)
            ts = e.get("timestamp")

            try:
                event_time = datetime.fromisoformat(ts) if isinstance(ts, str) else datetime.utcfromtimestamp(ts)
                age = (now - event_time).total_seconds()
            except Exception:
                age = 0  # Assume fresh if parsing fails

            decay = max(0.1, 1 - (age / window_seconds))  # Avoid full decay wipeout
            decayed_risks.append(raw_risk * decay)

        weighted_risk = np.mean(decayed_risks) * RISK_WEIGHT_MULTIPLIER
        trust_score -= weighted_risk

    # Step 2: Static agent metadata validation
    try:
        db = next(get_db_session())
        agent: Optional[Agent] = db.query(Agent).filter(Agent.hostname == hostname).first()
        if agent:
            if not agent.attestation_passed:
                trust_score -= ATTESTATION_PENALTY
            if not agent.binary_hash or not agent.policy_hash:
                trust_score -= MISSING_HASH_PENALTY
    except Exception as ex:
        print(f"[!] Trust Engine DB error: {ex}")

    # Step 3: Clamp trust score to [0, MAX_TRUST]
    return round(max(0.0, min(trust_score, MAX_TRUST)), 1)

from fastapi import APIRouter, Query
from datetime import datetime, timedelta

router = APIRouter()

# Simulated storage for MVP
fake_trust_history = {
    "host123": [
        {"timestamp": "2025-05-08T00:00:00Z", "trust_score": 92},
        {"timestamp": "2025-05-08T01:00:00Z", "trust_score": 88},
        {"timestamp": "2025-05-08T02:00:00Z", "trust_score": 79},
    ]
}

@router.get("/api/trust/history")
def get_trust_history(hostname: str = Query(...)):
    return fake_trust_history.get(hostname, [])

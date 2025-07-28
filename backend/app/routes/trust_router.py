from fastapi import APIRouter, HTTPException
from app.services.trust_engine_final import calculate_trust_score
from app.memory.memory_manager import memory_manager
from collections import defaultdict
from datetime import datetime

router = APIRouter()

@router.get("/host/{hostname}")
def get_trust_score(hostname: str):
    score = calculate_trust_score(hostname)
    if score is None:
        raise HTTPException(status_code=404, detail="Host not found")
    return {"hostname": hostname, "trust_score": score}

@router.get("/dashboard-collector")
def dashboard_trust_view():
    try:
        events = memory_manager.get_all_recent_events()
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Memory manager error: {str(e)}")

    trust_map = {}
    last_seen = defaultdict(float)

    for event in events:
        host = event.get("hostname", "")
        ts = event.get("timestamp", 0)
        if not host:
            continue
        last_seen[host] = max(last_seen[host], ts)
        if host not in trust_map:
            trust_map[host] = calculate_trust_score(host)

    scores = list(trust_map.values())
    avg_trust = round(sum(scores) / len(scores), 2) if scores else 0
    low_trust = [host for host, score in trust_map.items() if score < 60]

    return {
        "summary": {
            "total_endpoints": len(trust_map),
            "average_trust": avg_trust,
            "low_trust_hosts": low_trust,
            "last_seen": {k: datetime.utcfromtimestamp(v).isoformat() for k, v in last_seen.items()}
        },
        "scores": trust_map
    }

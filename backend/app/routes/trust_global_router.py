from fastapi import APIRouter
from app.memory.memory_manager import memory_manager
from app.services.trust_engine_final import calculate_trust_score

router = APIRouter()

@router.get("/trust/global")
def get_all_endpoint_trust():
    # One score per host that sent telemetry in the last 15 minutes. This used to
    # return eight hard-coded hosts, which the dashboard heatmap showed as real.
    hosts = sorted({e["hostname"] for e in memory_manager.get_all_recent_events() if e.get("hostname")})
    return [
        {"hostname": h, "trust_score": calculate_trust_score(h)["final_trust_score"]}
        for h in hosts
    ]

from fastapi import APIRouter
from app.memory.memory_manager import memory_manager
from app.services.gnn_explainer import get_gnn_trace

router = APIRouter()

@router.get("/api/explain/root_cause")
def get_root_cause(event_id: str):
    event = memory_manager.get_event_details(event_id)
    if not event:
        return {"error": "event not found"}

    trace = get_gnn_trace(event)
    if not trace or not trace.get("node"):
        return {"error": "trace unavailable"}

    return {
        "origin": trace["causal_parents"],
        "compromised": trace["node"],
        "neighbors": trace["related_nodes"]
    }

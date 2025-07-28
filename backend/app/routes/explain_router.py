from fastapi import APIRouter, Query
from app.memory.memory_manager import memory_manager
from app.explainers.shap_explainer import get_shap_explanation
from app.services.gnn_explainer import get_gnn_trace
from app.services.trust_gnn_engine import get_trust_shift_justification
from fastapi import APIRouter, Request

router = APIRouter()

@router.get("/api/explain/timeline_event")
def explain_event(event_id: str):
    # ✅ Correct way to fetch event
    event = memory_manager.get_event_details(event_id)
    if not event:
        return {"error": "event not found"}

    # ✅ Use each module
    shap_expl = get_shap_explanation(event)
    trust_expl = get_trust_shift_justification(event)
    gnn_expl = get_gnn_trace(event)

    return {
        "event_id": event_id,
        "shap_reasoning": shap_expl,
        "trust_score_reasoning": trust_expl,
        "gnn_context": gnn_expl,
    }
@router.post("/api/explain")
async def explain_fallback(request: Request):
    """
    Fallback route to prevent 404 if frontend still calls /api/explain directly.
    Ideally, frontend should use /api/explain/timeline_event or /api/explain/root_cause.
    """
    body = await request.json()
    return {
        "message": "This is a fallback /api/explain route. Update frontend to use the correct endpoint.",
        "received_payload": body
    }

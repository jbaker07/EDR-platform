from fastapi import APIRouter, HTTPException
from app.memory.memory_manager import memory_manager
from app.explainers.shap_explainer import get_shap_explanation
from app.services.trust_gnn_engine import get_trust_shift_justification
from app.services.gnn_explainer import get_gnn_trace

router = APIRouter()

# Simulated action store for MVP purposes
response_log = {}

@router.post("/api/response/action")
def respond_to_alert(alert_id: str, action: str = "isolate"):
    # 1. Get event/alert
    event = memory_manager.get_event_details(alert_id)
    if not event:
        raise HTTPException(status_code=404, detail="Alert not found.")

    # 2. Pull explainability
    shap_data = get_shap_explanation(event)
    trust_data = get_trust_shift_justification(event)
    gnn_data = get_gnn_trace(event)

    # 3. Justify the response
    reasons = []

    if shap_data["top_contributors"]:
        top = shap_data["top_contributors"][0]
        reasons.append(f"SHAP: '{top['feature']}' with impact {round(top['impact'], 2)}")

    if trust_data["delta"] < 0:
        reasons.append(f"Trust delta: {trust_data['delta']} due to " + ", ".join(trust_data["reasons"]))

    if gnn_data.get("causal_parents"):
        reasons.append(f"GNN: Causal parent(s): " + ", ".join(gnn_data["causal_parents"]))

    reason_text = f"Action '{action}' triggered due to: " + " | ".join(reasons)

    # 4. Simulate response action
    response_log[alert_id] = {
        "action": action,
        "reason": reason_text
    }

    return {
        "alert_id": alert_id,
        "action_taken": action,
        "justification": reason_text
    }




















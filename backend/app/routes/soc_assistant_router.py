from fastapi import APIRouter, Query
from app.memory.memory_manager import memory_manager
from app.explainers.shap_explainer import get_shap_explanation
from app.services.trust_gnn_engine import get_trust_shift_justification
from app.services.gnn_explainer import get_gnn_trace

router = APIRouter()

@router.get("/api/assistant/explain")
def soc_assistant_explain(event_id: str):
    event = memory_manager.get_event_details(event_id)
    if not event:
        return {"error": "event not found"}

    shap_data = get_shap_explanation(event)
    trust_data = get_trust_shift_justification(event)
    gnn_data = get_gnn_trace(event)

    top_feature = shap_data["top_contributors"][0] if shap_data["top_contributors"] else None
    summary = []

    if top_feature:
        summary.append(
            f"The top contributing factor was '{top_feature['feature']}' with an impact score of {round(top_feature['impact'], 2)}."
        )

    summary.append(f"The trust score changed by {trust_data['delta']} due to:")
    for reason in trust_data["reasons"]:
        summary.append(f"- {reason}")

    if gnn_data.get("node"):
        summary.append(
            f"GNN analysis shows node '{gnn_data['node']}' is connected to {len(gnn_data['related_nodes'])} nodes and has {len(gnn_data['causal_parents'])} causal parents."
        )

    return {
        "event_id": event_id,
        "natural_summary": " ".join(summary),
        "raw": {
            "shap": shap_data,
            "trust": trust_data,
            "gnn": gnn_data
        }
    }

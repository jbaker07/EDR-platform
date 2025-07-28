from fastapi import APIRouter, Query
from app.memory.memory_manager import memory_manager
from app.explainers.shap_explainer import get_shap_explanation
from app.services.trust_gnn_engine import get_trust_shift_justification
from app.services.gnn_explainer import get_gnn_trace

router = APIRouter()

@router.get("/api/remediation/suggest")
def suggest_remediation(event_id: str):
    event = memory_manager.get_event_details(event_id)
    if not event:
        return {"error": "event not found"}

    shap_data = get_shap_explanation(event)
    trust_data = get_trust_shift_justification(event)
    gnn_data = get_gnn_trace(event)

    actions = []

    # SHAP-Driven
    for feature in shap_data.get("top_contributors", []):
        if "memory" in feature["feature"]:
            actions.append("Isolate process due to memory anomaly.")
        if "cmdline" in feature["feature"]:
            actions.append("Investigate command line for obfuscation or LOLBins.")
        if "parent" in feature["feature"]:
            actions.append("Trace parent process for spawning malicious behavior.")

    # Trust triggers
    for trigger in trust_data.get("reasons", []):
        if "persistence" in trigger:
            actions.append("Scan for autoruns and registry persistence.")
        if "script" in trigger:
            actions.append("Quarantine and analyze suspect script.")

    # GNN
    if gnn_data.get("causal_parents"):
        actions.append("Trace GNN parent nodes and investigate chain of compromise.")
    if len(gnn_data.get("related_nodes", [])) > 3:
        actions.append("Clustered compromise: scan peer nodes.")

    return {
        "event_id": event_id,
        "recommendations": actions
    }

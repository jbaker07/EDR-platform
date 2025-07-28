import json
import os
import numpy as np
import pandas as pd
from typing import Dict, Any, Optional

TRUST_GNN_EXPLANATION_PATH = "backend/gnn_pipeline/data/processed/gnn_explanations.json"
TRUST_SHIFT_PATH = "backend/gnn_pipeline/data/processed/trust_shift_explanations.json"


def load_gnn_explanations() -> Dict[str, Any]:
    if not os.path.exists(TRUST_GNN_EXPLANATION_PATH):
        return {}
    with open(TRUST_GNN_EXPLANATION_PATH, "r") as f:
        return json.load(f)


def explain_gnn_score(endpoint_id: str, timestamp: str) -> Dict[str, Any]:
    data = load_gnn_explanations()
    if endpoint_id in data:
        for record in data[endpoint_id]:
            if record.get("timestamp") == timestamp:
                return {
                    "explanation": record.get("explanation"),
                    "score": record.get("score"),
                    "neighbors": record.get("neighbors"),
                    "features": record.get("features"),
                }
    return {
        "explanation": "No explanation available",
        "score": None,
        "neighbors": [],
        "features": {}
    }


def get_recent_gnn_alerts(limit: int = 10) -> list:
    if not os.path.exists(TRUST_GNN_EXPLANATION_PATH):
        return []

    with open(TRUST_GNN_EXPLANATION_PATH, "r") as f:
        data = json.load(f)

    alerts = []
    for endpoint, records in data.items():
        for record in records:
            if record.get("score", 0) >= 0.8:
                alerts.append({
                    "endpoint_id": endpoint,
                    "timestamp": record.get("timestamp"),
                    "score": record.get("score"),
                    "explanation": record.get("explanation", ""),
                })

    sorted_alerts = sorted(alerts, key=lambda x: x["score"], reverse=True)
    return sorted_alerts[:limit]


def load_trust_shift_explanations() -> Dict[str, Any]:
    if not os.path.exists(TRUST_SHIFT_PATH):
        return {}
    with open(TRUST_SHIFT_PATH, "r") as f:
        return json.load(f)


def get_trust_shift_justification(endpoint_id: str, timestamp: str) -> Dict[str, Any]:
    """
    Return the trust shift justification from GNN output for a specific endpoint and timestamp.
    """
    data = load_trust_shift_explanations()
    if endpoint_id in data:
        shifts = data[endpoint_id]
        for shift in shifts:
            if shift.get("timestamp") == timestamp:
                return {
                    "justification": shift.get("justification", "No justification provided"),
                    "risk_delta": shift.get("risk_delta"),
                    "related_nodes": shift.get("related_nodes", []),
                }
    return {
        "justification": "No matching trust shift found",
        "risk_delta": None,
        "related_nodes": []
    }
def score_graph(graph_data: dict) -> float:
    """
    Scores a GNN graph payload and returns a trust risk score between 0.0 and 1.0.
    This is a placeholder function for real-time GNN inference integration.
    """
    # Simulated logic: Penalize high node count or risky tags (just for testing/demo purposes)
    nodes = graph_data.get("nodes", [])
    edges = graph_data.get("edges", [])
    
    risky_nodes = [n for n in nodes if "risk" in n.get("tags", [])]
    score = min(1.0, 0.1 * len(risky_nodes) + 0.01 * len(edges))

    return round(score, 3)

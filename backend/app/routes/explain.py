# backend/app/routes/explain.py

from fastapi import APIRouter, HTTPException, Query
from pydantic import BaseModel
from typing import List, Dict, Any, Optional
import shap
import joblib
import numpy as np
from pathlib import Path
import json

router = APIRouter()

# Load model
MODEL_PATH = Path(__file__).resolve().parent.parent.parent / "llm_core" / "llm_pipeline_formation" / "models" / "mlp_model.joblib"
try:
    model = joblib.load(MODEL_PATH)
except Exception as e:
    model = None
    print(f"❌ Could not load model for SHAP: {e}")

class ExplainRequest(BaseModel):
    features: List[float]
    feature_names: List[str]

@router.post("/explain")
def explain_prediction(data: ExplainRequest):
    if model is None:
        raise HTTPException(status_code=500, detail="Model not available.")

    try:
        X = np.array([data.features])
        explainer = shap.Explainer(model.predict, X)
        shap_values = explainer(X)

        feature_weights = [
            {"feature": name, "weight": float(shap_values.values[0][i])}
            for i, name in enumerate(data.feature_names)
        ]

        return {"explanation": feature_weights}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@router.get("/explain")
def explain_for_alert(alert_id: Optional[int] = Query(None)):
    from app.services.feature_store import get_features

    if alert_id is None:
        raise HTTPException(status_code=400, detail="Missing alert_id")

    data = get_features(alert_id)
    if not data:
        raise HTTPException(status_code=404, detail="No features found for alert_id")

    features = np.array([data["features"]])
    feature_names = data["feature_names"]

    explainer = shap.Explainer(model.predict, features)
    shap_values = explainer(features)

    explanation = [
        {"feature": feature_names[i], "weight": float(shap_values.values[0][i])}
        for i in range(len(feature_names))
    ]
    return explanation


@router.get("/rules")
def get_rule_logs():
    LOG_PATH = Path("logs/rule_log.json")
    if not LOG_PATH.exists():
        return []
    try:
        return json.loads(LOG_PATH.read_text())
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error reading rule log: {e}")


@router.get("/explain")
def explain_sample():
    if model is None:
        raise HTTPException(status_code=500, detail="Model not available.")

    # Dummy recent feature vector (replace with real example if needed)
    example_features = [0.12, 0.54, 0.33, 0.77, 0.02, 0.88, 0.19, 0.61, 0.44, 0.29]
    example_feature_names = [f"feature_{i}" for i in range(len(example_features))]

    try:
        X = np.array([example_features])
        explainer = shap.Explainer(model.predict, X)
        shap_values = explainer(X)

        feature_weights = [
            {"feature": name, "weight": float(shap_values.values[0][i])}
            for i, name in enumerate(example_feature_names)
        ]
        return feature_weights
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

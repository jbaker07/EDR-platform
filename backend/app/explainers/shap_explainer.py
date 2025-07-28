import shap
import joblib
import numpy as np
from shap import Explainer

# Load your trained model
model = joblib.load("backend/llm_core/llm_pipeline_formation/models/mlp_model.joblib")  # or wherever it's stored

# Example masker: assumes input is a vector of fixed size
masker = shap.maskers.Independent(data=np.zeros((1, 10)))  # 10 features for example

# Initialize the SHAP explainer
shap_explainer = shap.Explainer(model.predict, masker)

def get_shap_explanation(event: dict) -> dict:
    features = np.array(event.get("vector", [0]*10)).reshape(1, -1)

    shap_values = shap_explainer(features)
    top_contributors = sorted(
        zip(shap_values.feature_names, shap_values.values[0]),
        key=lambda x: abs(x[1]),
        reverse=True
    )[:3]

    return {
        "top_contributors": [
            {"feature": name, "impact": float(value)}
            for name, value in top_contributors
        ]
    }

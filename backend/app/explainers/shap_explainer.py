import os
import joblib
import numpy as np

# shap is a heavy, optional dependency. Guard it so importing this module never
# crashes the app; the explainer is also built lazily on first use (it used to be
# constructed at import time, which loaded the model and shap eagerly).
try:
    import shap
except ImportError:  # pragma: no cover
    shap = None

# Resolve the model path relative to the repo root, not the process CWD.
_MODEL_PATH = os.path.join(
    os.path.dirname(os.path.abspath(__file__)),
    "..", "..", "llm_core", "llm_pipeline_formation", "models", "mlp_model.joblib",
)

_shap_explainer = None  # lazily initialised on first call


def _get_explainer():
    global _shap_explainer
    if shap is None:
        raise RuntimeError(
            "SHAP explainability not available (the 'shap' package is not installed)."
        )
    if _shap_explainer is None:
        model = joblib.load(_MODEL_PATH)
        masker = shap.maskers.Independent(data=np.zeros((1, 10)))  # 10 features (example)
        _shap_explainer = shap.Explainer(model.predict, masker)
    return _shap_explainer


def get_shap_explanation(event: dict) -> dict:
    explainer = _get_explainer()
    features = np.array(event.get("vector", [0]*10)).reshape(1, -1)

    shap_values = explainer(features)
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

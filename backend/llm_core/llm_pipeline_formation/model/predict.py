import joblib
import numpy as np
import os

MODEL_PATH = "backend/backend_2/llm_pipeline_2/model/mlp_model.joblib"
clf = joblib.load(MODEL_PATH)

def predict(features: list[float]) -> int:
    X = np.array(features).reshape(1, -1)
    return int(clf.predict(X)[0])

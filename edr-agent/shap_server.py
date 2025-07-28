from flask import Flask, request, jsonify
import shap
import xgboost as xgb
import numpy as np
import os

AUTH_TOKEN = os.environ.get("SHAP_AUTH_TOKEN", "super-secret-token")
MODEL_PATH = "models/shap_model.json"

# Load model and SHAP explainer
model = xgb.Booster()
model.load_model(MODEL_PATH)
explainer = shap.TreeExplainer(model)

app = Flask(__name__)

@app.route("/shap", methods=["POST"])
def shap_score():
    data = request.json
    if not data or data.get("auth") != AUTH_TOKEN:
        return jsonify({"error": "unauthorized"}), 403

    features = data.get("telemetry", {})
    input_array = np.array([[float(v) for v in features.values()]])
    
    shap_values = explainer.shap_values(input_array)
    shap_score = float(np.abs(shap_values).mean())
    
    return jsonify({"score": min(shap_score, 1.0)})

if __name__ == "__main__":
    app.run(host="127.0.0.1", port=5050)

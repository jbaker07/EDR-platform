from fastapi import FastAPI
from pydantic import BaseModel
import joblib
import numpy as np

class Features(BaseModel):
    features: list

app = FastAPI()

model = joblib.load("backend/backend_2/llm_pipeline_2/models/mlp_model.pkl")

@app.post("/api/predict")
def predict(data: Features):
    prediction = model.predict([np.array(data.features)])
    return {"prediction": prediction.tolist()}

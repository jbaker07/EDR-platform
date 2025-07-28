from fastapi import APIRouter
from pydantic import BaseModel
from typing import List
import joblib
import numpy as np
from pathlib import Path

router = APIRouter()

MODEL_PATH = Path("backend/llm_core/llm_pipeline_formation/models/mlp_model.joblib")


class ProcessInput(BaseModel):
    pid: int
    memory: int
    cpu_usage: float

@router.post("/score")
async def score_processes(processes: List[ProcessInput]):
    try:
        model = joblib.load(MODEL_PATH)
        features = [[p.memory, p.cpu_usage] for p in processes]
        preds = model.predict_proba(np.array(features))[:, 1]
        results = [{"pid": p.pid, "risk": round(float(score), 4)} for p, score in zip(processes, preds)]
        return {"status": "ok", "results": results}
    except Exception as e:
        return {"status": "error", "detail": str(e)}

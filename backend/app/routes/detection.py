from fastapi import APIRouter, HTTPException
from app.services.trust_engine import evaluate_rules
from pydantic import BaseModel
from typing import List, Dict, Any

router = APIRouter()

class TelemetryEvent(BaseModel):
    pid: int
    memory: float
    cpu_usage: float
    command: str
    timestamp: str

@router.post("/evaluate")
def evaluate_detection(events: List[TelemetryEvent]):
    try:
        results = evaluate_rules([event.dict() for event in events])
        return {"results": results}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

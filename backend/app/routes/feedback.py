from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import Optional, Literal, List
from pathlib import Path
import json
from datetime import datetime

router = APIRouter()

FEEDBACK_LOG = Path("logs/feedback.json")
FEEDBACK_LOG.parent.mkdir(parents=True, exist_ok=True)

if not FEEDBACK_LOG.exists():
    FEEDBACK_LOG.write_text(json.dumps([]))  # initialize with empty list

class FeedbackEntry(BaseModel):
    timestamp: Optional[str] = None  # can be set automatically
    source: Literal["analyst", "automation"]
    type: Literal["false_positive", "false_negative", "suggestion"]
    message: str
    related_pid: Optional[int] = None
    hostname: Optional[str] = None

@router.post("/feedback")
def submit_feedback(entry: FeedbackEntry):
    try:
        entry.timestamp = datetime.utcnow().isoformat()
        feedback = json.loads(FEEDBACK_LOG.read_text())
        feedback.append(entry.dict())
        FEEDBACK_LOG.write_text(json.dumps(feedback, indent=2))
        return {"status": "logged", "entry": entry}
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Could not log feedback: {e}")

@router.get("/feedback", response_model=List[FeedbackEntry])
def list_feedback():
    try:
        return json.loads(FEEDBACK_LOG.read_text())
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Could not load feedback: {e}")

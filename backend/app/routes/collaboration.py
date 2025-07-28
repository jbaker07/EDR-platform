from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import List
import json
from pathlib import Path

router = APIRouter()

COLLAB_LOG = Path("logs/collaboration.json")
COLLAB_LOG.parent.mkdir(parents=True, exist_ok=True)

if not COLLAB_LOG.exists():
    COLLAB_LOG.write_text(json.dumps([]))  # Start with an empty list

class Message(BaseModel):
    author: str
    content: str

@router.get("/collaboration", response_model=List[Message])
def get_messages():
    try:
        return json.loads(COLLAB_LOG.read_text())
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Failed to load messages: {e}")

@router.post("/collaboration")
def post_message(msg: Message):
    try:
        messages = json.loads(COLLAB_LOG.read_text())
        messages.append(msg.dict())
        COLLAB_LOG.write_text(json.dumps(messages, indent=2))
        return {"status": "sent", "message": msg}
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Failed to save message: {e}")

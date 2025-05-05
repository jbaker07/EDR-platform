# backend/app/routes/timeline.py
from fastapi import APIRouter, HTTPException
from pathlib import Path
import json

router = APIRouter()

TIMELINE_FILE = Path("logs/timeline.json")

@router.get("/timeline")
def get_timeline():
    try:
        if not TIMELINE_FILE.exists():
            raise HTTPException(status_code=404, detail="Timeline file not found")
        content = TIMELINE_FILE.read_text()
        return json.loads(content)
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Failed to load timeline: {str(e)}")

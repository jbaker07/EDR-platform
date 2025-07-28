# backend/app/routes/iocs.py

from fastapi import APIRouter, HTTPException
from pathlib import Path
import json

router = APIRouter()

IOCS_FILE = Path("logs/iocs.json")

@router.get("/iocs")
async def get_iocs():
    if not IOCS_FILE.exists():
        raise HTTPException(status_code=404, detail="IOCs file not found.")
    try:
        with IOCS_FILE.open() as f:
            return json.load(f)
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Failed to load IOCs: {str(e)}")

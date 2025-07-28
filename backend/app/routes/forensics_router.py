from fastapi import APIRouter, Request
from datetime import datetime

router = APIRouter()

@router.post("/api/forensics/run")
async def trigger_forensic_sweep(request: Request):
    body = await request.json()
    reason = body.get("reason", "manual")
    timestamp = body.get("timestamp", str(datetime.utcnow()))
    print(f"[+] Forensic sweep triggered via API. Reason: {reason} at {timestamp}")
    return {"status": "ok", "sweep_scheduled": True}

# backend/llm_pipeline/routes/telemetry_routes.py

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import List
from pathlib import Path
import json
import pandas as pd

router = APIRouter()

TELEMETRY_FILE = Path("logs/telemetry.json")
VECTORIZED_CSV = Path("logs/vectorized_processes.csv")
TELEMETRY_FILE.parent.mkdir(parents=True, exist_ok=True)
VECTORIZED_CSV.parent.mkdir(parents=True, exist_ok=True)

class ProcessInfo(BaseModel):
    pid: int
    name: str
    cmd: str
    memory: int
    cpu_usage: float
    status: str

class Telemetry(BaseModel):
    hostname: str
    timestamp: int
    processes: List[ProcessInfo]

# ✅ RECEIVE telemetry data (POST to /api/telemetry)
@router.post("/telemetry")
async def receive_telemetry(data: Telemetry):
    try:
        TELEMETRY_FILE.write_text(json.dumps(data.dict(), indent=2))

        rows = []
        for proc in data.processes:
            rows.append({
                "hostname": data.hostname,
                "timestamp": data.timestamp,
                "pid": proc.pid,
                "name": proc.name,
                "cmd": proc.cmd,
                "memory": proc.memory,
                "cpu_usage": proc.cpu_usage,
                "status": proc.status,
            })
        df = pd.DataFrame(rows)
        df.to_csv(VECTORIZED_CSV, mode="a", header=not VECTORIZED_CSV.exists(), index=False)

        return {"status": "received", "process_count": len(data.processes)}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# ✅ GET full telemetry (GET /api/telemetry/all)
@router.get("/telemetry/all")
def get_all_telemetry():
    if TELEMETRY_FILE.exists():
        return json.loads(TELEMETRY_FILE.read_text())
    return {"error": "No telemetry found"}

# ✅ GET telemetry timeline (GET /api/telemetry/timeline)
@router.get("/telemetry/timeline")
def get_timeline():
    if TELEMETRY_FILE.exists():
        try:
            data = json.loads(TELEMETRY_FILE.read_text())
            hostname = data.get("hostname", "unknown")
            processes = data.get("processes", [])
            process_count = len(processes)

            return [{
                "timestamp": data.get("timestamp", 0),
                "hostname": hostname,
                "process_count": process_count
            }]
        except Exception as e:
            raise HTTPException(status_code=500, detail=str(e))
    else:
        return []

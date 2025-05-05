from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import List, Dict, Any
import json
from pathlib import Path

router = APIRouter()

RULES_FILE = Path("logs/rules.json")

class Process(BaseModel):
    pid: int
    name: str
    cmd: str
    memory: int
    cpu_usage: float
    status: str

class HostTelemetry(BaseModel):
    hostname: str
    timestamp: int
    processes: List[Process]

@router.post("/rules/eval")
def evaluate_rules(data: HostTelemetry):
    if not RULES_FILE.exists():
        raise HTTPException(status_code=500, detail="Rules file not found")

    rules = json.loads(RULES_FILE.read_text())
    alerts = []

    for proc in data.processes:
        for rule in rules:
            try:
                if eval(rule["rule"], {}, {
                    "cpu_usage": proc.cpu_usage,
                    "memory": proc.memory,
                    "cmd": proc.cmd,
                    "name": proc.name
                }):
                    alerts.append({
                        "hostname": data.hostname,
                        "timestamp": data.timestamp,
                        "pid": proc.pid,
                        "process": proc.name,
                        "rule": rule["description"],
                        "severity": rule["severity"]
                    })
            except Exception as e:
                print(f"❌ Rule failed: {rule['rule']} — {e}")

    return {"alerts": alerts}

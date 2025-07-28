from fastapi import APIRouter, Request
from backend.app.rules.rules_engine import load_rules, apply_rules

router = APIRouter()

rules = load_rules()

@router.post("/api/telemetry")
async def receive_telemetry(request: Request):
    telemetry_data = await request.json()
    alerts = apply_rules(telemetry_data, rules, os_type=telemetry_data.get("os_type", "linux"))

    return {
        "received": telemetry_data,
        "alerts_triggered": alerts
    }

@router.get("/api/telemetry/all")
def get_latest_telemetry():
    return {
        "hostname": "demo-host",
        "timestamp":  int(__import__('time').time()),
        "processes": []
    }

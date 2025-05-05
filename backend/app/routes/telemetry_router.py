from fastapi import APIRouter, Request
import time
from datetime import datetime
from backend.app.memory.memory_manager import memory_manager
from backend.app.rules.rules_engine import load_rules, apply_rules

router = APIRouter()
rules = load_rules()

@router.post("/telemetry")
async def receive_telemetry(request: Request):
    payload    = await request.json()
    hostname   = payload.get("hostname", "unknown-host")
    os_type    = payload.get("os_type", "linux")
    endpoint   = payload.get("endpoint_id", hostname)
    processes  = payload.get("processes", [])

    all_alerts = []
    for proc in processes:
        event = {
            **proc,
            "hostname":    hostname,
            "os_type":     os_type,
            "endpoint_id": endpoint,
            "timestamp":   time.time(),
        }

        memory_manager.save_telemetry(hostname, event)
        alerts = apply_rules(event, rules, os_type=os_type)
        all_alerts.extend(alerts)

    return {"received_count": len(processes), "alerts_triggered": all_alerts}


@router.get("/telemetry")
def get_telemetry_alias():
    return get_latest_telemetry()


@router.get("/telemetry/all")
def get_latest_telemetry():
    recent = memory_manager.get_all_recent_events(window_seconds=900)

    # Ensure clean structure for frontend
    return {
        "hostname": "dashboard-collector",
        "timestamp": int(time.time()),
        "processes": [
            {
                "agent_id":    r.get("endpoint_id", "N/A"),
                "name":        r.get("process_name", "N/A"),
                "pid":         r.get("pid", "N/A"),
                "command":     r.get("cmd", "N/A"),
                "severity":    r.get("risk_level", "low").upper(),
                "timestamp":   r.get("timestamp", 0),
            }
            for r in recent
        ]
    }

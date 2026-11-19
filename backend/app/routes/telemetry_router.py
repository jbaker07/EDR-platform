# backend/app/routes/telemetry_router.py

from fastapi import APIRouter, Request
import time
from sqlmodel import select
from app.memory.memory_manager import memory_manager
from app.models.alert import Alert
from app.services.db import get_db_session

router = APIRouter()

# Rules run once per event, inside memory_manager.save_telemetry, against the
# hot-reloadable rule set. This module used to keep its own import-time copy of
# the rules and run them a second time.


@router.post("/telemetry")
async def receive_telemetry(request: Request):
    payload = await request.json()
    hostname = payload.get("hostname", "unknown-host")
    os_type = payload.get("os_type", "linux")
    endpoint = payload.get("endpoint_id", hostname)

    # Handle forensic-mode batch
    if "processes" in payload:
        all_alerts = []
        for proc in payload["processes"]:
            event = {
                **proc,
                "hostname": hostname,
                "os_type": os_type,
                "endpoint_id": endpoint,
                "timestamp": time.time(),
            }

            all_alerts.extend(memory_manager.save_telemetry(hostname, event))

        return {"received_count": len(payload["processes"]), "alerts_triggered": all_alerts}

    # Handle minimal-mode single process
    event = {
        **payload,
        "hostname": hostname,
        "os_type": os_type,
        "endpoint_id": endpoint,
        "timestamp": time.time(),
    }

    alerts = memory_manager.save_telemetry(hostname, event)
    return {"received_count": 1, "alerts_triggered": alerts}


@router.get("/telemetry")
def get_telemetry_alias():
    return get_latest_telemetry()


@router.get("/telemetry/all")
def get_latest_telemetry():
    recent = memory_manager.get_all_recent_events(window_seconds=900)

    return {
        "hostname": "dashboard-collector",
        "timestamp": int(time.time()),
        "processes": [
            {
                # The stored columns (process_name, cmd, cpu_percent, memory, ...),
                # which AlertTable and the Dashboard page read, plus the short
                # names ProcessTable reads.
                **r,
                "agent_id": r.get("endpoint_id", "N/A"),
                "name": r.get("process_name", "N/A"),
                "pid": r.get("pid", "N/A"),
                "command": r.get("cmd", "N/A"),
                "severity": r.get("risk_level", "low").upper(),
                "timestamp": r.get("timestamp", 0),
            }
            for r in recent
        ]
    }


_RISK_RANK = {"low": 0, "medium": 1, "high": 2}
_SEVERITY_TO_RISK = {"critical": "high", "high": "high", "medium": "medium"}


@router.get("/telemetry/agents")
def list_reporting_endpoints(window_seconds: int = 900, online_seconds: int = 300):
    """One entry per endpoint that sent telemetry in the window, from stored rows.

    status is "online" when the newest row is less than online_seconds old.
    riskLevel is the worst stored alert severity for that host in the window.
    The collector sends no IP address, so none is returned.
    """
    now = time.time()
    endpoints = {}
    for r in memory_manager.get_all_recent_events(window_seconds=window_seconds):
        key = r.get("endpoint_id") or r.get("hostname")
        ep = endpoints.setdefault(key, {"id": key, "hostname": r.get("hostname", key), "last_seen": 0.0, "process_count": 0})
        ep["last_seen"] = max(ep["last_seen"], r.get("timestamp", 0))
        ep["process_count"] += 1

    db = next(get_db_session())
    alerts = db.execute(select(Alert).where(Alert.timestamp >= now - window_seconds)).scalars().all()
    worst = {}
    for a in alerts:
        level = _SEVERITY_TO_RISK.get(a.severity.lower(), "low")
        if _RISK_RANK[level] > _RISK_RANK[worst.get(a.hostname, "low")]:
            worst[a.hostname] = level

    return [
        {
            **ep,
            "status": "online" if now - ep["last_seen"] < online_seconds else "offline",
            "riskLevel": worst.get(ep["hostname"], "low"),
        }
        for ep in endpoints.values()
    ]

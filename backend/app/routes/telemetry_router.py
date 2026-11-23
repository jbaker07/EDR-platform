# backend/app/routes/telemetry_router.py

from fastapi import APIRouter, HTTPException, Request
import time
from sqlmodel import select
from app.memory.memory_manager import memory_manager
from app.models.alert import Alert
from app.services.db import get_db_session

router = APIRouter()

# Rules run once per event, inside memory_manager.save_telemetry, against the
# hot-reloadable rule set.

# Fields save_telemetry stores. A record without one used to crash with a
# KeyError and a 500. Now it is a 422 that names the field.
_REQUIRED = ("pid", "process_name", "cmd", "cpu_percent", "memory")


def _check_fields(records: list) -> None:
    for i, rec in enumerate(records):
        if not isinstance(rec, dict):
            raise HTTPException(status_code=422, detail=f"record {i} is not an object")
        missing = [f for f in _REQUIRED if f not in rec]
        if missing:
            raise HTTPException(status_code=422, detail=f"record {i} is missing {', '.join(missing)}")


@router.post("/telemetry")
async def receive_telemetry(request: Request):
    payload = await request.json()
    if not isinstance(payload, dict):
        raise HTTPException(status_code=422, detail="body must be a JSON object")
    batch = payload.get("processes")
    if batch is not None and not isinstance(batch, list):
        raise HTTPException(status_code=422, detail="processes must be a list")
    # Check every record before storing any, so a bad batch stores nothing.
    _check_fields(batch if batch is not None else [payload])
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


@router.get("/telemetry/all")
def get_latest_telemetry(window_seconds: int = 900):
    """Stored process events from the last window_seconds, as stored."""
    return {
        "timestamp": int(time.time()),
        "processes": memory_manager.get_all_recent_events(window_seconds=window_seconds),
    }


# Host view. Every number here is a count of stored rows. There used to be a
# 0 to 100 "trust score", but it fell only through a risk_score field that no
# collector sends, so it read 100 for every host. Alert counts per severity
# replace it, because they are facts an analyst can check.
_SEVERITY_RANK = {"low": 0, "medium": 1, "high": 2, "critical": 3}


def _alerts_since(cutoff: float, hostname: str | None = None) -> list[Alert]:
    db = next(get_db_session())
    stmt = select(Alert).where(Alert.timestamp >= cutoff)
    if hostname is not None:
        stmt = stmt.where(Alert.hostname == hostname)
    return db.execute(stmt).scalars().all()


def _host_summaries(events: list[dict], alerts: list[Alert], now: float, online_seconds: int) -> dict:
    hosts = {}

    def entry(name):
        return hosts.setdefault(name, {
            "hostname": name,
            "last_seen": None,
            "process_events": 0,
            "alert_count": 0,
            "alerts_by_severity": {},
            "worst_severity": None,
        })

    for e in events:
        h = entry(e["hostname"])
        h["last_seen"] = max(h["last_seen"] or 0.0, e["timestamp"])
        h["process_events"] += 1

    for a in alerts:
        h = entry(a.hostname)
        sev = a.severity.lower()
        h["alert_count"] += 1
        h["alerts_by_severity"][sev] = h["alerts_by_severity"].get(sev, 0) + 1
        if h["worst_severity"] is None or _SEVERITY_RANK.get(sev, -1) > _SEVERITY_RANK.get(h["worst_severity"], -1):
            h["worst_severity"] = sev

    for h in hosts.values():
        seen = h["last_seen"]
        h["status"] = "online" if seen is not None and now - seen < online_seconds else "offline"
    return hosts


@router.get("/telemetry/agents")
def list_reporting_endpoints(window_seconds: int = 900, online_seconds: int = 300):
    """One entry per host with stored telemetry or alerts in the window.

    status is "online" when the newest telemetry row is under online_seconds old.
    worst_severity is None when the host has no stored alerts in the window.
    """
    now = time.time()
    events = memory_manager.get_all_recent_events(window_seconds=window_seconds)
    alerts = _alerts_since(now - window_seconds)
    hosts = _host_summaries(events, alerts, now, online_seconds)
    return sorted(hosts.values(), key=lambda h: h["hostname"])


@router.get("/host/{hostname}")
def get_host(hostname: str, window_seconds: int = 900, online_seconds: int = 300):
    now = time.time()
    events = memory_manager.get_events_for_host(hostname, window_seconds=window_seconds)
    alerts = _alerts_since(now - window_seconds, hostname)
    if not events and not alerts:
        raise HTTPException(status_code=404, detail=f"No telemetry or alerts for {hostname} in the last {window_seconds} seconds")
    return _host_summaries(events, alerts, now, online_seconds)[hostname]

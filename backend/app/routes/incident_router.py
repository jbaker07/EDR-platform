# backend/app/routes/incident_router.py
#
# POST /api/incidents   the agent's forwarder sends a batch of incidents
# GET  /api/incidents   newest first, for the dashboard
# GET  /api/incidents/{key}
#
# Both need the backend token like every other route (main.py). The body is checked
# field by field with size limits, so a bad agent or a stolen token cannot store
# unbounded text. A post replaces the row for the same host and incident id.

import json
from typing import Any, Dict, List, Optional

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel, Field, field_validator
from sqlmodel import select

from app.models.incident import AgentIncident
from app.services.db import get_db_session

router = APIRouter()

MAX_TEXT = 512
MAX_DETAILS = 8192


class IncidentIn(BaseModel):
    id: str = Field(min_length=1, max_length=200)
    host: Optional[str] = Field(default=None, max_length=255)
    title: Optional[str] = Field(default="", max_length=MAX_TEXT)
    sources: List[str] = Field(default_factory=list, max_length=16)
    exe: Optional[str] = Field(default="-", max_length=MAX_TEXT)
    primary_technique: Optional[str] = Field(default="-", max_length=32)
    techniques: List[str] = Field(default_factory=list, max_length=64)
    severity: Optional[str] = Field(default="none", max_length=16)
    risk: Optional[float] = Field(default=0.0, ge=0.0, le=1.5)
    alerts_count: Optional[int] = Field(default=0, ge=0)
    first_ts: Optional[int] = Field(default=0, ge=0)
    last_ts: Optional[int] = Field(default=0, ge=0)
    details: Optional[Dict[str, Any]] = None

    @field_validator("details")
    @classmethod
    def details_size(cls, v):
        if v is not None and len(json.dumps(v)) > MAX_DETAILS:
            raise ValueError(f"details is larger than {MAX_DETAILS} bytes")
        return v


class AgentInfo(BaseModel):
    host: str = Field(min_length=1, max_length=255)
    version: str = Field(default="", max_length=32)


class IncidentBatch(BaseModel):
    agent: AgentInfo
    incidents: List[IncidentIn] = Field(max_length=500)


def _row_dict(r: AgentIncident) -> dict:
    return {
        "key": r.key,
        "id": r.incident_id,
        "host": r.host,
        "title": r.title,
        "sources": json.loads(r.sources),
        "exe": r.exe,
        "primary_technique": r.primary_technique,
        "techniques": json.loads(r.techniques),
        "severity": r.severity,
        "risk": r.risk,
        "alerts_count": r.alerts_count,
        "first_ts": r.first_ts,
        "last_ts": r.last_ts,
        "details": json.loads(r.details),
        "agent_version": r.agent_version,
        "received_at": r.received_at,
    }


@router.post("/incidents")
def ingest_incidents(batch: IncidentBatch):
    db = next(get_db_session())
    stored = 0
    for inc in batch.incidents:
        # The host in the incident wins, the agent's host name fills a missing one.
        host = inc.host or batch.agent.host
        key = f"{host}|{inc.id}"
        row = AgentIncident(
            key=key,
            host=host,
            incident_id=inc.id,
            title=inc.title or "",
            sources=json.dumps(inc.sources),
            exe=inc.exe or "-",
            primary_technique=inc.primary_technique or "-",
            techniques=json.dumps(inc.techniques),
            severity=inc.severity or "none",
            risk=float(inc.risk or 0.0),
            alerts_count=int(inc.alerts_count or 0),
            first_ts=int(inc.first_ts or 0),
            last_ts=int(inc.last_ts or 0),
            details=json.dumps(inc.details or {}),
            agent_version=batch.agent.version,
        )
        db.merge(row)
        stored += 1
    db.commit()
    return {"stored": stored}


@router.get("/incidents")
def list_incidents(limit: int = 200):
    limit = max(1, min(limit, 1000))
    db = next(get_db_session())
    rows = db.exec(select(AgentIncident).order_by(AgentIncident.last_ts.desc()).limit(limit)).all()
    return [_row_dict(r) for r in rows]


@router.get("/incidents/{key:path}")
def get_incident(key: str):
    db = next(get_db_session())
    row = db.get(AgentIncident, key)
    if row is None:
        raise HTTPException(status_code=404, detail="Incident not found")
    return _row_dict(row)

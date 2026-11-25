# backend/app/models/incident.py
#
# Incidents the agent forwards (edr-agent/src/forwarder.rs). One row per (host, id).
# The agent sends the newest version of an incident each time it changes, so a post
# replaces the row instead of adding one.

import time
from typing import Optional

from sqlmodel import Field, SQLModel


class AgentIncident(SQLModel, table=True):
    __tablename__ = "agent_incidents"

    key: str = Field(primary_key=True)  # "<host>|<incident id>"
    host: str = Field(index=True)
    incident_id: str
    title: str
    sources: str  # JSON list
    exe: str
    primary_technique: str
    techniques: str  # JSON list
    severity: str
    risk: float
    alerts_count: int
    first_ts: int
    last_ts: int = Field(index=True)
    details: str  # JSON object
    agent_version: str
    received_at: float = Field(default_factory=time.time)

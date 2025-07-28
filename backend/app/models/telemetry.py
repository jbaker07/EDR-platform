# backend/app/models/telemetry.py

from datetime import datetime
from sqlmodel import SQLModel, Field

class TelemetryEvent(SQLModel, table=True):
    id:            int       = Field(default=None, primary_key=True)
    hostname:      str
    endpoint_id:   str
    os_type:       str
    pid:           int
    process_name:  str
    cmd:           str
    cpu_percent:   float
    memory:        int
    risk_score:    float
    risk_level:    str
    timestamp:     float
    created_at:    datetime = Field(default_factory=datetime.utcnow)

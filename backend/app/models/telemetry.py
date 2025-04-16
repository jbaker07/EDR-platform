# ~/edr-platform/backend/app/models/telemetry.py

from pydantic import BaseModel
from typing import List, Optional

class Process(BaseModel):
    pid: int
    name: str
    risk_score: int

class Telemetry(BaseModel):
    agent_id: str
    timestamp: str
    processes: Optional[List[Process]] = []

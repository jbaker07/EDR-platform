from sqlmodel import SQLModel, Field
from typing import Optional
import time

class Alert(SQLModel, table=True):
    id: Optional[int] = Field(default=None, primary_key=True)
    hostname: str
    process_name: str
    rule_id: str
    rule_name: str
    description: str
    severity: str
    timestamp: float
    created_at: float = Field(default_factory=lambda: time.time())

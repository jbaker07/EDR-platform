from sqlalchemy import Column, Integer, String, JSON, DateTime
from sqlalchemy.ext.declarative import declarative_base
import datetime

Base = declarative_base()

class Telemetry(Base):
    __tablename__ = "telemetry"

    id = Column(Integer, primary_key=True, index=True)
    agent_id = Column(String, index=True)
    timestamp = Column(DateTime, default=datetime.datetime.utcnow)
    data = Column(JSON)


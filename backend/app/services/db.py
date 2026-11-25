# backend/app/services/db.py

from sqlmodel import SQLModel, create_engine, Session
import os

DATABASE_URL = os.getenv("DATABASE_URL", "sqlite:///./edr.db")

# Setup engine with pool settings to prevent connection exhaustion
engine = create_engine(
    DATABASE_URL,
    echo=False,
    pool_size=20,
    max_overflow=30,
    pool_timeout=30,
    connect_args={"check_same_thread": False} if DATABASE_URL.startswith("sqlite") else {}
)

def init_db():
    from app.models.telemetry import TelemetryEvent  # required to register table
    from app.models.alert import Alert  # required to register table
    from app.models.incident import AgentIncident  # incidents forwarded by the agent
    from app.models.agent import Base as AgentBase
    SQLModel.metadata.create_all(engine)
    # Agent uses a plain SQLAlchemy Base, not SQLModel, so its table needs its own
    # create_all. Without it /api/agent and the trust lookup fail with "no such table".
    AgentBase.metadata.create_all(engine)

def get_db_session():
    with Session(engine) as session:
        try:
            yield session
        finally:
            session.close()

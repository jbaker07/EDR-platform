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
    SQLModel.metadata.create_all(engine)

def get_db_session():
    with Session(engine) as session:
        try:
            yield session
        finally:
            session.close()

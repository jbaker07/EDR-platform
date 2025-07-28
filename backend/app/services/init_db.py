from sqlmodel import SQLModel
from app.services.db import engine
from app.models.telemetry import TelemetryEvent  # Ensure this model is correctly defined and imported

def init_db():
    SQLModel.metadata.create_all(engine)

if __name__ == "__main__":
    init_db()

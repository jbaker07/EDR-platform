from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from pathlib import Path
from sqlalchemy.ext.declarative import declarative_base
import os

DATABASE_URL = os.getenv("DATABASE_URL", "sqlite:///./edr.db")

engine = create_engine(
    DATABASE_URL,
    connect_args={"check_same_thread": False} if DATABASE_URL.startswith("sqlite") else {},
    pool_size=20,        # Allow 20 concurrent connections
    max_overflow=40,     # Allow 40 additional connections if needed
    pool_timeout=30,     # Wait time before giving up on a connection
    pool_recycle=1800    # Recycle connections after 30 minutes
)

SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

Base = declarative_base()

def init_db():
    from app.models.agent import Base  # ✅ Ensure this import path is correct!
    Base.metadata.create_all(bind=engine)

def get_db_session():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

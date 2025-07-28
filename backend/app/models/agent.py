from sqlalchemy import Column, String, DateTime, Float, Boolean
from sqlalchemy.orm import declarative_base
from datetime import datetime

Base = declarative_base()

class Agent(Base):
    __tablename__ = "agents"

    hostname = Column(String, primary_key=True, index=True)
    pubkey = Column(String, nullable=False)
    last_seen = Column(DateTime, default=datetime.utcnow, onupdate=datetime.utcnow)
    binary_hash = Column(String, nullable=False)
    policy_hash = Column(String, nullable=False)
    attestation_passed = Column(Boolean, default=False)
    trust_score = Column(Float, default=100.0)

    def __repr__(self):
        return (
            f"<Agent(hostname='{self.hostname}', trust_score={self.trust_score}, "
            f"attestation_passed={self.attestation_passed}, last_seen={self.last_seen})>"
        )

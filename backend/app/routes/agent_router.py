# backend/app/routes/agent_router.py

from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy.orm import Session
from pydantic import BaseModel
from datetime import datetime
from typing import List
from app.models.agent import Agent
from app.services.db import get_db_session

router = APIRouter()

class AgentRegisterRequest(BaseModel):
    hostname: str
    pubkey: str
    binary_hash: str
    policy_hash: str
    attestation_passed: bool

class AgentResponse(BaseModel):
    hostname: str
    pubkey: str
    last_seen: datetime
    binary_hash: str
    policy_hash: str
    attestation_passed: bool
    trust_score: float

    class Config:
        orm_mode = True

@router.post("/agent/register", response_model=AgentResponse)
def register_agent(req: AgentRegisterRequest, db: Session = Depends(get_db_session)):
    agent = db.query(Agent).get(req.hostname)
    if not agent:
        agent = Agent(hostname=req.hostname)
    agent.pubkey = req.pubkey
    agent.binary_hash = req.binary_hash
    agent.policy_hash = req.policy_hash
    agent.attestation_passed = req.attestation_passed
    agent.last_seen = datetime.utcnow()
    db.add(agent)
    db.commit()
    db.refresh(agent)
    return agent

@router.get("/agent", response_model=List[AgentResponse])
def list_agents(db: Session = Depends(get_db_session)):
    return db.query(Agent).all()

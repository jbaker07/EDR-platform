from fastapi import APIRouter
from typing import List
from pydantic import BaseModel
from datetime import datetime

router = APIRouter()

class Achievement(BaseModel):
    username: str
    points: int
    badges: List[str]
    last_active: datetime

mock_data = [
    {"username": "AnalystA", "points": 1200, "badges": ["Hunter", "Responder"], "last_active": "2025-04-24T12:30:00"},
    {"username": "AnalystB", "points": 950, "badges": ["Hardened"], "last_active": "2025-04-24T09:00:00"},
    {"username": "AnalystC", "points": 450, "badges": ["Learner"], "last_active": "2025-04-23T20:45:00"},
]

@router.get("/gamification", response_model=List[Achievement])
def get_leaderboard():
    return mock_data

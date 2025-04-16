# backend/app/routes/telemetry.py

from fastapi import APIRouter
from app.models.telemetry import Telemetry

router = APIRouter()

@router.get("/telemetry/all")
async def get_telemetry():
    return []  # We'll replace this with real DB fetch logic

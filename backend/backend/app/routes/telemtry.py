from fastapi import APIRouter, Depends
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy.future import select
from app.services.db import get_db
from app.models.telemetry import Telemetry

router = APIRouter(prefix="/telemetry", tags=["Telemetry"])

@router.post("/ingest")
async def ingest(request, db: AsyncSession = Depends(get_db)):
    data = await request.json()
    entry = Telemetry(agent_id=data.get("agent_id"), data=data)
    db.add(entry)
    await db.commit()
    return {"status": "telemetry received"}

@router.get("/all")
async def get_all_telemetry(db: AsyncSession = Depends(get_db)):
    result = await db.execute(select(Telemetry))
    entries = result.scalars().all()
    return [  # Return as list of dicts
        {
            "id": entry.id,
            "agent_id": entry.agent_id,
            "timestamp": entry.timestamp.isoformat(),
            "data": entry.data
        } for entry in entries
    ]

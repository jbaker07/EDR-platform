# backend/llm_pipeline/routes/llm_routes.py

from fastapi import APIRouter
from pydantic import BaseModel
from llm_pipeline.handlers.llm_handler import get_summary, get_hunt_query

router = APIRouter(prefix="/api/llm")

class PromptRequest(BaseModel):
    prompt: str

class HuntRequest(BaseModel):
    threat_name: str

@router.post("/summary")
async def generate_summary(request: PromptRequest):
    result = get_summary(request.prompt)
    return {"summary": result}

@router.post("/hunt")
async def generate_hunt_query(request: HuntRequest):
    result = get_hunt_query(request.threat_name)
    return {"hunt_query": result}

from fastapi import APIRouter
from app.memory.memory_manager import memory_manager
from app.memory.correlation_engine import find_multi_endpoint_correlation

router = APIRouter()

@router.get("/api/multi_correlation")
def get_multi_correlation():
    attack_chains = find_multi_endpoint_correlation(memory_manager)
    return {"attack_chains_detected": attack_chains}

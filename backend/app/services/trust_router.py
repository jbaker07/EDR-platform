from fastapi import APIRouter
from app.services.trust_engine import calculate_trust_score

router = APIRouter()

@router.get("/trust/{hostname}")
def get_trust_score(hostname: str):
    score = calculate_trust_score(hostname)
    return {"hostname": hostname, "trust_score": score}

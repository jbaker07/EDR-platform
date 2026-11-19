from fastapi import APIRouter, Query

router = APIRouter()

@router.get("/trust/history")
def get_trust_history(hostname: str = Query(...)):
    # No trust history is stored yet, so every host gets an empty list. This used
    # to return three hard-coded points for a made-up host, "host123".
    return []

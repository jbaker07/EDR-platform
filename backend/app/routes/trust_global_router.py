from fastapi import APIRouter

router = APIRouter()

@router.get("/api/trust/global")
def get_all_endpoint_trust():
    # Replace with real DB/store lookup later
    return [
        {"hostname": "host-a", "trust_score": 93},
        {"hostname": "host-b", "trust_score": 72},
        {"hostname": "host-c", "trust_score": 58},
        {"hostname": "host-d", "trust_score": 34},
        {"hostname": "host-e", "trust_score": 88},
        {"hostname": "host-f", "trust_score": 27},
        {"hostname": "host-g", "trust_score": 65},
        {"hostname": "host-h", "trust_score": 45},
    ]

from fastapi import APIRouter
from backend.app.memory.memory_manager import memory_manager  # We'll make it global for now

router = APIRouter()

@router.get("/api/timeline/{endpoint_id}")
def get_timeline(endpoint_id: str):
    events = memory_manager.get_recent_events(endpoint_id, window_seconds=86400)  # 24 hours
    # Sort by timestamp for clean timeline
    sorted_events = sorted(events, key=lambda e: e.get("timestamp", 0))
    return {"endpoint_id": endpoint_id, "timeline": sorted_events}

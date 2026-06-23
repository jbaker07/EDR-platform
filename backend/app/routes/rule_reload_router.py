from fastapi import APIRouter
from app.rules.rules_engine import reload_rules
from app.memory.correlation_engine import reload_multi_endpoint_rules

router = APIRouter()

@router.post("/api/rules/reload")
def reload_all_rules():
    reload_rules()
    reload_multi_endpoint_rules()
    return {"message": "All rules reloaded successfully."}

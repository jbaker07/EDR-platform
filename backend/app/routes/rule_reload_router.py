from fastapi import APIRouter, HTTPException
from app.rules.rules_engine import reload_rules, get_loaded_rules
from app.memory.correlation_engine import reload_multi_endpoint_rules

router = APIRouter()

@router.post("/rules/reload")
def reload_all_rules():
    # Both reloads return True or False. Report that instead of always claiming success.
    stateless_ok = reload_rules()
    multi_endpoint_ok = reload_multi_endpoint_rules()
    if not stateless_ok:
        raise HTTPException(
            status_code=422,
            detail="Stateless rules failed to load. The previous rules stay active. See the edr.rules log.",
        )
    return {
        "stateless_rules_loaded": len(get_loaded_rules()),
        "multi_endpoint_rules_reloaded": multi_endpoint_ok,
    }

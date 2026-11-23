from fastapi import APIRouter, HTTPException
from app.rules.rules_engine import reload_rules, get_loaded_rules

router = APIRouter()


@router.get("/rules")
def list_loaded_rules():
    # The stateless rules every stored event is checked against.
    return get_loaded_rules()


@router.post("/rules/reload")
def reload_all_rules():
    # reload_rules() returns True or False. Report that instead of always claiming success.
    if not reload_rules():
        raise HTTPException(
            status_code=422,
            detail="Stateless rules failed to load. The previous rules stay active. See the edr.rules log.",
        )
    return {"stateless_rules_loaded": len(get_loaded_rules())}

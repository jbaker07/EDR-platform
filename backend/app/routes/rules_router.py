from fastapi import APIRouter

router = APIRouter()

@router.get("/rules/test")
async def test_rule_router():
    return {"message": "Rules router is working!"}

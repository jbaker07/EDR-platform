from fastapi import APIRouter
from backend.app.rules.rule_validator import validate_stateless_rules, validate_multi_endpoint_rules
from backend.app.rules.rules_engine import load_rules
from backend.app.memory.correlation_engine import load_multi_endpoint_rules

router = APIRouter()

@router.get("/api/rules/validate")
def validate_all_rules():
    stateless_rules = load_rules()
    multi_endpoint_rules = load_multi_endpoint_rules()

    stateless_errors = validate_stateless_rules(stateless_rules)
    multi_endpoint_errors = validate_multi_endpoint_rules(multi_endpoint_rules)

    return {
        "stateless_validation_errors": stateless_errors,
        "multi_endpoint_validation_errors": multi_endpoint_errors
    }

import json
import os
import re
import operator
from backend.app.rules.rule_validator import validate_stateless_rules
from backend.app.logs.logger import rules_logger
import math

RULES_PATH = "backend/app/rules/default_rules.json"
loaded_rules = []  
# Operator functions
def evaluate_condition(field_value, operator_type, expected_value):
    try:
        if operator_type == ">":
            return field_value > expected_value
        elif operator_type == "<":
            return field_value < expected_value
        elif operator_type == ">=":
            return field_value >= expected_value
        elif operator_type == "<=":
            return field_value <= expected_value
        elif operator_type == "==":
            return field_value == expected_value
        elif operator_type == "!=":
            return field_value != expected_value
        elif operator_type == "contains":
            return expected_value in field_value
        elif operator_type == "startswith":
            return str(field_value).startswith(str(expected_value))
        elif operator_type == "endswith":
            return str(field_value).endswith(str(expected_value))
        elif operator_type == "regex":
            return re.search(expected_value, str(field_value)) is not None
    except Exception as e:
        print(f"Error in condition evaluation: {e}")
        return False
    return False

def load_rules(path=RULES_PATH):
    if not os.path.exists(path):
        raise FileNotFoundError(f"Rules file not found: {path}")
    with open(path, "r") as f:
        rules = json.load(f)
    return rules

def save_rules(rules, path=RULES_PATH):
    with open(path, "w") as f:
        json.dump(rules, f, indent=2)

def check_exceptions(exceptions, telemetry_data):
    for exception in exceptions:
        field = exception.get("field")
        value = exception.get("value")
        if field in telemetry_data and telemetry_data[field] == value:
            return True  # Exception matched → skip rule
    return False

def evaluate_rule(rule, telemetry_data):
    if check_exceptions(rule.get("exceptions", []), telemetry_data):
        return False

    conditions = rule.get("conditions", [])
    logic = rule.get("logic", "AND")

    results = []
    for condition in conditions:
        field = condition.get("field")
        operator_type = condition.get("operator")
        expected_value = condition.get("value")
        actual_value = telemetry_data.get(field)

        if actual_value is None:
            results.append(False)
            continue

        result = evaluate_condition(actual_value, operator_type, expected_value)
        results.append(result)

    if logic == "AND":
        return all(results)
    elif logic == "OR":
        return any(results)
    else:
        return False

def apply_rules(telemetry_data, rules, os_type="linux"):
    triggered_alerts = []
    for rule in rules:
        target_os = rule.get("target_os", [])
        if os_type not in target_os:
            continue  # Skip rules not meant for this OS

        if evaluate_rule(rule, telemetry_data):
            triggered_alerts.append({
                "rule_id": rule["id"],
                "name": rule["name"],
                "description": rule["description"],
                "severity": rule.get("severity", "low"),
                "actions": rule.get("actions", ["generate_alert"]),
                "telemetry": telemetry_data
            })
    return triggered_alerts

OPERATORS = {
    ">": operator.gt,
    "<": operator.lt,
    ">=": operator.ge,
    "<=": operator.le,
    "==": operator.eq,
    "!=": operator.ne,
    "contains": lambda a, b: b in a,
    "startswith": lambda a, b: str(a).startswith(str(b)),
    "endswith": lambda a, b: str(a).endswith(str(b)),
}

BETA = {
    "intercept": -3.0,
    "cpu": -0.03,
    "memory": -0.002,
    "signed": 1.5,
    "known_bad": -2.0,
    "history": -0.4
}


def sigmoid(x):
    return 1 / (1 + math.exp(-x))

def calculate_trust_probability(telemetry: dict) -> float:
    cpu = telemetry.get("cpu_percent", 0)
    memory = telemetry.get("memory", 0) / (1024 * 1024)  # bytes → MB
    signed = 1 if telemetry.get("file_signature_verified", True) else 0
    known_bad = 1 if "mimikatz" in telemetry.get("process_name", "").lower() else 0
    history = telemetry.get("previous_violations", 0)

    # Weighted sum
    z = (
        BETA["intercept"]
        + BETA["cpu"] * cpu
        + BETA["memory"] * memory
        + BETA["signed"] * signed
        + BETA["known_bad"] * known_bad
        + BETA["history"] * history
    )

    trust_prob = sigmoid(z)
    return trust_prob  # ∈ [0, 1]

def reload_rules():
    global loaded_rules
    try:
        rules_logger.info("Attempting to reload stateless rules.")
        candidate_rules = load_rules()
        validation_errors = validate_stateless_rules(candidate_rules)

        if validation_errors:
            rules_logger.error("Stateless rule validation failed:")
            for err in validation_errors:
                rules_logger.error(f"  - {err}")
            return False

        loaded_rules = candidate_rules
        rules_logger.info("Stateless rules reloaded successfully.")
        return True

    except Exception as e:
        rules_logger.error(f"Failed to reload stateless rules: {e}")
        return False


def get_loaded_rules():
    return loaded_rules
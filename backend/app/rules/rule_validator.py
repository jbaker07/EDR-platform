def validate_stateless_rules(rules: list):
    errors = []
    allowed_operators = [">", "<", ">=", "<=", "==", "!=", "contains", "startswith", "endswith", "regex"]

    for idx, rule in enumerate(rules):
        if not isinstance(rule, dict):
            errors.append(f"Rule at index {idx} is not a dictionary.")
            continue

        # Checks the schema rules_engine.evaluate_rule reads: conditions, not an
        # 'expression' string. The old 'expression' check failed every shipped rule,
        # so reload_rules() never loaded any.
        for key in ("id", "name", "description"):
            if not rule.get(key):
                errors.append(f"Rule at index {idx} missing '{key}'.")

        if rule.get("logic", "AND") not in ("AND", "OR"):
            errors.append(f"Rule at index {idx} has invalid 'logic' (must be AND or OR).")

        if not isinstance(rule.get("target_os"), list):
            errors.append(f"Rule at index {idx} missing 'target_os' list.")

        conditions = rule.get("conditions")
        if not isinstance(conditions, list) or not conditions:
            errors.append(f"Rule at index {idx} missing non-empty 'conditions' list.")
            continue

        for c_idx, cond in enumerate(conditions):
            if not isinstance(cond, dict) or not cond.get("field"):
                errors.append(f"Rule at index {idx}, condition {c_idx} missing 'field'.")
            elif cond.get("operator") not in allowed_operators:
                errors.append(f"Rule at index {idx}, condition {c_idx} has unknown operator {cond.get('operator')!r}.")

    return errors

def validate_multi_endpoint_rules(rules: list):
    errors = []
    allowed_logics = ["AND", "OR", "SEQUENCE"]

    for idx, rule in enumerate(rules):
        if not isinstance(rule, dict):
            errors.append(f"Multi-endpoint rule at index {idx} is not a dictionary.")
            continue

        if "id" not in rule or not rule["id"]:
            errors.append(f"Multi-endpoint rule at index {idx} missing 'id'.")

        if "description" not in rule or not rule["description"]:
            errors.append(f"Multi-endpoint rule at index {idx} missing 'description'.")

        if "pattern" not in rule or not isinstance(rule["pattern"], list):
            errors.append(f"Multi-endpoint rule at index {idx} missing valid 'pattern' list.")

        if "logic" in rule and rule["logic"] not in allowed_logics:
            errors.append(f"Multi-endpoint rule at index {idx} has invalid 'logic' (must be AND, OR, SEQUENCE).")

        if "window_seconds" not in rule or not isinstance(rule["window_seconds"], int):
            errors.append(f"Multi-endpoint rule at index {idx} missing 'window_seconds' integer.")

    return errors

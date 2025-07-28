def validate_stateless_rules(rules: list):
    errors = []
    allowed_operators = [">", "<", ">=", "<=", "==", "!=", "contains", "startswith", "endswith", "regex"]

    for idx, rule in enumerate(rules):
        if not isinstance(rule, dict):
            errors.append(f"Rule at index {idx} is not a dictionary.")
            continue

        if "id" not in rule or not rule["id"]:
            errors.append(f"Rule at index {idx} missing 'id'.")

        if "description" not in rule or not rule["description"]:
            errors.append(f"Rule at index {idx} missing 'description'.")

        if "expression" not in rule or not rule["expression"]:
            errors.append(f"Rule at index {idx} missing 'expression'.")

        # (Optional advanced check): Could even parse expressions for basic syntax safety

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

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

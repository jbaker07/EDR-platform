from backend.app.memory.memory_manager import MemoryManager
import json
import os 
from backend.app.rules.rule_validator import validate_multi_endpoint_rules
from backend.app.logs.logger import rules_logger

MULTI_ENDPOINT_RULES_PATH = "backend/app/memory/multi_endpoint_rules.json"
loaded_multi_endpoint_rules = []

def reload_multi_endpoint_rules():
    global loaded_multi_endpoint_rules
    try:
        candidate_rules = load_multi_endpoint_rules()
        validation_errors = validate_multi_endpoint_rules(candidate_rules)

        if validation_errors:
            print("[!] Multi-endpoint rule validation failed:")
            for err in validation_errors:
                print("  -", err)
            return False

        loaded_multi_endpoint_rules = candidate_rules
        print("[+] Multi-endpoint rules reloaded successfully.")
        return True
    except Exception as e:
        print(f"[!] Failed to reload multi-endpoint rules: {e}")
        return False
    
def find_correlation(memory_manager: MemoryManager, endpoint_id: str, correlation_rules: list):
    matched_correlations = []
    recent_events = memory_manager.get_recent_events(endpoint_id)

    for rule in correlation_rules:
        pattern = rule.get("pattern", [])
        logic = rule.get("logic", "AND")
        window_seconds = rule.get("window_seconds", 900)  # default 15 minutes

        # Filter only events within the time window
        events_in_window = memory_manager.get_recent_events(endpoint_id, window_seconds)

        pattern_matches = []
        for condition in pattern:
            field = condition["field"]
            expected_value = condition["value"]

            match = any(event.get(field) == expected_value for event in events_in_window)
            pattern_matches.append(match)

        if logic == "AND" and all(pattern_matches):
            matched_correlations.append({
                "correlation_id": rule["id"],
                "description": rule["description"],
                "severity": rule.get("severity", "high"),
                "events": events_in_window
            })
        elif logic == "OR" and any(pattern_matches):
            matched_correlations.append({
                "correlation_id": rule["id"],
                "description": rule["description"],
                "severity": rule.get("severity", "high"),
                "events": events_in_window
            })

    return matched_correlations

def load_multi_endpoint_rules(path=MULTI_ENDPOINT_RULES_PATH):
    if not os.path.exists(path):
        raise FileNotFoundError(f"Multi-endpoint rules file not found: {path}")
    with open(path, "r") as f:
        rules = json.load(f)
    return rules

def reload_multi_endpoint_rules():
    global loaded_multi_endpoint_rules
    try:
        rules_logger.info("Attempting to reload multi-endpoint rules.")
        candidate_rules = load_multi_endpoint_rules()
        validation_errors = validate_multi_endpoint_rules(candidate_rules)

        if validation_errors:
            rules_logger.error("Multi-endpoint rule validation failed:")
            for err in validation_errors:
                rules_logger.error(f"  - {err}")
            return False

        loaded_multi_endpoint_rules = candidate_rules
        rules_logger.info("Multi-endpoint rules reloaded successfully.")
        return True

    except Exception as e:
        rules_logger.error(f"Failed to reload multi-endpoint rules: {e}")
        return False

def get_loaded_multi_endpoint_rules():
    return loaded_multi_endpoint_rules


def find_multi_endpoint_correlation(memory_manager, multi_endpoint_rules_path="backend/app/memory/multi_endpoint_rules.json"):
    matched_chains = []

    # Load multi-endpoint correlation rules
    with open(multi_endpoint_rules_path, "r") as f:
        rules = json.load(f)

    # Get all events across all endpoints
    all_events = memory_manager.get_all_recent_events(window_seconds=1800)  # Last 30 minutes for now

    for rule in rules:
        pattern = rule.get("pattern", [])
        logic = rule.get("logic", "SEQUENCE")
        window_seconds = rule.get("window_seconds", 1800)

        matched = []
        current_index = 0

        for condition in pattern:
            field = condition.get("field", "event_type")
            expected_value = condition.get("value")

            found = False
            while current_index < len(all_events):
                event = all_events[current_index]
                if event.get(field) == expected_value:
                    matched.append(event)
                    found = True
                    current_index += 1
                    break
                current_index += 1

            if not found:
                break  # Pattern broken

        if len(matched) == len(pattern):
            matched_chains.append({
                "correlation_id": rule["id"],
                "description": rule["description"],
                "severity": rule.get("severity", "critical"),
                "matched_events": matched
            })

    return matched_chains

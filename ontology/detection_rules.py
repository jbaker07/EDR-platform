import json
import logging
import os

RULE_FILE_PATH = os.path.join(os.path.dirname(__file__), "detection_rules.json")

# Guard the module-level load: detection_rules.json is currently malformed
# (two concatenated JSON objects), which previously crashed app import. Degrade
# to an empty rule set rather than taking the whole API down. The consumer below
# handles an empty list gracefully.
try:
    with open(RULE_FILE_PATH, "r") as f:
        _loaded = json.load(f)
    # The consumer iterates a list of rule dicts; tolerate a {"rules": [...]} wrapper.
    detection_rules = _loaded.get("rules", []) if isinstance(_loaded, dict) else _loaded
except Exception as e:
    logging.getLogger("edr.ontology").warning(
        "Could not load detection rules from %s (%s); using empty rule set.",
        RULE_FILE_PATH, e,
    )
    detection_rules = []

def apply_detection_rules(event, tags):
    """
    Apply detection rules to the event and its tags.
    Returns a list of rules that match.
    """
    matched_rules = []
    for rule in detection_rules:
        context = rule.get("context", "")
        if all(req in tags + [event.get("event_type", "")] for req in context.split("+")):
            matched_rules.append(rule)
    return matched_rules

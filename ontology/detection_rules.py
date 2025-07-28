import json
import os

RULE_FILE_PATH = os.path.join(os.path.dirname(__file__), "detection_rules.json")

with open(RULE_FILE_PATH, "r") as f:
    detection_rules = json.load(f)

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

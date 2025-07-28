
import json
import re

class TrustRuleParser:
    def __init__(self, rule_file_path):
        with open(rule_file_path, 'r') as f:
            self.rules = json.load(f)

    def match_rules(self, input_string, mode="minimal_mode"):
        matched_rules = []
        for category, rules in self.rules.get(mode, {}).items():
            if isinstance(rules, list):
                for rule in rules:
                    pattern = rule.get("pattern")
                    if pattern and re.search(re.escape(pattern), input_string, re.IGNORECASE):
                        enriched_match = rule.copy()
                        enriched_match["category"] = category
                        matched_rules.append(enriched_match)
        return matched_rules

    def extract_tags(self, matches):
        tags = set()
        for match in matches:
            tags.update(match.get("semantic_tags", []))
        return list(tags)

    def calculate_impact(self, matches):
        return sum([match.get("impact_weight", 0.5) for match in matches])

    def should_trigger_replay(self, matches):
        return any(match.get("replay_hook_trigger", False) for match in matches)

    def fallback_profiles(self, matches):
        return list({m.get("unknown_risk_profile_fallback") for m in matches if m.get("confidence_level") == "low"})

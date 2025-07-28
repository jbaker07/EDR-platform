import json
import os
from typing import Dict, List, Any

BASE_PATH = os.path.dirname(__file__)
SEMANTIC_TAGS_PATH = os.path.join(BASE_PATH, "semantic_tags.json")
THREAT_PATTERNS_PATH = os.path.join(BASE_PATH, "threat_patterns.json")
UNKNOWN_PROFILES_PATH = os.path.join(BASE_PATH, "unknown_risk_profiles.json")

def load_json_file(path: str) -> Any:
    with open(path, "r") as f:
        return json.load(f)

def match_semantic_tags() -> Dict[str, Dict[str, Any]]:
    return load_json_file(SEMANTIC_TAGS_PATH)

def match_threat_patterns() -> Dict[str, List[str]]:
    return load_json_file(THREAT_PATTERNS_PATH)

def match_unknown_risk_profiles() -> Dict[str, Dict[str, Any]]:
    return load_json_file(UNKNOWN_PROFILES_PATH)

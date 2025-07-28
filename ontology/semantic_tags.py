import json
import os

TAG_FILE_PATH = os.path.join(os.path.dirname(__file__), "semantic_tags.json")

with open(TAG_FILE_PATH, "r") as f:
    semantic_tags = json.load(f)

def tag_event(event):
    """
    Check which semantic tags apply to the given event based on its fields.
    Returns a list of matched tags.
    """
    matched_tags = []
    for tag in semantic_tags:
        required_fields = tag.get("required_fields", [])
        if all(field in event.get("fields", {}) for field in required_fields):
            matched_tags.append(tag["tag"])
    return matched_tags

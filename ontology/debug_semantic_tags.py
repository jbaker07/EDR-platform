import json

with open("semantic_tags.json", "r") as f:
    data = json.load(f)

print("Top-level type:", type(data))
if isinstance(data, list):
    print("First element:", data[0])
elif isinstance(data, dict):
    print("First key-value:", list(data.items())[0])

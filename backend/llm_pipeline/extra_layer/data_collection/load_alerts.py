import json

def load_alerts(file_path: str):
    with open(file_path, 'r') as f:
        alerts = json.load(f)
    return alerts

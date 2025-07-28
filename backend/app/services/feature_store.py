# backend/app/services/feature_store.py

feature_store = {}  # Key: pid or alert_id, Value: dict with 'features' and 'feature_names'

def store_features(alert_id: int, features: list, feature_names: list):
    feature_store[alert_id] = {
        "features": features,
        "feature_names": feature_names
    }

def get_features(alert_id: int):
    return feature_store.get(alert_id)

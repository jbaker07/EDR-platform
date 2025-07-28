import json
import datetime

# Default thresholds (fallback)
DEFAULTS = {
    "developer": 0.35,
    "executive": 0.25,
    "contractor": 0.30,
    "root": 0.20,
    "server": 0.40,
    "container": 0.38,
    "off_hours_multiplier": 0.8  # Stricter at night
}

def load_policy(path="config/role_thresholds.json"):
    try:
        with open(path, "r") as f:
            return json.load(f)
    except:
        return DEFAULTS

def is_off_hours():
    now = datetime.datetime.now()
    return now.hour < 6 or now.hour > 20

def get_adaptive_threshold(user_role, host_type, policy=None):
    policy = policy or DEFAULTS

    base_role_thresh = policy.get(user_role, DEFAULTS.get(user_role, 0.3))
    base_host_thresh = policy.get(host_type, DEFAULTS.get(host_type, 0.4))

    combined = min(base_role_thresh, base_host_thresh)

    if is_off_hours():
        combined *= policy.get("off_hours_multiplier", 0.8)

    return round(combined, 3)

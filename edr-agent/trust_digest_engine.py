import hashlib
import json
from typing import Dict, Any

TRUST_DIGEST_DB = {}

def compute_digest(feature_map: Dict[str, Any]) -> str:
    """
    Generate a SHA256 hash of the sorted feature map — the behavioral fingerprint.
    """
    sorted_features = json.dumps(feature_map, sort_keys=True).encode('utf-8')
    return hashlib.sha256(sorted_features).hexdigest()

def compare_digest(entity_id: str, new_digest: str) -> bool:
    """
    Check if digest for an entity has changed. Returns True if it's a *new* behavior.
    """
    old_digest = TRUST_DIGEST_DB.get(entity_id)
    if old_digest is None:
        TRUST_DIGEST_DB[entity_id] = new_digest
        return True  # First time seen

    if old_digest != new_digest:
        TRUST_DIGEST_DB[entity_id] = new_digest
        return True  # Changed behavior
    return False  # No change

def track_and_score(entity_id: str, feature_map: Dict[str, Any]) -> float:
    """
    Digest → Change → Score impact
    """
    digest = compute_digest(feature_map)
    changed = compare_digest(entity_id, digest)

    if changed:
        print(f"[Digest] Behavior shift for {entity_id}")
        return -0.2  # Penalize for deviation
    else:
        return 0.1   # Reward consistency

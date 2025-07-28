import json
import uuid
import os
from datetime import datetime
from typing import Dict, Any
from pathlib import Path
from sklearn.metrics.pairwise import cosine_similarity
import numpy as np

DB_PATH = "data/attacker_loyalty_db.json"
os.makedirs(os.path.dirname(DB_PATH), exist_ok=True)

def load_loyalty_db() -> Dict[str, Any]:
    if not os.path.exists(DB_PATH):
        return {}
    with open(DB_PATH, 'r') as f:
        return json.load(f)

def save_loyalty_db(db: Dict[str, Any]):
    with open(DB_PATH, 'w') as f:
        json.dump(db, f, indent=2)

def normalize_vector(vec: list) -> list:
    norm = np.linalg.norm(vec)
    return list((np.array(vec) / norm).round(6)) if norm != 0 else vec

def cosine_distance(vec1: list, vec2: list) -> float:
    if not vec1 or not vec2:
        return 0.0
    try:
        return cosine_similarity([vec1], [vec2])[0][0]
    except:
        return 0.0

def track_attacker_session(session: Dict[str, Any]) -> Dict[str, Any]:
    db = load_loyalty_db()
    new_decay_vector = normalize_vector(session.get("decay_vector", []))
    similarity_threshold = 0.92
    attacker_id = None
    best_score = 0

    for aid, profile in db.items():
        prev_vector = normalize_vector(profile.get("decay_vector", []))
        similarity = cosine_distance(prev_vector, new_decay_vector)
        tag_overlap = len(set(profile.get("semantic_tags", [])) & set(session.get("semantic_tags", [])))
        match_score = similarity * 0.7 + (tag_overlap / 10.0) * 0.3

        if match_score > similarity_threshold and match_score > best_score:
            attacker_id = aid
            best_score = match_score

    if not attacker_id:
        attacker_id = str(uuid.uuid4())

    profile = db.get(attacker_id, {
        "attacker_id": attacker_id,
        "origin_ips": [],
        "first_seen": datetime.utcnow().isoformat(),
        "replay_ids": [],
        "session_ids": [],
        "semantic_tags": [],
        "cross_host": False,
    })

    profile["last_seen"] = datetime.utcnow().isoformat()
    profile["decay_vector"] = new_decay_vector
    profile["semantic_tags"] = list(set(profile["semantic_tags"] + session.get("semantic_tags", [])))
    profile["origin_ips"] = list(set(profile["origin_ips"] + [session.get("ip_address")]))
    profile["replay_ids"] = list(set(profile["replay_ids"] + [session.get("replay_id")]))
    profile["session_ids"] = list(set(profile["session_ids"] + [session.get("session_id")]))
    profile["gnn_clusters"] = list(set(profile.get("gnn_clusters", []) + [session.get("gnn_cluster")]))
    profile["roles"] = list(set(profile.get("roles", []) + [session.get("role_profile")]))
    
    db[attacker_id] = profile
    save_loyalty_db(db)

    return {
        "attacker_id": attacker_id,
        "matched_score": round(best_score, 4),
        "semantic_tags": profile["semantic_tags"]
    }

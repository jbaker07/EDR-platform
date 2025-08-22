import json
from collections import defaultdict
from typing import Dict, List
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.cluster import KMeans

REPLAY_PATH = "replays/confirmed_sessions.json"
TAG_DB_PATH = "config/semantic_tags.json"
OUTPUT_PATH = "suggestions/new_tags_suggested.json"

NUM_CLUSTERS = 12
TAG_THRESHOLD = 0.65  # Consider sessions for tagging only if GNN score exceeds this

def extract_behavior_phrases(session: Dict) -> str:
    # Combine command_line, file_accesses, and network domains into a text blob
    cmd = session.get("command_line", "")
    files = " ".join(session.get("file_accesses", []))
    net = " ".join(session.get("domains", []))
    return f"{cmd} {files} {net}"

def run_tag_discovery():
    with open(REPLAY_PATH, "r") as f:
        sessions = json.load(f)
    with open(TAG_DB_PATH, "r") as f:
        tag_db = json.load(f)

    untagged_phrases = []
    metadata_map = []

    for session in sessions:
        tags = session.get("tags", [])
        score = session.get("gnn_score", 0)
        if score >= TAG_THRESHOLD and len(tags) <= 1:
            phrase = extract_behavior_phrases(session)
            if phrase.strip():
                untagged_phrases.append(phrase)
                metadata_map.append({
                    "session_id": session.get("session_id", ""),
                    "gnn_score": score,
                    "existing_tags": tags,
                    "process": session.get("process_name", "")
                })

    print(f"[🔎] Collected {len(untagged_phrases)} suspicious but untagged session phrases.")

    if len(untagged_phrases) < NUM_CLUSTERS:
        print("Not enough data to generate clusters.")
        return

    tfidf = TfidfVectorizer(stop_words="english", max_features=500)
    vectors = tfidf.fit_transform(untagged_phrases)
    km = KMeans(n_clusters=NUM_CLUSTERS, random_state=42)
    labels = km.fit_predict(vectors)

    suggestions = defaultdict(list)

    for i, label in enumerate(labels):
        suggestions[f"cluster_{label}"].append(metadata_map[i])

    tag_suggestions = []
    for cluster, items in suggestions.items():
        top_cmd = items[0]["process"]
        example_tags = items[0]["existing_tags"]
        tag_suggestions.append({
            "proposed_tag": f"suspicious_{top_cmd}_{cluster}",
            "description": f"Auto-generated tag from cluster of {len(items)} similar sessions.",
            "example_sessions": [item["session_id"] for item in items],
            "inferred_from_tags": example_tags
        })

    with open(OUTPUT_PATH, "w") as f:
        json.dump(tag_suggestions, f, indent=4)

    print(f"[✅] Proposed {len(tag_suggestions)} new semantic tags written to {OUTPUT_PATH}")

if __name__ == "__main__":
    run_tag_discovery()

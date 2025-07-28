
import json
from datetime import datetime

class AnchorNodeEngine:
    def __init__(self, anchor_definitions_path):
        with open(anchor_definitions_path) as f:
            self.anchor_definitions = json.load(f)

    def evaluate_node(self, node):
        matches = []
        now = datetime.utcnow().timestamp()
        for anchor in self.anchor_definitions:
            required_tags = set(anchor.get("required_tags", []))
            node_tags = set(node.get("tags", []))
            if not required_tags:
                continue
            matched_tags = required_tags.intersection(node_tags)
            confidence = len(matched_tags) / len(required_tags)

            if confidence >= anchor.get("confidence_threshold", 1.0):
                decay_penalty = 1.0
                if "last_seen" in node and (now - node["last_seen"]) > 86400:
                    decay_penalty = 0.5  # decay if anchor is stale
                score = confidence * decay_penalty
                matches.append({
                    "anchor_id": anchor["id"],
                    "anchor_type": anchor.get("anchor_type", "unknown"),
                    "confidence": confidence,
                    "score": score,
                    "matched_tags": list(matched_tags),
                    "category": anchor.get("category"),
                    "min_risk": anchor.get("min_risk")
                })
        return matches

    def label_graph(self, graph):
        for node in graph.get("nodes", []):
            results = self.evaluate_node(node)
            if results:
                top_match = max(results, key=lambda x: x["score"])
                node["is_anchor"] = True
                node["anchor_id"] = top_match["anchor_id"]
                node["anchor_type"] = top_match["anchor_type"]
                node["anchor_confidence"] = top_match["confidence"]
                node["anchor_score"] = top_match["score"]
                print(f"[ANCHOR MATCH] Node tagged with anchor {top_match['anchor_id']} at confidence {top_match['confidence']:.2f}")
        return graph

def check_anchor_trigger(node, anchor_engine):
    """
    External utility to check if a given node triggers any anchor conditions.
    Returns a list of matched anchor IDs.
    """
    matches = []
    now = datetime.utcnow().timestamp()
    for anchor in anchor_engine.anchor_definitions:
        required_tags = set(anchor.get("required_tags", []))
        node_tags = set(node.get("tags", []))
        if not required_tags:
            continue
        matched_tags = required_tags.intersection(node_tags)
        confidence = len(matched_tags) / len(required_tags)

        if confidence >= anchor.get("confidence_threshold", 1.0):
            matches.append(anchor["id"])
    return matches

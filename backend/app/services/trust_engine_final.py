
# === Existing Imports ===
import time

from app.services.robust_enhancer import normalize_command, decode_base64_segment, extract_chain_features, detect_suspicious_conjunctions

def preprocess_input(input_str):
    enriched = {}
    normalized = normalize_command(input_str)
    enriched["normalized_command"] = normalized
    enriched["decoded_segments"] = decode_base64_segment(normalized)
    enriched["chain_features"] = extract_chain_features(normalized)
    enriched["suspicious_conjunction"] = detect_suspicious_conjunctions(normalized)
    return enriched

import os
import json
import re
import logging
from datetime import datetime
from typing import Dict, Any, Tuple, List
from replay_engine.queues.replay_queue_writer import write_to_replay_queue
from app.services.trust_rule_parser import TrustRuleParser
from app.memory.memory_manager import memory_manager
from app.services.db import get_db_session
from app.models.agent import Agent

from app.services.trust_gnn_engine import score_graph
from ontology.semantic_tags import tag_event
from ontology.detection_rules import apply_detection_rules

# === New Module Integrations ===
from graph_output.anchor_node_engine import check_anchor_trigger
from graph_output.trust_decay_engine import apply_decay
from graph_output.trust_retrainer import retrain_if_needed
from graph_output.replay_sync_validator import validate_and_sync_replay

import networkx as nx
from gnn_score_engine import score_node_with_gnn
from gnn_node_updater import update_node_score
from gnn_hook import run_gnn_hook
from trust_hook import evaluate_trust_payload

# Load telemetry graph
telemetry_graph = nx.read_gpickle("backend/gnn_pipeline/data/processed/telemetry_graph.gpickle")

def process_node(node_id: str, telemetry_data: dict):
    node_score = score_node_with_gnn(telemetry_graph, node_id)
    update_node_score(telemetry_graph, node_id, node_score)
    gnn_summary = run_gnn_hook(node_score)
    trust_output = evaluate_trust_payload(telemetry_data)
    return {"gnn_summary": gnn_summary, "trust_output": trust_output}

# === (Your existing logic continues here) ===

import time

from app.services.robust_enhancer import normalize_command, decode_base64_segment, extract_chain_features, detect_suspicious_conjunctions

def preprocess_input(input_str):
    enriched = {}
    normalized = normalize_command(input_str)
    enriched["normalized_command"] = normalized
    enriched["decoded_segments"] = decode_base64_segment(normalized)
    enriched["chain_features"] = extract_chain_features(normalized)
    enriched["suspicious_conjunction"] = detect_suspicious_conjunctions(normalized)
    return enriched

import os
import json
import re
import logging
from datetime import datetime
from typing import Dict, Any, Tuple, List
from replay_engine.queues.replay_queue_writer import write_to_replay_queue
from app.services.trust_rule_parser import TrustRuleParser
from app.memory.memory_manager import memory_manager
from app.services.db import get_db_session
from app.models.agent import Agent

from app.services.trust_gnn_engine import score_graph
from ontology.semantic_loader import (
    match_semantic_tags,
    match_threat_patterns,
    match_unknown_risk_profiles
)

semantic_tags = match_semantic_tags()
threat_patterns = match_threat_patterns()
unknown_profiles = match_unknown_risk_profiles()

# ——— CONFIGURATION ———
RULE_FILE_PATH = "backend/app/services/expanded_rules_combined.json"
rule_parser = TrustRuleParser(RULE_FILE_PATH)

RULES_PATH = os.path.join(os.path.dirname(__file__), "expanded_rules_combined.json")
MAX_TRUST = 100.0
GNN_WEIGHT = 30.0
DECAY_WEIGHT = 1.25
RISK_WEIGHT_MULTIPLIER = 1.25
ATTESTATION_PENALTY = 25.0
MISSING_HASH_PENALTY = 15.0
FORENSIC_TRIGGER = 40.0
GNN_TRIGGER = 0.85

MINIMAL_WEIGHTS = {
    "lolbins": 10, "dangerous_extensions": 7, "evasion_obfuscation_cmds": 6,
    "memory_injection_keywords": 9, "recon_privilege_cred_access": 8,
    "dual_use_binary_paths": 5, "network_behavior": 7, "masquerading_names": 6,
    "script_automation_abuse": 5,
}
RESOURCE_ABUSE_WEIGHTS = {"cpu": 8, "memory": 6, "off_hours": 4}
FORENSIC_WEIGHTS = {
    "credential_access": 12, "privilege_escalation": 10, "internal_recon": 8,
    "dual_use_tools": 9, "memory_injection": 11, "persistence_indicators": 7,
    "malicious_scripts": 8,
}

logger = logging.getLogger(__name__)

def load_rules() -> Dict[str, Dict[str, Any]]:
    raw = json.load(open(RULES_PATH))
    compiled = {}
    for mode, buckets in raw.items():
        compiled[mode] = {}
        for bucket, entry in buckets.items():
            if isinstance(entry, list):
                if all(isinstance(pat, dict) and "pattern" in pat for pat in entry):
                    compiled[mode][bucket] = [
                        re.compile(r"\b" + re.escape(p["pattern"]) + r"\b", re.IGNORECASE)
                        for p in entry
                    ]
                else:
                    compiled[mode][bucket] = [
                        re.compile(r"\b" + re.escape(pat) + r"\b", re.IGNORECASE)
                        for pat in entry
                    ]
            else:
                compiled[mode][bucket] = entry
    return compiled


RULES = load_rules()

RULE_FILE_PATH = "backend/app/services/expanded_rules_combined.json"
rule_parser = TrustRuleParser(RULE_FILE_PATH)


# Initialize anchor engine
with open("anchor_definitions.json", "r") as f:
    anchor_definitions = json.load(f)
anchor_engine = anchor_definitions  # You may later wrap this in a class if needed

# Simulated event (replace with actual logic that reads/ingests a telemetry event)
event = {
    "timestamp": time.time(),
    "tags": ["privilege_escalation", "persistence"],
    "exe": "privilege_escalation_tool",
    "cmdline": "--gain-root",
    "platform": "macos",
    "session_id": "abc123"
}

# Now call functions safely
tagged_event = tag_event(event)
updated_event = apply_detection_rules(tagged_event)
matched_anchors = check_anchor_trigger(event, anchor_engine)
trust_delta = apply_decay(updated_event)
retrain_if_needed()
validate_and_sync_replay(event)

# === Integrated Function Calls ===

# Tag the event using semantic tags
event = tag_event(event)

# Apply detection rules to the event
rule_matches = apply_detection_rules(event)

# Check for anchor triggers
anchor_matches = check_anchor_trigger(event, anchor_engine)

# Apply trust decay based on current rule matches
event = apply_decay(event, rule_matches)

# Validate and sync replay queue if required
validate_and_sync_replay(event)

# Retrain model if conditions are met
retrain_if_needed(event)

def evaluate_trust_from_input(input_text, mode="minimal_mode"):
    matches = rule_parser.match_rules(input_text, mode=mode)
    tags = rule_parser.extract_tags(matches)
    impact_score = rule_parser.calculate_impact(matches)
    trigger_replay = rule_parser.should_trigger_replay(matches)
    fallback_profiles = rule_parser.fallback_profiles(matches)

    if trigger_replay:
        write_to_replay_queue({
            "input": input_text,
            "tags": tags,
            "impact_score": impact_score,
            "mode": mode,
            "timestamp": time.time()
    })
    # Default trust score (100 is max)
    trust_score = 100.0 - (impact_score * 100.0)

    response = {
        "trust_score": max(0, round(trust_score, 2)),
        "semantic_tags": tags,
        "replay_triggered": trigger_replay,
        "fallback_profiles_used": fallback_profiles,
        "matches": matches,
    }

    # Optional: Attach explanations
    if fallback_profiles:
        response["explanation"] = "Low-confidence rules detected, fallback profile(s) applied."
    elif trigger_replay:
        response["explanation"] = "High-impact pattern detected, replay hook triggered."
    else:
        response["explanation"] = "Normal pattern evaluation completed."

    return response

def evaluate_input_against_rules(input_text, mode="minimal_mode"):
    matches = rule_parser.match_rules(input_text, mode=mode)
    tags = rule_parser.extract_tags(matches)
    impact_score = rule_parser.calculate_impact(matches)
    trigger_replay = rule_parser.should_trigger_replay(matches)
    fallbacks = rule_parser.fallback_profiles(matches)

    return {
        "matches": matches,
        "semantic_tags": tags,
        "impact_score": impact_score,
        "trigger_replay": trigger_replay,
        "fallback_profiles": fallbacks
    }

def score_minimal(event: Dict[str, Any]) -> Tuple[float, List[str]]:
    score = 0.0
    fired = []
    cmd = event.get("cmd", "") or ""
    cpu = float(event.get("cpu_percent", 0.0))
    mem_mb = float(event.get("memory", 0.0)) / (1024 * 1024)
    rules = RULES["minimal_mode"]

    for bucket, weight in MINIMAL_WEIGHTS.items():
        for rx in rules.get(bucket, []):
            if rx.search(cmd):
                score += weight
                fired.append(bucket)
                break

    abuse_cfg = rules.get("resource_abuse", {})
    if cpu > abuse_cfg.get("cpu_threshold", float("inf")):
        score += RESOURCE_ABUSE_WEIGHTS["cpu"]
        fired.append("cpu_threshold")
    if mem_mb > abuse_cfg.get("memory_threshold_mb", float("inf")):
        score += RESOURCE_ABUSE_WEIGHTS["memory"]
        fired.append("memory_threshold")
    try:
        ts = int(event.get("timestamp", datetime.utcnow().timestamp()))
        hour = datetime.utcfromtimestamp(ts).strftime("%H")
        if hour in abuse_cfg.get("off_hours", []):
            score += RESOURCE_ABUSE_WEIGHTS["off_hours"]
            fired.append("off_hours")
    except Exception as e:
        logger.debug(f"Off-hours check failed: {e}")

    return min(score, MAX_TRUST), fired

def score_forensic(event: Dict[str, Any]) -> Tuple[float, List[str]]:
    base_score, fired = score_minimal(event)
    rules = RULES["forensic_mode"]
    for bucket, weight in FORENSIC_WEIGHTS.items():
        for rx in rules.get(bucket, []):
            if rx.search(event.get("cmd", "") or ""):
                base_score += weight
                fired.append(bucket)
                break
    return min(base_score, MAX_TRUST), fired

def calculate_trust_score(hostname: str, window_seconds: int = 3600) -> Dict[str, Any]:
    trust = MAX_TRUST
    now = datetime.utcnow()
    event = memory_manager.get_events_for_host(hostname, window_seconds)
    decay_sum = 0.0
    fired_buckets = []

    for ev in event:
        risk = float(ev.get("risk_score", 0.0))
        age = (now - datetime.utcfromtimestamp(int(ev.get("timestamp", now.timestamp())))).total_seconds()
        decay = max(0.0, 1.0 - (age / window_seconds))
        decay_sum += (risk * decay * DECAY_WEIGHT)
        _, buckets = score_forensic(ev)
        fired_buckets.extend(buckets)

    trust -= decay_sum * RISK_WEIGHT_MULTIPLIER
    gnn_score = score_graph(event)
    trust -= gnn_score * GNN_WEIGHT

    session = get_db_session()
    agent = session.query(Agent).filter_by(hostname=hostname).first()
    if agent:
        if not agent.attested:
            trust -= ATTESTATION_PENALTY
        if not agent.binary_hash:
            trust -= MISSING_HASH_PENALTY

    trust = max(0.0, min(trust, MAX_TRUST))
    escalate = trust < FORENSIC_TRIGGER or gnn_score > GNN_TRIGGER
    tags = match_semantic_tags(event)
    patterns = match_threat_patterns(event)

    return {
        "final_trust_score": trust,
        "fired_buckets": list(set(fired_buckets)),
        "gnn_score": gnn_score,
        "adjusted_gnn_weight": gnn_score * GNN_WEIGHT,
        "mode_escalated": escalate,
        "replay_required": escalate,
        "tags": tags,
        "threat_patterns": patterns
    }

# === Production Enhancements Added ===

# 1. Exception handling for key access
def safe_get(dct, key, default=None):
    try:
        return dct[key]
    except KeyError:
        return default

# 2. Anchor trust penalty
def apply_anchor_penalty(trust_score, enriched_payload):
    anchors = enriched_payload.get('activated_anchors', [])
    if anchors:
        trust_score -= 10 * len(anchors)
    return trust_score

# 3. Optional audit logger
ENABLE_AUDIT = True

def save_trust_decision_to_disk(trust_payload, path='/tmp/trust_audit.log'):
    with open(path, 'a') as f:
        f.write(json.dumps(trust_payload) + '\n')

# 4. Decay engine integration
try:
    from graph_output.trust_decay_engine import apply_decay
except ImportError:
    def apply_decay(payload, base_risk, previous_trust):
        return base_risk, 100 - base_risk  # fallback if decay engine missing

# Example patch inside evaluation logic (must be manually inserted):
# trust_score = apply_anchor_penalty(trust_score, enriched_payload)
# risk_score, trust_score = apply_decay(enriched_payload, risk_score, previous_trust)
# if ENABLE_AUDIT: save_trust_decision_to_disk(trust_payload)

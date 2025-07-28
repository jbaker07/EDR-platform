import requests
import json
import time
from trust_engine_final import evaluate_trust_from_input
from shap_explainer import explain_decision
from trust_digest_engine import compute_behavior_digest

# Supported API targets (example config)
SUPPORTED_INTEGRATIONS = {
    "crowdstrike": {
        "base_url": "https://api.crowdstrike.com",
        "token": "CROWDSTRIKE_API_TOKEN",
        "fetch_path": "/devices/queries/devices/v1",
        "data_path": "/devices/entities/devices/v1",
    },
    "sentinelone": {
        "base_url": "https://api.sentinelone.com/web/api/v2.1",
        "token": "SENTINELONE_API_TOKEN",
        "fetch_path": "/agents",
    },
    "splunk": {
        "base_url": "https://splunk.company.com:8089",
        "token": "SPLUNK_API_TOKEN",
        "fetch_path": "/services/search/jobs",
    }
}

def fetch_external_events(platform_key):
    if platform_key not in SUPPORTED_INTEGRATIONS:
        raise Exception(f"Unsupported integration: {platform_key}")

    cfg = SUPPORTED_INTEGRATIONS[platform_key]
    headers = {
        "Authorization": f"Bearer {cfg['token']}",
        "Content-Type": "application/json"
    }

    url = cfg["base_url"] + cfg["fetch_path"]
    try:
        response = requests.get(url, headers=headers)
        if response.status_code != 200:
            print(f"[!] API Error from {platform_key}: {response.status_code}")
            return []
        return response.json()
    except Exception as e:
        print(f"[!] Failed to fetch data from {platform_key}: {e}")
        return []

def process_external_event(event, platform="unknown"):
    telemetry_input = extract_fields(event, platform)
    trust_result = evaluate_trust_from_input(telemetry_input)
    shap_explanation = explain_decision(telemetry_input)
    digest = compute_behavior_digest(telemetry_input)

    enriched = {
        "original_event": event,
        "platform": platform,
        "trust_score": trust_result.get("trust_score"),
        "trust_level": trust_result.get("trust_level"),
        "trust_tags": trust_result.get("tags", []),
        "digest": digest,
        "shap_explanation": shap_explanation,
        "analysis_timestamp": time.time()
    }

    return enriched

def extract_fields(event, platform):
    # Normalize and extract minimal fields required for trust eval
    try:
        if platform == "crowdstrike":
            return {
                "process_name": event.get("product_type_desc", "unknown"),
                "cpu": float(event.get("cpu_utilization", 0)),
                "memory": float(event.get("memory_usage", 0)),
                "user": event.get("user_name", "unknown"),
                "command_line": event.get("cmdline", "")
            }
        elif platform == "sentinelone":
            return {
                "process_name": event.get("processName", "unknown"),
                "cpu": float(event.get("cpuUsage", 0)),
                "memory": float(event.get("memoryUsage", 0)),
                "user": event.get("username", "unknown"),
                "command_line": event.get("commandLine", "")
            }
        elif platform == "splunk":
            raw = event.get("result", {})
            return {
                "process_name": raw.get("proc_name", "unknown"),
                "cpu": float(raw.get("cpu_pct", 0)),
                "memory": float(raw.get("mem_bytes", 0)),
                "user": raw.get("user", "unknown"),
                "command_line": raw.get("cmd", "")
            }
        else:
            return event  # fallback
    except Exception as e:
        print(f"[!] Failed to normalize event for {platform}: {e}")
        return {}

def run_overlay_audit(platform_key):
    print(f"[→] Starting overlay trust audit for: {platform_key}")
    events = fetch_external_events(platform_key)
    if not events:
        print(f"[!] No events retrieved for {platform_key}")
        return

    enriched_results = []
    for event in events:
        enriched = process_external_event(event, platform_key)
        enriched_results.append(enriched)

    # Output / log or post-process
    for result in enriched_results:
        print(json.dumps(result, indent=2))

# Example CLI use
if __name__ == "__main__":
    for key in SUPPORTED_INTEGRATIONS:
        run_overlay_audit(key)

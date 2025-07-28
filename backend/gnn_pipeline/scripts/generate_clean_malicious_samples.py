import csv
import os
import random
import json
import time
import platform

OUTPUT_PATH = "data/malicious_samples_triggering_rules.csv"
os.makedirs("data", exist_ok=True)

FIELDS = [
    "timestamp", "hostname", "endpoint_id", "os_type", "pid", "ppid",
    "process_name", "cmd", "cpu_percent", "memory", "label"
]

HOSTNAME = platform.node()
OS_TYPE = platform.system().lower()

# Load correct rules file
with open("backend/app/services/expanded_rules_combined.json") as f:
    rules = json.load(f)["minimal_mode"]

category_map = {
    "lolbins": "lolbins",
    "dangerous_extensions": "dangerous_extensions",
    "obfuscation": "evasion_obfuscation_cmds",
    "memory_injection": "memory_injection_keywords",
    "recon": "recon_privilege_cred_access",
    "dual_use": "dual_use_binary_paths",
    "network": "network_behavior",
    "masquerade": "masquerading_names",
    "scripts": "script_automation_abuse"
}

def generate_row(trigger_term, category, pid, is_resource=False):
    return {
        "timestamp": int(time.time()),
        "hostname": HOSTNAME,
        "endpoint_id": HOSTNAME,
        "os_type": OS_TYPE,
        "pid": pid,
        "ppid": 1,
        "process_name": f"{category}_agent",
        "cmd": f"{trigger_term}",  # <<< FIXED TO TRIGGER RULE MATCH
        "cpu_percent": round(random.uniform(80, 95), 2) if is_resource else round(random.uniform(1.0, 20.0), 2),
        "memory": random.randint(800_000_000, 1_500_000_000) if is_resource else random.randint(50_000_000, 300_000_000),
        "label": 1
    }

rows = []
pid = 10000

# Proper rule-based rows
for category, rule_key in category_map.items():
    patterns = rules.get(rule_key, [])
    if not patterns:
        continue
    selected = random.choices(patterns, k=50)
    for term in selected:
        rows.append(generate_row(term, category, pid))
        pid += 1

# Add 50 resource abuse examples (simulate off-hours + resource spike)
for _ in range(50):
    rows.append(generate_row("resource_abuse", "resource_abuse", pid, is_resource=True))
    pid += 1

# Write to CSV
with open(OUTPUT_PATH, "w", newline="") as f:
    writer = csv.DictWriter(f, fieldnames=FIELDS)
    writer.writeheader()
    writer.writerows(rows)

print(f"[✓] Cleaned and regenerated 500 malicious samples → {OUTPUT_PATH}")


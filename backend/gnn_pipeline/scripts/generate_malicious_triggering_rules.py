import csv
import os
import random
import json
import time
import platform

# Output path
OUTPUT_PATH = "data/malicious_samples_triggering_rules.csv"
os.makedirs("data", exist_ok=True)

# Final telemetry format (minimal mode + label)
FIELDS = [
    "timestamp", "hostname", "endpoint_id", "os_type", "pid", "ppid",
    "process_name", "cmd", "cpu_percent", "memory", "label"
]

# Use your real hostname and platform
HOSTNAME = platform.node()
OS_TYPE = platform.system().lower()

# Load real rule patterns
with open("backend/app/services/expanded_rules_combined.json") as f:
    rules = json.load(f)["minimal_mode"]

# Pattern-matching rule categories
pattern_rule_map = {
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

# Generator for pattern-based malicious rows
def generate_pattern_row(pattern: str, category: str, pid: int) -> dict:
    return {
        "timestamp": int(time.time()),
        "hostname": HOSTNAME,
        "endpoint_id": HOSTNAME,
        "os_type": OS_TYPE,
        "pid": pid,
        "ppid": 1,
        "process_name": f"{category}_agent",
        "cmd": f"/usr/bin/{pattern} --simulate",
        "cpu_percent": round(random.uniform(1.0, 30.0), 2),
        "memory": random.randint(50_000_000, 300_000_000),
        "label": 1
    }

# Generator for resource-abuse malicious rows
def generate_resource_row(pid: int) -> dict:
    fake_off_hour = int(time.time()) - random.randint(3600, 14400)
    return {
        "timestamp": fake_off_hour,
        "hostname": HOSTNAME,
        "endpoint_id": HOSTNAME,
        "os_type": OS_TYPE,
        "pid": pid,
        "ppid": 1,
        "process_name": "resource_abuse_agent",
        "cmd": "/usr/bin/stresser",
        "cpu_percent": round(random.uniform(80.0, 99.0), 2),
        "memory": random.randint(800_000_000, 1_500_000_000),
        "label": 1
    }

# Build final dataset
rows = []
pid = 10000

# Add 50 rows per pattern-based rule category
for category, rule_key in pattern_rule_map.items():
    patterns = rules.get(rule_key, [])
    if not patterns:
        continue
    sampled = random.choices(patterns, k=50)
    for p in sampled:
        rows.append(generate_pattern_row(p, category, pid))
        pid += 1

# Add 50 rows for resource abuse
for _ in range(50):
    rows.append(generate_resource_row(pid))
    pid += 1

# Save to CSV
with open(OUTPUT_PATH, "w", newline="") as f:
    writer = csv.DictWriter(f, fieldnames=FIELDS)
    writer.writeheader()
    writer.writerows(rows)

print(f"[✓] 500 malicious samples saved to: {OUTPUT_PATH}")

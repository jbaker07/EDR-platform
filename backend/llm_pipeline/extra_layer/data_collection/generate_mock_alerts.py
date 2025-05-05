# backend/llm_pipeline/data_collection/generate_mock_alerts.py

import json
import random
import time
from datetime import datetime, timedelta

# Some sample MITRE TTPs
mitre_techniques = [
    {"id": "T1059", "name": "Command and Scripting Interpreter"},
    {"id": "T1547", "name": "Boot or Logon Autostart Execution"},
    {"id": "T1003", "name": "Credential Dumping"},
    {"id": "T1027", "name": "Obfuscated Files or Information"},
    {"id": "T1086", "name": "PowerShell"},
]

# Sample process names to simulate
sample_processes = [
    ("powershell.exe", "powershell -nop -exec bypass"),
    ("cmd.exe", "cmd /c whoami"),
    ("regsvr32.exe", "regsvr32 /s malicious.dll"),
    ("lsass.exe", "dump_lsass.exe"),
    ("explorer.exe", "explorer launchpad")
]

def generate_alert():
    proc = random.choice(sample_processes)
    ttp = random.choice(mitre_techniques)
    score = random.randint(30, 98)
    return {
        "timestamp": int((datetime.now() - timedelta(seconds=random.randint(0, 86400))).timestamp()),
        "hostname": f"endpoint-{random.randint(1,5)}",
        "pid": random.randint(1000, 9999),
        "process_name": proc[0],
        "cmd": proc[1],
        "risk_score": score,
        "risk_level": (
            "high" if score > 80 else "medium" if score > 50 else "low"
        ),
        "mitre_technique": ttp
    }

def save_alerts(n=100, output_file="sample_alerts.json"):
    alerts = [generate_alert() for _ in range(n)]
    with open(output_file, "w") as f:
        json.dump(alerts, f, indent=2)
    print(f"✅ Generated {n} alerts to {output_file}")

if __name__ == "__main__":
    save_alerts(100, "llm_pipeline/extra_layer/data_collection/sample_alerts.json")

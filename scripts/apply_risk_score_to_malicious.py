import csv
import sys
import os
import time
import platform

# Setup local import
sys.path.append("backend")
from app.services.trust_engine_final import risk_score_minimal

INPUT_PATH = "data/malicious_samples_triggering_rules.csv"
OUTPUT_PATH = "data/malicious_samples_with_scores.csv"
os.makedirs("data", exist_ok=True)

FIELDS = [
    "timestamp", "hostname", "endpoint_id", "os_type", "pid", "ppid",
    "process_name", "cmd", "cpu_percent", "memory", "risk_score", "label"
]

scored_rows = []

with open(INPUT_PATH, newline="") as infile:
    reader = csv.DictReader(infile)
    for row in reader:
        try:
            row["cpu_percent"] = float(row["cpu_percent"])
            row["memory"] = int(row["memory"])
            row["risk_score"] = risk_score_minimal(row)
            scored_rows.append(row)
        except Exception as e:
            print(f"[!] Error scoring row: {e}\n{row}")

# Save result
with open(OUTPUT_PATH, mode="w", newline="") as outfile:
    writer = csv.DictWriter(outfile, fieldnames=FIELDS)
    writer.writeheader()
    writer.writerows(scored_rows)

print(f"[✓] Scored {len(scored_rows)} malicious samples → saved to {OUTPUT_PATH}")

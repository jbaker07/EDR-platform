# scripts/save_telemetry_to_csv.py

import csv
from datetime import datetime
from app.memory.memory_manager import memory_manager

OUTPUT_CSV = "collected_telemetry.csv"

events = memory_manager.get_all_recent_events()

if not events:
    print("[!] No telemetry events found. Is the daemon running?")
    exit(1)

rows = []
for event in events:
    rows.append({
        "timestamp": datetime.utcfromtimestamp(event.get("timestamp", 0)).isoformat(),
        "hostname": event.get("hostname", ""),
        "endpoint_id": event.get("endpoint_id", ""),
        "os_type": event.get("os_type", ""),
        "pid": event.get("pid", -1),
        "ppid": event.get("ppid", -1),  # ✅ Fixed reference from `proc` to `event`
        "process_name": event.get("process_name", ""),
        "cmd": event.get("cmd", ""),
        "cpu_percent": event.get("cpu_percent", 0.0),
        "memory": event.get("memory", 0),
        "risk_score": event.get("risk_score", 0),
        "risk_level": event.get("risk_level", "low"),
    })

# Write CSV
with open(OUTPUT_CSV, "w", newline="") as f:
    writer = csv.DictWriter(f, fieldnames=rows[0].keys())
    writer.writeheader()
    writer.writerows(rows)

print(f"[+] Saved {len(rows)} rows to {OUTPUT_CSV}")

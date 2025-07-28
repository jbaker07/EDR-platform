import psutil
import platform
import time
import csv
import os
import random
import sys

# Trust engine path setup
sys.path.append("backend")  # Ensure `backend.app.services` is discoverable
from app.services.trust_engine_final import risk_score_minimal

OUTPUT_PATH = "data/benign_samples_with_risk.csv"
os.makedirs("data", exist_ok=True)

FIELDS = [
    "timestamp", "hostname", "endpoint_id", "os_type", "pid", "ppid",
    "process_name", "cmd", "cpu_percent", "memory", "risk_score", "label"
]

def collect_snapshot(sample_limit=15):
    hostname = platform.node()
    os_type = platform.system().lower()
    timestamp = int(time.time())
    endpoint_id = hostname

    data = []
    processes = list(psutil.process_iter(['pid', 'name', 'cmdline', 'cpu_percent', 'memory_info', 'ppid']))
    random.shuffle(processes)

    for proc in processes[:sample_limit]:
        try:
            info = proc.info
            row = {
                "timestamp": timestamp,
                "hostname": hostname,
                "endpoint_id": endpoint_id,
                "os_type": os_type,
                "pid": info.get("pid"),
                "ppid": info.get("ppid"),
                "process_name": info.get("name", ""),
                "cmd": ' '.join(info.get("cmdline", [])) if info.get("cmdline") else '',
                "cpu_percent": float(info.get("cpu_percent", 0.0)),
                "memory": info.get("memory_info").rss if info.get("memory_info") else 0,
                "label": 0  # benign
            }

            # Get rule-based risk score
            row["risk_score"] = risk_score_minimal(row)

            data.append(row)
        except Exception:
            continue

    return data

def main():
    print("[✓] Starting benign telemetry collection with risk scores...")
    try:
        with open(OUTPUT_PATH, mode='w', newline='') as f:
            writer = csv.DictWriter(f, fieldnames=FIELDS)
            writer.writeheader()

            while True:
                snapshot = collect_snapshot(sample_limit=15)
                writer.writerows(snapshot)
                print(f"[+] Logged {len(snapshot)} benign processes with risk scores")
                time.sleep(5)

    except KeyboardInterrupt:
        print(f"[!] Interrupted — partial benign data saved to: {OUTPUT_PATH}")

    print(f"[✓] Done. Final saved file: {OUTPUT_PATH}")

if __name__ == "__main__":
    main()

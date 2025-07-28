# scripts/send_real_telemetry_daemon_minimal.py

import psutil
import platform
import time
import json
import requests
from datetime import datetime

API_URL = "http://localhost:8000/api/telemetry"

def collect_minimal_telemetry():
    hostname = platform.node()
    os_type = platform.system().lower()
    timestamp = int(time.time())
    endpoint_id = hostname

    telemetry_batch = []

    for proc in psutil.process_iter(['pid', 'name', 'cmdline', 'cpu_percent', 'memory_info', 'ppid']):
        try:
            info = proc.info
            pid = info.get("pid")
            ppid = info.get("ppid")
            process_name = info.get("name", "")
            cmd = ' '.join(info.get("cmdline", [])) if info.get("cmdline") else ''
            cpu = info.get("cpu_percent")
            memory = info.get("memory_info").rss if info.get("memory_info") else 0

            if None in [pid, ppid, process_name, cmd, cpu, memory]:
                continue  # skip incomplete telemetry

            minimal_payload = {
                "timestamp": timestamp,
                "hostname": hostname,
                "endpoint_id": endpoint_id,
                "os_type": os_type,
                "pid": pid,
                "ppid": ppid,
                "process_name": process_name,
                "cmd": cmd,
                "cpu_percent": float(cpu),
                "memory": memory,
                "source": "real_minimal",
                "label": -1  # unlabeled live telemetry
            }

            telemetry_batch.append(minimal_payload)

        except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
            continue
        except Exception as e:
            print(f"[!] Error gathering minimal telemetry: {e}")
            continue

    return telemetry_batch

def main():
    while True:
        processes = collect_minimal_telemetry()

        for proc in processes:
            try:
                print(json.dumps(proc, indent=2))
                response = requests.post(API_URL, headers={"Content-Type": "application/json"}, data=json.dumps(proc))
                print(f"[✓] Sent: {proc['process_name']} | Status: {response.status_code}")
            except Exception as e:
                print(f"[!] Failed to send telemetry: {e}")

        time.sleep(5)

if __name__ == "__main__":
    main()

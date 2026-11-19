
import psutil
import requests
import json
import os
import platform
import time

API_URL = os.environ.get("EDR_API_URL", "http://localhost:8000/api/telemetry")
CPU_SAMPLE_SECONDS = 1.0

# Field names: cpu_percent and cmd. They match the TelemetryEvent columns, the
# save path in memory_manager and backend/app/rules/default_rules.json.

def collect_and_send():
    hostname = platform.node()
    os_type = platform.system().lower()

    # cpu_percent() returns 0.0 the first time it is called for a process, so a
    # single pass reports 0.0 for everything. Prime each process, wait, then read.
    procs = list(psutil.process_iter(['pid', 'name', 'cmdline', 'memory_info']))
    for proc in procs:
        try:
            proc.cpu_percent(None)
        except psutil.Error:
            pass
    time.sleep(CPU_SAMPLE_SECONDS)

    telemetry_batch = []

    for proc in procs:
        try:
            info = proc.info
            telemetry = {
                "hostname": hostname,
                "os_type": os_type,
                "pid": info['pid'],
                "process_name": info['name'],
                "cmd": ' '.join(info['cmdline']) if info['cmdline'] else '',
                "cpu_percent": proc.cpu_percent(None),
                "memory": info['memory_info'].rss if info['memory_info'] else 0,
                "timestamp": int(time.time())
            }
            telemetry_batch.append(telemetry)
        except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
            continue

    for telemetry in telemetry_batch:
        try:
            response = requests.post(API_URL, headers={"Content-Type": "application/json"}, data=json.dumps(telemetry))
            print("✅ Sent:", telemetry["process_name"], "| Status:", response.status_code)
        except Exception as e:
            print("❌ Failed to send telemetry:", e)

if __name__ == "__main__":
    collect_and_send()

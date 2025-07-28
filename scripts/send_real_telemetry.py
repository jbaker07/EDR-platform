
import psutil
import requests
import json
import platform
import time

API_URL = "http://localhost:8000/api/telemetry"

def collect_and_send():
    hostname = platform.node()
    os_type = platform.system().lower()

    telemetry_batch = []

    for proc in psutil.process_iter(['pid', 'name', 'cmdline', 'cpu_percent', 'memory_info']):
        try:
            info = proc.info
            telemetry = {
                "hostname": hostname,
                "os_type": os_type,
                "pid": info['pid'],
                "process_name": info['name'],
                "cmd": ' '.join(info['cmdline']) if info['cmdline'] else '',
                "cpu_percent": info['cpu_percent'],
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

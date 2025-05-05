import psutil
import requests
import time
import socket

def collect_and_send():
    processes = []
    for proc in psutil.process_iter(['pid', 'name', 'cmdline', 'cpu_percent', 'memory_info']):
        try:
            info = proc.info
            memory = info['memory_info'].rss if info['memory_info'] else 0
            processes.append({
                "pid": info['pid'],
                "name": info['name'],
                "cmd": " ".join(info['cmdline']) if info['cmdline'] else "",
                "cpu_usage": info['cpu_percent'],
                "memory": memory,
                "risk_score": 0,
                "risk_level": "low"
            })
        except (psutil.NoSuchProcess, psutil.AccessDenied, KeyError):
            continue

    payload = {
        "hostname": socket.gethostname(),
        "os_type": "darwin",  # Or "linux" depending on your platform
        "processes": processes
    }

    res = requests.post("http://localhost:8000/api/telemetry", json=payload)
    print(f"[+] Sent {len(processes)} processes — Response: {res.status_code}")

while True:
    collect_and_send()
    time.sleep(5)

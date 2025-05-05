#!/usr/bin/env python3
import time
import requests
import random
import socket
import platform

API_URL = "http://localhost:8000/api/telemetry"
HOST    = socket.gethostname()
OS_TYPE = platform.system().lower()

def make_fake_process(pid: int):
    return {
        "pid":          pid,
        "process_name": f"proc-{pid}",
        "cmd":          f"/usr/bin/proc-{pid}",
        "cpu_percent":  round(random.uniform(0, 100), 1),
        "memory":       random.randint(10, 500) * 1024 * 1024,  # 10–500 MB
    }

def send_batch(n_procs=5):
    procs = [ make_fake_process(pid) for pid in range(1000, 1000 + n_procs) ]
    payload = {
        "hostname":  HOST,
        "os_type":   OS_TYPE,
        "timestamp": time.time(),
        "processes": procs,
    }
    resp = requests.post(API_URL, json=payload, timeout=5)
    resp.raise_for_status()
    print(f"→ Sent {n_procs} procs: {resp.json()}")

if __name__ == "__main__":
    # send 3 batches five seconds apart
    for i in range(3):
        send_batch(n_procs=3 + i)
        time.sleep(5)

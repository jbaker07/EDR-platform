#!/usr/bin/env python3

import time
import socket
import platform
import psutil
import requests

API_URL     = "http://localhost:8000/api/telemetry"
HOSTNAME    = socket.gethostname()
ENDPOINT_ID = f"{HOSTNAME}.{platform.node()}"

def collect_process_list():
    procs = []
    for p in psutil.process_iter(["pid", "name", "cmdline", "cpu_percent", "memory_info"]):
        try:
            info = p.info
            # Safely get memory RSS
            mem_info = info.get("memory_info")
            mem_rss  = mem_info.rss if mem_info is not None else 0

            # Safely get command line parts
            cmd_parts = info.get("cmdline") or []
            # ensure it's iterable
            if not isinstance(cmd_parts, (list, tuple)):
                cmd_parts = []

            procs.append({
                "pid":           info.get("pid", 0),
                "process_name":  info.get("name", "") or "",
                "cmd":           " ".join(cmd_parts),
                "cpu_percent":   info.get("cpu_percent", 0) or 0,
                "memory":        mem_rss,
            })
        except (psutil.NoSuchProcess, psutil.AccessDenied):
            continue
    return procs

def main():
    while True:
        event = {
            "endpoint_id": ENDPOINT_ID,
            "hostname":    HOSTNAME,
            "os_type":     platform.system().lower(),
            "timestamp":   time.time(),
            "processes":   collect_process_list(),
        }
        try:
            resp = requests.post(API_URL, json=event, timeout=5)
            resp.raise_for_status()
            print(f"[+] Sent {len(event['processes'])} processes @ {event['timestamp']}")
        except Exception as e:
            print(f"[!] Error sending telemetry: {e}")
        time.sleep(5)

if __name__ == "__main__":
    main()

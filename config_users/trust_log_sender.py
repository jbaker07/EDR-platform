import requests, json, socket
import time

def send_log():
    with open("trust_output.json") as f:
        data = json.load(f)

    payload = {
        "endpoint_id": socket.gethostname(),
        "timestamp": int(time.time()),
        "trust_score": data["final_trust_score"],
        "flags": data["flags"],
        "user": data["user"],
        "role": data["role"],
        "command": data["process"]
    }

    try:
        r = requests.post("http://<COLLECTOR_IP>:8080/submit_log", json=payload)
        print("Log sent:", r.status_code)
    except Exception as e:
        print("Failed to send:", e)

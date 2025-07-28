import sys
sys.path.append("backend")  # Let Python find your trust engine

from app.services.trust_engine_final import risk_score_minimal

# Simulated suspicious process
test_event = {
    "cmd": "powershell -enc aW52b2tlLWV4cHJlc3M=",
    "process_name": "powershell",
    "cpu_percent": 87.3,
    "memory": 1100000000,  # ~1.1 GB
    "timestamp": 1723490034,
    "hostname": "test-host",
    "endpoint_id": "test-host",
    "os_type": "darwin",
    "pid": 4242,
    "ppid": 1
}

# Run it through the risk engine
score = risk_score_minimal(test_event)
print(f"[✓] Risk Score: {score}")

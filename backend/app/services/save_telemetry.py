import os
import csv
from app.services.db import get_db_session
from app.models.telemetry import TelemetryEvent
from datetime import datetime

EXPORT_PATH = "backend/telemetry_logs/real_telemetry_log.csv"
os.makedirs(os.path.dirname(EXPORT_PATH), exist_ok=True)

def dump_telemetry():
    session = next(get_db_session())
    events = session.query(TelemetryEvent).all()

    with open(EXPORT_PATH, mode='w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow([
            "hostname", "pid", "process_name", "cmd", "memory", "cpu_percent",
            "risk_score", "risk_level", "timestamp", "os_type", "endpoint_id"
        ])
        for e in events:
            writer.writerow([
                e.hostname,
                e.pid,
                e.process_name,
                e.cmd,
                e.memory,
                e.cpu_percent,
                e.risk_score,
                e.risk_level,
                datetime.utcfromtimestamp(e.timestamp).isoformat(),
                e.os_type,
                e.endpoint_id
            ])
    print(f"✅ Saved {len(events)} telemetry events → {EXPORT_PATH}")

if __name__ == "__main__":
    dump_telemetry()

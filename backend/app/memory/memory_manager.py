import time
from sqlmodel import select
from app.services.db import get_db_session
from app.models.telemetry import TelemetryEvent
from app.models.alert import Alert
from backend.app.rules.rules_engine import apply_rules, get_loaded_rules

class MemoryManager:
    def save_telemetry(self, hostname: str, event: dict):
        # Use .get to safely access endpoint_id
        endpoint_id = event.get("endpoint_id", hostname)

        tel = TelemetryEvent(
            hostname=hostname,
            endpoint_id=endpoint_id,
            os_type=event["os_type"],
            pid=event["pid"],
            process_name=event["process_name"],
            cmd=event["cmd"],
            cpu_percent=event["cpu_percent"],
            memory=event["memory"],
            risk_score=event.get("risk_score", 0),
            risk_level=event.get("risk_level", "low"),
            timestamp=event["timestamp"],
        )

        db = next(get_db_session())
        db.add(tel)
        db.commit()

        # Apply rules (e.g., for alert generation)
        loaded_rules = get_loaded_rules()
        alerts = apply_rules(event, loaded_rules, os_type=event.get("os_type", "linux"))

        for alert in alerts:
            alert_record = Alert(
                hostname=hostname,
                process_name=event["process_name"],
                rule_id=alert["rule_id"],
                rule_name=alert["name"],
                description=alert["description"],
                severity=alert["severity"],
                timestamp=event["timestamp"]
            )
            db.add(alert_record)

        if alerts:
            db.commit()

    def get_all_recent_events(self, window_seconds: int = 900):
        """Return all events within the last `window_seconds`."""
        cutoff = time.time() - window_seconds
        db = next(get_db_session())
        stmt = select(TelemetryEvent).where(TelemetryEvent.timestamp >= cutoff)
        results = db.execute(stmt).scalars().all()
        return [r.dict(exclude={"id", "created_at"}) for r in results]
    
    def get_events_for_host(self, hostname: str, window_seconds: int = 900):
        """Return events for a specific host within the last `window_seconds`."""
        cutoff = time.time() - window_seconds
        db = next(get_db_session())
        stmt = select(TelemetryEvent).where(
            TelemetryEvent.hostname == hostname,
            TelemetryEvent.timestamp >= cutoff
        )
        results = db.execute(stmt).scalars().all()
        return [r.dict(exclude={"id", "created_at"}) for r in results]

# Global instance
memory_manager = MemoryManager()

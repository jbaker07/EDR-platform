import time
from sqlmodel import select
from app.services.db import get_db_session
from app.models.telemetry import TelemetryEvent
from app.models.alert import Alert
from app.rules.rules_engine import apply_rules, get_loaded_rules

class MemoryManager:
    def save_telemetry(self, hostname: str, event: dict) -> list:
        """Store one event, run the loaded rules on it once, store any alerts.

        Returns the alerts so the caller can report them without running the
        rules a second time.
        """
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
            timestamp=event["timestamp"],
        )

        db = next(get_db_session())
        db.add(tel)
        db.commit()

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
        return alerts

    def get_all_recent_events(self, window_seconds: int = 900):
        cutoff = time.time() - window_seconds
        db = next(get_db_session())
        stmt = select(TelemetryEvent).where(TelemetryEvent.timestamp >= cutoff)
        results = db.execute(stmt).scalars().all()
        return [r.dict(exclude={"id", "created_at"}) for r in results]

    def get_events_for_host(self, hostname: str, window_seconds: int = 900):
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

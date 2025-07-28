import time
from sqlmodel import select
from app.services.db import get_db_session
from app.models.telemetry import TelemetryEvent
from app.models.alert import Alert
from app.rules.rules_engine import apply_rules, get_loaded_rules

memory_store = {}

class MemoryManager:
    def __init__(self):
        self._embeddings = {}       # hostname or pid → embedding vector
        self._trust_scores = {}     # hostname → trust score
        self._flags = set()         # hostnames flagged for forensic mode
        self._isolation = set()     # hostnames flagged for isolation (future use)

    def save_telemetry(self, hostname: str, event: dict):
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

    def get_event_details(event_id: str) -> dict:
        return memory_store.get(event_id)

    # === GNN & Trust Support ===

    def store_embedding(self, hostname: str, embedding):
        self._embeddings[hostname] = embedding

    def get_embedding(self, hostname: str):
        return self._embeddings.get(hostname)

    def store_embedding_by_pid(self, pid: int, embedding):
        self._embeddings[str(pid)] = embedding

    def get_last_embedding(self, pid: int):
        return self._embeddings.get(str(pid))

    def set_trust_score(self, hostname: str, score: float):
        self._trust_scores[hostname] = score

    def get_trust_score(self, hostname: str) -> float:
        return self._trust_scores.get(hostname)

    # === Mode Flags ===

    def flag_for_forensic_mode(self, hostname: str):
        print(f"[MODE SHIFT] {hostname} → forensic mode")
        self._flags.add(hostname)

    def return_to_minimal_mode(self, hostname: str):
        if hostname in self._flags:
            self._flags.remove(hostname)
            print(f"[MODE RECOVERY] {hostname} returned to minimal mode")

    def is_flagged(self, hostname: str) -> bool:
        return hostname in self._flags

    def flag_for_isolation(self, hostname: str):
        print(f"[CRITICAL] {hostname} flagged for isolation mode!")
        self._isolation.add(hostname)

    def is_isolated(self, hostname: str) -> bool:
        return hostname in self._isolation

# Global instance
memory_manager = MemoryManager()

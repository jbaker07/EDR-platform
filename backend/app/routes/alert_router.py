from fastapi import APIRouter
from app.services.db import get_db_session
from app.models.alert import Alert

router = APIRouter()

@router.get("/alerts")
def get_alerts():
    db = next(get_db_session())
    return db.query(Alert).order_by(Alert.timestamp.desc()).limit(100).all()

@router.get("/alerts/{alert_id}")
def get_alert_by_id(alert_id: int):
    db = next(get_db_session())
    alert = db.query(Alert).filter(Alert.id == alert_id).first()
    if not alert:
        return {"error": "Alert not found"}
    return {
        "id": alert.id,
        "hostname": alert.hostname,
        "process_name": alert.process_name,
        "rule_id": alert.rule_id,
        "rule_name": alert.rule_name,
        "description": alert.description,
        "severity": alert.severity,
        "timestamp": alert.timestamp.isoformat(),
    }

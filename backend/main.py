# backend/main.py

from fastapi import FastAPI, HTTPException, Request
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import List, Dict
import json
from pathlib import Path
import joblib

# Init
from app.services.db import init_db
from app.rules.rules_engine import reload_rules, apply_rules, loaded_rules

# Routers
from app.routes.score import router as score_router
from app.routes.timeline import router as timeline_router
from app.routes.collaboration import router as collaboration_router
from app.routes.explain import router as explain_router
from app.routes.rules import router as rules_router
from app.routes.rules_router import router as advanced_rules_router
from app.routes.feedback import router as feedback_router
from app.routes.gamification import router as gamification_router
from app.routes.iocs import router as iocs_router
from app.routes.telemetry_router import router as telemetry_router
from llm_pipeline.routes.llm_routes import router as llm_router
from app.routes.multi_correlation_router import router as multi_correlation_router
from app.routes.rule_reload_router import router as rule_reload_router
from app.routes.alert_router import router as alert_router
from app.routes.agent_router import router as agent_router
from app.routes.trust_history_router import router as trust_history_router
from app.routes.trust_global_router import router as global_trust_router
from app.routes.trust_router import router as trust_logic_router
from app.routes.forensics_router import router as forensics_router
from app.routes.remediation_router import router as remediation_router
from app.routes.root_cause_router import router as root_cause_router
from app.routes.action_router import router as action_router
from app.routes.gnn_replay_router import router as replay_router
from app.routes.gnn_archive_router import router as gnn_archive_router
from app.routes.gnn_router import router as gnn_router

# Services
from app.services.trust_gnn_engine import get_trust_shift_justification
from app.services.trust_digest_engine import compute_digest_signature
from app.services.adaptive_threshold_engine import evaluate_dynamic_thresholds
from app.services.update_manifest_checker import compare_update_manifests
from app.services.intel_pack_signer import sign_intel_pack

# App setup
app = FastAPI()

@app.on_event("startup")
def on_startup():
    init_db()
    reload_rules()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:5173"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Routers
routers = [
    score_router,
    timeline_router,
    collaboration_router,
    explain_router,
    rules_router,
    feedback_router,
    gamification_router,
    iocs_router,
    telemetry_router,
    advanced_rules_router,
    llm_router,
    multi_correlation_router,
    rule_reload_router,
    alert_router,
    agent_router,
    trust_history_router,
    forensics_router,
    remediation_router,
    root_cause_router,
    action_router,
    trust_logic_router,
    global_trust_router,
    replay_router,
    gnn_archive_router,
    gnn_router,
]

for r in routers:
    app.include_router(r, prefix="/api")

# Trust log persistence
TRUST_LOG_FILE = Path("backend/app/logs/trust_log.json")
TRUST_LOG_FILE.parent.mkdir(parents=True, exist_ok=True)
if not TRUST_LOG_FILE.exists():
    TRUST_LOG_FILE.write_text("[]")

@app.post("/api/trustlog")
async def receive_trust_log(payload: Dict):
    try:
        logs = json.loads(TRUST_LOG_FILE.read_text())
        logs.append(payload)
        TRUST_LOG_FILE.write_text(json.dumps(logs, indent=2))
        return {"status": "logged"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Alert analyzer
ALERT_LOG_FILE = Path("backend/app/logs/alert_log.json")
ALERT_LOG_FILE.parent.mkdir(parents=True, exist_ok=True)
if not ALERT_LOG_FILE.exists():
    ALERT_LOG_FILE.write_text("[]")

@app.post("/api/analyze")
async def analyze_telemetry(request: Request):
    data = await request.json()
    alerts = apply_rules(data, loaded_rules)
    if alerts:
        existing = json.loads(ALERT_LOG_FILE.read_text())
        existing.extend(alerts)
        ALERT_LOG_FILE.write_text(json.dumps(existing, indent=2))
    return {"alerts": alerts}

# IOC storage
IOC_FILE = Path("logs/iocs.json")
IOC_FILE.parent.mkdir(parents=True, exist_ok=True)
if not IOC_FILE.exists():
    IOC_FILE.write_text(json.dumps([]))

@app.get("/api/iocs")
def get_iocs():
    try:
        return json.loads(IOC_FILE.read_text())
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/api/iocs")
def add_ioc(ioc: Dict):
    try:
        iocs = json.loads(IOC_FILE.read_text())
        iocs.append(ioc)
        IOC_FILE.write_text(json.dumps(iocs, indent=2))
        return {"status": "added", "ioc": ioc}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# ML prediction endpoint
try:
    MODEL_PATH = Path("backend/llm_core/llm_pipeline_formation/models/mlp_model.joblib")
    mlp_model = joblib.load(MODEL_PATH)
except Exception as e:
    print(f"❌ Failed to load model: {e}")
    mlp_model = None

class PredictionRequest(BaseModel):
    features: List[float]

@app.post("/api/predict")
async def predict(request: PredictionRequest):
    if mlp_model is None:
        raise HTTPException(status_code=500, detail="Model not loaded.")
    try:
        prediction = mlp_model.predict([request.features])
        label = "malicious" if prediction[0] == 1 else "benign"
        return {"prediction": label}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Health check
@app.get("/api/health")
def health_check():
    return {"status": "ok"}

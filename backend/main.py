# backend/main.py

from fastapi import FastAPI, HTTPException, Request
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import List, Dict
import json
from pathlib import Path
import joblib

# Database initialization
from app.services.db import init_db

# Rule engine
from app.rules.rules_engine import reload_rules, apply_rules, loaded_rules, calculate_trust_probability

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
from app.routes.rule_reload_router import router as rule_reload_router
from app.routes import alert_router
from app.routes.agent_router import router as agent_router
from fastapi.middleware.cors import CORSMiddleware
from app.services.trust_router import router as trust_router




app = FastAPI()

# Initialize database and reload rules at startup
@app.on_event("startup")
def on_startup():
    init_db()
    reload_rules()

# CORS middleware for React frontend
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:5173"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Mount all routers under /api where appropriate
app.include_router(score_router, prefix="/api")
app.include_router(timeline_router, prefix="/api")
app.include_router(collaboration_router, prefix="/api")
app.include_router(explain_router, prefix="/api")
app.include_router(rules_router, prefix="/api")
app.include_router(feedback_router, prefix="/api")
app.include_router(gamification_router, prefix="/api")
app.include_router(iocs_router, prefix="/api")
app.include_router(telemetry_router, prefix="/api")
app.include_router(advanced_rules_router, prefix="/api")
app.include_router(llm_router, prefix="/api")
app.include_router(multi_correlation_router, prefix="/api")
app.include_router(rule_reload_router, prefix="/api")
app.include_router(alert_router.router, prefix="/api")
app.include_router(agent_router, prefix="/api")
app.include_router(trust_router, prefix="/api")

# Persistent alert log (for /api/analyze)
ALERT_LOG_FILE = Path("backend/app/logs/alert_log.json")
ALERT_LOG_FILE.parent.mkdir(parents=True, exist_ok=True)
if not ALERT_LOG_FILE.exists():
    ALERT_LOG_FILE.write_text("[]")

# Analyze telemetry against loaded rules
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

# Machine Learning model loading for /api/predict
try:
    BASE_DIR   = Path(__file__).resolve().parent
    MODEL_PATH = BASE_DIR / "llm_core" / "llm_pipeline_formation" / "models" / "mlp_model.joblib"
    mlp_model  = joblib.load(MODEL_PATH)
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
        return {"prediction": int(prediction[0])}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# backend/main.py

# Make the repo root importable so sibling top-level packages (replay_engine,
# ontology, ...) resolve regardless of the launch CWD. The canonical run is
# `cd backend && uvicorn main:app`, which puts backend/ — but not its parent —
# on sys.path; these packages live one level up.
import os
import sys

_REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
if _REPO_ROOT not in sys.path:
    sys.path.insert(0, _REPO_ROOT)

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
# NOTE: app.routes.gnn_router is EXCLUDED on purpose — it is experimental and
# non-functional: it requires torch + torch_geometric (heavy, optional) plus
# unshipped per-wave .pt weights, and referenced symbols that don't exist
# (GCN / adjust_trust_from_gnn). See that file's header for details.

# NOTE: a block of top-level "Services" imports lived here
# (get_trust_shift_justification, compute_digest_signature,
# evaluate_dynamic_thresholds, compare_update_manifests, sign_intel_pack).
# None were referenced anywhere in this module, and two of them pointed at
# modules that do not exist (trust_digest_engine, adaptive_threshold_engine),
# which crashed app import. Removed as dead imports. The service modules that
# do exist are still importable directly by the routers that use them.

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
    # gnn_router intentionally excluded (experimental; see import note above).
]

for r in routers:
    app.include_router(r, prefix="/api")

# Anchor runtime-written files to this file's directory so they don't depend on
# the process CWD (the previous "backend/..." paths created stray backend/backend/
# trees when the app was launched from backend/).
_BACKEND_DIR = Path(__file__).resolve().parent

# Trust log persistence
TRUST_LOG_FILE = _BACKEND_DIR / "app" / "logs" / "trust_log.json"
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
ALERT_LOG_FILE = _BACKEND_DIR / "app" / "logs" / "alert_log.json"
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
IOC_FILE = _BACKEND_DIR / "logs" / "iocs.json"
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

# ML prediction endpoint.
#
# Loads a small DEMO classifier so /api/predict returns a real prediction
# out-of-the-box on a fresh clone. The model is a scikit-learn Pipeline
# (StandardScaler + LogisticRegression) trained on a TRANSPARENT SYNTHETIC
# dataset by backend/ml/train.py — it is NOT a real-incident detector. See
# that file's header for the honesty notice + the feature contract.
#
# Feature contract: the client sends {"features": [cpu_percent, memory, pid]}
# (the column order from the historical labeled telemetry CSV); the list is fed
# straight into model.predict([features]). FEATURE_ORDER is imported from the
# trainer so the serving vector and the training vector can never drift apart.
#
# Path is resolved relative to this file (not the process CWD) and overridable
# via PREDICT_MODEL_PATH. Loading is defensive: any failure leaves the endpoint
# returning a clear 503 instead of crashing app import.
try:
    from ml.train import FEATURE_ORDER as PREDICT_FEATURE_ORDER
except Exception:
    # Fallback that still documents the contract if the trainer can't import.
    PREDICT_FEATURE_ORDER = ["cpu_percent", "memory", "pid"]

_DEFAULT_MODEL_PATH = (
    Path(__file__).resolve().parent / "ml" / "models" / "demo_classifier.joblib"
)
MODEL_PATH = Path(os.environ.get("PREDICT_MODEL_PATH", str(_DEFAULT_MODEL_PATH)))

try:
    predict_model = joblib.load(MODEL_PATH)
except Exception as e:
    print(f"Failed to load predict model from {MODEL_PATH}: {e}")
    print("Train it with: python backend/ml/train.py")
    predict_model = None

class PredictionRequest(BaseModel):
    # Ordered numeric feature vector; see PREDICT_FEATURE_ORDER for the meaning
    # of each slot. Example: {"features": [3.2, 50000000, 412]}.
    features: List[float]

@app.post("/api/predict")
async def predict(request: PredictionRequest):
    if predict_model is None:
        raise HTTPException(
            status_code=503,
            detail=(
                "Predict model not loaded. Generate it with "
                "`python backend/ml/train.py` (or set PREDICT_MODEL_PATH)."
            ),
        )
    expected = len(PREDICT_FEATURE_ORDER)
    if len(request.features) != expected:
        raise HTTPException(
            status_code=422,
            detail=(
                f"Expected {expected} features in order {PREDICT_FEATURE_ORDER}, "
                f"got {len(request.features)}."
            ),
        )
    try:
        # 2D row, matching how train.py shapes X for .fit()/.predict().
        prediction = predict_model.predict([request.features])
        label = "malicious" if int(prediction[0]) == 1 else "benign"
        result = {"prediction": label, "label": label}
        # Surface a probability when the estimator exposes one (LogisticRegression
        # does), so callers can see confidence — best-effort, never fatal.
        try:
            proba = predict_model.predict_proba([request.features])[0]
            result["confidence"] = round(float(max(proba)), 4)
        except Exception:
            pass
        return result
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Health check
@app.get("/api/health")
def health_check():
    return {"status": "ok"}

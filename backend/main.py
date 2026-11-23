# backend/main.py
#
# Canonical run: cd backend && EDR_API_TOKEN=... uvicorn main:app

import json
import os
from pathlib import Path
from typing import Dict, List

import joblib
from fastapi import Depends, FastAPI, HTTPException, Request
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel

from app.auth import DEV_NO_AUTH_ENV, load_token, require_token
from app.services.db import init_db
# get_loaded_rules, not loaded_rules: a from-import copies the list object once,
# and reload_rules() later rebinds the module name to a new list.
from app.rules.rules_engine import reload_rules, apply_rules, get_loaded_rules

from app.routes.agent_router import router as agent_router
from app.routes.alert_router import router as alert_router
from app.routes.collaboration import router as collaboration_router
from app.routes.feedback import router as feedback_router
from app.routes.rule_reload_router import router as rule_reload_router
from app.routes.rules import router as rules_router
from app.routes.telemetry_router import router as telemetry_router

# Read once at import. load_token() raises when no token is set and the dev flag
# is off, so uvicorn never starts an open API by accident.
API_TOKEN = load_token()
DEV_NO_AUTH = API_TOKEN is None

# require_token runs for every route on the app, including the ones defined in
# this file. The /docs pages are added outside that dependency, so they exist
# only in dev mode.
app = FastAPI(
    dependencies=[Depends(require_token)],
    docs_url="/docs" if DEV_NO_AUTH else None,
    redoc_url=None,
    openapi_url="/openapi.json" if DEV_NO_AUTH else None,
)
app.state.api_token = API_TOKEN

if DEV_NO_AUTH:
    print(f"WARNING: {DEV_NO_AUTH_ENV}=1, the API accepts requests without a token.")

@app.on_event("startup")
def on_startup():
    init_db()
    reload_rules()

# Browsers may call the API only from the dashboard origin. The bearer token goes
# in a header, not a cookie, so credentials stay off.
DASHBOARD_ORIGIN = os.environ.get("EDR_DASHBOARD_ORIGIN", "http://localhost:5173")
app.add_middleware(
    CORSMiddleware,
    allow_origins=[DASHBOARD_ORIGIN],
    allow_credentials=False,
    allow_methods=["GET", "POST"],
    allow_headers=["Authorization", "Content-Type"],
)

routers = [
    telemetry_router,
    alert_router,
    agent_router,
    rule_reload_router,
    rules_router,
    feedback_router,
    collaboration_router,
]

# The /api prefix is added here only. Router paths are relative ("/telemetry",
# not "/api/telemetry"), or the route ends up at /api/api/....
for r in routers:
    app.include_router(r, prefix="/api")

# Anchor runtime-written files to this file's directory so they don't depend on
# the process CWD.
_BACKEND_DIR = Path(__file__).resolve().parent

# Dry run: which loaded rules match this event. Nothing is stored. POST
# /api/telemetry is the path that stores events and alerts.
@app.post("/api/analyze")
async def analyze_telemetry(request: Request):
    data = await request.json()
    alerts = apply_rules(data, get_loaded_rules(), os_type=data.get("os_type", "linux"))
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
# dataset by backend/ml/train.py. It is NOT a real-incident detector. See
# that file's header for the honesty notice + the feature contract.
#
# Feature contract: the client sends {"features": [cpu_percent, memory, pid]};
# the list is fed straight into model.predict([features]). FEATURE_ORDER is
# imported from the trainer so the serving vector and the training vector can
# never drift apart.
#
# Path is resolved relative to this file (not the process CWD) and overridable
# via PREDICT_MODEL_PATH. Loading is defensive: any failure leaves the endpoint
# returning a clear 503 instead of crashing app import.
try:
    from ml.train import FEATURE_ORDER as PREDICT_FEATURE_ORDER
except Exception:
    # Fallback that still documents the contract if the trainer can't import.
    PREDICT_FEATURE_ORDER = ["cpu_percent", "memory", "pid"]

_DEFAULT_MODEL_PATH = _BACKEND_DIR / "ml" / "models" / "demo_classifier.joblib"
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
        # does), so callers can see confidence. Best-effort, never fatal.
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

# Health check. It needs the token too, like every other route.
@app.get("/api/health")
def health_check():
    return {"status": "ok"}

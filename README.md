# EDR Platform

A prototype Endpoint Detection & Response (EDR) platform: a FastAPI backend that
ingests telemetry, runs rule-based detection, scores events with a small scikit-learn
model, and exposes trust/forensics/IOC APIs. A React dashboard and a Go/eBPF agent
live alongside it.

> **Status: prototype.** The core API boots and serves real predictions. Several
> advanced subsystems (GNN trust scoring, LLM summarisation, SHAP explainability)
> are experimental, dependency-heavy, and ship without their model weights — they are
> import-guarded so the app runs without them. This README documents what genuinely
> works today versus what is a stub.

## Repository layout

| Path | What it is | State |
|---|---|---|
| `backend/` | FastAPI app (`main.py`), rules engine, trust engine, routers | **Works** (see below) |
| `backend/gnn_pipeline/`, `backend/app/routes/gnn_router.py` | GNN trust scoring (torch / torch_geometric) | Experimental, not wired in |
| `backend/llm_pipeline/` | Mistral-7B + LoRA summary/hunt endpoints | Experimental, inert without weights |
| `dashboard/` | React/Vite front-end | Prototype |
| `edr-agent/` | Go + eBPF telemetry agent | Prototype (separate toolchain) |

## What genuinely works

- **`GET /api/health`** → `{"status": "ok"}`.
- **`POST /api/predict`** — serves a committed scikit-learn `Pipeline`
  (`StandardScaler` + `LogisticRegression`) at
  `backend/ml/models/demo_classifier.joblib`. It returns a real prediction
  **out-of-the-box on a fresh clone** — the tiny model (~1.4 KB) is committed, so no
  training step is required. **It is a DEMO model trained on a transparent
  *synthetic* dataset (high CPU + memory ⇒ malicious), NOT a real-incident
  detector.** Feature contract — a **3-feature** vector in the order
  `[cpu_percent, memory_bytes, pid]` (the column order of the historical labeled
  telemetry CSV):

  ```json
  {"features": [92.0, 3200000000, 731]}  →  {"prediction": "malicious", "label": "malicious", "confidence": 1.0}
  {"features": [2.0, 80000000, 501]}     →  {"prediction": "benign",    "label": "benign",    "confidence": 0.999}
  ```

  Sending the wrong number of features returns a clear `422`. Override the model
  path with `PREDICT_MODEL_PATH`.
- **`POST /api/analyze`** and **`POST /api/telemetry`** — JSON rule matching via the
  stateless rules engine (`backend/app/rules/`).
- **`/api/iocs`, `/api/trustlog`, `/api/feedback`, `/api/collaboration`,
  `/api/host/{hostname}` (trust score)** and the other routers included in `main.py`.
- The app starts with a **SQLite** database by default (`sqlite:///./edr.db`); set
  `DATABASE_URL` to a Postgres URL to use asyncpg/Postgres instead.

## Prototype / experimental (import-guarded, NOT required to boot)

- **SHAP explainability** (`/api/explain`, remediation explainers): needs `shap`.
  Returns HTTP 503 until installed.
- **GNN trust scoring** (`app/routes/gnn_router.py`): needs `torch` +
  `torch_geometric` **and** per-wave `.pt` weights that are not in the repo. It also
  referenced symbols that do not exist in this codebase, so it is **intentionally
  excluded from the router include list** in `main.py`.
- **LLM summary / hunt** (`/api/llm/*`): tries to load `mistralai/Mistral-7B-Instruct`
  + a LoRA adapter via `transformers`/`peft`/`torch`. Without those deps (and the
  unshipped adapter weights) the endpoints stay inert and return a clear error.

## Quick start (backend)

Requires Python 3.10+ (a recent 3.x is fine).

```bash
cd backend
python3 -m venv .venv && source .venv/bin/activate
pip install -r requirements.txt
# Run the API (module is `main`, app object is `app`):
uvicorn main:app --reload --port 8000
```

Then:

```bash
curl localhost:8000/api/health
# 3 features in order [cpu_percent, memory_bytes, pid]:
curl -X POST localhost:8000/api/predict \
  -H 'Content-Type: application/json' \
  -d '{"features": [92.0, 3200000000, 731]}'
# → {"prediction":"malicious","label":"malicious","confidence":1.0}
```

The DEMO model ships in the repo, so this works on a clean clone with no
training step. (To regenerate it: `python backend/ml/train.py`.)

`requirements.txt` installs only the light/core stack. The optional subsystems above
have their install lines documented at the bottom of `backend/requirements.txt`.

## Run with Docker Compose

The root `docker-compose.yml` builds the backend, Postgres, and Redis:

```bash
docker compose up --build
```

The backend container runs `uvicorn main:app` on port 8000.

## The /api/predict demo model

The model served by `/api/predict` is produced by **`backend/ml/train.py`** and saved
to **`backend/ml/models/demo_classifier.joblib`**. It is a small scikit-learn
`Pipeline(StandardScaler + LogisticRegression)` trained on a **transparent synthetic
dataset** — two clearly-separated clusters (low CPU+memory ⇒ benign, high CPU+memory ⇒
malicious), with `pid` included only to satisfy the feature contract. It is a
demonstration that the serving path works end-to-end; **it is not a real-incident
detector and makes no security claims.**

Regenerate it any time:

```bash
python backend/ml/train.py   # rewrites backend/ml/models/demo_classifier.joblib (~1.4 KB)
```

The single source of truth for feature order is `FEATURE_ORDER` in `train.py`, which
`backend/main.py` imports so the training vector and the serving vector can never
drift apart.

> Other training scripts under `backend/llm_core/llm_pipeline_formation/` and
> `backend/ml/` (`train_mlp.py`, `train_model.py`, `train_from_real.py`) are legacy
> experiments and are **not** what `/api/predict` serves.

## Configuration & secrets

Copy `.env.example` to `.env` and fill in real values. **Do not commit `.env`** — it
is git-ignored. Keys:

- `SHAP_AUTH_TOKEN`, `GNN_AUTH_TOKEN` — auth tokens for the explain / GNN endpoints.
- `DATABASE_URL` — defaults to SQLite; set to a Postgres URL for production.

> Historical `.env` files were previously committed to this repository. They have been
> removed from tracking, **but they still exist in prior git history.** Rotate every
> credential they referenced — removing them from the working tree does not
> un-leak past values.

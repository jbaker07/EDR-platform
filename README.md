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
- **`POST /api/predict`** — serves the committed scikit-learn `MLPClassifier`
  (`backend/llm_core/llm_pipeline_formation/models/mlp_model.joblib`). Verified end to
  end: the shipped model expects a **2-feature** vector, e.g.
  `{"features": [0.1, 0.5]}` → `{"prediction": "malicious"}`.
  (Passing the wrong number of features returns a 500 from the model — that is the
  model's input contract, not a boot bug.)
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
curl -X POST localhost:8000/api/predict \
  -H 'Content-Type: application/json' \
  -d '{"features": [0.1, 0.5]}'
```

`requirements.txt` installs only the light/core stack. The optional subsystems above
have their install lines documented at the bottom of `backend/requirements.txt`.

## Run with Docker Compose

The root `docker-compose.yml` builds the backend, Postgres, and Redis:

```bash
docker compose up --build
```

The backend container runs `uvicorn main:app` on port 8000.

## Retraining the scikit-learn model

Training scripts live under `backend/llm_core/llm_pipeline_formation/` and
`backend/ml/` (e.g. `train_mlp.py`, `train_model.py`). They use
`sklearn.neural_network.MLPClassifier`. Note: not all training scripts write to the
exact path the API serves from — confirm the output path matches
`backend/llm_core/llm_pipeline_formation/models/mlp_model.joblib` before relying on a
retrained model.

## Configuration & secrets

Copy `.env.example` to `.env` and fill in real values. **Do not commit `.env`** — it
is git-ignored. Keys:

- `SHAP_AUTH_TOKEN`, `GNN_AUTH_TOKEN` — auth tokens for the explain / GNN endpoints.
- `DATABASE_URL` — defaults to SQLite; set to a Postgres URL for production.

> Historical `.env` files were previously committed to this repository. They have been
> removed from tracking, **but they still exist in prior git history.** Rotate every
> credential they referenced — removing them from the working tree does not
> un-leak past values.

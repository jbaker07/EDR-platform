"""Pytest fixtures + path/setup for the serving tests.

Responsibilities:
  * Put ``backend/`` on sys.path so ``import main`` works exactly like the
    canonical ``cd backend && uvicorn main:app`` launch.
  * Guarantee the DEMO model exists before any test imports ``main`` — on a
    fresh clone the committed model is already present, but if it was deleted
    (or .gitignore'd away in some environment) we regenerate it on the fly so
    the suite is self-sufficient.
"""

from __future__ import annotations

import sys
from pathlib import Path

import pytest

_REPO_ROOT = Path(__file__).resolve().parent.parent
_BACKEND_DIR = _REPO_ROOT / "backend"
_MODEL_PATH = _BACKEND_DIR / "ml" / "models" / "demo_classifier.joblib"

# `backend/` first so `import main` and `import ml.train` resolve as they do at
# runtime; repo root too so the sibling top-level packages main.py expects load.
for p in (str(_BACKEND_DIR), str(_REPO_ROOT)):
    if p not in sys.path:
        sys.path.insert(0, p)


def _ensure_model() -> None:
    if _MODEL_PATH.exists():
        return
    # Train it if missing (idempotent, fast, deterministic).
    from ml.train import train

    train()


# Run BEFORE `main` is imported anywhere (module import loads the model once).
_ensure_model()


@pytest.fixture(scope="session")
def client():
    from fastapi.testclient import TestClient

    import main

    return TestClient(main.app)

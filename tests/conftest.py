"""Pytest fixtures + path/setup for the backend tests.

Responsibilities:
  * Put ``backend/`` on sys.path so ``import main`` works exactly like the
    canonical ``cd backend && uvicorn main:app`` launch.
  * Set EDR_API_TOKEN before ``main`` is imported, because main refuses to
    load without a token (or the explicit dev flag).
  * Guarantee the DEMO model exists before any test imports ``main``. On a
    fresh clone the committed model is already present, but if it was deleted
    (or .gitignore'd away in some environment) we regenerate it on the fly so
    the suite is self-sufficient.
"""

from __future__ import annotations

import os
import sys
from pathlib import Path

import pytest

_REPO_ROOT = Path(__file__).resolve().parent.parent
_BACKEND_DIR = _REPO_ROOT / "backend"
_MODEL_PATH = _BACKEND_DIR / "ml" / "models" / "demo_classifier.joblib"

# A fixed test token. 40 characters, above the 32 character minimum.
TEST_TOKEN = "test-token-0123456789abcdefghijklmnopqrs"
os.environ["EDR_API_TOKEN"] = TEST_TOKEN
os.environ.pop("EDR_DEV_NO_AUTH", None)

# `backend/` first so `import main` and `import ml.train` resolve as they do at
# runtime.
if str(_BACKEND_DIR) not in sys.path:
    sys.path.insert(0, str(_BACKEND_DIR))


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
    """A client that sends the right token on every request."""
    from fastapi.testclient import TestClient

    import main

    return TestClient(main.app, headers={"Authorization": f"Bearer {TEST_TOKEN}"})


@pytest.fixture(scope="session")
def anon_client():
    """A client that sends no token."""
    from fastapi.testclient import TestClient

    import main

    return TestClient(main.app)

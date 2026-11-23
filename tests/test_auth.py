"""Bearer token tests for the FastAPI backend.

Every route must answer 401 without the right token, the right token must get
through, and the app must refuse to start with no token unless the dev flag is
set on purpose.
"""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

import pytest
from fastapi.routing import APIRoute

from conftest import TEST_TOKEN

_BACKEND_DIR = Path(__file__).resolve().parent.parent / "backend"


def _api_routes(app):
    return [r for r in app.routes if isinstance(r, APIRoute)]


def _requests(app):
    """One (method, path) per route and method, with path parameters filled in."""
    out = []
    for route in _api_routes(app):
        path = route.path
        for name in route.param_convertors:
            path = path.replace("{" + name + "}", "1")
        for method in sorted(route.methods):
            out.append((method, path))
    return out


def test_every_route_has_the_token_dependency(client):
    from app.auth import require_token

    app = client.app
    # No plain Route or Mount that could skip the dependency. The /docs and
    # /openapi.json pages would be plain routes, so they are off outside dev mode.
    assert [r.path for r in app.routes if not isinstance(r, APIRoute)] == []
    for route in _api_routes(app):
        calls = [d.call for d in route.dependant.dependencies]
        assert require_token in calls, f"{route.path} is not behind require_token"


def test_no_token_is_401_on_every_route(client, anon_client):
    reqs = _requests(client.app)
    assert len(reqs) >= 20
    for method, path in reqs:
        res = anon_client.request(method, path, json={})
        assert res.status_code == 401, f"{method} {path} gave {res.status_code}"
        assert res.headers["www-authenticate"] == "Bearer"


@pytest.mark.parametrize(
    "header",
    [
        "Bearer wrong-token-but-long-enough-0123456789",
        f"Bearer {TEST_TOKEN}x",
        f"Bearer {TEST_TOKEN[:-1]}",
        f"Basic {TEST_TOKEN}",
        f"{TEST_TOKEN}",
        "Bearer ",
    ],
)
def test_wrong_token_is_401(anon_client, header):
    res = anon_client.get("/api/health", headers={"Authorization": header})
    assert res.status_code == 401


def test_wrong_token_is_401_on_every_route(client, anon_client):
    bad = {"Authorization": "Bearer wrong-token-but-long-enough-0123456789"}
    for method, path in _requests(client.app):
        res = anon_client.request(method, path, json={}, headers=bad)
        assert res.status_code == 401, f"{method} {path} gave {res.status_code}"


def test_right_token_is_200(client):
    res = client.get("/api/health")
    assert res.status_code == 200
    assert res.json() == {"status": "ok"}


def test_scheme_is_case_insensitive(anon_client):
    res = anon_client.get("/api/health", headers={"Authorization": f"bearer {TEST_TOKEN}"})
    assert res.status_code == 200


def test_load_token_fails_closed(monkeypatch):
    from app import auth

    monkeypatch.delenv(auth.TOKEN_ENV, raising=False)
    monkeypatch.delenv(auth.DEV_NO_AUTH_ENV, raising=False)
    with pytest.raises(auth.AuthConfigError):
        auth.load_token()

    # Any value other than "1" is not the dev flag.
    monkeypatch.setenv(auth.DEV_NO_AUTH_ENV, "true")
    with pytest.raises(auth.AuthConfigError):
        auth.load_token()

    monkeypatch.setenv(auth.DEV_NO_AUTH_ENV, "1")
    assert auth.load_token() is None

    monkeypatch.setenv(auth.TOKEN_ENV, "short")
    with pytest.raises(auth.AuthConfigError):
        auth.load_token()

    monkeypatch.setenv(auth.TOKEN_ENV, TEST_TOKEN)
    assert auth.load_token() == TEST_TOKEN


def _import_main(env_overrides):
    env = {k: v for k, v in os.environ.items() if k not in ("EDR_API_TOKEN", "EDR_DEV_NO_AUTH")}
    env.update(env_overrides)
    return subprocess.run(
        [sys.executable, "-c", "import main"],
        cwd=_BACKEND_DIR, env=env, capture_output=True, text=True, timeout=60,
    )


def test_server_refuses_to_start_without_token():
    res = _import_main({})
    assert res.returncode != 0
    assert "EDR_API_TOKEN is not set" in res.stderr


def test_server_starts_with_dev_flag():
    res = _import_main({"EDR_DEV_NO_AUTH": "1"})
    assert res.returncode == 0, res.stderr


def test_cors_allows_only_the_dashboard_origin(anon_client):
    preflight = {"Access-Control-Request-Method": "GET", "Access-Control-Request-Headers": "authorization"}
    ok = anon_client.options("/api/health", headers={"Origin": "http://localhost:5173", **preflight})
    assert ok.headers.get("access-control-allow-origin") == "http://localhost:5173"
    assert "access-control-allow-credentials" not in ok.headers

    other = anon_client.options("/api/health", headers={"Origin": "http://evil.example", **preflight})
    assert "access-control-allow-origin" not in other.headers

"""Agent incidents reaching the backend.

The agent's forwarder (edr-agent/src/forwarder.rs) posts batches to POST
/api/incidents with the backend token. Its Rust test checks that the payload it
builds equals edr-agent/tests/fixtures/agent_incident_batch.json, and these tests
post that same file, so the two halves are held to one contract.
"""

from __future__ import annotations

import copy
import json
import os
from pathlib import Path

import pytest
from fastapi import Depends, FastAPI
from fastapi.testclient import TestClient
from sqlmodel import create_engine

_REPO_ROOT = Path(__file__).resolve().parent.parent
FIXTURE = json.loads((_REPO_ROOT / "edr-agent" / "tests" / "fixtures" / "agent_incident_batch.json").read_text())


@pytest.fixture
def backend(tmp_path, monkeypatch):
    import main
    from app.services import db

    engine = create_engine(f"sqlite:///{tmp_path / 'inc.db'}", connect_args={"check_same_thread": False})
    monkeypatch.setattr(db, "engine", engine)
    db.init_db()
    return main


def test_agent_batch_is_stored_and_listed(backend, client):
    res = client.post("/api/incidents", json=FIXTURE)
    assert res.status_code == 200, res.text
    assert res.json() == {"stored": 1}
    rows = client.get("/api/incidents").json()
    assert len(rows) == 1
    row = rows[0]
    sent = FIXTURE["incidents"][0]
    assert row["id"] == sent["id"]
    assert row["host"] == "edr-lab-1"
    assert row["sources"] == ["guard"]
    assert row["techniques"] == ["T1610", "T1611"]
    assert row["details"]["guard"] == "runtime_socket"
    assert row["details"]["mode"] == "enforce"
    assert row["details"]["caller_exe"] == "/tmp/g6_spoof/docker"
    assert row["agent_version"] == "0.1.0"
    one = client.get(f"/api/incidents/{row['key']}")
    assert one.status_code == 200 and one.json()["title"] == sent["title"]
    assert client.get("/api/incidents/edr-lab-1%7Cnope").status_code == 404


def test_a_newer_version_replaces_the_row(backend, client):
    client.post("/api/incidents", json=FIXTURE)
    newer = copy.deepcopy(FIXTURE)
    newer["incidents"][0]["alerts_count"] = 5
    newer["incidents"][0]["last_ts"] = 1800000099
    client.post("/api/incidents", json=newer)
    rows = client.get("/api/incidents").json()
    assert len(rows) == 1
    assert rows[0]["alerts_count"] == 5


def test_posting_needs_the_token(backend, anon_client, client):
    assert anon_client.post("/api/incidents", json=FIXTURE).status_code == 401
    bad = {"Authorization": "Bearer wrong-token-wrong-token-wrong-token-x"}
    assert anon_client.post("/api/incidents", json=FIXTURE, headers=bad).status_code == 401
    assert client.get("/api/incidents").json() == []


@pytest.mark.parametrize(
    "mutate",
    [
        lambda b: b.pop("agent"),
        lambda b: b["incidents"][0].update(id=""),
        lambda b: b["incidents"][0].update(risk=9),
        lambda b: b["incidents"][0].update(title="x" * 5000),
        lambda b: b["incidents"][0].update(details={"blob": "y" * 9000}),
        lambda b: b.update(incidents=[b["incidents"][0]] * 501),
    ],
)
def test_bad_batches_are_422_and_store_nothing(backend, client, mutate):
    body = copy.deepcopy(FIXTURE)
    mutate(body)
    assert client.post("/api/incidents", json=body).status_code == 422
    assert client.get("/api/incidents").json() == []


def test_a_token_file_rotates_without_a_restart(tmp_path):
    from app.auth import AuthConfigError, TokenFile, require_token

    f = tmp_path / "token"
    # Different lengths, so the change shows in the size even when two writes land in
    # the same mtime tick.
    old, new = "o" * 40, "n" * 44
    f.write_text(old + "\n")
    os.chmod(f, 0o644)
    with pytest.raises(AuthConfigError, match="chmod 600"):
        TokenFile(f)
    os.chmod(f, 0o600)

    app = FastAPI(dependencies=[Depends(require_token)])
    app.state.api_token = TokenFile(f)

    @app.get("/x")
    def x():
        return {"ok": True}

    c = TestClient(app)
    hdr = lambda t: {"Authorization": f"Bearer {t}"}
    assert c.get("/x", headers=hdr(old)).status_code == 200
    f.write_text(new + "\n")
    os.chmod(f, 0o600)
    assert c.get("/x", headers=hdr(new)).status_code == 200
    assert c.get("/x", headers=hdr(old)).status_code == 401
    # A short token in the file is refused and the current one stays.
    f.write_text("short\n")
    assert c.get("/x", headers=hdr(new)).status_code == 200

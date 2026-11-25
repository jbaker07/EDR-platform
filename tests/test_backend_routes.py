"""Route table, rule loading and telemetry tests for the FastAPI backend.

Covers these bugs:
  * 11 routers were mounted at /api/api/... and some dashboard calls had no route.
  * /api/analyze imported loaded_rules by value, so it always saw an empty list.
  * default_rules.json read cpu_usage and command, but the collector sends
    cpu_percent and cmd.
  * /api/telemetry ran the rules twice per event.
  * The host view served a trust score that was 100 for every host.
"""

from __future__ import annotations

import ast
import copy
import re
from pathlib import Path

import pytest
from sqlmodel import create_engine
from starlette.routing import Match

_REPO_ROOT = Path(__file__).resolve().parent.parent

# Every backend call in dashboard/src, from:
#   grep -rnoE "['\"\`](https?://[a-z0-9.:]+)?/(api|telemetry)[^'\"\` ?]*" dashboard/src
# {param} marks a ${...} part of a template string. Files:
#   components/AgentRegistryPanel.tsx, CollaborationPanel.tsx,
#   DetectionLogicPanel.tsx, EndpointGrid.tsx, IOCViewerPanel.tsx,
#   ProcessTable.tsx, TokenGate.tsx, pages/AgentIncidents.tsx, pages/Dashboard.tsx,
#   pages/FeedbackPage.tsx
DASHBOARD_CALLS = [
    ("GET", "/api/agent"),
    ("GET", "/api/alerts"),
    ("GET", "/api/collaboration"),
    ("POST", "/api/collaboration"),
    ("GET", "/api/feedback"),
    ("GET", "/api/health"),
    ("GET", "/api/incidents"),
    ("GET", "/api/iocs"),
    ("GET", "/api/rules"),
    ("POST", "/api/rules/reload"),
    ("GET", "/api/telemetry/agents"),
    ("GET", "/api/telemetry/all"),
]

# Same keys scripts/send_real_telemetry.py sends for one process.
COLLECTOR_RECORD = {
    "hostname": "test-host",
    "os_type": "linux",
    "pid": 4242,
    "process_name": "bash",
    "cmd": "rm -rf /tmp/scratch",
    "cpu_percent": 1.5,
    "memory": 123456,
    "timestamp": 1700000000,
}


@pytest.fixture
def backend(tmp_path, monkeypatch):
    """Point the app at a throwaway SQLite file, with rules loaded."""
    import main
    from app.rules import rules_engine
    from app.services import db

    engine = create_engine(f"sqlite:///{tmp_path / 'edr_test.db'}", connect_args={"check_same_thread": False})
    monkeypatch.setattr(db, "engine", engine)
    db.init_db()

    # monkeypatch puts the original list back after the test.
    monkeypatch.setattr(rules_engine, "loaded_rules", [])
    assert rules_engine.reload_rules() is True
    return main


def _has_route(app, method: str, path: str) -> bool:
    scope = {"type": "http", "path": path, "method": method, "root_path": ""}
    return any(route.matches(scope)[0] == Match.FULL for route in app.routes)


def test_no_double_api_prefix(client):
    paths = [route.path for route in client.app.routes]
    assert [p for p in paths if p.startswith("/api/api")] == []


@pytest.mark.parametrize("method, template", DASHBOARD_CALLS)
def test_every_dashboard_call_has_a_route(client, method, template):
    path = template.replace("{param}", "1")
    assert _has_route(client.app, method, path), f"{method} {path} has no backend route"


def test_dashboard_call_list_is_current():
    pattern = re.compile(r"""['"`](?:https?://[^/'"`]+)?(/(?:api|telemetry)[^'"`?\s]*)""")
    found = set()
    for src in (_REPO_ROOT / "dashboard" / "src").rglob("*.ts*"):
        for path in pattern.findall(src.read_text()):
            found.add(re.sub(r"\$\{[^}]*\}", "{param}", path))
    listed = {template for _, template in DASHBOARD_CALLS}
    assert found - listed == set(), "new dashboard call, add it to DASHBOARD_CALLS"
    assert listed - found == set(), "DASHBOARD_CALLS lists a call the dashboard no longer makes"


def test_analyze_applies_loaded_rules_and_follows_reload(backend, client, monkeypatch):
    from app.rules import rules_engine

    # The by-value import is gone. main reads the rules through get_loaded_rules().
    assert not hasattr(backend, "loaded_rules")

    event = {"os_type": "linux", "process_name": "stress", "cmd": "stress --cpu 4", "cpu_percent": 95.0}
    res = client.post("/api/analyze", json=event)
    assert res.status_code == 200
    assert [a["rule_id"] for a in res.json()["alerts"]] == ["R001"]

    # Raise the CPU threshold above the event and reload. The result must change.
    stricter = copy.deepcopy(rules_engine.load_rules())
    stricter[0]["conditions"][0]["value"] = 99
    monkeypatch.setattr(rules_engine, "load_rules", lambda: stricter)
    res = client.post("/api/rules/reload")
    assert res.status_code == 200
    assert res.json()["stateless_rules_loaded"] == 2
    assert client.post("/api/analyze", json=event).json()["alerts"] == []

    # A rule file that fails validation is refused and the previous rules stay.
    monkeypatch.setattr(rules_engine, "load_rules", lambda: [{"id": "BAD"}])
    assert client.post("/api/rules/reload").status_code == 422
    assert rules_engine.get_loaded_rules() is stricter


def test_collector_record_triggers_rule_once(backend, client, monkeypatch):
    from app.rules import rules_engine

    evaluated = []
    real_evaluate = rules_engine.evaluate_rule

    def counting_evaluate(rule, data):
        evaluated.append(rule["id"])
        return real_evaluate(rule, data)

    monkeypatch.setattr(rules_engine, "evaluate_rule", counting_evaluate)

    res = client.post("/api/telemetry", json=COLLECTOR_RECORD)
    assert res.status_code == 200
    assert [a["rule_id"] for a in res.json()["alerts_triggered"]] == ["R002"]
    # Both linux rules are checked one time each, not twice.
    assert sorted(evaluated) == ["R001", "R002"]
    # And the alert is stored one time.
    assert [a["rule_id"] for a in client.get("/api/alerts").json()] == ["R002"]


def test_default_rule_fields_match_collector():
    """Every field a default rule reads must be a field the collector sends."""
    tree = ast.parse((_REPO_ROOT / "scripts" / "send_real_telemetry.py").read_text())
    collector_keys = {
        key.value
        for node in ast.walk(tree)
        if isinstance(node, ast.Assign) and isinstance(node.value, ast.Dict)
        and getattr(node.targets[0], "id", None) == "telemetry"
        for key in node.value.keys
    }
    assert {"cpu_percent", "cmd"} <= collector_keys

    from app.rules import rules_engine

    rule_fields = {c["field"] for rule in rules_engine.load_rules() for c in rule["conditions"]}
    assert rule_fields <= collector_keys


def test_dashboard_endpoints_return_stored_data(backend, client):
    client.post("/api/telemetry", json=COLLECTOR_RECORD)

    # Host view: counts of stored rows, no invented score.
    host = client.get("/api/host/test-host")
    assert host.status_code == 200
    body = host.json()
    assert body["process_events"] == 1
    assert body["alert_count"] == 1
    assert body["alerts_by_severity"] == {"critical": 1}
    assert body["worst_severity"] == "critical"
    assert body["status"] == "online"
    assert "trust_score" not in body
    assert client.get("/api/host/no-such-host").status_code == 404

    agents = client.get("/api/telemetry/agents").json()
    assert [(a["hostname"], a["status"], a["worst_severity"]) for a in agents] == [("test-host", "online", "critical")]

    proc = client.get("/api/telemetry/all").json()["processes"][0]
    assert (proc["process_name"], proc["cmd"], proc["cpu_percent"]) == ("bash", "rm -rf /tmp/scratch", 1.5)
    # Fields no collector sends are not served.
    assert "risk_score" not in proc and "risk_level" not in proc


def test_host_without_alerts_has_no_severity(backend, client):
    quiet = {**COLLECTOR_RECORD, "hostname": "quiet-host", "cmd": "sleep 1"}
    client.post("/api/telemetry", json=quiet)
    body = client.get("/api/host/quiet-host").json()
    assert (body["alert_count"], body["alerts_by_severity"], body["worst_severity"]) == (0, {}, None)


def test_rules_route_lists_loaded_rules(backend, client):
    assert [r["id"] for r in client.get("/api/rules").json()] == ["R001", "R002"]


def test_collaboration_round_trip(client, tmp_path, monkeypatch):
    from app.routes import collaboration

    log = tmp_path / "collaboration.json"
    log.write_text("[]")
    monkeypatch.setattr(collaboration, "COLLAB_LOG", log)

    # The dashboard used to send "message", which the model rejected with 422.
    assert client.post("/api/collaboration", json={"author": "a", "message": "x"}).status_code == 422
    assert client.post("/api/collaboration", json={"author": "a", "content": "x"}).status_code == 200
    [msg] = client.get("/api/collaboration").json()
    assert (msg["author"], msg["content"]) == ("a", "x")
    assert msg["timestamp"]


_NO_PID = {k: v for k, v in COLLECTOR_RECORD.items() if k != "pid"}


@pytest.mark.parametrize(
    "body",
    [{}, _NO_PID, {"processes": [COLLECTOR_RECORD, {"pid": 1}]}, {"processes": "x"}, []],
)
def test_bad_telemetry_is_422_and_stores_nothing(backend, client, body):
    res = client.post("/api/telemetry", json=body)
    assert res.status_code == 422, res.text
    assert client.get("/api/telemetry/all").json()["processes"] == []

"""Route table, rule loading and telemetry tests for the FastAPI backend.

Covers four bugs that were fixed together:
  * 11 routers were mounted at /api/api/... and some dashboard calls had no route.
  * /api/analyze imported loaded_rules by value, so it always saw an empty list.
  * default_rules.json read cpu_usage and command, but the collector sends
    cpu_percent and cmd.
  * /api/telemetry ran the rules twice per event.
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
#   api/useScore.ts, components/AgentRegistryPanel.tsx, AlertTable.tsx,
#   ArchivedReplayViewer.tsx, CollaborationPanel.tsx, Dashboard.tsx,
#   EndpointGrid.tsx, ExplainPanel.tsx, FeedbackLogPanel.tsx, GNNReplay.tsx,
#   GamificationPanel.tsx, GlobalRiskHeatmap.tsx, IOCViewerPanel.tsx,
#   MetricsRow.tsx, ProcessDetailModal.tsx, ProcessTable.tsx, RootCauseGraph.tsx,
#   RootCauseTrace.tsx, SOCAssistantPanel.tsx, TimelinePanel.tsx,
#   TrustHistoryChart.tsx, TrustPanel.tsx, hooks/useTelemetry.ts,
#   pages/Dashboard.tsx, pages/ExplainPage.tsx, pages/FeedbackPage.tsx
DASHBOARD_CALLS = [
    ("POST", "/api/score"),
    ("GET", "/api/agent"),
    ("GET", "/api/alerts/{param}"),
    ("GET", "/api/assistant/explain"),
    ("GET", "/api/collaboration"),
    ("POST", "/api/collaboration"),
    ("POST", "/api/explain"),
    ("GET", "/api/explain/root_cause"),
    ("GET", "/api/explain/timeline_event"),
    ("GET", "/api/feedback"),
    ("GET", "/api/gamification"),
    ("GET", "/api/gnn/archive/list"),
    ("GET", "/api/gnn/archive/load"),
    ("GET", "/api/gnn/replay"),
    ("GET", "/api/host/{param}"),
    ("GET", "/api/iocs"),
    ("GET", "/api/remediation/suggest"),
    ("POST", "/api/response/action"),
    ("GET", "/api/telemetry"),
    ("GET", "/api/telemetry/agents"),
    ("GET", "/api/telemetry/all"),
    ("GET", "/api/trust/global"),
    ("GET", "/api/trust/history"),
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
    """Point the app at a throwaway SQLite file and alert log, with rules loaded."""
    import main
    from app.rules import rules_engine
    from app.services import db

    engine = create_engine(f"sqlite:///{tmp_path / 'edr_test.db'}", connect_args={"check_same_thread": False})
    monkeypatch.setattr(db, "engine", engine)
    db.init_db()

    alert_log = tmp_path / "alert_log.json"
    alert_log.write_text("[]")
    monkeypatch.setattr(main, "ALERT_LOG_FILE", alert_log)

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

    res = client.get("/api/host/test-host")
    assert res.status_code == 200
    assert isinstance(res.json()["trust_score"], float)
    assert client.get("/api/host/no-such-host").status_code == 404

    agents = client.get("/api/telemetry/agents").json()
    assert [(a["hostname"], a["status"], a["riskLevel"]) for a in agents] == [("test-host", "online", "high")]

    assert [t["hostname"] for t in client.get("/api/trust/global").json()] == ["test-host"]

    proc = client.get("/api/telemetry/all").json()["processes"][0]
    assert (proc["process_name"], proc["cmd"], proc["cpu_percent"]) == ("bash", "rm -rf /tmp/scratch", 1.5)

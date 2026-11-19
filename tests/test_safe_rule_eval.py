"""Tests for the safe rule evaluator behind POST /rules/eval.

The endpoint used to run rules with eval(), so a rule could run any Python on
the server. These tests prove normal rules still work and that code-execution
rules are rejected before they run.
"""

from __future__ import annotations

import json

import pytest

from app.rules.safe_expr import RuleError, compile_rule, evaluate

FIELDS = ("cpu_usage", "memory", "cmd", "name")
PROC = {"cpu_usage": 91.5, "memory": 2_000_000, "cmd": "PowerShell -EncodedCommand SQBFAFgA", "name": "powershell.exe"}


def run(rule, values=PROC):
    return evaluate(compile_rule(rule, FIELDS), values)


@pytest.mark.parametrize(
    "rule, expected",
    [
        ("cpu_usage > 80", True),
        ("cpu_usage > 80 and memory < 1000", False),
        ("cpu_usage > 95 or memory > 1_000_000", True),
        ("not cpu_usage < 10", True),
        ("memory > 500 * 1024", True),
        ("'encodedcommand' in cmd.lower()", True),
        ("name in ['nc', 'ncat', 'netcat']", False),
        ("name.endswith('.exe') and cmd.startswith('PowerShell')", True),
        ("10 < cpu_usage <= 100", True),
        ("name != 'explorer.exe'", True),
    ],
)
def test_normal_rules_still_work(rule, expected):
    assert run(rule) is expected


@pytest.mark.parametrize(
    "rule",
    [
        "__import__('os').system('touch /tmp/edr_rce_proof')",
        "open('/etc/passwd').read()",
        "().__class__.__bases__[0].__subclasses__()",
        "cmd.__class__",
        "cmd.format(x=1)",
        "(lambda: 1)()",
        "[c for c in cmd]",
        "cmd[0]",
        "globals()",
        "unknown_field > 1",
        "cpu_usage ** 999999",
        "cmd.startswith(prefix='x')",
    ],
)
def test_code_execution_and_unknown_syntax_are_rejected(rule):
    with pytest.raises(RuleError):
        compile_rule(rule, FIELDS)


def test_rejected_rule_has_no_side_effect(tmp_path):
    proof = tmp_path / "proof"
    with pytest.raises(RuleError):
        compile_rule("__import__('os').system('touch %s')" % proof, FIELDS)
    assert not proof.exists()


def test_string_arithmetic_is_blocked_at_run_time():
    # Would build a gigabyte string if arithmetic on text were allowed.
    with pytest.raises(RuleError):
        run("cmd * 1000000000 == ''")


def test_bad_input_is_a_rule_error_not_a_crash():
    with pytest.raises(RuleError):
        run("cpu_usage / 0 > 1")
    with pytest.raises(RuleError):
        run("cmd > 5")
    with pytest.raises(RuleError):
        compile_rule("", FIELDS)
    with pytest.raises(RuleError):
        compile_rule("cpu_usage > 1 and " * 200 + "True", FIELDS)


def test_endpoint_skips_malicious_rules(tmp_path, monkeypatch):
    pytest.importorskip("fastapi")
    from app.routes import rules as rules_route

    rules = [
        {"rule": "cpu_usage > 80", "description": "High CPU", "severity": "medium"},
        {"rule": "__import__('os').system('touch %s')" % (tmp_path / "proof"), "description": "evil", "severity": "high"},
    ]
    rules_file = tmp_path / "rules.json"
    rules_file.write_text(json.dumps(rules))
    monkeypatch.setattr(rules_route, "RULES_FILE", rules_file)

    data = rules_route.HostTelemetry(
        hostname="host1",
        timestamp=1,
        processes=[rules_route.Process(pid=7, name="a", cmd="b", memory=1, cpu_usage=95.0, status="running")],
    )
    result = rules_route.evaluate_rules(data)

    assert [a["rule"] for a in result["alerts"]] == ["High CPU"]
    assert not (tmp_path / "proof").exists()

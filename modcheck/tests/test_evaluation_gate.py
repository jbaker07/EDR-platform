"""The evaluation gate must build its inputs or report itself blocked.

A case that needs a built artifact and finds no builder is skipped. A skipped
case has neither detected nor failed to detect anything, so counting the run as
a pass would report coverage the run never had. These tests pin both halves:
the gate supplies builders, and where it cannot, it says so instead of exiting
zero.
"""
from __future__ import annotations

from modcheck import evaluate as evaluate_mod
from modcheck.cli import build_parser
from modcheck.paths import project_root
from modcheck.store import Store


def _summary(**overrides):
    base = {"failed": 0, "skipped": 0}
    base.update(overrides)
    return base


def test_a_skipped_case_blocks_the_gate_rather_than_passing_it():
    assert evaluate_mod.gate_verdict(_summary(skipped=1)) == "blocked"
    assert evaluate_mod.gate_verdict(_summary()) == "passed"
    assert evaluate_mod.gate_verdict(_summary(failed=1)) == "failed"


def test_missing_builders_block_the_gate_even_with_nothing_failing():
    assert evaluate_mod.gate_verdict(
        _summary(), blocked_reason="builders unavailable") == "blocked"


def test_a_failure_outranks_a_block_in_the_reported_verdict():
    """Blocked is not a way to soften a real failure."""
    assert evaluate_mod.gate_verdict(
        _summary(failed=2, skipped=3)) == "failed"


def test_the_gate_loads_real_builders_in_this_checkout():
    builders, reason = evaluate_mod.load_builders()
    assert reason is None, reason
    assert "content_patcher_pack_with_changes" in builders


def test_every_case_the_gate_runs_actually_runs(tmp_path):
    """The gate's own run must not skip: it supplies the builders."""
    builders, reason = evaluate_mod.load_builders()
    assert reason is None, reason
    results = evaluate_mod.run(project_root() / "evaluation" / "cases", Store(),
                               builders=builders, workdir=tmp_path)
    summary = evaluate_mod.summarize(results)
    assert summary["skipped"] == 0, summary["skipped_case_ids"]
    assert summary["witness_cases"] > 0
    assert summary["witnesses_skipped"] == 0
    assert evaluate_mod.gate_verdict(summary) == "passed"


def test_without_builders_the_witnesses_skip_and_the_gate_does_not_pass():
    """The regression this guards: the old gate exited 0 on this run."""
    results = evaluate_mod.run(project_root() / "evaluation" / "cases", Store(),
                               builders=None)
    summary = evaluate_mod.summarize(results)
    assert summary["skipped"] > 0
    # Not every witness needs a built artifact, but at least one does, and one
    # unrun witness is enough to make the run's coverage claim untrue.
    assert 0 < summary["witnesses_skipped"] <= summary["witness_cases"]
    assert summary["failed"] == 0  # nothing *failed* -- that was the trap
    assert evaluate_mod.gate_verdict(summary) == "blocked"


def test_render_leads_with_the_verdict():
    results = evaluate_mod.run(project_root() / "evaluation" / "cases", Store(),
                               builders=None)
    text = evaluate_mod.render(results, evaluate_mod.summarize(results))
    assert text.splitlines()[0] == "Evaluation -- gate BLOCKED"
    assert "establishes nothing" in text or "establish nothing" in text


def test_cli_evaluate_exits_nonzero_when_blocked(monkeypatch, capsys):
    monkeypatch.setattr(evaluate_mod, "load_builders",
                        lambda: (None, "artifact builders could not be imported"))
    args = build_parser().parse_args(["evaluate"])
    assert args.func(args) == 1
    out = capsys.readouterr().out
    assert "gate BLOCKED" in out
    assert "artifact builders could not be imported" in out


def test_cli_evaluate_exits_zero_only_when_nothing_skipped(capsys):
    args = build_parser().parse_args(["evaluate"])
    assert args.func(args) == 0
    assert "gate PASSED" in capsys.readouterr().out


def test_the_creator_walkthrough_runs_or_says_why_not(capsys, monkeypatch):
    """The end-to-end loop must not silently no-op.

    It either runs on the pinned artifacts, or reports the evidence cache
    missing. What it must never do is print a conclusion it did not derive.
    """
    import subprocess
    import sys

    from modcheck.paths import project_root
    script = project_root() / "evaluation" / "runtime" / "walkthrough.py"
    result = subprocess.run([sys.executable, str(script)], capture_output=True,
                            text=True, timeout=180)
    assert result.returncode in (0, 2), result.stderr
    if result.returncode == 2:
        assert "BLOCKED" in result.stdout
        return
    # It ran. The steps that matter must all be present, including the one that
    # says the runtime half did not happen.
    for marker in ("1. requested outcome", "3. proposed change",
                   "4. static re-check", "6. native observation",
                   "-- player --", "-- creator --"):
        assert marker in result.stdout, marker
    assert "BLOCKED in this environment" in result.stdout
    assert "not_yet_observed" in result.stdout
    # And it must not claim the creator got what they asked for.
    assert "does NOT settle" in result.stdout

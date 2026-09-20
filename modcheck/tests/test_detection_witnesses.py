"""Every `detectable: yes` claim must have a runnable witness.

The finding-code registry (T1) established that a claimed check_id names a code
some analyzer emits. That is necessary but not sufficient: it does not show the
code is emitted *for this failure*, about the right artifact. These tests close
that gap.

A witness supplies an exact input, invokes the real analyzer, and requires the
expected code to be emitted about the expected subject and target. Cases needing
built artifacts are skipped by the CLI; here the builders are supplied, so they
actually run.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from fixtures import build
from modcheck import evaluate as evaluate_mod
from modcheck.paths import project_root
from modcheck.store import Store

CASES = project_root() / "evaluation" / "cases"

# The builder registry is defined once, in fixtures.build, and loaded by the
# CLI gate through evaluate.load_builders. Importing it here rather than
# restating it keeps the gate and these tests on the same set.
BUILDERS = build.BUILDERS


@pytest.fixture(scope="module")
def results(tmp_path_factory):
    workdir = tmp_path_factory.mktemp("witnesses")
    return evaluate_mod.run(CASES, Store(), builders=BUILDERS, workdir=workdir)


def test_every_witness_case_passes(results):
    witnesses = [r for r in results if r.case.witness_for]
    assert witnesses, "no witness cases found"
    failures = [(r.case.id, r.notes) for r in witnesses if not r.passed]
    assert not failures, f"witness cases failed: {failures}"


def test_no_case_is_skipped_when_builders_are_supplied(results):
    skipped = [r.case.id for r in results if r.skipped]
    assert not skipped, f"cases skipped despite builders being available: {skipped}"


def test_every_detectable_yes_failure_has_a_witness(results):
    """The rule this module exists to enforce."""
    store = Store()
    claimed = {
        (r.game, r.id)
        for r in store.all_records("failure")
        if (r.get("detector") or {}).get("detectable") == "yes"
    }
    witnessed = {(r.case.game, r.case.witness_for) for r in results if r.case.witness_for}
    missing = sorted(claimed - witnessed)
    assert not missing, (
        "these failures claim detectable: yes but no witness proves it. Either add "
        f"a witness case or lower the claim: {missing}")


def test_a_witness_fails_when_the_finding_is_about_the_wrong_artifact(tmp_path):
    """The check that makes a witness meaningful rather than decorative."""
    case = evaluate_mod.Case.from_dict({
        "id": "wrong_subject",
        "game": "minecraft",
        "kind": "witness",
        "witness_for": "missing_required_dependency_prevents_launch",
        "description": "expects the finding about a mod that is not the dependent one",
        "configuration": {
            "game_version": "26.3", "loader": "fabric",
            "loader_versions": {"fabric": "0.19.5", "java": "25"},
            "files_known_complete": True,
            "artifacts": [{
                "name": "dependent.jar", "mod_id": "dependent", "version": "1.0.0",
                "dependencies": [{"id": "somelib", "versions": ">=2.0.0",
                                  "required": True}],
            }],
        },
        "expect": {"findings": ["dependency.missing"], "subject": "someone_else.jar"},
    })
    result = evaluate_mod.run_case(case, Store())
    assert not result.passed
    assert any("not about" in note for note in result.notes)


def test_a_witness_fails_when_the_target_is_wrong(tmp_path):
    case = evaluate_mod.Case.from_dict({
        "id": "wrong_target",
        "game": "minecraft",
        "kind": "witness",
        "witness_for": "missing_required_dependency_prevents_launch",
        "description": "expects the finding to target a dependency it does not name",
        "configuration": {
            "game_version": "26.3", "loader": "fabric",
            "loader_versions": {"fabric": "0.19.5", "java": "25"},
            "files_known_complete": True,
            "artifacts": [{
                "name": "dependent.jar", "mod_id": "dependent", "version": "1.0.0",
                "dependencies": [{"id": "somelib", "versions": ">=2.0.0",
                                  "required": True}],
            }],
        },
        "expect": {"findings": ["dependency.missing"], "target": "a_different_lib"},
    })
    result = evaluate_mod.run_case(case, Store())
    assert not result.passed
    assert any("no finding targets" in note for note in result.notes)


def test_skipped_witness_never_counts_as_passed():
    """A witness that could not run has established nothing."""
    case = evaluate_mod.Case.from_dict({
        "id": "needs_artifacts",
        "game": "skyrimse",
        "kind": "witness",
        "witness_for": "light_plugin_formid_overflow",
        "description": "needs a built artifact",
        "configuration": {"files_known_complete": True},
        "artifacts": [{"builder": "bethesda_plugin_with_record", "name": "x.esp"}],
        "expect": {"findings": ["esplugin.invalid_light_plugin"]},
    })
    result = evaluate_mod.run_case(case, Store())  # no builders supplied
    assert result.skipped is True
    assert result.passed is False

    summary = evaluate_mod.summarize([result])
    assert summary["skipped"] == 1
    assert summary["passed"] == 0
    assert summary["witnesses_skipped"] == 1


def test_light_plugin_witness_and_its_control_discriminate(tmp_path):
    """The witness must fail for an out-of-range plugin and stay silent for a
    valid one -- otherwise it is not detecting anything."""
    from modcheck.integrations import esplugin

    if not esplugin.available():
        pytest.skip("esplugin helper not built")

    bad = build.bethesda_plugin_with_record(tmp_path / "Bad.esp", light=True,
                                            object_index=0x005000)
    good = build.bethesda_plugin_with_record(tmp_path / "Good.esp", light=True,
                                             object_index=0x000123)
    from modcheck.analyze.config import Installation

    def findings(path):
        inst = Installation.from_paths("skyrimse", [path], files_known_complete=True)
        return [f.code for f in esplugin.analyze(inst, full=True)]

    assert "esplugin.invalid_light_plugin" in findings(bad)
    assert "esplugin.invalid_light_plugin" not in findings(good)

"""Dependency resolution and reporting.

Every detection test is paired with a control: a configuration where the right
answer is to report nothing. A checker that flags everything is not useful.
"""
from __future__ import annotations

import json

import pytest

from fixtures import build
from modcheck.analyze import requirements
from modcheck.analyze.config import Installation, InstalledArtifact
from modcheck.analyze.versions import fabric_satisfies, maven_satisfies, Version
from modcheck.report import analyze_artifact, analyze_installation, render_text


def config(game="minecraft", loader="fabric", **kwargs) -> Installation:
    """A configuration we are told is complete, unless a test says otherwise."""
    kwargs.setdefault("files_known_complete", True)
    return Installation(game=game, loader=loader, **kwargs)


def artifact(name, mod_id, version, deps=None, **kwargs) -> InstalledArtifact:
    return InstalledArtifact(name=name, mod_id=mod_id, version=version,
                             declared_dependencies=deps or [], **kwargs)


# --- version comparison --------------------------------------------------
@pytest.mark.parametrize("version,constraint,expected", [
    ("26.3", "~26.3-", True),
    ("26.4", "~26.3-", False),
    ("0.19.5", ">=0.19.3", True),
    ("0.19.2", ">=0.19.3", False),
    ("1.21.4", "1.21.x", True),
    ("1.22.0", "1.21.x", False),
    ("2.5.0", "^2.0.0", True),
    ("3.0.0", "^2.0.0", False),
    ("1.0.0", "*", True),
    ("0.161.0+26.3", ">=0.161.0", True),   # build metadata ignored
    ("1.2.0", ">=1.0 <2.0", True),          # space separated = AND
    ("2.2.0", ">=1.0 <2.0", False),
])
def test_fabric_version_ranges(version, constraint, expected):
    assert fabric_satisfies(version, constraint) is expected


def test_fabric_array_constraint_is_or():
    assert fabric_satisfies("1.0.0", [">=2.0", "<1.5"]) is True
    assert fabric_satisfies("1.7.0", [">=2.0", "<1.5"]) is False


def test_non_semver_version_compares_only_for_equality():
    assert fabric_satisfies("some-build", "some-build") is True
    assert fabric_satisfies("some-build", ">=1.0.0") is None  # undecidable, not false


def test_empty_prerelease_is_the_earliest_in_its_series():
    assert Version.parse("1.2-").compare(Version.parse("1.2.0")) < 0
    assert Version.parse("1.2.0").compare(Version.parse("1.2.0-alpha")) > 0


@pytest.mark.parametrize("version,constraint,expected", [
    ("1.21.4", "[1.21,1.22)", True),
    ("1.22", "[1.21,1.22)", False),
    ("47.1", "[47,)", True),
    ("46.0", "[47,)", False),
])
def test_maven_ranges(version, constraint, expected):
    assert maven_satisfies(version, constraint) is expected


# --- dependency resolution ----------------------------------------------
def test_detects_missing_dependency():
    inst = config()
    inst.artifacts = [artifact("a.jar", "a", "1.0",
                               [{"id": "needed", "versions": "*", "required": True}])]
    codes = [f.code for f in requirements.analyze(inst)]
    assert "dependency.missing" in codes


def test_control_present_dependency_is_not_reported():
    inst = config()
    inst.artifacts = [
        artifact("a.jar", "a", "1.0", [{"id": "b", "versions": ">=1.0", "required": True}]),
        artifact("b.jar", "b", "1.5"),
    ]
    assert requirements.analyze(inst) == []


def test_optional_missing_dependency_is_not_an_error():
    inst = config()
    inst.artifacts = [artifact("a.jar", "a", "1.0",
                               [{"id": "jei", "versions": "*", "required": False}])]
    assert requirements.analyze(inst) == []


def test_detects_version_mismatch_and_says_what_it_does_not_establish():
    inst = config(game_version="1.21.4", loader_versions={"fabric": "0.16.9"})
    inst.artifacts = [artifact("a.jar", "a", "1.0",
                               [{"id": "minecraft", "versions": "~26.3", "required": True}])]
    findings = [f for f in requirements.analyze(inst) if f.code == "dependency.version_mismatch"]
    assert len(findings) == 1
    assert findings[0].evidence_class == "derived"
    assert "only declares a narrower range" in findings[0].not_established


def test_unknown_platform_version_is_unresolved_not_a_mismatch():
    inst = config()  # no game_version supplied
    inst.artifacts = [artifact("a.jar", "a", "1.0",
                               [{"id": "minecraft", "versions": "~26.3", "required": True}])]
    codes = [f.code for f in requirements.analyze(inst)]
    assert codes == ["dependency.platform_unknown"]


def test_duplicate_mod_id_is_a_real_collision():
    inst = config()
    inst.artifacts = [artifact("a.jar", "same", "1.0"), artifact("b.jar", "same", "2.0")]
    findings = [f for f in requirements.analyze(inst) if f.code == "dependency.duplicate_mod_id"]
    assert len(findings) == 1
    assert findings[0].evidence_class == "extracted"


def test_jar_in_jar_module_satisfies_a_dependency(tmp_path):
    """Resolution that ignores nested modules reports a false missing dependency."""
    from modcheck.inspect import inspect_path
    import zipfile

    inner = build.fabric_jar(tmp_path / "inner.jar", mod_id="nestedlib", version="2.0.0")
    outer = build.fabric_jar(tmp_path / "outer.jar", mod_id="outer", version="1.0.0")
    with zipfile.ZipFile(outer, "a") as zf:
        zf.write(inner, "META-INF/jars/nestedlib-2.0.0.jar")

    ins = inspect_path(outer)
    inst = config()
    inst.artifacts = [
        InstalledArtifact(name="outer.jar", mod_id="outer", version="1.0.0", inspection=ins,
                          declared_dependencies=[]),
        artifact("consumer.jar", "consumer", "1.0",
                 [{"id": "nestedlib", "versions": ">=2.0.0", "required": True}]),
    ]
    findings = requirements.analyze(inst)
    assert not [f for f in findings if f.code == "dependency.missing"]
    nested = [f for f in findings if f.code == "dependency.satisfied_by_nested_module"]
    assert nested and "nested in outer.jar" in nested[0].summary


def test_declared_breaks_is_reported_when_the_other_mod_is_present(tmp_path):
    from modcheck.inspect import inspect_path
    jar = build.fabric_jar(tmp_path / "b.jar", mod_id="breaker",
                           breaks={"victim": "*"})
    ins = inspect_path(jar)
    inst = config()
    inst.artifacts = [
        InstalledArtifact(name="b.jar", mod_id="breaker", version="1.0.0", inspection=ins),
        artifact("v.jar", "victim", "1.0"),
    ]
    codes = [f.code for f in requirements.analyze(inst)]
    assert "dependency.breaks" in codes


def test_control_declared_breaks_absent_mod_is_silent(tmp_path):
    from modcheck.inspect import inspect_path
    jar = build.fabric_jar(tmp_path / "b.jar", mod_id="breaker", breaks={"victim": "*"})
    ins = inspect_path(jar)
    inst = config()
    inst.artifacts = [InstalledArtifact(name="b.jar", mod_id="breaker", version="1.0.0",
                                        inspection=ins)]
    assert not [f for f in requirements.analyze(inst) if f.code == "dependency.breaks"]


# --- reports -------------------------------------------------------------
def test_report_separates_evidence_and_states_coverage(tmp_path):
    jar = build.fabric_jar(tmp_path / "a.jar")
    report = analyze_artifact(jar, game="minecraft")
    assert report.checked and report.not_checked
    text = render_text(report)
    assert "what was NOT checked" in text
    assert "not a guarantee" in text
    # No invented confidence numbers anywhere.
    assert "%" not in text
    assert "guaranteed" not in text.lower()


def test_report_round_trips_as_json(tmp_path):
    jar = build.fabric_jar(tmp_path / "a.jar")
    report = analyze_artifact(jar, game="minecraft")
    data = json.loads(json.dumps(report.as_dict(), default=str))
    assert data["kind"] == "artifact"
    assert data["coverage"]["not_checked"]
    assert data["subject"]["artifact"]["sha256"]


def test_partial_file_view_is_flagged_so_absence_is_not_read_as_evidence(tmp_path):
    jar = build.fabric_jar(tmp_path / "a.jar")
    report = analyze_artifact(jar, game="minecraft")
    assert [f for f in report.findings if f.code == "coverage.partial_file_view"]


def test_incomplete_configuration_does_not_claim_a_dependency_is_missing():
    """Absence from a partial listing is not evidence of absence."""
    deps = [{"id": "somelib", "versions": ">=2.0.0", "required": True}]
    partial = config(files_known_complete=False)
    partial.artifacts = [artifact("a.jar", "a", "1.0", deps)]
    codes = [f.code for f in requirements.analyze(partial)]
    assert codes == ["dependency.presence_unresolved"]

    complete = config(files_known_complete=True)
    complete.artifacts = [artifact("a.jar", "a", "1.0", deps)]
    codes = [f.code for f in requirements.analyze(complete)]
    assert codes == ["dependency.missing"]

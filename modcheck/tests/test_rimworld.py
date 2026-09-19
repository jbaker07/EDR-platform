"""RimWorld interaction analysis: declared rules and the community database.

The community-database fixture is synthetic. The real database carries no
licence file, so its reuse terms are unknown and its content is not reproduced
here; only its structure is, which is what the parser must handle.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from fixtures import build
from modcheck.analyze.config import Installation, InstalledArtifact
from modcheck.inspect import inspect_path
from modcheck.integrations import rimworld as rw

RULES = Path(__file__).parent / "fixtures" / "rimworld_community_rules.json"


@pytest.fixture(scope="module")
def rules() -> rw.CommunityRules:
    return rw.CommunityRules.from_bytes(RULES.read_bytes(),
                                        source_id="rimsort_community_rules")


def install(*mods: str) -> Installation:
    inst = Installation(game="rimworld", files_known_complete=True)
    for index, package_id in enumerate(mods):
        inst.artifacts.append(InstalledArtifact(
            name=f"{package_id}.zip", mod_id=package_id, load_index=index))
    return inst


# --- community rules -----------------------------------------------------
def test_detects_wrong_order_from_the_community_database(rules):
    findings = rules.analyze(install("example.patch", "example.core"))
    order = [f for f in findings if f.code == "rimworld.load_order"]
    assert len(order) == 1
    assert "Example Core" in order[0].summary
    assert order[0].resolutions[0]["method"] == "load_order_change"


def test_control_correct_order_is_silent(rules):
    assert rules.analyze(install("example.core", "example.patch")) == []


def test_detects_community_recorded_incompatibility(rules):
    findings = rules.analyze(install("example.core", "example.rival"))
    inc = [f for f in findings if f.code.endswith("incompatible_present")]
    assert len(inc) == 1
    assert inc[0].severity == "error"


def test_control_unknown_mods_produce_nothing(rules):
    assert rules.analyze(install("nobody.knows", "this.either")) == []


def test_package_ids_are_matched_case_insensitively(rules):
    findings = rules.analyze(install("Example.Patch", "EXAMPLE.CORE"))
    assert [f for f in findings if f.code == "rimworld.load_order"]


def test_rule_for_an_absent_mod_is_not_reported(rules):
    """example.patch must load before example.late, which is not installed."""
    findings = rules.analyze(install("example.core", "example.patch"))
    assert not [f for f in findings if "Example Late" in f.summary]


# --- declared rules ------------------------------------------------------
def test_declared_load_after_is_checked_without_any_external_database(tmp_path):
    a = build.rimworld_mod(tmp_path / "a.zip", package_id="example.after",
                           load_after=["example.first"])
    b = build.rimworld_mod(tmp_path / "b.zip", package_id="example.first")
    inst = Installation(game="rimworld", files_known_complete=True)
    for index, path in enumerate([a, b]):
        ins = inspect_path(path)
        inst.artifacts.append(InstalledArtifact(
            name=path.name, mod_id=ins.fact("mod_id"), load_index=index, inspection=ins))
    findings = rw.analyze_declared(inst)
    order = [f for f in findings if f.code == "rimworld.load_order"]
    assert len(order) == 1
    assert order[0].evidence_class == "declared"


def test_control_declared_order_satisfied_is_silent(tmp_path):
    a = build.rimworld_mod(tmp_path / "a.zip", package_id="example.after",
                           load_after=["example.first"])
    b = build.rimworld_mod(tmp_path / "b.zip", package_id="example.first")
    inst = Installation(game="rimworld", files_known_complete=True)
    for index, path in enumerate([b, a]):   # correct order
        ins = inspect_path(path)
        inst.artifacts.append(InstalledArtifact(
            name=path.name, mod_id=ins.fact("mod_id"), load_index=index, inspection=ins))
    assert rw.analyze_declared(inst) == []


def test_coverage_states_what_is_not_checked():
    checked, not_checked = rw.coverage(with_community=False)
    assert checked
    assert any("Def-level" in item for item in not_checked)
    assert any("absence of a rule is not evidence" in item for item in not_checked)
    assert any("community rules" in item for item in not_checked)


def test_rejects_a_file_that_is_not_a_rules_database():
    with pytest.raises(ValueError, match="not a community rules database"):
        rw.CommunityRules.from_bytes(b'{"something": 1}', source_id="x")

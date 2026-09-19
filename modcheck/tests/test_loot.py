"""LOOT integration tests against real upstream metadata.

The fixture is a verbatim excerpt of the CC0-licensed LOOT masterlist for
Skyrim SE, so these assert ModCheck reproduces LOOT's own semantics on genuine
data -- including the controls, where the right behaviour is to say nothing.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from modcheck.analyze.config import Installation, InstalledArtifact, normalize_path
from modcheck.integrations.loot import LootMasterlist
from modcheck.integrations.loot_conditions import (ConditionError, compile_path_regex,
                                                   evaluate, is_regex, tri_and, tri_or)

EXCERPT = Path(__file__).parent / "fixtures" / "loot_skyrimse_excerpt.yaml"

# CRCs taken from the real masterlist entries in the fixture.
UPDATE_ESM_DIRTY_CRC = 397106720          # recorded as 334 ITM / 92 UDR / 3 NAV
FLORA_RESPAWN_CLEAN_CRC = 2681493071      # recorded as verified clean


@pytest.fixture(scope="module")
def masterlist() -> LootMasterlist:
    return LootMasterlist.from_bytes(EXCERPT.read_bytes(), game="skyrimse",
                                     source_id="loot_masterlist_skyrimse")


def make_install(names, *, complete_files=True, crcs=None, files=None) -> Installation:
    inst = Installation(game="skyrimse", files_known_complete=complete_files)
    for index, name in enumerate(names):
        inst.artifacts.append(InstalledArtifact(
            name=name, load_index=index, crc32=(crcs or {}).get(name)))
        inst.files.add(normalize_path(name))
    for f in files or []:
        inst.files.add(normalize_path(f))
    return inst


# --- true positives ------------------------------------------------------
def test_detects_a_real_recorded_incompatibility(masterlist):
    inst = make_install(["WM Flora Fixes.esp", "FloraRespawnFix.esp"])
    findings = masterlist.analyze(inst)
    inc = [f for f in findings if f.code == "loot.incompatible_present"]
    assert any(f.subject == "WM Flora Fixes.esp"
               and "FloraRespawnFix.esp" in f.summary for f in inc)
    finding = inc[0]
    assert finding.evidence_class == "derived"
    assert finding.sources == ["loot_masterlist_skyrimse"]
    assert finding.not_established


def test_detects_mutually_incompatible_pair_from_both_sides(masterlist):
    inst = make_install(["Unofficial Skyrim Modders Patch.esp",
                         "Unofficial Skyrim Modders Patch VR.esp"])
    subjects = {f.subject for f in masterlist.analyze(inst)
                if f.code == "loot.incompatible_present"}
    assert subjects == {"Unofficial Skyrim Modders Patch.esp",
                        "Unofficial Skyrim Modders Patch VR.esp"}


def test_detects_dirty_edits_only_for_the_exact_matching_copy(masterlist):
    dirty = make_install(["Update.esm"], crcs={"Update.esm": UPDATE_ESM_DIRTY_CRC})
    findings = [f for f in masterlist.analyze(dirty) if f.code == "loot.dirty_edits"]
    assert len(findings) == 1
    assert "334 ITM" in findings[0].summary and "92 UDR" in findings[0].summary
    assert f"{UPDATE_ESM_DIRTY_CRC:08X}" in findings[0].detail

    # A different copy of the same-named plugin must NOT inherit that record.
    other = make_install(["Update.esm"], crcs={"Update.esm": 0x12345678})
    assert not [f for f in masterlist.analyze(other) if f.code == "loot.dirty_edits"]


def test_matching_version_string_is_not_matching_artifact(masterlist):
    """A plugin with no CRC at all yields no dirty finding, not a guessed one."""
    unknown = make_install(["Update.esm"])  # no CRC supplied
    assert not [f for f in masterlist.analyze(unknown) if f.code == "loot.dirty_edits"]


def test_detects_wrong_load_order(masterlist):
    wrong = make_install(["ButterfliesUnchained.esp", "Complete Alchemy & Cooking Overhaul.esp"])
    findings = [f for f in masterlist.analyze(wrong) if f.code == "loot.load_order"]
    assert len(findings) == 1
    assert findings[0].resolutions[0]["method"] == "load_order_change"

    right = make_install(["Complete Alchemy & Cooking Overhaul.esp", "ButterfliesUnchained.esp"])
    assert not [f for f in masterlist.analyze(right) if f.code == "loot.load_order"]


def test_regex_plugin_entry_matches(masterlist):
    inst = make_install(["Skyrim Project Optimization - Full ESL Version.esm",
                         "unofficial skyrim special edition patch.esp"])
    findings = masterlist.analyze(inst)
    # The regex entry carries an `after` rule; the order above violates it.
    assert any(f.code == "loot.load_order" for f in findings)


def test_message_conditions_are_evaluated_and_substituted(masterlist):
    inst = make_install(["ButterfliesUnchained.esp", "Unofficial Skyrim Modders Patch.esp"])
    msgs = [f for f in masterlist.analyze(inst) if f.code.startswith("loot.message")]
    # condition: active("Unofficial Skyrim Modders Patch.esp") -> true here
    hit = [f for f in msgs if f.subject == "ButterfliesUnchained.esp" and f.severity == "error"]
    assert hit, [f.summary for f in msgs]
    assert "{0}" not in hit[0].summary and "Unofficial Skyrim Modder" in hit[0].summary


# --- controls: the analysis must stay silent -----------------------------
def test_control_clean_configuration_produces_no_errors(masterlist):
    """A configuration LOOT records nothing bad about must not be flagged."""
    inst = make_install(["FloraRespawnFix.esp"], crcs={"FloraRespawnFix.esp": FLORA_RESPAWN_CLEAN_CRC})
    findings = masterlist.analyze(inst)
    assert not [f for f in findings if f.severity in ("error", "blocker")]
    assert [f for f in findings if f.code == "loot.verified_clean"]


def test_control_unrelated_plugins_produce_nothing(masterlist):
    inst = make_install(["SomeModNobodyHasHeardOf.esp", "AnotherOne.esp"])
    assert masterlist.analyze(inst) == []


def test_control_two_plugins_touching_nothing_shared_are_not_flagged(masterlist):
    """Co-presence is not a conflict. Only a recorded rule makes it one."""
    inst = make_install(["FloraRespawnFix.esp", "ButterfliesUnchained.esp"])
    assert not [f for f in masterlist.analyze(inst)
                if f.code == "loot.incompatible_present"]


# --- honest unknowns -----------------------------------------------------
def test_unknown_file_presence_is_unresolved_not_a_missing_requirement(masterlist):
    """We do not know whether SKSE is installed, so we must not claim it is missing."""
    inst = make_install(["WM Flora Fixes.esp"], complete_files=False)
    findings = masterlist.analyze(inst)
    unresolved = [f for f in findings if f.code == "loot.requirement_unresolved"]
    missing = [f for f in findings if f.code == "loot.requirement_missing"]
    assert unresolved, "expected an unresolved requirement"
    assert all(f.evidence_class == "unresolved" for f in unresolved)
    assert not missing, "must not assert a requirement is missing from an incomplete view"


def test_complete_file_view_turns_unknown_into_a_real_missing_requirement(masterlist):
    inst = make_install(["WM Flora Fixes.esp"], complete_files=True)
    findings = masterlist.analyze(inst)
    missing = [f for f in findings if f.code == "loot.requirement_missing"]
    assert missing
    assert all(f.evidence_class == "derived" for f in missing)


def test_condition_gates_a_requirement_off(masterlist):
    """WM Flora Fixes requires SKSE-VR only when SkyrimVR.exe is present."""
    vr = make_install(["WM Flora Fixes.esp"], complete_files=True,
                      files=["../skyrimvr.exe", "../sksevr_loader.exe"])
    names = {f.targets[0]["id"] for f in masterlist.analyze(vr)
             if f.code == "loot.requirement_missing"}
    # The VR loader is present and the non-VR requirement is gated off by the
    # `not file("../SkyrimVR.exe")` condition, so neither should be reported.
    assert "../sksevr_loader.exe" not in names
    assert "../skse64_loader.exe" not in names


# --- condition evaluator -------------------------------------------------
def test_three_valued_logic_does_not_collapse_unknown():
    assert tri_and(True, None) is None
    assert tri_and(False, None) is False      # false dominates and
    assert tri_or(True, None) is True         # true dominates or
    assert tri_or(False, None) is None


def test_regex_detection_follows_the_documented_rule():
    assert is_regex("Example\\.esm")
    assert is_regex("a|b.esp")
    assert not is_regex("Update.esm")


def test_path_regex_only_applies_to_the_filename():
    pattern = compile_path_regex("skse/plugins/(SSE)?ShaderTools\\.dll")
    assert pattern.match("skse/plugins/SSEShaderTools.dll")
    assert pattern.match("skse/plugins/ShaderTools.dll")
    assert not pattern.match("other/plugins/ShaderTools.dll")


def test_malformed_condition_is_an_error_not_a_silent_true():
    with pytest.raises(ConditionError):
        evaluate("file(", lambda call: True)


def test_licensing_is_stated_per_masterlist_not_generalised():
    """The FNV masterlist is GPL-3.0 while the other two are CC0-1.0.

    This test exists because the module previously claimed all three were CC0,
    which was wrong for Fallout: New Vegas. Licence terms differ per repository
    and must not be generalised from one of them.
    """
    import modcheck.integrations.loot as loot_module

    doc = loot_module.__doc__
    assert "GPL-3.0" in doc, "the FNV masterlist's differing licence must be stated"
    assert "CC0-1.0" in doc
    assert "The masterlists are CC0" not in doc

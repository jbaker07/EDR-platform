"""Cross-validation of our header parser against esplugin.

Our Bethesda inspector reads the TES4 header with our own code. esplugin is the
parser LOOT uses. Where both read the same file they must agree -- that is what
turns a synthetic fixture from something we wrote against our own understanding
into something an independent, production implementation confirms.

Skipped when the optional helper is not built.
"""
from __future__ import annotations

import pytest

from fixtures import build
from modcheck.analyze.config import Installation, InstalledArtifact
from modcheck.inspect import inspect_path
from modcheck.integrations import esplugin

needs_helper = pytest.mark.skipif(
    not esplugin.available(),
    reason="esplugin helper not built (cd helpers/esplugin-cli && cargo build --release)")


@needs_helper
@pytest.mark.parametrize("light,master,masters", [
    (True, False, ["Skyrim.esm", "Update.esm"]),
    (False, False, ["Skyrim.esm"]),
    (False, True, []),
])
def test_our_header_parser_agrees_with_esplugin(tmp_path, light, master, masters):
    path = build.bethesda_plugin(tmp_path / "Test.esp", light=light, master_flag=master,
                                 masters=masters, record_count=42, header_version=1.71)
    ours = inspect_path(path, game="skyrimse")
    theirs = esplugin.run("skyrimse", [path])["plugins"][0]

    assert ours.fact("masters") == theirs["masters"]
    assert ours.fact("light_flag_set") is theirs["is_light"]
    assert ours.fact("is_master") is theirs["is_master"]
    assert ours.fact("record_count") == theirs["record_and_group_count"]
    assert abs(ours.fact("header_version") - theirs["header_version"]) < 1e-4


@needs_helper
def test_esplugin_reports_a_corrupt_plugin_rather_than_guessing(tmp_path):
    bad = tmp_path / "Broken.esp"
    bad.write_bytes(b"TES4" + b"\xff" * 40)
    data = esplugin.run("skyrimse", [bad])
    assert "error" in data["plugins"][0]


@needs_helper
def test_analysis_reports_overlap_as_a_place_to_look_not_a_conflict(tmp_path):
    a = build.bethesda_plugin(tmp_path / "A.esp", masters=["Skyrim.esm"])
    b = build.bethesda_plugin(tmp_path / "B.esp", masters=["Skyrim.esm"])
    inst = Installation(game="skyrimse", files_known_complete=True)
    for index, path in enumerate([a, b]):
        inst.artifacts.append(InstalledArtifact(
            name=path.name, path=str(path), load_index=index))
    findings = esplugin.analyze(inst, full=True)
    for finding in findings:
        if finding.code == "esplugin.record_overlap":
            assert finding.severity == "note"
            assert "not a conflict on its own" in finding.detail


def test_unavailable_helper_is_reported_not_silently_skipped():
    checked, not_checked = esplugin.coverage("skyrimse", ran=False, full=False)
    assert checked == []
    assert any("helper is not built" in item for item in not_checked)


def test_non_bethesda_games_are_a_no_op():
    inst = Installation(game="minecraft")
    assert esplugin.analyze(inst) == []
    assert esplugin.coverage("minecraft", ran=True, full=True) == ([], [])


@needs_helper
def test_fixtures_are_valid_complete_plugins_not_just_valid_headers(tmp_path):
    """esplugin's full parse rejects trailing bytes after the header record.

    This test exists because it failed: the fixture builder used to pad the
    file, which made it header-valid but not a valid plugin. Our own
    header-only reader could not have caught that.
    """
    path = build.bethesda_plugin(tmp_path / "Full.esp", masters=["Skyrim.esm"])
    data = esplugin.run("skyrimse", [path], full=True)
    entry = data["plugins"][0]
    assert "error" not in entry, entry.get("error")
    assert entry["override_record_count"] == 0
    assert data["overlap_checked"] is True

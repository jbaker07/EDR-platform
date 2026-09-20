"""Provisioning must produce a runnable install, not a parseable one.

The distinction has teeth. Content Patcher skips a `Load` whose `FromFile` does
not exist, logs a warning, and the patch then reports as not applied -- which is
indistinguishable, in a transcript, from losing a priority contest. An install
missing its assets would therefore produce transcripts that look like evidence
about resolution and are evidence about packaging.

Network-dependent tests are skipped where the upstream pack is not reachable;
the pure logic is tested unconditionally.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

import pytest

from modcheck.jsonc import strip_jsonc
from modcheck.observe.capture import load_capture, validate
from modcheck.observe import load_predictions
from modcheck.paths import project_root

sys.path.insert(0, str(project_root() / "evaluation" / "runtime"))
import provision  # noqa: E402

PREDICTIONS = project_root() / "evaluation" / "runtime" / "predictions"
CACHED_CONTENT = (project_root() / "evidence_cache" / "17" /
                  "1700837712028ded37fd7b7d4c238a1338fcbd8ddfa8eb9c430acc9fb336bc0e.json")


def test_every_case_names_a_real_prediction():
    known = {p.id for p in load_predictions(PREDICTIONS)}
    assert set(provision.CASES) <= known, set(provision.CASES) - known


def test_a_config_token_expands_over_every_allowed_value():
    """Fetching only the default would leave four of five configs unrunnable."""
    content = {
        "ConfigSchema": {"WhichBear": {"AllowValues": "brown, black, panda",
                                       "Default": "brown"}},
        "Changes": [{"Action": "Load", "FromFile": "assets/BearMount_{{WhichBear}}.png"},
                    {"Action": "EditImage", "FromFile": "assets/SaddleOverlay.png"}],
    }
    assert provision.referenced_assets(content) == [
        "assets/BearMount_black.png", "assets/BearMount_brown.png",
        "assets/BearMount_panda.png", "assets/SaddleOverlay.png"]


def test_an_unknown_token_is_left_alone_rather_than_guessed():
    content = {"ConfigSchema": {},
               "Changes": [{"FromFile": "assets/{{Season}}.png"}]}
    assert provision.referenced_assets(content) == ["assets/{{Season}}.png"]


@pytest.mark.skipif(not CACHED_CONTENT.exists(), reason="evidence cache not present")
def test_the_real_pack_references_ten_distinct_assets():
    content = json.loads(strip_jsonc(CACHED_CONTENT.read_bytes()))
    assets = provision.referenced_assets(content)
    assert len(assets) == 10, assets
    assert "assets/BearMount_brown.png" in assets      # the default bear
    assert "assets/StableOverlay.png" in assets


def test_case_resolution_is_case_insensitive_and_keeps_the_published_name():
    """SMAPI resolves this on Linux/Android; NTFS and APFS do it elsewhere."""
    available = {"assets/bearmount_brown.png": "assets/BearMount_Brown.png"}
    assert provision._resolve_case_insensitively(
        "assets/BearMount_brown.png", available) == "assets/BearMount_Brown.png"
    assert provision._resolve_case_insensitively(
        "assets/BearMount_grizzly.png", available) is None


def test_config_is_written_from_the_schema_defaults_plus_the_change(tmp_path):
    pack = tmp_path / "pack"
    pack.mkdir()
    (pack / "content.json").write_bytes(json.dumps({
        "ConfigSchema": {"UseSaddle": {"Default": "false"},
                         "ShowReins": {"Default": "false"}},
        "Changes": []}).encode())
    name, digest = provision.write_config(pack, {"UseSaddle": "true"})
    assert name == "config.json"
    written = json.loads((pack / "config.json").read_text())
    assert written == {"UseSaddle": "true", "ShowReins": "false"}, (
        "the controlled change must be the only difference from the declared defaults")
    assert len(digest) == 64


def test_no_config_is_written_when_a_case_changes_none(tmp_path):
    pack = tmp_path / "pack"
    pack.mkdir()
    (pack / "content.json").write_bytes(b'{"ConfigSchema": {}, "Changes": []}')
    assert provision.write_config(pack, {}) is None
    assert not (pack / "config.json").exists()


def test_provision_refuses_a_non_empty_output_directory(tmp_path, monkeypatch, capsys):
    """It writes mod folders; it must never be aimed at a real install."""
    out = tmp_path / "mods"
    out.mkdir()
    (out / "SomeonesRealMod").mkdir()
    monkeypatch.setattr(sys, "argv",
                        ["provision.py", "--case", "sv_baseline_bear_mounts",
                         "--out", str(out)])
    assert provision.main() == 2
    assert "is not empty" in capsys.readouterr().err
    assert (out / "SomeonesRealMod").exists(), "it must not have touched anything"


def test_transcript_filenames_are_derived_from_the_command():
    assert provision._slug('patch summary asset "Animals/horse" full') == (
        "summary-asset-animals-horse-full")
    assert provision._slug("patch dump applied") == "dump-applied"


@pytest.mark.skipif(not CACHED_CONTENT.exists(), reason="evidence cache not present")
def test_provisioning_produces_a_manifest_that_binds(tmp_path, monkeypatch):
    """End to end: what provisioning writes must satisfy the binding check.

    Only the versions and transcript hashes are left for a human, so a
    mis-provisioned install fails here rather than after someone has spent an
    evening capturing transcripts against it.
    """
    out = tmp_path / "mods"
    monkeypatch.setattr(sys, "argv",
                        ["provision.py", "--case", "sv_deliberate_conflict",
                         "--out", str(out)])
    try:
        assert provision.main() == 0
    except SystemExit as exc:                      # fetch failed; not a logic failure
        pytest.skip(f"upstream unreachable: {exc}")

    manifest = next(out.glob("*.capture.yaml"))
    capture = load_capture(manifest)
    prediction = next(p for p in load_predictions(PREDICTIONS)
                      if p.id == "sv_deliberate_conflict")

    # Fill only what a human is meant to fill.
    capture.versions = {"stardew_valley": "1.6.15", "smapi": "4.1.10",
                        "content_patcher": "2.8.0"}
    capture.transcripts = []
    binding = validate(capture, prediction, Path(out))
    assert binding.ok, binding.problems

    # And the install really is complete: every referenced asset resolves.
    pack = out / provision.BEAR_DIR
    content = json.loads(strip_jsonc((pack / "content.json").read_bytes()))
    present = {p.relative_to(pack).as_posix().lower()
               for p in pack.rglob("*") if p.is_file()}
    for asset in provision.referenced_assets(content):
        assert asset.lower() in present, f"{asset} would make its Load silently skip"

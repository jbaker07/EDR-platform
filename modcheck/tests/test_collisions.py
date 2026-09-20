"""Same-target collisions.

Each detection is paired with a control proving the analyzer distinguishes a
real collision from mere co-presence.
"""
from __future__ import annotations

import pytest

from fixtures import build
from modcheck.analyze import collisions
from modcheck.analyze.config import Installation, InstalledArtifact
from modcheck.inspect import inspect_path


def install(game: str, *paths) -> Installation:
    inst = Installation(game=game, files_known_complete=True)
    for index, path in enumerate(paths):
        ins = inspect_path(path, game=game)
        inst.artifacts.append(InstalledArtifact(
            name=path.name, mod_id=ins.fact("mod_id"), load_index=index, inspection=ins))
    return inst


# --- The Sims 4 ----------------------------------------------------------
def test_sims4_same_resource_key_is_a_collision(tmp_path):
    key = (0x220557DA, 0, 0x0123456789ABCDEF)
    a = build.dbpf_package(tmp_path / "a.package", resources=[key])
    b = build.dbpf_package(tmp_path / "b.package", resources=[key])
    findings = collisions.analyze(install("sims4", a, b))
    assert [f.code for f in findings] == ["collision.sims4_resource_key"]
    assert findings[0].evidence_class == "extracted"
    assert "which package the game loads last" in findings[0].not_established


def test_control_sims4_different_resource_keys_are_not_a_collision(tmp_path):
    a = build.dbpf_package(tmp_path / "a.package",
                           resources=[(0x220557DA, 0, 0x1111111111111111)])
    b = build.dbpf_package(tmp_path / "b.package",
                           resources=[(0x220557DA, 0, 0x2222222222222222)])
    assert collisions.analyze(install("sims4", a, b)) == []


# --- Project Zomboid -----------------------------------------------------
def test_pz_same_lua_path_is_a_collision(tmp_path):
    a = build.pz_mod(tmp_path / "a.zip", mod_id="A", lua=["A/media/lua/client/Shared.lua"])
    b = build.pz_mod(tmp_path / "b.zip", mod_id="B", lua=["B/media/lua/client/Shared.lua"])
    findings = collisions.analyze(install("projectzomboid", a, b))
    assert [f.code for f in findings] == ["collision.pz_lua_path"]
    assert findings[0].targets[0]["id"] == "media/lua/client/Shared.lua"


def test_control_pz_different_lua_paths_are_not_a_collision(tmp_path):
    a = build.pz_mod(tmp_path / "a.zip", mod_id="A", lua=["A/media/lua/client/A.lua"])
    b = build.pz_mod(tmp_path / "b.zip", mod_id="B", lua=["B/media/lua/client/B.lua"])
    assert collisions.analyze(install("projectzomboid", a, b)) == []


# --- Cyberpunk 2077 ------------------------------------------------------
def test_cyberpunk_same_install_path_is_a_collision(tmp_path):
    a = build.cyberpunk_zip(tmp_path / "a.zip", archive_name="shared.archive")
    b = build.cyberpunk_zip(tmp_path / "b.zip", archive_name="shared.archive")
    findings = collisions.analyze(install("cyberpunk2077", a, b))
    assert [f.code for f in findings] == ["collision.cyberpunk_install_path"]


def test_control_cyberpunk_different_paths_are_not_a_collision(tmp_path):
    a = build.cyberpunk_zip(tmp_path / "a.zip", archive_name="a.archive")
    b = build.cyberpunk_zip(tmp_path / "b.zip", archive_name="b.archive")
    assert collisions.analyze(install("cyberpunk2077", a, b)) == []


# --- Stardew Valley / Content Patcher ------------------------------------
def test_two_packs_loading_the_same_asset_conflict(tmp_path):
    a = build.content_patcher_pack(tmp_path / "a.zip", unique_id="A.Pack",
                                   targets=["Data/Objects", "Portraits/Abigail"])
    b = build.content_patcher_pack(tmp_path / "b.zip", unique_id="B.Pack",
                                   targets=["Data/Objects", "Portraits/Abigail"])
    findings = collisions.analyze(install("stardewvalley", a, b))
    loads = [f for f in findings if f.code == "contentpatcher.exclusive_conflict"]
    assert len(loads) == 1
    assert loads[0].severity == "error"
    assert loads[0].targets[0]["id"] == "Portraits/Abigail"


def test_editing_the_same_entry_is_reported_as_something_to_check(tmp_path):
    a = build.content_patcher_pack(tmp_path / "a.zip", unique_id="A.Pack",
                                   fields={"128": {"Price": 100}})
    b = build.content_patcher_pack(tmp_path / "b.zip", unique_id="B.Pack",
                                   fields={"128": {"Price": 200}})
    findings = collisions.analyze(install("stardewvalley", a, b))
    same = [f for f in findings if f.code == "collision.contentpatcher_same_entry"]
    assert len(same) == 1
    assert same[0].severity == "warning"   # not an error: order decides, both apply


def test_control_editing_different_entries_of_one_asset_is_silent(tmp_path):
    """This is the normal, supported way content packs coexist."""
    a = build.content_patcher_pack(tmp_path / "a.zip", unique_id="A.Pack",
                                   fields={"128": {"Price": 100}})
    b = build.content_patcher_pack(tmp_path / "b.zip", unique_id="B.Pack",
                                   fields={"129": {"Price": 200}})
    assert collisions.analyze(install("stardewvalley", a, b)) == []


def test_conditional_load_is_not_treated_as_an_unconditional_conflict(tmp_path):
    """A Load guarded by a When condition may never apply at the same time."""
    a = build.content_patcher_pack(tmp_path / "a.zip", unique_id="A.Pack",
                                   targets=["Data/Objects", "Portraits/Abigail"],
                                   when={"Season": "spring"})
    b = build.content_patcher_pack(tmp_path / "b.zip", unique_id="B.Pack",
                                   targets=["Data/Objects", "Portraits/Abigail"])
    findings = collisions.analyze(install("stardewvalley", a, b))
    assert not [f for f in findings if f.code == "contentpatcher.exclusive_conflict"]


# --- ecosystems we deliberately do not analyse ---------------------------
def test_minecraft_shared_class_paths_are_not_reported(tmp_path):
    """Two jars with the same class path is usually a shaded library."""
    a = build.fabric_jar(tmp_path / "a.jar", mod_id="a")
    b = build.fabric_jar(tmp_path / "b.jar", mod_id="b")
    assert collisions.analyze(install("minecraft", a, b)) == []


@pytest.mark.parametrize("game", sorted(collisions.NOT_ANALYSED))
def test_unanalysed_games_say_why_in_their_coverage(game):
    checked, not_checked = collisions.coverage(game)
    assert checked == []
    assert any("same-target collisions" in item for item in not_checked)

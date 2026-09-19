"""T3: back the dependency_analysis claim per game, or do not make it.

No new resolver: analyze/requirements.py already handles these. What was
missing was evidence that it fires on each ecosystem's own dependency
declarations, so three packs were left understating what works.

Every positive is paired with a control where the dependency IS satisfied and
the right answer is silence.
"""
from __future__ import annotations

import pytest

from fixtures import build
from modcheck.analyze import requirements
from modcheck.analyze.config import Installation, InstalledArtifact
from modcheck.inspect import inspect_path


def installation(game: str, *paths, complete: bool = True) -> Installation:
    inst = Installation(game=game, files_known_complete=complete)
    for index, path in enumerate(paths):
        ins = inspect_path(path, game=game)
        inst.artifacts.append(InstalledArtifact(
            name=path.name, path=str(path), sha256=ins.sha256, load_index=index,
            version=ins.fact("version"), mod_id=ins.fact("mod_id"), inspection=ins,
            declared_dependencies=ins.fact("dependencies") or []))
    return inst


def codes(inst: Installation) -> list[str]:
    return [f.code for f in requirements.analyze(inst)]


# --- Stardew Valley: SMAPI Dependencies and ContentPackFor ---------------
def test_stardew_missing_smapi_dependency_is_detected(tmp_path):
    mod = build.smapi_mod(tmp_path / "mod.zip", unique_id="Example.Mod",
                          deps=[{"UniqueID": "Some.Library", "MinimumVersion": "2.0.0",
                                 "IsRequired": True}])
    assert "dependency.missing" in codes(installation("stardewvalley", mod))


def test_control_stardew_satisfied_dependency_is_silent(tmp_path):
    mod = build.smapi_mod(tmp_path / "mod.zip", unique_id="Example.Mod",
                          deps=[{"UniqueID": "Some.Library", "MinimumVersion": "2.0.0",
                                 "IsRequired": True}])
    library = build.smapi_mod(tmp_path / "lib.zip", unique_id="Some.Library")
    # The library ships version 1.2.0 in the fixture, so raise the bar it meets.
    inst = installation("stardewvalley", mod, library)
    assert "dependency.missing" not in codes(inst)


def test_stardew_content_pack_for_becomes_a_real_dependency_edge(tmp_path):
    """ContentPackFor is a dependency even though it is not in Dependencies."""
    pack = build.content_patcher_pack(tmp_path / "pack.zip", unique_id="Example.Pack")
    found = codes(installation("stardewvalley", pack))
    assert "dependency.missing" in found

    inst = installation("stardewvalley", pack)
    missing = [f for f in requirements.analyze(inst) if f.code == "dependency.missing"]
    assert any("Pathoschild.ContentPatcher" in str(f.targets) for f in missing)


def test_stardew_optional_dependency_absent_is_not_an_error(tmp_path):
    mod = build.smapi_mod(tmp_path / "mod.zip", unique_id="Example.Mod",
                          deps=[{"UniqueID": "Nice.ToHave", "IsRequired": False}])
    assert codes(installation("stardewvalley", mod)) == []


# --- Project Zomboid: mod.info require= ----------------------------------
def test_projectzomboid_missing_required_mod_is_detected(tmp_path):
    mod = build.pz_mod(tmp_path / "a.zip", mod_id="Alpha", require=["Beta"])
    assert "dependency.missing" in codes(installation("projectzomboid", mod))


def test_control_projectzomboid_satisfied_requirement_is_silent(tmp_path):
    alpha = build.pz_mod(tmp_path / "a.zip", mod_id="Alpha", require=["Beta"])
    beta = build.pz_mod(tmp_path / "b.zip", mod_id="Beta")
    assert "dependency.missing" not in codes(installation("projectzomboid", alpha, beta))


def test_projectzomboid_duplicate_mod_id_is_detected(tmp_path):
    a = build.pz_mod(tmp_path / "a.zip", mod_id="Same")
    b = build.pz_mod(tmp_path / "b.zip", mod_id="Same")
    assert "dependency.duplicate_mod_id" in codes(installation("projectzomboid", a, b))


# --- RimWorld: modDependencies and the per-version variant ---------------
def test_rimworld_missing_dependency_is_detected(tmp_path):
    mod = build.rimworld_mod(tmp_path / "a.zip", package_id="example.mod")
    # The fixture declares brrainz.harmony as a dependency.
    assert "dependency.missing" in codes(installation("rimworld", mod))


def test_control_rimworld_satisfied_dependency_is_silent(tmp_path):
    mod = build.rimworld_mod(tmp_path / "a.zip", package_id="example.mod")
    harmony = build.rimworld_mod(tmp_path / "h.zip", package_id="brrainz.harmony")
    inst = installation("rimworld", mod, harmony)
    # The harmony fixture itself declares a dependency on brrainz.harmony, which
    # it provides, so resolution must see it as satisfied.
    assert "dependency.missing" not in codes(inst)


def test_rimworld_duplicate_package_id_is_detected(tmp_path):
    a = build.rimworld_mod(tmp_path / "a.zip", package_id="same.id")
    b = build.rimworld_mod(tmp_path / "b.zip", package_id="same.id")
    assert "dependency.duplicate_mod_id" in codes(installation("rimworld", a, b))


# --- the honesty rule ----------------------------------------------------
@pytest.mark.parametrize("game", ["stardewvalley", "projectzomboid", "rimworld"])
def test_incomplete_view_downgrades_missing_to_unresolved(game):
    """The same discipline must hold for every game, not just Minecraft."""
    inst = Installation(game=game, files_known_complete=False)
    inst.artifacts.append(InstalledArtifact(
        name="a.zip", mod_id="a", version="1.0",
        declared_dependencies=[{"id": "absent", "versions": "*", "required": True}]))
    assert codes(inst) == ["dependency.presence_unresolved"]

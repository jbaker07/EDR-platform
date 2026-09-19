"""Impact: what an install or update would change."""
from __future__ import annotations

import json

from fixtures import build
from modcheck.analyze import impact as impact_mod
from modcheck.analyze.config import Installation, InstalledArtifact
from modcheck.report import impact_of


def config(game_version="26.3", java="25", artifacts=()) -> Installation:
    inst = Installation(game="minecraft", loader="fabric", game_version=game_version,
                        loader_versions={"fabric": "0.19.5", "java": java},
                        files_known_complete=True)
    inst.artifacts = list(artifacts)
    return inst


def mod(name, mod_id, version, deps=None, sha=None):
    return InstalledArtifact(name=name, mod_id=mod_id, version=version, sha256=sha,
                             declared_dependencies=deps or [])


NEEDS_263 = [{"id": "minecraft", "versions": "~26.3", "required": True}]


def test_game_update_that_breaks_a_mod_is_reported_as_introduced():
    before = config("26.3", artifacts=[mod("a.jar", "a", "1.0", NEEDS_263)])
    after = config("1.21.4", artifacts=[mod("a.jar", "a", "1.0", NEEDS_263)])
    result = impact_of(before, after)
    codes = [f.code for f in result.introduced]
    assert "dependency.version_mismatch" in codes
    assert result.resolved == []


def test_updating_the_game_back_resolves_the_finding():
    before = config("1.21.4", artifacts=[mod("a.jar", "a", "1.0", NEEDS_263)])
    after = config("26.3", artifacts=[mod("a.jar", "a", "1.0", NEEDS_263)])
    result = impact_of(before, after)
    assert [f.code for f in result.resolved] == ["dependency.version_mismatch"]
    assert result.introduced == []


def test_a_neutral_change_says_so_without_claiming_nothing_changed():
    before = config(artifacts=[mod("a.jar", "a", "1.0")])
    after = config(artifacts=[mod("a.jar", "a", "1.0"), mod("b.jar", "b", "1.0")])
    result = impact_of(before, after)
    assert result.is_neutral
    text = impact_mod.render(result)
    assert "introduces no new findings and resolves none" in text
    assert "not the same as changing nothing" in text


def test_artifact_changes_are_reported_by_hash_when_available():
    before = config(artifacts=[mod("a.jar", "a", "1.0", sha="a" * 64)])
    after = config(artifacts=[mod("a.jar", "a", "2.0", sha="b" * 64)])
    result = impact_of(before, after)
    assert result.artifacts_changed[0]["from_sha256"] == "a" * 64
    assert result.artifacts_changed[0]["to_sha256"] == "b" * 64


def test_version_only_comparison_is_flagged_as_not_verified():
    """Matching version strings do not establish matching artifacts."""
    before = config(artifacts=[mod("a.jar", "a", "1.0")])
    after = config(artifacts=[mod("a.jar", "a", "2.0")])
    result = impact_of(before, after)
    assert "declared change, not a verified one" in result.artifacts_changed[0]["note"]
    assert any("do not establish matching artifacts" in c for c in result.caveats)


def test_removing_a_mod_resolves_but_does_not_claim_a_fix():
    before = config(artifacts=[mod("a.jar", "a", "1.0",
                                   [{"id": "gone", "versions": "*", "required": True}])])
    after = config(artifacts=[])
    result = impact_of(before, after)
    assert [f.code for f in result.resolved] == ["dependency.missing"]
    assert any("does not mean the underlying problem was fixed" in c
               for c in result.caveats)


def test_previewing_an_install_inspects_the_real_artifact(tmp_path):
    jar = build.fabric_jar(tmp_path / "new.jar", mod_id="newmod",
                           depends={"minecraft": "~26.3"})
    before = config(artifacts=[])
    after = impact_mod.with_artifact(before, jar)
    assert after.artifacts[-1].mod_id == "newmod"
    assert after.artifacts[-1].sha256
    # the original configuration must not be mutated
    assert before.artifacts == []
    result = impact_of(before, after)
    assert result.artifacts_added == ["newmod"]


def test_impact_serialises(tmp_path):
    before = config(artifacts=[mod("a.jar", "a", "1.0", NEEDS_263)])
    after = config("1.21.4", artifacts=[mod("a.jar", "a", "1.0", NEEDS_263)])
    data = json.loads(json.dumps(impact_of(before, after).as_dict(), default=str))
    assert data["findings"]["introduced"]
    assert data["caveats"]

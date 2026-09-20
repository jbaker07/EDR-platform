"""Applicability is an eligibility check, not a ranking bonus.

The defect this fixes: `guide` added 1.0 to a recipe's score when its loader
matched, and did nothing when it did not. Asked for Forge, it returned Fabric
recipes at the top of the "applicable" list. A recipe for another loader is not
a weaker answer -- it is the wrong answer, and ranking it lower still puts it
first when nothing else matches.

Unknown applicability gets the same treatment. A recipe whose version or loader
we could not check is not applicable; it is unchecked, and the two must not
render the same.
"""
from __future__ import annotations

from modcheck.creator.guide import guide
from modcheck.store import Store

QUERY = "add a mixin to extend behaviour"


def _guide(**kwargs):
    return guide(Store(), QUERY, "minecraft", **kwargs)


def test_a_wrong_loader_recipe_is_never_applicable():
    result = _guide(loader="forge", game_version="26.3")
    assert result.applicable == []
    assert result.wrong_loader, "the Fabric recipes must still be shown, separately"
    assert all("fabric" in (m.record.get("applies_to") or {}).get("loader", "")
               for m in result.wrong_loader)


def test_the_right_loader_still_works():
    result = _guide(loader="fabric", game_version="26.3")
    assert result.applicable
    assert result.wrong_loader == []


def test_an_unchecked_loader_is_not_applicable():
    """No loader given: we cannot say these apply, so we do not say it."""
    result = _guide(game_version="26.3")
    assert result.applicable == []
    assert result.unknown_applicability
    assert any("Unknown is not applicable" in n for n in result.notes)


def test_an_unchecked_version_is_not_applicable():
    result = _guide(loader="fabric")
    assert result.applicable == []
    assert result.unknown_applicability
    assert any("no game version was given" in n for n in result.notes)


def test_a_wrong_version_is_separated_from_a_wrong_loader():
    """They are different problems and lead to different next steps."""
    result = _guide(loader="fabric", game_version="1.16.5")
    assert result.applicable == []
    assert result.wrong_version
    assert result.wrong_loader == []


def test_a_wrong_loader_outranks_a_wrong_version_in_the_explanation():
    """Reporting 'wrong version' for a Forge/Fabric mismatch would mislead."""
    result = _guide(loader="forge", game_version="1.16.5")
    assert result.wrong_loader
    assert result.wrong_version == [], (
        "a recipe for the wrong loader is not usefully described as a version problem")


def test_the_rendered_output_never_shows_an_ineligible_recipe_as_an_answer():
    from modcheck.creator.guide import render
    text = render(_guide(loader="forge", game_version="26.3"))
    assert "Nothing in the library applies here as asked." in text
    assert "written for a different loader" in text
    # And the ineligible ones must not appear under an applicable heading.
    head = text.split("-- matched, but written for a different loader --")[0]
    assert "fabric_extend_behaviour_with_mixin" not in head


def test_json_output_keeps_the_buckets_distinct():
    result = _guide(loader="forge", game_version="26.3")
    payload = result.as_dict()
    assert payload["applicable"] == []
    assert payload["not_applicable_to_this_loader"]
    assert "applicability_unknown" in payload


def test_relevance_is_still_required_before_anything_else():
    """A build-tested recipe must not surface for an unrelated question."""
    result = guide(Store(), "how do I bake a cake", "minecraft",
                   loader="fabric", game_version="26.3")
    assert result.applicable == []
    assert result.wrong_loader == []
    assert result.unknown_applicability == []


# -- the ecosystem decides what "unspecified" means -------------------------

def test_a_single_loader_ecosystem_does_not_demand_the_loader_be_named():
    """Project Zomboid has one loader; naming it would add nothing.

    The recipe still lands in `unknown_applicability`, but for the OTHER
    reason -- its game_versions is prose. That distinction is the point of the
    test: loader ambiguity was not what stopped it.
    """
    result = guide(Store(), "add a sandbox option players can configure",
                   "projectzomboid", game_version="42.0")
    assert result.wrong_loader == []
    matches = result.applicable + result.unknown_applicability
    assert matches, "the recipe must still be found"
    assert not any("no loader was given" in n for n in result.notes)


def test_a_prose_version_constraint_is_reported_as_the_records_problem():
    """13 of 30 recipes declare game_versions as a sentence, not a range."""
    from modcheck.creator.guide import VERSION_UNDECIDED_PROSE
    result = guide(Store(), "add a sandbox option players can configure",
                   "projectzomboid", game_version="42.0")
    assert result.applicable == []
    assert any(m.undecided_because == VERSION_UNDECIDED_PROSE
               for m in result.unknown_applicability)
    assert any("cannot be fixed by supplying a version" in n for n in result.notes)


def test_the_three_undecidable_reasons_stay_distinct():
    """Each points at different work, so collapsing them would mislead."""
    from modcheck.creator.guide import (VERSION_UNDECIDED_NO_CONSTRAINT,
                                        VERSION_UNDECIDED_NO_INPUT,
                                        VERSION_UNDECIDED_PROSE, _version_applies)
    from modcheck.store import Record

    def rec(constraint):
        return Record(kind="recipe", game="minecraft", id="x",
                      data={"applies_to": {"game_versions": constraint} if constraint
                            else {}}, path=None)

    assert _version_applies(rec(None), "26.3")[1] == VERSION_UNDECIDED_NO_CONSTRAINT
    assert _version_applies(rec(">=26.1"), None)[1] == VERSION_UNDECIDED_NO_INPUT
    assert _version_applies(rec("Build 42 (per the wiki)"), "42.0")[1] == VERSION_UNDECIDED_PROSE
    assert _version_applies(rec(">=26.1"), "26.3") == (True, "")
    assert _version_applies(rec(">=26.1"), "1.16.5") == (False, "")


def test_a_multi_loader_ecosystem_does_demand_it():
    """Minecraft declares fabric, quilt, forge and neoforge."""
    result = guide(Store(), "start a new mod project", "minecraft",
                   game_version="26.3")
    assert result.applicable == []
    assert result.unknown_applicability
    assert any("has 4 of them" in n for n in result.notes)


def test_the_rule_comes_from_the_pack_not_from_our_recipe_inventory():
    """We only ship Fabric recipes; that must not make Fabric the only answer."""
    from modcheck.store import Store as S
    pack = S().pack("minecraft")
    declared = [l["id"] for l in pack.manifest["ecosystem"]["loaders"]]
    assert len(declared) > 1 and "forge" in declared
    recipe_loaders = {(r.get("applies_to") or {}).get("loader")
                      for r in pack.records("recipe")}
    assert recipe_loaders == {"fabric"}, (
        "inventory is one loader; the ecosystem is four. The eligibility rule "
        "must follow the ecosystem, or a Forge creator silently gets Fabric.")

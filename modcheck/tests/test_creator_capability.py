"""The capability layer: shared labels, game-specific mechanisms, honest gaps.

Two rules this layer exists to hold:

* A shared capability id is navigation. The evidence is the mechanism
  underneath it, in the game's own terms, with exact targets preserved.
* A blank cell means "no record", never "unsupported" and never "probably
  fine". The coverage map's job is to make the gaps nameable.
"""
from __future__ import annotations

from modcheck.creator.capability import BY_ID, CREATOR_CAPABILITIES, IDS, suggest
from modcheck.creator.coverage import build_map
from modcheck.store import Store


def test_the_taxonomy_is_small_and_ids_are_unique():
    assert len(CREATOR_CAPABILITIES) == len(IDS) == 10
    for capability in CREATOR_CAPABILITIES:
        assert capability.phrases, f"{capability.id} has no retrieval phrases"


def test_creator_capabilities_are_not_the_product_capabilities():
    """Two lists share the word. Conflating them would overstate both."""
    from modcheck.paths import CAPABILITIES as PRODUCT_CAPABILITIES
    assert not (IDS & set(PRODUCT_CAPABILITIES)), (
        "a name appearing in both lists would be read as one thing and mean two")


def test_suggest_finds_inflected_words_not_just_exact_stems():
    request = ("A lantern charges when exposed to rain. Its stored charge can be "
               "spent to affect nearby hostile creatures. Display remaining charge "
               "and allow server configuration.")
    found = {cid for cid, _ in suggest(request)}
    assert {"persist_state", "display_information", "add_configuration"} <= found


def test_suggest_returns_the_words_that_caused_each_hit():
    """A suggestion a reader cannot check is not a lead, it is a guess."""
    for cid, words in suggest("store the charge and display it"):
        assert words, f"{cid} was suggested with no supporting word"


def test_a_suggestion_is_not_evidence():
    """Wording about a capability says nothing about whether a game offers it.

    Stated as a property rather than against a particular empty cell, so that
    filling a gap -- which this batch did for Minecraft sync_state -- does not
    make the test pass for the wrong reason.
    """
    hits = {cid for cid, _ in suggest("synchronise the stored value to every client")}
    assert "sync_state" in hits, "the wording must suggest the capability"
    coverage = build_map()
    gaps = [(g, c) for g, c in coverage.gaps() if c in hits]
    assert gaps, "some game must still lack a record for a suggested capability"
    for game, capability in gaps:
        assert not coverage.cell(game, capability).covered, (
            "a capability suggested by the request's wording must not become "
            "covered by having been suggested")


def test_every_capability_record_names_a_known_capability():
    store = Store()
    unknown = [(r.game, r.id, r.get("capability"))
               for r in store.all_records("capability")
               if r.get("capability") not in IDS]
    assert not unknown, f"records pointing at no capability in the taxonomy: {unknown}"


def test_every_capability_record_keeps_an_exact_mechanism():
    """The label is navigation; the mechanism is what an implementation uses."""
    store = Store()
    for record in store.all_records("capability"):
        mechanism = record.get("mechanism") or []
        assert mechanism, f"{record.game}/{record.id} has no mechanism"
        for item in mechanism:
            assert item.get("kind") and item.get("id"), (
                f"{record.game}/{record.id} has a mechanism entry without an exact "
                f"target: {item}")


def test_a_derived_record_never_claims_more_than_its_recipe():
    """A capability derived from a documented recipe stays documented."""
    store = Store()
    for record in store.all_records("capability"):
        for recipe_id in record.get("recipes") or []:
            recipe = store.pack(record.game).record("recipe", recipe_id)
            if recipe is None:
                continue
            if recipe.get("verification_state") == "documented":
                assert record.get("evidence_state") in ("documented", "source_confirmed",
                                                        "example_confirmed"), (
                    f"{record.id} claims {record.get('evidence_state')} while its "
                    f"recipe {recipe_id} is only documented")


def test_the_coverage_map_reports_gaps_not_assumptions():
    coverage = build_map()
    summary = coverage.summary()
    assert summary["cells"] == len(coverage.games) * len(coverage.capabilities)
    assert summary["covered"] + summary["gaps"] == summary["cells"]
    # Every gap is a real absence, checkable against the store.
    for game, capability in coverage.gaps():
        assert not coverage.cell(game, capability).records


def test_all_ten_games_appear_in_the_map():
    """Reference coverage is maintained for ten games, not quietly reduced."""
    coverage = build_map()
    assert len(coverage.games) == 10
    for game in ("minecraft", "sims4", "skyrimse", "fallout4", "cyberpunk2077",
                 "bg3", "falloutnv", "stardewvalley", "projectzomboid", "rimworld"):
        assert game in coverage.games


def test_a_cell_carries_the_strongest_evidence_but_never_invents_one():
    coverage = build_map()
    from modcheck.creator.coverage import EVIDENCE_ORDER
    for cell in coverage.cells.values():
        assert cell.evidence_state in EVIDENCE_ORDER
        assert cell.records


def test_game_tested_is_claimed_by_nothing():
    """No capability may read as game-tested without a real game run."""
    coverage = build_map()
    assert coverage.summary().get("game_tested", 0) == 0, (
        "a capability claims game_tested; that requires a recorded game run")

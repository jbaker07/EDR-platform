"""IDEA -> applicable approach retrieval."""
from __future__ import annotations

import pytest

from modcheck.creator.guide import Guidance, guide, render, tokenize
from modcheck.store import Store


@pytest.fixture(scope="module")
def store() -> Store:
    return Store()


def test_a_relevant_idea_finds_the_right_recipe(store):
    # Minecraft declares four loaders, so the loader is part of the question.
    result = guide(store, "extend an existing method without replacing it",
                   "minecraft", game_version="26.3", loader="fabric")
    ids = [m.record.id for m in result.applicable]
    assert "fabric_extend_behaviour_with_mixin" in ids
    top = result.applicable[0]
    assert top.reasons, "a match must say why it matched"


def test_an_unrelated_idea_says_there_is_no_recipe(store):
    """A build-tested recipe is not an answer to an unrelated question."""
    result = guide(store, "underwater basket weaving simulator", "minecraft")
    assert result.applicable == []
    assert result.wrong_version == []
    assert any("no recorded approach" in note for note in result.notes)


def test_verification_state_ranks_but_does_not_create_relevance(store):
    result = guide(store, "start a new mod project", "minecraft",
                   game_version="26.3", loader="fabric")
    assert result.applicable
    for match in result.applicable:
        # every match must have at least one relevance reason, not only a bonus
        assert any("mentions" in r or "category" in r for r in match.reasons)


def test_recipes_for_another_version_are_listed_separately_not_dropped(store):
    result = guide(store, "start a new mod project", "minecraft",
                   game_version="1.21.4", loader="fabric")
    assert result.applicable == []
    assert result.wrong_version, "must not silently drop version-inapplicable recipes"
    assert any("listed separately rather than dropped" in n for n in result.notes)


def test_missing_game_version_is_stated_not_assumed(store):
    """An unchecked version cannot produce an applicable recipe."""
    result = guide(store, "start a new mod project", "minecraft", loader="fabric")
    assert result.applicable == [], (
        "without a version we cannot say a recipe applies, so we must not say it")
    assert result.unknown_applicability, "and the matches must still be shown"
    assert all(m.applies_to_version is None for m in result.unknown_applicability)
    assert any("no game version was given" in n for n in result.notes)


def test_related_failures_are_surfaced(store):
    result = guide(store, "build my project with gradle", "minecraft",
                   game_version="26.3", loader="fabric")
    titles = " ".join(r.get("title", "") for r in result.related_failures)
    assert "Build fails" in titles or result.related_failures == []


def test_empty_query_is_rejected_clearly(store):
    result = guide(store, "the a of", "minecraft")
    assert result.applicable == []
    assert any("no searchable terms" in n for n in result.notes)


def test_tokenizer_drops_stopwords_and_short_words():
    assert tokenize("I want to add a new item") == {"item"}


def test_render_is_readable(store):
    result = guide(store, "start a new mod project", "minecraft",
                   game_version="26.3", loader="fabric")
    text = render(result)
    assert "why:" in text and "applies to:" in text and "modcheck show" in text


def test_guidance_serialises(store):
    result = guide(store, "start a new mod project", "minecraft",
                   game_version="26.3", loader="fabric")
    data = result.as_dict()
    assert data["applicable"][0]["reasons"]
    assert "not_applicable_to_this_version" in data

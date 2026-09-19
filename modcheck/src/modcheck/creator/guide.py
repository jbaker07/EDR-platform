"""IDEA -> applicable approach.

The first step of the creator workflow is turning a description of an idea into
the recipes that actually apply, for the version the creator is on. This is
deliberately plain retrieval: token overlap, scored, with the reason for each
match shown, and version applicability checked against the recipe's own
`applies_to` range.

It is honest about the two ways it can fail. If nothing matches it says so
rather than returning the least-bad recipe, and if a recipe matches but does
not apply to the stated version it is listed separately instead of silently
dropped or silently included.
"""
from __future__ import annotations

import dataclasses
import re
from typing import Any

from ..analyze.versions import fabric_satisfies
from ..store import Record, Store

STOPWORDS = {
    "a", "an", "and", "the", "to", "for", "of", "in", "on", "with", "my", "i",
    "want", "how", "do", "make", "add", "new", "that", "this", "it", "is", "be",
    "can", "should", "would", "like", "mod", "game",
}

# Words that signal a recipe category, so an idea phrased in ordinary terms
# reaches the right kind of recipe.
CATEGORY_HINTS = {
    "add_content": {"item", "block", "creature", "npc", "weapon", "armour", "armor",
                    "food", "plant", "recipe", "craft", "content"},
    "modify_data_record": {"change", "edit", "tweak", "balance", "value", "stat",
                           "price", "damage", "record", "data"},
    "event_behavior": {"event", "trigger", "when", "hook", "listen", "react",
                       "behaviour", "behavior"},
    "extend_system": {"extend", "override", "patch", "inject", "wrap", "mixin",
                      "harmony", "hook", "system"},
    "content_pack": {"texture", "sprite", "portrait", "asset", "retexture", "pack",
                     "image", "sound", "music"},
    "optional_integration": {"integration", "optional", "compat", "support", "api"},
    "compatibility_patch": {"compatibility", "patch", "conflict", "incompatible", "fix"},
    "asset_workflow": {"model", "mesh", "animation", "asset", "import", "export"},
    "tooling": {"project", "setup", "start", "scaffold", "build", "template", "begin"},
}


def tokenize(text: str) -> set[str]:
    words = re.findall(r"[a-z0-9]+", (text or "").lower())
    return {w for w in words if w not in STOPWORDS and len(w) > 2}


@dataclasses.dataclass
class Match:
    record: Record
    score: float
    reasons: list[str]
    applies_to_version: bool | None  # None = undecidable

    def as_dict(self) -> dict[str, Any]:
        return {
            "id": self.record.id,
            "game": self.record.game,
            "title": self.record.get("title"),
            "category": self.record.get("category"),
            "verification_state": self.record.get("verification_state"),
            "applies_to": self.record.get("applies_to"),
            "applies_to_version": self.applies_to_version,
            "score": round(self.score, 2),
            "reasons": self.reasons,
        }


def _version_applies(record: Record, game_version: str | None) -> bool | None:
    if game_version is None:
        return None
    constraint = (record.get("applies_to") or {}).get("game_versions")
    if not constraint:
        return None
    return fabric_satisfies(game_version, constraint)


def _score(record: Record, query_tokens: set[str], loader: str | None) -> tuple[float, list[str]]:
    reasons: list[str] = []
    score = 0.0

    title_tokens = tokenize(str(record.get("title") or ""))
    intent_tokens = tokenize(str(record.get("intent") or ""))
    overlap_title = query_tokens & title_tokens
    overlap_intent = query_tokens & intent_tokens
    if overlap_title:
        score += 3.0 * len(overlap_title)
        reasons.append(f"title mentions {', '.join(sorted(overlap_title))}")
    if overlap_intent:
        score += 1.5 * len(overlap_intent)
        reasons.append(f"intent mentions {', '.join(sorted(overlap_intent))}")

    category = record.get("category")
    if category and query_tokens & CATEGORY_HINTS.get(category, set()):
        hit = query_tokens & CATEGORY_HINTS[category]
        score += 2.5
        reasons.append(f"category {category} matches {', '.join(sorted(hit))}")

    target_tokens: set[str] = set()
    for target in record.get("extension_points", []) or []:
        target_tokens |= tokenize(str(target.get("id", ""))) | tokenize(str(target.get("label", "")))
    overlap_targets = query_tokens & target_tokens
    if overlap_targets:
        score += 1.0 * len(overlap_targets)
        reasons.append(f"extension point mentions {', '.join(sorted(overlap_targets))}")

    # Everything above is relevance. Nothing below may make an irrelevant recipe
    # match: a recipe that is merely build-tested is not an answer to an
    # unrelated question.
    if score <= 0:
        return 0.0, []

    if loader and str((record.get("applies_to") or {}).get("loader") or "").lower() == loader.lower():
        score += 1.0
        reasons.append(f"targets the {loader} loader")

    # Among relevant recipes, one whose build actually ran here ranks higher.
    state = record.get("verification_state")
    if state == "game_tested":
        score += 1.5
        reasons.append("game-tested")
    elif state == "build_tested":
        score += 1.0
        reasons.append("build-tested here")

    return score, reasons


@dataclasses.dataclass
class Guidance:
    query: str
    game: str
    applicable: list[Match]
    wrong_version: list[Match]
    related_failures: list[Record]
    notes: list[str] = dataclasses.field(default_factory=list)

    def as_dict(self) -> dict[str, Any]:
        return {
            "query": self.query,
            "game": self.game,
            "applicable": [m.as_dict() for m in self.applicable],
            "not_applicable_to_this_version": [m.as_dict() for m in self.wrong_version],
            "related_failures": [{"id": r.id, "title": r.get("title"),
                                  "symptom": r.get("symptom")}
                                 for r in self.related_failures],
            "notes": self.notes,
        }


def guide(store: Store, query: str, game: str, *, game_version: str | None = None,
          loader: str | None = None, limit: int = 5) -> Guidance:
    pack = store.pack(game)
    query_tokens = tokenize(query)
    if not query_tokens:
        return Guidance(query=query, game=game, applicable=[], wrong_version=[],
                        related_failures=[],
                        notes=["the description contained no searchable terms"])

    applicable: list[Match] = []
    wrong_version: list[Match] = []
    for record in pack.records("recipe"):
        score, reasons = _score(record, query_tokens, loader)
        if score <= 0:
            continue
        version_ok = _version_applies(record, game_version)
        match = Match(record=record, score=score, reasons=reasons,
                      applies_to_version=version_ok)
        (wrong_version if version_ok is False else applicable).append(match)

    applicable.sort(key=lambda m: -m.score)
    wrong_version.sort(key=lambda m: -m.score)

    failures = []
    for record in pack.records("failure"):
        text = tokenize(f"{record.get('title')} {record.get('symptom')} "
                        f"{record.get('mechanism')}")
        if query_tokens & text:
            failures.append(record)

    notes: list[str] = []
    if not applicable and not wrong_version:
        notes.append(
            f"No recipe in the {game} pack matches this description. That means we "
            "have no recorded approach for it, not that the task is impossible.")
    if wrong_version and game_version:
        notes.append(
            f"{len(wrong_version)} recipe(s) matched but do not apply to {game_version}; "
            "they are listed separately rather than dropped.")
    if game_version is None and (applicable or wrong_version):
        notes.append(
            "No game version was given, so version applicability was not checked.")

    return Guidance(query=query, game=game, applicable=applicable[:limit],
                    wrong_version=wrong_version[:limit], related_failures=failures[:limit],
                    notes=notes)


def render(guidance: Guidance) -> str:
    lines = [f'Approaches for: "{guidance.query}"  [{guidance.game}]', ""]
    if guidance.applicable:
        for match in guidance.applicable:
            record = match.record
            state = record.get("verification_state")
            lines.append(f"  {record.id}   ({state})")
            lines.append(f"    {record.get('title')}")
            lines.append(f"    why: {'; '.join(match.reasons)}")
            applies = record.get("applies_to") or {}
            lines.append(f"    applies to: {applies.get('game_versions', '?')}"
                         + (f", loader {applies['loader']}" if applies.get("loader") else ""))
            lines.append(f"    show it with: modcheck show {record.id} --game {guidance.game}")
            lines.append("")
    if guidance.wrong_version:
        lines.append("  -- matched, but not for this game version --")
        for match in guidance.wrong_version:
            applies = (match.record.get("applies_to") or {}).get("game_versions", "?")
            lines.append(f"    {match.record.id}  (applies to {applies})")
        lines.append("")
    if guidance.related_failures:
        lines.append("  -- documented failures worth knowing about first --")
        for record in guidance.related_failures:
            lines.append(f"    {record.id}: {record.get('title')}")
        lines.append("")
    for note in guidance.notes:
        lines.append(f"  note: {note}")
    return "\n".join(lines)

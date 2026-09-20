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
    # Three-valued, both of them. None means undecidable, and an undecidable
    # applicability is never rendered as applicable.
    applies_to_version: bool | None = None
    applies_to_loader: bool | None = None
    # Why applicability could not be decided, when it could not.
    undecided_because: str = ""

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


# Why a version check could not be decided. Kept apart because they are
# different pieces of work: the caller can supply a version, but a constraint
# written as prose can only be fixed in the record.
VERSION_UNDECIDED_NO_INPUT = "no game version was given"
VERSION_UNDECIDED_NO_CONSTRAINT = "the recipe declares no game version constraint"
VERSION_UNDECIDED_PROSE = (
    "the recipe's game_versions is prose, not a machine-checkable range, so "
    "applicability cannot be decided from it")


def _version_applies(record: Record, game_version: str | None) -> tuple[bool | None, str]:
    """Whether a record applies to a game version, and where the answer came from.

    Returning the reason matters: "you did not tell us the version" and "this
    record's constraint is a sentence rather than a range" both come out as
    undecided, and only one of them is the caller's to fix.
    """
    constraint = (record.get("applies_to") or {}).get("game_versions")
    if not constraint:
        return None, VERSION_UNDECIDED_NO_CONSTRAINT
    if game_version is None:
        return None, VERSION_UNDECIDED_NO_INPUT
    result = fabric_satisfies(game_version, constraint)
    if result is None:
        return None, VERSION_UNDECIDED_PROSE
    return result, ""


def _loader_applies(record: Record, loader: str | None,
                    ecosystem_loaders: int = 0) -> bool | None:
    """Whether a record applies to the requested loader.

    Three-valued on purpose, and the third value follows the ecosystem's own
    rules rather than a single policy.

    * Caller names a loader that differs from the record's -> False. A Fabric
      recipe answering a Forge question is not a weaker answer; it is the wrong
      answer, and merely ranking it lower still puts it first when nothing else
      matches.
    * Caller names no loader, and the game HAS competing loaders -> None. For
      Minecraft, whose pack declares fabric, quilt, forge and neoforge, not
      naming one leaves the question genuinely open.
    * Caller names no loader, and the game has ONE -> True. Project Zomboid has
      a single loader; demanding it be named would manufacture an ambiguity
      that does not exist in that ecosystem.

    `ecosystem_loaders` is the count the pack itself declares under
    `ecosystem.loaders`, so this rule comes from the game rather than from what
    our recipe inventory happens to contain.
    """
    declared = (record.get("applies_to") or {}).get("loader")
    if loader is None:
        if ecosystem_loaders <= 1:
            return True
        return None
    if not declared:
        return None
    return str(declared).lower() == loader.lower()


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
    wrong_loader: list[Match] = dataclasses.field(default_factory=list)
    unknown_applicability: list[Match] = dataclasses.field(default_factory=list)
    notes: list[str] = dataclasses.field(default_factory=list)

    def as_dict(self) -> dict[str, Any]:
        return {
            "query": self.query,
            "game": self.game,
            "applicable": [m.as_dict() for m in self.applicable],
            "not_applicable_to_this_version": [m.as_dict() for m in self.wrong_version],
            "not_applicable_to_this_loader": [m.as_dict() for m in self.wrong_loader],
            "applicability_unknown": [m.as_dict() for m in self.unknown_applicability],
            "related_failures": [{"id": r.id, "title": r.get("title"),
                                  "symptom": r.get("symptom")}
                                 for r in self.related_failures],
            "notes": self.notes,
        }


def guide(store: Store, query: str, game: str, *, game_version: str | None = None,
          loader: str | None = None, limit: int = 5) -> Guidance:
    pack = store.pack(game)
    # How many loaders this ecosystem actually has. One means an unnamed loader
    # is not ambiguous; several means it is.
    ecosystem_loaders = len(((pack.manifest or {}).get("ecosystem") or {}).get("loaders") or [])
    query_tokens = tokenize(query)
    if not query_tokens:
        return Guidance(query=query, game=game, applicable=[], wrong_version=[],
                        related_failures=[],
                        notes=["the description contained no searchable terms"])

    applicable: list[Match] = []
    wrong_version: list[Match] = []
    wrong_loader: list[Match] = []
    unknown_applicability: list[Match] = []
    for record in pack.records("recipe"):
        score, reasons = _score(record, query_tokens, loader)
        if score <= 0:
            continue
        version_ok, version_why = _version_applies(record, game_version)
        loader_ok = _loader_applies(record, loader, ecosystem_loaders)
        match = Match(record=record, score=score, reasons=reasons,
                      applies_to_version=version_ok, applies_to_loader=loader_ok,
                      undecided_because=version_why)
        # Eligibility first, ranking second. A recipe that does not apply is not
        # a lower-ranked answer, and unknown applicability is not applicable:
        # both get their own list so nothing silently reads as "use this".
        if loader_ok is False:
            wrong_loader.append(match)
        elif version_ok is False:
            wrong_version.append(match)
        elif version_ok is None or loader_ok is None:
            unknown_applicability.append(match)
        else:
            applicable.append(match)

    for bucket in (applicable, wrong_version, wrong_loader, unknown_applicability):
        bucket.sort(key=lambda m: -m.score)

    failures = []
    for record in pack.records("failure"):
        text = tokenize(f"{record.get('title')} {record.get('symptom')} "
                        f"{record.get('mechanism')}")
        if query_tokens & text:
            failures.append(record)

    notes: list[str] = []
    if not applicable and not wrong_version and not wrong_loader and not unknown_applicability:
        notes.append(
            f"No recipe in the {game} pack matches this description. That means we "
            "have no recorded approach for it, not that the task is impossible.")
    if wrong_version and game_version:
        notes.append(
            f"{len(wrong_version)} recipe(s) matched but do not apply to {game_version}; "
            "they are listed separately rather than dropped.")
    if wrong_loader:
        notes.append(
            f"{len(wrong_loader)} recipe(s) matched the description but target a "
            f"different loader than {loader!r}. They are listed separately: a recipe "
            "for another loader is the wrong answer, not a weaker one.")
    if unknown_applicability:
        reasons_seen: list[str] = []
        for match in unknown_applicability:
            if match.undecided_because and match.undecided_because not in reasons_seen:
                reasons_seen.append(match.undecided_because)
        if loader is None and ecosystem_loaders > 1:
            reasons_seen.append(
                f"no loader was given and {game} has {ecosystem_loaders} of them")
        detail = "; ".join(reasons_seen) or "applicability is not declared"
        notes.append(
            f"{len(unknown_applicability)} recipe(s) matched but applicability could "
            f"not be checked ({detail}). Unknown is not applicable -- supply the "
            "missing detail, or check the recipe's own applies_to before using it.")
        if any(m.undecided_because == VERSION_UNDECIDED_PROSE
               for m in unknown_applicability):
            notes.append(
                "A prose game_versions cannot be fixed by supplying a version: the "
                "record has to state a checkable range before this check can run.")

    return Guidance(query=query, game=game, applicable=applicable[:limit],
                    wrong_version=wrong_version[:limit],
                    wrong_loader=wrong_loader[:limit],
                    unknown_applicability=unknown_applicability[:limit],
                    related_failures=failures[:limit], notes=notes)


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
    if not guidance.applicable:
        lines.append("  Nothing in the library applies here as asked.")
        lines.append("")
    if guidance.wrong_version:
        lines.append("  -- matched, but not for this game version --")
        for match in guidance.wrong_version:
            applies = (match.record.get("applies_to") or {}).get("game_versions", "?")
            lines.append(f"    {match.record.id}  (applies to {applies})")
        lines.append("")
    if guidance.wrong_loader:
        lines.append("  -- matched, but written for a different loader --")
        for match in guidance.wrong_loader:
            declared = (match.record.get("applies_to") or {}).get("loader", "?")
            lines.append(f"    {match.record.id}  (targets {declared})")
        lines.append("")
    if guidance.unknown_applicability:
        lines.append("  -- matched, applicability NOT established --")
        for match in guidance.unknown_applicability:
            applies = match.record.get("applies_to") or {}
            lines.append(f"    {match.record.id}  (declares versions "
                         f"{applies.get('game_versions', '?')}, loader "
                         f"{applies.get('loader', 'none')})")
        lines.append("")
    if guidance.related_failures:
        lines.append("  -- documented failures worth knowing about first --")
        for record in guidance.related_failures:
            lines.append(f"    {record.id}: {record.get('title')}")
        lines.append("")
    for note in guidance.notes:
        lines.append(f"  note: {note}")
    return "\n".join(lines)

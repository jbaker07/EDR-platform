"""Content Patcher: which patch wins, and why.

Two content packs touching one asset is not a verdict by itself. Content
Patcher has an exact documented rule, and applying it gives one of a small
number of outcomes -- one of which is "this cannot be decided from what we
were given", which must be said rather than guessed.

The governing rule, from Content Patcher's own author guide for the ``Load``
action (recorded as source ``content_patcher_load``):

    When multiple patches or mods load the same asset, the priority which
    decides which one is applied. Default `Exclusive`.

    * `Low`, `Medium`, or `High`: the highest-priority patch is applied. If
      multiple patches have the same priority, the first one in the list (by
      load order + patch order) is applied.
    * `Exclusive`: all or nothing. If one patch uses it, it's applied and all
      other load patches are ignored. If multiple patches use it, then _no
      patches_ are applied and an error message is shown.

The default levels are -1000 (low), 0 (medium), 1000 (high), and an offset may
be written as ``"High + 2"`` or ``"Medium - 10"``. The field does not support
tokens and capitalisation does not matter.

Two consequences shape everything below:

* A single ``Exclusive`` patch competing with others is a *selection*, not a
  failure: it applies and the others are silently ignored. A player whose other
  pack does nothing deserves to be told that.
* Two ``Exclusive`` patches are a real failure: **neither** applies.

Edit patches are ordered, not exclusive, so they compose; they are handled
separately and only reported where they touch the same entry.
"""
from __future__ import annotations

import dataclasses
import re
from typing import Any

LOAD_LEVELS = {"low": -1000, "medium": 0, "high": 1000}
EDIT_LEVELS = {"early": -1000, "default": 0, "late": 1000}

# Grammar taken from the published JSON schema's LoadPriority/EditPriority:
#   ^\s*(Low|Medium|High)\s*(?:[\+\-]\s*\d+)\s*$
_OFFSET = re.compile(r"^\s*(?P<level>[A-Za-z]+)\s*(?:(?P<sign>[+-])\s*(?P<offset>\d+))?\s*$")

DEFAULT_LOAD_PRIORITY = "Exclusive"
DEFAULT_EDIT_PRIORITY = "Default"

GOVERNING_RULE_LOAD = (
    "Content Patcher: when multiple patches load the same asset, Priority decides "
    "which applies (default Exclusive). Low/Medium/High: the highest priority wins, "
    "ties broken by load order then patch order. Exclusive: if one patch uses it, it "
    "applies and all other load patches are ignored; if several use it, no patches "
    "are applied and an error is shown."
)
RULE_SOURCE = "content_patcher_load"


# Fields Content Patcher rejects a pack for using below a given Format version.
# These are not "ignored if too old": Migration_2_0.TryMigrate returns an error
# and the ENTIRE pack fails to load. That matters because the visible symptom --
# the patch does not apply -- looks identical to losing a priority contest, so a
# pack rejected this way can be mistaken for a pack that resolved unfavourably.
#
# Read from ContentPatcher/Framework/Migrations/Migration_2_0.cs at
# Pathoschild/StardewMods 76565e83ede4bc8b3c293f1c659032ba9c39c213.
FIELD_MINIMUM_FORMAT: dict[str, tuple[int, int]] = {
    "priority": (2, 0),
}


def parse_format(raw: Any) -> tuple[int, int] | None:
    """Parse a content pack's `Format` as (major, minor); None if unparsable."""
    if raw is None:
        return None
    match = re.match(r"^\s*(\d+)\.(\d+)", str(raw))
    if not match:
        return None
    return int(match.group(1)), int(match.group(2))


def fields_below_minimum_format(change: dict, declared_format: Any) -> list[tuple[str, str]]:
    """Fields this patch uses that its declared Format is too old for.

    Returns (field, required_format) pairs. An unparsable or absent Format
    yields nothing: we do not know what was declared, and guessing would invent
    a failure.
    """
    version = parse_format(declared_format)
    if version is None:
        return []
    problems = []
    for field, minimum in FIELD_MINIMUM_FORMAT.items():
        if change.get(field) in (None, ""):
            continue
        if version < minimum:
            problems.append((field, f"{minimum[0]}.{minimum[1]}"))
    return problems


@dataclasses.dataclass(frozen=True)
class Priority:
    kind: str           # "exclusive" | "numeric" | "unparsed"
    value: int | None
    raw: str

    @property
    def is_exclusive(self) -> bool:
        return self.kind == "exclusive"


def parse_load_priority(raw: Any) -> Priority:
    """Parse a Load patch's Priority. Absent means Exclusive, per the docs."""
    if raw is None:
        return Priority("exclusive", None, DEFAULT_LOAD_PRIORITY)
    return _parse(str(raw), LOAD_LEVELS, allow_exclusive=True)


def parse_edit_priority(raw: Any) -> Priority:
    if raw is None:
        return Priority("numeric", EDIT_LEVELS["default"], DEFAULT_EDIT_PRIORITY)
    return _parse(str(raw), EDIT_LEVELS, allow_exclusive=False)


def _parse(raw: str, levels: dict[str, int], allow_exclusive: bool) -> Priority:
    text = raw.strip()
    if allow_exclusive and text.lower() == "exclusive":
        return Priority("exclusive", None, raw)
    match = _OFFSET.match(text)
    if not match:
        return Priority("unparsed", None, raw)
    level = match.group("level").lower()
    if level not in levels:
        return Priority("unparsed", None, raw)
    value = levels[level]
    if match.group("offset"):
        delta = int(match.group("offset"))
        value += delta if match.group("sign") == "+" else -delta
    return Priority("numeric", value, raw)


@dataclasses.dataclass
class CompetingPatch:
    """One patch competing for an asset, and where to find it again."""

    artifact: str
    mod_id: str | None
    target: str
    priority: Priority
    index: int
    source_file: str | None = None
    log_name: str | None = None
    from_file: str | None = None
    when: dict | None = None
    load_index: int | None = None

    @property
    def location(self) -> str:
        """Where a creator opens this patch."""
        name = self.log_name or self.from_file or f"patch #{self.index}"
        return f"{self.artifact}:{self.source_file or 'content.json'} #{self.index} ({name})"

    def as_dict(self) -> dict[str, Any]:
        return {
            "artifact": self.artifact, "mod_id": self.mod_id, "target": self.target,
            "priority": self.priority.raw, "priority_value": self.priority.value,
            "index": self.index, "source_file": self.source_file,
            "log_name": self.log_name, "from_file": self.from_file,
            "when": self.when, "location": self.location,
        }


@dataclasses.dataclass
class Resolution:
    target: str
    outcome: str
    patches: list[CompetingPatch]
    winner: CompetingPatch | None = None
    ignored: list[CompetingPatch] = dataclasses.field(default_factory=list)
    unresolved_because: str | None = None
    missing_inputs: list[str] = dataclasses.field(default_factory=list)
    conditions: list[dict] = dataclasses.field(default_factory=list)

    @property
    def decided(self) -> bool:
        return self.unresolved_because is None


def resolve_load(target: str, patches: list[CompetingPatch]) -> Resolution:
    """Apply the documented rule to the patches competing for one asset."""
    if len(patches) < 2:
        return Resolution(target=target, outcome="no_competition", patches=patches)

    conditioned = [p for p in patches if p.when]
    unparsed = [p for p in patches if p.priority.kind == "unparsed"]
    if unparsed:
        return Resolution(
            target=target, outcome="unresolved", patches=patches,
            unresolved_because=(
                "a competing patch declares a Priority this analysis cannot parse: "
                + ", ".join(f"{p.location} -> {p.priority.raw!r}" for p in unparsed)),
            missing_inputs=["a Priority value matching the documented grammar"])

    if conditioned:
        # Conditions decide whether these patches are ever live at the same time.
        # Evaluating them needs game state we do not have.
        return Resolution(
            target=target, outcome="unresolved", patches=patches,
            unresolved_because=(
                "competing patches are conditional, so whether they ever apply at "
                "the same time depends on game state this analysis does not have"),
            missing_inputs=[
                f"the value of the When conditions on {p.location}: {p.when}"
                for p in conditioned],
            conditions=[{"kind": "state", "description": f"{p.location} applies only When {p.when}"}
                        for p in conditioned])

    exclusive = [p for p in patches if p.priority.is_exclusive]
    if len(exclusive) > 1:
        return Resolution(
            target=target, outcome="exclusive_conflict", patches=patches,
            ignored=list(patches))
    if len(exclusive) == 1:
        return Resolution(
            target=target, outcome="exclusive_supersedes", patches=patches,
            winner=exclusive[0],
            ignored=[p for p in patches if p is not exclusive[0]])

    best = max(p.priority.value for p in patches)
    top = [p for p in patches if p.priority.value == best]
    if len(top) == 1:
        return Resolution(
            target=target, outcome="priority_selection", patches=patches,
            winner=top[0], ignored=[p for p in patches if p is not top[0]])

    return Resolution(
        target=target, outcome="unresolved", patches=patches,
        unresolved_because=(
            f"{len(top)} patches share the highest priority "
            f"({top[0].priority.raw}); the rule then breaks the tie by mod load "
            "order and patch order, and the mod load order for this configuration "
            "was not supplied"),
        missing_inputs=[
            "SMAPI's actual mod load order (it is dependency-sorted, not the order "
            "mods are listed here)"],
    )


def competing_patches(installation, action: str = "Load") -> dict[str, list[CompetingPatch]]:
    """Group an installation's content-pack patches by the asset they target."""
    by_target: dict[str, list[CompetingPatch]] = {}
    for artifact in installation.artifacts:
        ins = artifact.inspection
        if ins is None:
            continue
        for change in ins.fact("content_changes") or []:
            if str(change.get("action") or "").lower() != action.lower():
                continue
            for target in change.get("targets") or []:
                by_target.setdefault(target, []).append(CompetingPatch(
                    artifact=artifact.name,
                    mod_id=artifact.mod_id,
                    target=target,
                    priority=parse_load_priority(change.get("priority")),
                    index=change.get("index", 0),
                    source_file=change.get("source_file"),
                    log_name=change.get("log_name"),
                    from_file=change.get("from_file"),
                    when=change.get("when"),
                    load_index=artifact.load_index,
                ))
    return by_target

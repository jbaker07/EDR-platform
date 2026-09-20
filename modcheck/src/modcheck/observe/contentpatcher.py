"""Parse Content Patcher's own console diagnostics.

Content Patcher answers "did this patch apply, and why not" better than we ever
could from the outside: it is the code that made the decision. So this module
reads its output instead of rebuilding its reasoning.

The grammars below were written against the source that emits them, at
Pathoschild/StardewMods commit 76565e83ede4bc8b3c293f1c659032ba9c39c213:

* ``ContentPatcher/Framework/Commands/Commands/SummaryCommand.cs``
  (sha256 d307375f5be445355b5408aa1954c7afee11a56795264c732b4b636ac7d60c80)
  -- the ``Patches:`` and ``Current changes:`` tables of ``patch summary``.
* ``ContentPatcher/Framework/Commands/Commands/DumpCommand.cs``
  -- ``patch dump applied`` and ``patch dump order``.

Reading the emitter rather than a pasted sample matters because the column
widths are computed at runtime (``priority`` pads to the widest value in the
group, asset names to the longest target), so a parser tuned to one transcript's
spacing would break on the next. These patterns key on the delimiters, not the
widths.

This is a reader, not a judge. It reports what the transcript says. It does not
decide whether that matches what we expected -- ``prediction.py`` does that, and
only against a prediction that was recorded first.
"""
from __future__ import annotations

import dataclasses
import re

# Emitted as: $"      [{X}]     | [{X}]        | [{X}]     | {priority.PadRight(w)} | {path}"
# Priority labels never contain '|', so it is safe to claim the column; anything
# after the fourth delimiter belongs to the name, including a '|' inside it.
_PATCH_ROW = re.compile(
    r"^\s+\[(?P<loaded>[ X])\]\s*\|"
    r"\s*\[(?P<conditions>[ X])\]\s*\|"
    r"\s*\[(?P<applied>[ X])\]\s*\|"
    r"(?P<priority>[^|]*)\|"
    r"\s?(?P<rest>.*)$")
_PATCH_HEADER = re.compile(r"^\s+loaded\s+\|\s*conditions\s*\|\s*applied\s*\|")
_RULE = re.compile(r"^\s*-{3,}[\s|-]*$")
_NOTE_SPLIT = re.compile(r"\s+//\s*")
_LOCALE = re.compile(r"\s\(locale: (?P<locale>.+?) only\)$")
# " (EditImage Animals/horse => Animals/horse)", " (=> Animals/horse)",
# or " (EditImage Animals/horse)".
_RESOLVED = re.compile(r"\s\((?P<raw>[^()]*?)?\s*(?:=>\s*(?P<parsed>[^()]*))?\)$")
_CHANGES_HEADER = re.compile(r"^\s+asset name\s*\|\s*changes\s*$")
_CHANGES_ROW = re.compile(r"^\s+(?P<target>.+?)\s*\|\s*(?P<changes>.*?)\s*$")
_MOD_HEADING = re.compile(r"^(?P<name>\S.*):$")

_DUMP_APPLIED_ROW = re.compile(r"^\s+\[(?P<applied>[ X])\]\s+(?P<type>\S+)\s+(?P<path>.+?)\s*$")
_DUMP_ORDER_HEADER = re.compile(r"^\s+order\s+index path\s+patch\s*$")
_DUMP_ORDER_ROW = re.compile(r"^\s+(?P<order>\S+)\s{2,}(?P<index_path>\S+)\s{2,}(?P<patch>.+?)\s*$")


@dataclasses.dataclass
class ObservedPatch:
    """One row of the `Patches:` table. Every field is what the game reported."""
    mod: str
    name: str
    loaded: bool
    conditions_match: bool
    applied: bool
    priority: str | None
    raw_target: str | None = None
    resolved_target: str | None = None
    locale: str | None = None
    reason_not_loaded: str | None = None
    hints: list[str] = dataclasses.field(default_factory=list)
    raw_line: str = ""

    @property
    def location(self) -> str:
        return f"{self.mod} > {self.name}"


@dataclasses.dataclass
class ModSection:
    name: str
    patches: list[ObservedPatch] = dataclasses.field(default_factory=list)
    # target -> change labels. Absent target means Content Patcher reported no
    # current change for it, which is not the same as the patch being broken.
    current_changes: dict[str, list[str]] = dataclasses.field(default_factory=dict)
    no_current_changes: bool = False


@dataclasses.dataclass
class Summary:
    mods: list[ModSection] = dataclasses.field(default_factory=list)

    @property
    def patches(self) -> list[ObservedPatch]:
        return [p for mod in self.mods for p in mod.patches]

    def mod(self, name: str) -> ModSection | None:
        for section in self.mods:
            if section.name.lower() == name.lower():
                return section
        return None

    def applied_to(self, target: str) -> list[ObservedPatch]:
        """Patches the game reported as applied against `target`.

        Matched on the resolved target where Content Patcher printed one, and on
        the patch name otherwise -- the summary prints the resolved value only
        when it differs from the name.
        """
        wanted = target.lower().replace("\\", "/")
        hits = []
        for patch in self.patches:
            candidates = [c.lower().replace("\\", "/") for c in
                          (patch.resolved_target, patch.raw_target, patch.name) if c]
            if patch.applied and any(wanted in c for c in candidates):
                hits.append(patch)
        return hits


def _flag(value: str) -> bool:
    return value.strip().upper() == "X"


def _split_details(rest: str) -> tuple[str, dict[str, object]]:
    """Peel the suffixes Content Patcher appends, in the order it appends them."""
    parts = _NOTE_SPLIT.split(rest)
    head, notes = parts[0].rstrip(), [p.strip() for p in parts[1:] if p.strip()]

    detail: dict[str, object] = {"hints": [], "reason_not_loaded": None}
    for note in notes:
        if note.lower().startswith("hint:"):
            detail["hints"].append(note[len("hint:"):].strip())  # type: ignore[union-attr]
        else:
            detail["reason_not_loaded"] = note

    locale = _LOCALE.search(head)
    if locale:
        detail["locale"] = locale.group("locale")
        head = head[: locale.start()].rstrip()

    resolved = _RESOLVED.search(head)
    if resolved and (resolved.group("raw") or resolved.group("parsed")):
        raw = (resolved.group("raw") or "").strip() or None
        parsed = (resolved.group("parsed") or "").strip() or None
        # The raw value is printed as "<PatchType> <asset>"; keep the asset.
        if raw and " " in raw:
            raw = raw.split(" ", 1)[1].strip()
        detail["raw_target"] = raw
        detail["resolved_target"] = parsed
        head = head[: resolved.start()].rstrip()

    return head, detail


def parse_summary(text: str) -> Summary:
    """Parse the `Patches:` and `Current changes:` tables of `patch summary`.

    Sections of the output we do not model (global tokens, locations) are
    skipped rather than half-parsed: a field we do not read is better than a
    field we guess at.
    """
    summary = Summary()
    section: ModSection | None = None
    mode: str | None = None

    for line in text.splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        if _RULE.match(line):
            continue

        if _PATCH_HEADER.match(line):
            mode = "patches"
            continue
        if _CHANGES_HEADER.match(line):
            mode = "changes"
            continue
        if stripped in ("Patches:", "Local tokens:", "Current changes:"):
            continue
        if stripped == "No current changes.":
            if section is not None:
                section.no_current_changes = True
            mode = None
            continue

        if mode == "patches":
            row = _PATCH_ROW.match(line)
            if row and section is not None:
                name, detail = _split_details(row.group("rest"))
                section.patches.append(ObservedPatch(
                    mod=section.name, name=name,
                    loaded=_flag(row.group("loaded")),
                    conditions_match=_flag(row.group("conditions")),
                    applied=_flag(row.group("applied")),
                    priority=row.group("priority").strip() or None,
                    raw_target=detail.get("raw_target"),  # type: ignore[arg-type]
                    resolved_target=detail.get("resolved_target"),  # type: ignore[arg-type]
                    locale=detail.get("locale"),  # type: ignore[arg-type]
                    reason_not_loaded=detail.get("reason_not_loaded"),  # type: ignore[arg-type]
                    hints=list(detail["hints"]),  # type: ignore[arg-type]
                    raw_line=line.rstrip()))
                continue
            mode = None  # fell out of the table

        if mode == "changes":
            row = _CHANGES_ROW.match(line)
            if row and section is not None and "|" in line:
                labels = [c.strip() for c in row.group("changes").split(";") if c.strip()]
                section.current_changes[row.group("target").strip()] = labels
                continue
            mode = None

        heading = _MOD_HEADING.match(line)
        if heading and not line.startswith(" ") and not line.startswith("="):
            section = ModSection(name=heading.group("name"))
            summary.mods.append(section)
            mode = None

    return summary


@dataclasses.dataclass
class DumpApplied:
    """`patch dump applied`: patches grouped by current target, in apply order."""
    by_target: dict[str, list[tuple[bool, str, str]]] = dataclasses.field(default_factory=dict)

    def order_for(self, target: str) -> list[str]:
        for name, rows in self.by_target.items():
            if name.lower().replace("\\", "/") == target.lower().replace("\\", "/"):
                return [path for _, _, path in rows]
        return []

    def applied_for(self, target: str) -> list[str]:
        for name, rows in self.by_target.items():
            if name.lower().replace("\\", "/") == target.lower().replace("\\", "/"):
                return [path for applied, _, path in rows if applied]
        return []


def parse_dump_applied(text: str) -> DumpApplied:
    dump = DumpApplied()
    current: str | None = None
    lines = text.splitlines()
    for index, line in enumerate(lines):
        stripped = line.strip()
        if not stripped:
            continue
        row = _DUMP_APPLIED_ROW.match(line)
        if row and current is not None:
            dump.by_target[current].append(
                (_flag(row.group("applied")), row.group("type"), row.group("path")))
            continue
        # An asset heading is a flush-left line underlined by exactly its length.
        nxt = lines[index + 1].strip() if index + 1 < len(lines) else ""
        if not line.startswith(" ") and set(nxt) == {"-"} and len(nxt) == len(stripped):
            current = stripped
            dump.by_target.setdefault(current, [])
    return dump


@dataclasses.dataclass
class DumpOrder:
    """`patch dump order`: the global patch definition order."""
    rows: list[tuple[str, str, str]] = dataclasses.field(default_factory=list)

    def position_of(self, name_fragment: str) -> int | None:
        for index, (_, _, patch) in enumerate(self.rows):
            if name_fragment.lower() in patch.lower():
                return index
        return None


def parse_dump_order(text: str) -> DumpOrder:
    dump = DumpOrder()
    started = False
    for line in text.splitlines():
        if _DUMP_ORDER_HEADER.match(line):
            started = True
            continue
        if not started or not line.strip() or _RULE.match(line):
            continue
        row = _DUMP_ORDER_ROW.match(line)
        if row:
            dump.rows.append((row.group("order"), row.group("index_path"), row.group("patch")))
    return dump

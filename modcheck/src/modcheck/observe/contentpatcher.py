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

# SummaryCommand prints these when `patch summary` was narrowed. They are the
# strongest affirmative evidence that a transcript was meant to cover a given
# mod or asset, which is what a negative claim needs before it can be confirmed.
_MOD_FILTER = re.compile(r"^\(Filtered to content pack ID[s]?: (?P<ids>.+?)\.\)\s*$")
_ASSET_FILTER = re.compile(r"^\(Filtered to asset name[s]?: (?P<names>.+?)\.\)\s*$")
_CONTENT_PATCHES_BANNER = "== Content patches =="

_DUMP_APPLIED_ROW = re.compile(r"^\s+\[(?P<applied>[ X])\]\s+(?P<type>\S+)\s+(?P<path>.+?)\s*$")
_DUMP_ORDER_HEADER = re.compile(r"^\s+order\s+index path\s+patch\s*$")
_DUMP_ORDER_ROW = re.compile(r"^\s+(?P<order>\S+)\s{2,}(?P<index_path>\S+)\s{2,}(?P<patch>.+?)\s*$")


# Content Patcher's own PatchType names. A row whose action we did not observe
# carries None, and a claim that turns on the action must then stay unresolved:
# the summary only prints the type when the patch's name does not already
# contain its target, so absence here is genuinely "not reported", not "not a
# Load".
LOAD_ACTIONS = frozenset({"load"})
EDIT_ACTIONS = frozenset({"editdata", "editimage", "editmap"})


def normalise_asset(name: str | None) -> str | None:
    """Content Patcher's asset identity, for exact comparison.

    Case-insensitive with either separator, matching PathUtilities
    .NormalizeAssetName and the case-insensitive grouping the summary uses. What
    it deliberately does NOT do is substring matching: 'Animals/horse' and
    'Animals/horseFancy' are different assets, and a claim about one must not be
    satisfied by the other.
    """
    if name is None:
        return None
    text = name.strip().replace("\\", "/").strip("/")
    return text.lower() or None


@dataclasses.dataclass
class ObservedPatch:
    """One row of the `Patches:` table. Every field is what the game reported."""
    mod: str
    name: str
    loaded: bool
    conditions_match: bool
    applied: bool
    priority: str | None
    action: str | None = None
    raw_target: str | None = None
    resolved_target: str | None = None
    locale: str | None = None
    reason_not_loaded: str | None = None
    hints: list[str] = dataclasses.field(default_factory=list)
    raw_line: str = ""

    @property
    def location(self) -> str:
        return f"{self.mod} > {self.name}"

    @property
    def target(self) -> str | None:
        """The asset this patch acts on, where the transcript reported one."""
        return normalise_asset(self.resolved_target or self.raw_target)

    @property
    def role(self) -> str:
        """"load", "edit", or "unknown" -- never guessed from the name."""
        if self.action is None:
            return "unknown"
        lowered = self.action.lower()
        if lowered in LOAD_ACTIONS:
            return "load"
        if lowered in EDIT_ACTIONS:
            return "edit"
        return "unknown"


@dataclasses.dataclass
class ModSection:
    name: str
    patches: list[ObservedPatch] = dataclasses.field(default_factory=list)
    # target -> change labels. Absent target means Content Patcher reported no
    # current change for it, which is not the same as the patch being broken.
    current_changes: dict[str, list[str]] = dataclasses.field(default_factory=dict)
    no_current_changes: bool = False


@dataclasses.dataclass
class Coverage:
    """What the transcript demonstrably contained -- not what it lacked.

    This exists for one reason. "No patch applied against X" is a negative, and
    a parser that recognised nothing produces exactly the same empty result as a
    game in which nothing applied. Without a record of what was successfully
    read, a truncated paste, a transcript of the wrong command, or a Content
    Patcher version whose output we cannot parse would all confirm the negative.

    So a negative claim consults this first, and is answered `unobserved` unless
    the transcript affirmatively shows it enumerated the thing being denied.
    """
    saw_content_patches_banner: bool = False
    patch_tables: int = 0
    rows_parsed: int = 0
    unparsed_table_lines: list[str] = dataclasses.field(default_factory=list)
    mod_filters: list[str] = dataclasses.field(default_factory=list)
    asset_filters: list[str] = dataclasses.field(default_factory=list)

    @property
    def usable(self) -> bool:
        """Did we read a patch table at all, and read all of it?"""
        return (self.saw_content_patches_banner and self.patch_tables > 0
                and not self.unparsed_table_lines)

    def why_unusable(self) -> str | None:
        if not self.saw_content_patches_banner:
            return ("the transcript has no '== Content patches ==' section, so it is "
                    "not `patch summary` output, or it was truncated above the part "
                    "that matters")
        if self.patch_tables == 0:
            return "the transcript has no 'Patches:' table, so no patch was enumerated"
        if self.unparsed_table_lines:
            return (f"{len(self.unparsed_table_lines)} line(s) inside a patch table "
                    f"were not recognised, so the enumeration is incomplete: "
                    f"{self.unparsed_table_lines[0]!r}")
        return None

    def covers_asset(self, target: str) -> tuple[bool, str]:
        """Can this transcript answer a negative claim about `target`?"""
        if not self.usable:
            return False, self.why_unusable() or "the transcript is not usable"
        if not self.asset_filters:
            # Unfiltered `patch summary` enumerates every loaded patch, so an
            # asset absent from it really is an asset nothing patches.
            return True, "unfiltered `patch summary`, which enumerates every patch"
        wanted = normalise_asset(target)
        listed = {normalise_asset(a) for a in self.asset_filters}
        if wanted in listed:
            return True, f"transcript is filtered to this asset ({', '.join(self.asset_filters)})"
        return False, (f"the transcript is filtered to {', '.join(self.asset_filters)}, "
                       f"which does not include {target!r}: an asset outside the "
                       "filter is absent for a reason that has nothing to do with "
                       "whether a patch applied to it")


@dataclasses.dataclass
class Summary:
    mods: list[ModSection] = dataclasses.field(default_factory=list)
    coverage: Coverage = dataclasses.field(default_factory=Coverage)

    @property
    def patches(self) -> list[ObservedPatch]:
        return [p for mod in self.mods for p in mod.patches]

    def mod(self, name: str) -> ModSection | None:
        for section in self.mods:
            if section.name.lower() == name.lower():
                return section
        return None

    def for_target(self, target: str) -> list[ObservedPatch]:
        """Every row whose reported target is exactly `target`.

        Exact, normalised identity -- never a substring. A row whose target the
        transcript did not report is not included here, because we do not know
        what it acts on.
        """
        wanted = normalise_asset(target)
        return [p for p in self.patches if p.target is not None and p.target == wanted]

    def applied_to(self, target: str) -> list[ObservedPatch]:
        return [p for p in self.for_target(target) if p.applied]

    def applied_loads_to(self, target: str) -> tuple[list[ObservedPatch], list[ObservedPatch]]:
        """Applied patches against `target`, split into loads and role-unknown.

        The second list is the honest part: a row whose action Content Patcher
        did not print cannot be counted as a load or excluded as an edit, so a
        load-specific claim must stay unresolved while it is non-empty.
        """
        applied = self.applied_to(target)
        return ([p for p in applied if p.role == "load"],
                [p for p in applied if p.role == "unknown"])


def _flag(value: str) -> bool:
    return value.strip().upper() == "X"


def _split_details(rest: str) -> tuple[str, dict[str, object]]:
    """Peel the suffixes Content Patcher appends, in the order it appends them."""
    parts = _NOTE_SPLIT.split(rest)
    head, notes = parts[0].rstrip(), [p.strip() for p in parts[1:] if p.strip()]

    detail: dict[str, object] = {"hints": [], "reason_not_loaded": None, "action": None}
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
        # The raw value is printed as "<PatchType> <asset>". Split it: the type
        # is the only place the summary states a patch's action, and a claim
        # about loads-versus-edits depends on having it.
        if raw and " " in raw:
            action, raw = raw.split(" ", 1)
            detail["action"] = action.strip()
            raw = raw.strip()
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
    coverage = summary.coverage
    section: ModSection | None = None
    mode: str | None = None

    for line in text.splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        if stripped == _CONTENT_PATCHES_BANNER:
            coverage.saw_content_patches_banner = True
            continue
        mod_filter = _MOD_FILTER.match(stripped)
        if mod_filter:
            coverage.mod_filters.extend(
                part.strip() for part in mod_filter.group("ids").split(","))
            continue
        asset_filter = _ASSET_FILTER.match(stripped)
        if asset_filter:
            coverage.asset_filters.extend(
                part.strip() for part in asset_filter.group("names").split(","))
            continue
        if _RULE.match(line):
            continue

        if _PATCH_HEADER.match(line):
            mode = "patches"
            coverage.patch_tables += 1
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
                coverage.rows_parsed += 1
                section.patches.append(ObservedPatch(
                    mod=section.name, name=name,
                    loaded=_flag(row.group("loaded")),
                    conditions_match=_flag(row.group("conditions")),
                    applied=_flag(row.group("applied")),
                    priority=row.group("priority").strip() or None,
                    action=detail.get("action"),  # type: ignore[arg-type]
                    raw_target=detail.get("raw_target"),  # type: ignore[arg-type]
                    resolved_target=detail.get("resolved_target"),  # type: ignore[arg-type]
                    locale=detail.get("locale"),  # type: ignore[arg-type]
                    reason_not_loaded=detail.get("reason_not_loaded"),  # type: ignore[arg-type]
                    hints=list(detail["hints"]),  # type: ignore[arg-type]
                    raw_line=line.rstrip()))
                continue
            # A line that looks like a table row but did not parse means the
            # enumeration is incomplete, and every negative claim that relies on
            # it must be withheld. A line that looks like prose is the table
            # simply ending.
            if "|" in line and "[" in line:
                coverage.unparsed_table_lines.append(line.rstrip())
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
class DumpRow:
    """One row of `patch dump applied`. The action type here is authoritative."""
    applied: bool
    action: str
    path: str

    @property
    def role(self) -> str:
        lowered = self.action.lower()
        if lowered in LOAD_ACTIONS:
            return "load"
        if lowered in EDIT_ACTIONS:
            return "edit"
        return "unknown"


@dataclasses.dataclass
class DumpApplied:
    """`patch dump applied`: active patches grouped by current target.

    Upstream describes this as "the active patches grouped by their current
    target value. Within each group, patches are listed in the expected apply
    order and the checkbox indicates whether each patch is currently applied."

    Two consequences the callers must respect, and which this class encodes
    rather than leaving to the caller's memory:

    * listed order is *expected apply order* for the group, so an ordering claim
      about what actually applied must filter to the checked rows first;
    * a target with no group here was not reported, which is different from a
      target with an empty group.
    """
    by_target: dict[str, list[DumpRow]] = dataclasses.field(default_factory=dict)
    groups_seen: int = 0
    unparsed_lines: list[str] = dataclasses.field(default_factory=list)
    saw_banner: bool = False

    @property
    def usable(self) -> bool:
        return self.groups_seen > 0 and not self.unparsed_lines

    def why_unusable(self) -> str | None:
        if self.groups_seen == 0:
            return ("the transcript lists no target groups, so it is not `patch dump "
                    "applied` output, or it was captured before any asset was loaded")
        if self.unparsed_lines:
            return (f"{len(self.unparsed_lines)} line(s) in a target group were not "
                    f"recognised: {self.unparsed_lines[0]!r}")
        return None

    def rows_for(self, target: str) -> list[DumpRow] | None:
        """Rows for `target`, or None where the dump did not report that target.

        None and [] are deliberately different answers: "not reported" is not
        "reported as empty", and only the second can support a negative claim.
        """
        wanted = normalise_asset(target)
        for name, rows in self.by_target.items():
            if normalise_asset(name) == wanted:
                return rows
        return None

    def order_for(self, target: str, applied_only: bool = True) -> list[str] | None:
        rows = self.rows_for(target)
        if rows is None:
            return None
        return [r.path for r in rows if r.applied or not applied_only]

    def applied_loads_for(self, target: str) -> tuple[list[DumpRow], list[DumpRow]] | None:
        """(loads, role-unknown) among the applied rows for `target`."""
        rows = self.rows_for(target)
        if rows is None:
            return None
        applied = [r for r in rows if r.applied]
        return ([r for r in applied if r.role == "load"],
                [r for r in applied if r.role == "unknown"])


def parse_dump_applied(text: str) -> DumpApplied:
    dump = DumpApplied()
    current: str | None = None
    lines = text.splitlines()
    for index, line in enumerate(lines):
        stripped = line.strip()
        if not stripped:
            continue
        if "grouped by their current target" in stripped:
            dump.saw_banner = True
            continue
        row = _DUMP_APPLIED_ROW.match(line)
        if row and current is not None:
            dump.by_target[current].append(DumpRow(
                applied=_flag(row.group("applied")), action=row.group("type"),
                path=row.group("path")))
            continue
        # An asset heading is a flush-left line underlined by exactly its length.
        nxt = lines[index + 1].strip() if index + 1 < len(lines) else ""
        if not line.startswith(" ") and set(nxt) == {"-"} and len(nxt) == len(stripped):
            current = stripped
            dump.by_target.setdefault(current, [])
            dump.groups_seen += 1
            continue
        if current is not None and stripped.startswith("["):
            dump.unparsed_lines.append(line.rstrip())
    return dump


@dataclasses.dataclass
class DumpOrder:
    """`patch dump order`: the global patch DEFINITION order.

    Upstream's own wording: "the global patch definition order across all loaded
    content packs, which affects the order that patches are applied."

    Affects is not is. This table says where each patch sits in Content
    Patcher's hierarchical definition order across every loaded pack. It does
    not say which patches are active for an asset, which of them applied, or
    what order they applied in for that asset -- those are `patch dump applied`.
    Treating this as apply-order evidence would let a patch that never applied
    satisfy a claim about what applied, so nothing here answers an apply-order
    claim, and the comparison refuses to substitute one for the other.
    """
    rows: list[tuple[str, str, str]] = dataclasses.field(default_factory=list)
    saw_banner: bool = False

    @property
    def usable(self) -> bool:
        return bool(self.rows)

    def position_of(self, name_fragment: str) -> int | None:
        """Definition-order position of the one row matching, else None.

        Returns None for an ambiguous fragment as well as an absent one: two
        matching rows mean we cannot say which patch was meant, and picking the
        first would silently answer a different question.
        """
        hits = [index for index, (_, _, patch) in enumerate(self.rows)
                if name_fragment.lower() in patch.lower()]
        return hits[0] if len(hits) == 1 else None


def parse_dump_order(text: str) -> DumpOrder:
    dump = DumpOrder()
    started = False
    for line in text.splitlines():
        if "global patch definition order" in line:
            dump.saw_banner = True
        if _DUMP_ORDER_HEADER.match(line):
            started = True
            continue
        if not started or not line.strip() or _RULE.match(line):
            continue
        row = _DUMP_ORDER_ROW.match(line)
        if row:
            dump.rows.append((row.group("order"), row.group("index_path"), row.group("patch")))
    return dump

"""LOOT masterlist integration for Skyrim SE, Fallout 4 and Fallout: New Vegas.

LOOT already maintains, by hand and at scale, the requirement / incompatibility
/ load-order / dirty-edit knowledge for the Bethesda games. Rebuilding that
would be pointless. What ModCheck adds is applying it to a *specific*
configuration and being explicit about which of its rules we could and could
not decide.

The masterlists are CC0 1.0, so reuse is unrestricted; we still do not
redistribute them -- we fetch them with provenance and read them from the
evidence cache, so the analysis always reflects the current upstream data.

Semantics preserved from the libloot metadata documentation:

* a plugin entry ``name`` is a regular expression if it contains any of
  ``:\\*?|``, otherwise an exact filename; regexes are anchored and
  case-insensitive;
* ``after``, ``req`` and ``inc`` hold *file* structures whose names are never
  regexes;
* any entry may carry a ``condition``; where we cannot decide it, the finding
  is reported as unresolved rather than dropped or assumed.
"""
from __future__ import annotations

import re
from dataclasses import dataclass
from typing import Any, Iterable

import yaml

from ..analyze.config import Installation, normalize_path
from ..analyze.findings import Finding
from .loot_conditions import (Call, ConditionError, Tri, compile_path_regex, evaluate,
                              is_regex)

GAMES = {"skyrimse": "skyrimse", "fallout4": "fallout4", "falloutnv": "falloutnv"}


def _as_file(entry: Any) -> dict[str, Any]:
    """LOOT file structures have a scalar form (just the name) and a map form."""
    if isinstance(entry, str):
        return {"name": entry}
    if isinstance(entry, dict):
        return entry
    return {"name": str(entry)}


def _norm(path: str) -> str:
    return normalize_path(path)


def _substitute(content: str, subs: list[Any]) -> str:
    """LOOT messages carry ``{0}``-style placeholders filled from ``subs``."""
    for index, value in enumerate(subs):
        content = content.replace("{%d}" % index, str(value))
    return content


class InstallationResolver:
    """Answers LOOT condition functions against a real configuration."""

    def __init__(self, installation: Installation) -> None:
        self.inst = installation
        self.unresolved: list[str] = []

    def __call__(self, call: Call) -> Tri:
        handler = getattr(self, f"_fn_{call.name}", None)
        if handler is None:
            self.unresolved.append(f"{call.name}() is not a condition function we evaluate")
            return None
        try:
            return handler(*call.args)
        except (TypeError, ValueError):
            self.unresolved.append(f"{call.name}({', '.join(call.args)}) could not be evaluated")
            return None

    # -- functions we can decide ----------------------------------------
    def _fn_file(self, path: str) -> Tri:
        if is_regex(path):
            pattern = compile_path_regex(path)
            if any(pattern.match(p) for p in self.inst.known_paths()):
                return True
            return False if self.inst.files_known_complete else None
        return self.inst.has_file(path)

    def _fn_active(self, path: str) -> Tri:
        if is_regex(path):
            pattern = compile_path_regex(path)
            if any(pattern.match(p) for p in self.inst.active_names()):
                return True
            return False if self.inst.artifacts else None
        if not self.inst.artifacts:
            return None
        return _norm(path) in self.inst.active_names()

    def _fn_checksum(self, path: str, expected: str) -> Tri:
        artifact = self.inst.by_name(_norm(path).split("/")[-1])
        if artifact is None or artifact.crc32 is None:
            self.unresolved.append(f"checksum({path}) needs that file's CRC, which we do not have")
            return None
        try:
            return artifact.crc32 == int(expected, 16)
        except ValueError:
            return None

    def _fn_many(self, regex: str) -> Tri:
        pattern = compile_path_regex(regex)
        hits = sum(1 for p in self.inst.known_paths() if pattern.match(p))
        if hits > 1:
            return True
        return False if self.inst.files_known_complete else None

    def _fn_many_active(self, regex: str) -> Tri:
        pattern = compile_path_regex(regex)
        hits = sum(1 for p in self.inst.active_names() if pattern.match(p))
        if hits > 1:
            return True
        return False if self.inst.artifacts else None

    def _fn_is_master(self, path: str) -> Tri:
        artifact = self.inst.by_name(_norm(path).split("/")[-1])
        if artifact is None or artifact.inspection is None:
            return None
        value = artifact.inspection.fact("is_master")
        return value if isinstance(value, bool) else None

    # Version-based functions need file version resources we do not read.
    def _fn_version(self, *args: str) -> Tri:
        self.unresolved.append("version() needs file version information we do not collect")
        return None

    _fn_product_version = _fn_version
    _fn_filename_version = _fn_version

    def _fn_readable(self, path: str) -> Tri:
        return self._fn_file(path)

    def _fn_file_size(self, *args: str) -> Tri:
        return None

    def _fn_is_executable(self, path: str) -> Tri:
        return None

    def _fn_description_contains(self, *args: str) -> Tri:
        return None


@dataclass
class LootMasterlist:
    game: str
    source_id: str
    plugins: list[dict]
    exact: dict[str, list[dict]]
    regexes: list[tuple[re.Pattern[str], dict]]

    @classmethod
    def from_bytes(cls, raw: bytes, *, game: str, source_id: str) -> "LootMasterlist":
        data = yaml.safe_load(raw)
        if not isinstance(data, dict) or "plugins" not in data:
            raise ValueError("not a LOOT masterlist: no 'plugins' key")
        exact: dict[str, list[dict]] = {}
        regexes: list[tuple[re.Pattern[str], dict]] = []
        for entry in data.get("plugins") or []:
            name = entry.get("name")
            if not name:
                continue
            if is_regex(name):
                try:
                    regexes.append((re.compile(f"^(?:{name})$", re.IGNORECASE), entry))
                except re.error:
                    continue
            else:
                exact.setdefault(name.lower(), []).append(entry)
        return cls(game=game, source_id=source_id, plugins=data["plugins"],
                   exact=exact, regexes=regexes)

    def entries_for(self, plugin_name: str) -> list[dict]:
        name = plugin_name.lower()
        out = list(self.exact.get(name, []))
        out.extend(entry for pattern, entry in self.regexes if pattern.match(plugin_name))
        return out

    # -- analysis --------------------------------------------------------
    def analyze(self, installation: Installation) -> list[Finding]:
        findings: list[Finding] = []
        resolver = InstallationResolver(installation)
        order = {a.name.lower(): (a.load_index if a.load_index is not None else 0)
                 for a in installation.artifacts}

        for artifact in installation.artifacts:
            for entry in self.entries_for(artifact.name):
                findings += self._entry_findings(entry, artifact, installation, resolver, order)
        return findings

    def _gate(self, item: dict, resolver: InstallationResolver) -> Tri:
        condition = item.get("condition")
        if not condition:
            return True
        try:
            return evaluate(condition, resolver)
        except ConditionError:
            return None

    def _entry_findings(self, entry: dict, artifact, inst: Installation,
                        resolver: InstallationResolver, order: dict[str, int]) -> list[Finding]:
        out: list[Finding] = []
        subject = artifact.name

        # --- requirements ------------------------------------------------
        for raw in entry.get("req", []) or []:
            item = _as_file(raw)
            gate = self._gate(item, resolver)
            if gate is False:
                continue
            present = inst.has_file(item["name"])
            display = item.get("display") or item["name"]
            if present is True:
                continue
            if present is False:
                out.append(Finding(
                    code="loot.requirement_missing",
                    severity="error",
                    subject=subject,
                    summary=f"{subject} requires {display}, which is not installed",
                    detail=f"LOOT records {item['name']!r} as a requirement of {subject}.",
                    evidence_class="derived",
                    targets=[{"kind": "file_path", "id": item["name"]}],
                    conditions=([{"kind": "other", "description": item["condition"]}]
                                if item.get("condition") else []),
                    sources=[self.source_id],
                    not_established="whether the requirement is satisfied by a differently "
                                    "named file, or by a loader that provides it",
                ))
            else:
                out.append(Finding(
                    code="loot.requirement_unresolved",
                    severity="unresolved",
                    subject=subject,
                    summary=f"{subject} requires {display}; we cannot tell whether it is installed",
                    detail="The configuration we were given does not list this file, and it is "
                           "not a complete file listing, so its absence proves nothing.",
                    evidence_class="unresolved",
                    targets=[{"kind": "file_path", "id": item["name"]}],
                    sources=[self.source_id],
                    not_established="presence of the required file",
                ))

        # --- incompatibilities -------------------------------------------
        for raw in entry.get("inc", []) or []:
            item = _as_file(raw)
            gate = self._gate(item, resolver)
            if gate is False:
                continue
            present = inst.has_file(item["name"])
            display = item.get("display") or item["name"]
            if present is True:
                out.append(Finding(
                    code="loot.incompatible_present",
                    severity="error",
                    subject=subject,
                    summary=f"{subject} is recorded as incompatible with {display}, "
                            "which is installed",
                    detail=f"LOOT records {item['name']!r} as incompatible with {subject}."
                           + (f" Condition: {item['condition']}" if item.get("condition") else ""),
                    evidence_class="derived",
                    targets=[{"kind": "file_path", "id": item["name"]}],
                    conditions=([{"kind": "other", "description": item["condition"]}]
                                if item.get("condition") else []),
                    sources=[self.source_id],
                    not_established="what the incompatibility does in game, and whether a "
                                    "patch resolves it",
                ))
            elif present is None and gate is not False:
                out.append(Finding(
                    code="loot.incompatible_unresolved",
                    severity="unresolved",
                    subject=subject,
                    summary=f"{subject} is recorded as incompatible with {display}; "
                            "we cannot tell whether that is installed",
                    evidence_class="unresolved",
                    targets=[{"kind": "file_path", "id": item["name"]}],
                    sources=[self.source_id],
                    not_established="presence of the incompatible file",
                ))

        # --- load order ---------------------------------------------------
        for raw in entry.get("after", []) or []:
            item = _as_file(raw)
            if self._gate(item, resolver) is False:
                continue
            other = _norm(item["name"]).split("/")[-1]
            if other not in order:
                continue
            if artifact.load_index is None:
                continue
            if order[other] > artifact.load_index:
                out.append(Finding(
                    code="loot.load_order",
                    severity="warning",
                    subject=subject,
                    summary=f"{subject} should load after {item['name']}, but currently "
                            "loads before it",
                    detail="LOOT records this ordering rule to resolve a specific "
                           "compatibility issue.",
                    evidence_class="derived",
                    targets=[{"kind": "file_path", "id": item["name"]}],
                    sources=[self.source_id],
                    resolutions=[{"method": "load_order_change",
                                  "step": f"Load {subject} after {item['name']}. "
                                          "Running LOOT sorts this automatically."}],
                    not_established="whether the current order actually manifests a problem "
                                    "in this configuration",
                ))

        # --- dirty / clean, matched by exact artifact identity -------------
        for raw in entry.get("dirty", []) or []:
            crc = raw.get("crc")
            if crc is None or artifact.crc32 is None or int(crc) != artifact.crc32:
                continue
            counts = ", ".join(f"{raw[k]} {k.upper()}" for k in ("itm", "udr", "nav") if raw.get(k))
            out.append(Finding(
                code="loot.dirty_edits",
                severity="warning",
                subject=subject,
                summary=f"this exact copy of {subject} is recorded as having dirty edits"
                        + (f" ({counts})" if counts else ""),
                detail=f"Matched by CRC {artifact.crc32:08X}, so this is the specific file "
                       "LOOT recorded, not merely a same-named plugin.",
                evidence_class="declared",
                targets=[{"kind": "file_path", "id": artifact.name}],
                sources=[self.source_id],
                resolutions=[{"method": "known_patch",
                              "step": f"Clean with {raw.get('util', 'the game''s xEdit')}."}],
                not_established="whether the dirty edits cause a problem in this configuration",
            ))
        for raw in entry.get("clean", []) or []:
            crc = raw.get("crc")
            if crc is None or artifact.crc32 is None or int(crc) != artifact.crc32:
                continue
            out.append(Finding(
                code="loot.verified_clean",
                severity="note",
                subject=subject,
                summary=f"this exact copy of {subject} is recorded as verified clean",
                detail=f"Matched by CRC {artifact.crc32:08X}.",
                evidence_class="declared",
                sources=[self.source_id],
                not_established="anything beyond the absence of dirty edits",
            ))

        # --- messages ------------------------------------------------------
        for raw in entry.get("msg", []) or []:
            if not isinstance(raw, dict):
                continue
            gate = self._gate(raw, resolver)
            if gate is False:
                continue
            content = raw.get("content")
            if isinstance(content, list):
                english = next((c.get("str") for c in content
                                if isinstance(c, dict) and c.get("lang") == "en"), None)
                content = english or (content[0].get("str") if isinstance(content[0], dict) else None)
            if not content:
                continue
            content = _substitute(str(content), raw.get("subs") or [])
            kind = raw.get("type", "say")
            out.append(Finding(
                code=f"loot.message.{kind}",
                severity={"error": "error", "warn": "warning"}.get(kind, "note")
                if gate is True else "unresolved",
                subject=subject,
                summary=str(content)[:300],
                evidence_class="declared" if gate is True else "unresolved",
                conditions=([{"kind": "other", "description": raw["condition"]}]
                            if raw.get("condition") else []),
                sources=[self.source_id],
                not_established=("whether this message's condition holds for this "
                                 "configuration") if gate is None else "",
            ))

        return out

    def bash_tags_for(self, plugin_name: str) -> list[str]:
        tags: list[str] = []
        for entry in self.entries_for(plugin_name):
            for tag in entry.get("tag", []) or []:
                tags.append(tag if isinstance(tag, str) else tag.get("name", ""))
        return [t for t in tags if t]


def coverage() -> tuple[list[str], list[str]]:
    """What a LOOT-backed analysis does and does not establish."""
    checked = [
        "LOOT-recorded requirements, incompatibilities and load-order rules for each plugin",
        "LOOT dirty/clean records matched by exact CRC",
        "LOOT messages whose conditions we could evaluate",
    ]
    not_checked = [
        "record-level conflicts between plugins (LOOT does not do this; xEdit does)",
        "plugins with no masterlist entry -- absence of a rule is not evidence of safety",
        "conditions needing file versions or checksums we were not given",
        "anything about meshes, textures, scripts or other loose files",
    ]
    return checked, not_checked

"""Stardew Valley: SMAPI's own mod metadata.

SMAPI ships a metadata file recording, per mod and per version range, whether
it is `Obsolete`, `AssumeBroken` or `AssumeCompatible`, usually with a
human-readable reason. That is authoritative version-aware compatibility
knowledge maintained by the loader's own author, and it is exactly the kind of
thing that should be reused rather than re-derived.

Key grammar, from the file's own documentation:

    Each key consists of a field name prefixed with any combination of version
    range and 'Default', separated by pipes (whitespace trimmed).
    The version format is 'min~max' (where either side can be blank for
    unbounded), or a single version number.

So ``~1.0.0 | Status`` applies to every version up to and including 1.0.0,
``~ | Status`` applies to all versions, and ``1.2 | Status`` to exactly 1.2.

Licensing: the SMAPI repository is LGPL-3.0. ModCheck reads this file as data
from the evidence cache at analysis time; it does not redistribute it, link
against SMAPI, or vendor any of it. Whether the metadata file is itself covered
by the code licence has not been established here, and the source record says
so rather than assuming the question away.
"""
from __future__ import annotations

import json
import re
from dataclasses import dataclass, field
from functools import lru_cache
from typing import Any

from ..analyze.config import Installation
from ..analyze.findings import Finding
from ..analyze.versions import Version

STATUS_SEVERITY = {
    "assumebroken": "error",
    "obsolete": "warning",
    "assumecompatible": "note",
}


def strip_jsonc(raw: bytes | str) -> str:
    """Remove // and /* */ comments, respecting string literals.

    A regex would corrupt any string containing `//`, such as a URL, so this
    walks the text instead.
    """
    text = raw.decode("utf-8-sig") if isinstance(raw, bytes) else raw
    out: list[str] = []
    i, n, in_string = 0, len(text), False
    while i < n:
        char = text[i]
        if in_string:
            out.append(char)
            if char == "\\" and i + 1 < n:
                out.append(text[i + 1])
                i += 2
                continue
            if char == '"':
                in_string = False
            i += 1
            continue
        if char == '"':
            in_string = True
            out.append(char)
            i += 1
            continue
        if text.startswith("/*", i):
            end = text.find("*/", i + 2)
            i = end + 2 if end >= 0 else n
            continue
        if text.startswith("//", i):
            end = text.find("\n", i)
            i = end if end >= 0 else n
            continue
        out.append(char)
        i += 1
    # trailing commas are legal in this file but not in JSON
    return re.sub(r",(\s*[}\]])", r"\1", "".join(out))


@dataclass(frozen=True)
class VersionRange:
    low: str | None
    high: str | None
    exact: str | None = None

    @classmethod
    def parse(cls, text: str) -> "VersionRange":
        text = text.strip()
        if "~" not in text:
            return cls(low=None, high=None, exact=text or None)
        low, _, high = text.partition("~")
        return cls(low=low.strip() or None, high=high.strip() or None)

    def contains(self, version_text: str) -> bool | None:
        version = Version.parse(version_text)
        if self.exact is not None:
            result = version.compare(Version.parse(self.exact))
            return None if result is None else result == 0
        if self.low is not None:
            result = version.compare(Version.parse(self.low))
            if result is None:
                return None
            if result < 0:
                return False
        if self.high is not None:
            result = version.compare(Version.parse(self.high))
            if result is None:
                return None
            if result > 0:
                return False
        return True


@dataclass
class ModEntry:
    display_name: str
    mod_id: str
    former_ids: list[str] = field(default_factory=list)
    overrides: list[tuple[VersionRange | None, bool, str, Any]] = field(default_factory=list)

    def identifiers(self) -> set[str]:
        out = {self.mod_id.lower()} if self.mod_id else set()
        out |= {f.lower() for f in self.former_ids}
        return out

    def applicable(self, version: str | None, name: str) -> Any:
        """Value of field `name` for this version, or None if none applies."""
        for version_range, _is_default, field_name, value in self.overrides:
            if field_name.lower() != name.lower():
                continue
            if version_range is None:
                return value
            if version is None:
                continue
            if version_range.contains(version) is True:
                return value
        return None

    def has_unresolved_range(self, version: str | None, name: str) -> bool:
        """True when a rule for this field exists but the version is undecidable."""
        for version_range, _d, field_name, _v in self.overrides:
            if field_name.lower() != name.lower() or version_range is None:
                continue
            if version is None or version_range.contains(version) is None:
                return True
        return False


@dataclass
class SmapiMetadata:
    source_id: str
    entries: list[ModEntry]

    @classmethod
    def from_bytes(cls, raw: bytes, *, source_id: str) -> "SmapiMetadata":
        data = json.loads(strip_jsonc(raw))
        mod_data = data.get("ModData")
        if not isinstance(mod_data, dict):
            raise ValueError("not a SMAPI metadata file: no 'ModData' object")
        entries = []
        for display_name, raw_entry in mod_data.items():
            if not isinstance(raw_entry, dict):
                continue
            mod_id = raw_entry.get("Id") or raw_entry.get("ID") or ""
            former = [f.strip() for f in str(raw_entry.get("FormerIds", "")).split("|")
                      if f.strip()]
            overrides = []
            for key, value in raw_entry.items():
                if key in ("Id", "ID", "FormerIds", "SuppressWarnings"):
                    continue
                parts = [p.strip() for p in key.split("|")]
                field_name = parts[-1]
                is_default = any(p.lower() == "default" for p in parts[:-1])
                range_parts = [p for p in parts[:-1] if p.lower() != "default"]
                version_range = VersionRange.parse(range_parts[0]) if range_parts else None
                if version_range is not None and version_range.exact is None \
                        and version_range.low is None and version_range.high is None:
                    version_range = None  # bare "~" means all versions
                overrides.append((version_range, is_default, field_name, value))
            entries.append(ModEntry(display_name=display_name, mod_id=str(mod_id),
                                    former_ids=former, overrides=overrides))
        return cls(source_id=source_id, entries=entries)

    @classmethod
    def cached(cls, raw: bytes, *, source_id: str) -> "SmapiMetadata":
        return _cached_metadata(raw, source_id)

    def find(self, unique_id: str) -> ModEntry | None:
        key = (unique_id or "").strip().lower()
        if not key:
            return None
        for entry in self.entries:
            if key in entry.identifiers():
                return entry
        return None

    def analyze(self, installation: Installation) -> list[Finding]:
        findings: list[Finding] = []
        for artifact in installation.artifacts:
            unique_id = artifact.mod_id or ""
            entry = self.find(unique_id)
            if entry is None:
                continue
            version = artifact.version
            status = entry.applicable(version, "Status")
            reason = (entry.applicable(version, "StatusReasonPhrase")
                      or entry.applicable(version, "StatusReasonDetails") or "")

            if status is None:
                if entry.has_unresolved_range(version, "Status"):
                    findings.append(Finding(
                        code="smapi.status_unresolved",
                        severity="unresolved",
                        subject=artifact.name,
                        summary=f"SMAPI records version-specific compatibility status for "
                                f"{entry.display_name}, but this copy's version "
                                f"({version or 'unknown'}) could not be compared",
                        evidence_class="unresolved",
                        sources=[self.source_id],
                        not_established="whether this version is one SMAPI refuses to load",
                    ))
                continue

            key = str(status).strip().lower()
            severity = STATUS_SEVERITY.get(key, "note")
            summary = {
                "assumebroken": f"SMAPI will refuse to load this version of "
                                f"{entry.display_name}",
                "obsolete": f"SMAPI records {entry.display_name} as obsolete",
                "assumecompatible": f"SMAPI records {entry.display_name} as compatible "
                                    "despite its own checks",
            }.get(key, f"SMAPI records status {status!r} for {entry.display_name}")

            findings.append(Finding(
                code=f"smapi.status_{key}",
                severity=severity,
                subject=artifact.name,
                summary=summary,
                detail=str(reason),
                evidence_class="declared",
                targets=[{"kind": "registry_key", "id": entry.mod_id or unique_id}],
                conditions=[{"kind": "version",
                             "description": f"applies to the installed version {version}"}]
                           if version else [],
                sources=[self.source_id],
                resolutions=([{"method": "version_change",
                               "step": "Update the mod to a version SMAPI will load, or "
                                       "remove it."}]
                             if key == "assumebroken" else
                             [{"method": "remove_conflict",
                               "step": "This mod is no longer needed and can be removed."}]
                             if key == "obsolete" else []),
                not_established="anything about this mod's interaction with other mods",
            ))
        return findings


@lru_cache(maxsize=4)
def _cached_metadata(raw: bytes, source_id: str) -> SmapiMetadata:
    return SmapiMetadata.from_bytes(raw, source_id=source_id)


def coverage(available: bool) -> tuple[list[str], list[str]]:
    checked = ["declared SMAPI manifest dependencies"]
    if available:
        checked.append("SMAPI's own per-version compatibility status for each installed mod")
    not_checked = [
        "mods SMAPI has no metadata entry for -- absence of an entry is not evidence "
        "of compatibility",
        "Harmony patches and C# behaviour inside mod assemblies",
        "whether two Content Patcher packs editing the same asset actually conflict",
    ]
    if not available:
        not_checked.append("SMAPI compatibility status: the metadata file was not available")
    return checked, not_checked

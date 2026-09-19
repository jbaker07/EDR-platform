"""RimWorld interaction analysis.

Two independent sources of ordering and compatibility rules:

* **Declared** — each mod's own About.xml lists ``loadAfter``, ``loadBefore``,
  ``incompatibleWith`` and ``modDependencies``. This needs nothing external.
* **Community** — the RimSort Community Rules Database, a maintained JSON file
  of load-order and incompatibility rules keyed by packageId, covering cases
  authors did not declare themselves.

RimWorld loads mods in the order the player sets and merges their Defs by
that order, so "these two must be ordered" is a real, checkable statement,
while "these two both add Defs" is not a conflict.

Reuse note: the community database carries no licence file, so its reuse terms
are unknown. ModCheck reads it from the evidence cache at analysis time and does
not redistribute it.
"""
from __future__ import annotations

import json
from dataclasses import dataclass
from functools import lru_cache
from typing import Any

from ..analyze.config import Installation
from ..analyze.findings import Finding

# RimWorld treats packageIds case-insensitively.
def _pid(value: Any) -> str:
    return str(value or "").strip().lower()


def _display(entry: Any, fallback: str) -> str:
    if isinstance(entry, dict):
        names = entry.get("name")
        if isinstance(names, list) and names:
            return str(names[0])
        if isinstance(names, str):
            return names
    return fallback


@dataclass
class CommunityRules:
    """The RimSort Community Rules Database."""

    source_id: str
    rules: dict[str, dict[str, Any]]
    timestamp: Any = None

    @classmethod
    def from_bytes(cls, raw: bytes, *, source_id: str) -> "CommunityRules":
        data = json.loads(raw.decode("utf-8"))
        if not isinstance(data, dict) or "rules" not in data:
            raise ValueError("not a community rules database: no 'rules' key")
        rules = {_pid(k): v for k, v in data["rules"].items() if isinstance(v, dict)}
        return cls(source_id=source_id, rules=rules, timestamp=data.get("timestamp"))

    @classmethod
    def cached(cls, raw: bytes, *, source_id: str) -> "CommunityRules":
        return _cached_rules(raw, source_id)

    def for_mod(self, package_id: str) -> dict[str, Any]:
        return self.rules.get(_pid(package_id), {})

    def analyze(self, installation: Installation) -> list[Finding]:
        return _order_findings(installation, self._edges, self.source_id, "community")

    def _edges(self, package_id: str) -> dict[str, list[tuple[str, str]]]:
        entry = self.for_mod(package_id)
        out: dict[str, list[tuple[str, str]]] = {}
        for key in ("loadAfter", "loadBefore", "incompatibleWith"):
            section = entry.get(key)
            if not isinstance(section, dict):
                continue
            out[key] = [(_pid(other), _display(meta, other))
                        for other, meta in section.items()]
        for key in ("loadTop", "loadBottom"):
            section = entry.get(key)
            if isinstance(section, dict) and str(section.get("value", "")).lower() == "true":
                out[key] = [("", key)]
        return out


@lru_cache(maxsize=4)
def _cached_rules(raw: bytes, source_id: str) -> CommunityRules:
    return CommunityRules.from_bytes(raw, source_id=source_id)


def _index(installation: Installation) -> dict[str, Any]:
    index = {}
    for artifact in installation.artifacts:
        index[_pid(artifact.mod_id or artifact.name)] = artifact
    return index


def _order_findings(installation: Installation, edges_for, source_id: str,
                    origin: str) -> list[Finding]:
    index = _index(installation)
    findings: list[Finding] = []
    origin_label = ("the RimSort community rules database" if origin == "community"
                    else "the mod's own About.xml")
    evidence = "declared" if origin == "declared" else "derived"

    for package_id, artifact in index.items():
        edges = edges_for(package_id)
        position = artifact.load_index if artifact.load_index is not None else 0

        for other_id, display in edges.get("loadAfter", []):
            other = index.get(other_id)
            if other is None or other.load_index is None:
                continue
            if other.load_index > position:
                findings.append(_ordering(artifact, display, other_id, "after",
                                          source_id, origin_label, evidence))
        for other_id, display in edges.get("loadBefore", []):
            other = index.get(other_id)
            if other is None or other.load_index is None:
                continue
            if other.load_index < position:
                findings.append(_ordering(artifact, display, other_id, "before",
                                          source_id, origin_label, evidence))
        for other_id, display in edges.get("incompatibleWith", []):
            if other_id in index:
                findings.append(Finding(
                    code=f"rimworld.{origin}.incompatible_present",
                    severity="error",
                    subject=artifact.name,
                    summary=f"{artifact.name} is recorded as incompatible with {display}, "
                            "which is installed",
                    detail=f"Rule from {origin_label}.",
                    evidence_class=evidence,
                    targets=[{"kind": "registry_key", "id": other_id}],
                    sources=[source_id] if source_id else [],
                    not_established="what goes wrong in game, and whether a patch exists",
                ))
    return findings


def _ordering(artifact, display: str, other_id: str, direction: str,
              source_id: str, origin_label: str, evidence: str) -> Finding:
    return Finding(
        code=f"rimworld.load_order",
        severity="warning",
        subject=artifact.name,
        summary=f"{artifact.name} should load {direction} {display}, but currently "
                f"loads {'before' if direction == 'after' else 'after'} it",
        detail=f"Rule from {origin_label}. RimWorld merges Defs in load order, so the "
               "order decides which mod's values survive.",
        evidence_class=evidence,
        targets=[{"kind": "registry_key", "id": other_id}],
        sources=[source_id] if source_id else [],
        resolutions=[{"method": "load_order_change",
                      "step": f"Move {artifact.name} {direction} {display} in the mod list."}],
        not_established="whether the current order actually changes behaviour in this "
                        "configuration",
    )


def analyze_declared(installation: Installation) -> list[Finding]:
    """Rules each mod declares in its own About.xml. Needs nothing external."""

    def edges_for(package_id: str) -> dict[str, list[tuple[str, str]]]:
        for artifact in installation.artifacts:
            if _pid(artifact.mod_id or artifact.name) != package_id:
                continue
            ins = artifact.inspection
            if ins is None:
                return {}
            return {
                "loadAfter": [(_pid(v), v) for v in (ins.fact("load_after") or [])],
                "loadBefore": [(_pid(v), v) for v in (ins.fact("load_before") or [])],
                "incompatibleWith": [(_pid(v), v)
                                     for v in (ins.fact("incompatible_with") or [])],
            }
        return {}

    return _order_findings(installation, edges_for, "", "declared")


def coverage(with_community: bool) -> tuple[list[str], list[str]]:
    checked = ["load order and incompatibility rules each mod declares in About.xml"]
    if with_community:
        checked.append("load order and incompatibility rules from the RimSort community "
                       "rules database")
    not_checked = [
        "Def-level conflicts: which mods patch or replace the same Def",
        "xpath targets of individual PatchOperations",
        "Harmony patches inside shipped assemblies",
        "mods with no declared or recorded rule -- absence of a rule is not evidence "
        "of safety",
    ]
    if not with_community:
        not_checked.append("community rules: the database was not available")
    return checked, not_checked

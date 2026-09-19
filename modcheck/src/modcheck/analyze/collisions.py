"""Same-target collisions, using each ecosystem's own composition rule.

Two mods touching "the same thing" means different things in different games,
and in some of them it means nothing at all. This module only reports a
collision where the ecosystem's documented composition actually makes one
modification supersede another:

* **The Sims 4** — packages are keyed by resource (type, group, instance). Two
  packages containing the same key override the same game resource and the
  later-loaded one wins. This is the ecosystem's real conflict.
* **Project Zomboid** — Lua is loaded by its path under ``media/``. Two mods
  shipping the same relative path collide; the later one wins.
* **Cyberpunk 2077** — the install path is the loader contract. Two mods
  shipping the same file path cannot both be installed.
* **Stardew Valley / Content Patcher** — two ``Load`` actions on one asset
  conflict, because only one can supply it. Two ``EditData`` actions on the
  same asset normally coexist, because they patch different entries; they are
  reported only when they touch the same entry or field, and then as something
  to check rather than as a failure.

Minecraft is deliberately absent. Two jars containing the same class path is
usually a shaded library, not a conflict, and we have no evidence to
distinguish the cases.
"""
from __future__ import annotations

from collections import defaultdict
from typing import Any, Iterable

from .config import Installation
from .findings import Finding


def _pairs(owners: list[str]) -> str:
    return ", ".join(sorted(owners))


def _collect(installation: Installation, fact: str,
             transform=lambda v: v) -> dict[Any, list[str]]:
    index: dict[Any, list[str]] = defaultdict(list)
    for artifact in installation.artifacts:
        ins = artifact.inspection
        if ins is None:
            continue
        for value in ins.fact(fact) or []:
            index[transform(value)].append(artifact.name)
    return index


def sims4(installation: Installation) -> list[Finding]:
    findings = []
    for key, owners in sorted(_collect(installation, "resource_keys").items()):
        unique = sorted(set(owners))
        if len(unique) < 2:
            continue
        findings.append(Finding(
            code="collision.sims4_resource_key",
            severity="warning",
            subject=_pairs(unique),
            summary=f"{len(unique)} packages contain the same resource key {key}",
            detail="Sims 4 keys resources by type, group and instance. Two packages "
                   "containing the same key override the same game resource, and only "
                   "the one the game loads last takes effect.",
            evidence_class="extracted",
            targets=[{"kind": "record", "id": key, "label": "DBPF resource key"}],
            sources=[],
            resolutions=[{"method": "known_patch",
                          "step": "Use a merged or patched package that combines both "
                                  "changes, or keep only one."}],
            not_established="which package the game loads last, and whether the two "
                            "resources actually differ",
        ))
    return findings


def projectzomboid(installation: Installation) -> list[Finding]:
    findings = []
    for path, owners in sorted(_collect(installation, "lua_files").items()):
        unique = sorted(set(owners))
        if len(unique) < 2:
            continue
        findings.append(Finding(
            code="collision.pz_lua_path",
            severity="warning",
            subject=_pairs(unique),
            summary=f"{len(unique)} mods ship the same Lua path {path}",
            detail="Project Zomboid loads Lua by its path under media/. Two mods "
                   "shipping the same path collide and the later one wins.",
            evidence_class="extracted",
            targets=[{"kind": "file_path", "id": path}],
            resolutions=[{"method": "load_order_change",
                          "step": "Order the mods so the one whose version of this file "
                                  "you want loads last, or use a compatibility patch."}],
            not_established="whether the two files actually differ, and which the game "
                            "loads last",
        ))
    return findings


def cyberpunk2077(installation: Installation) -> list[Finding]:
    findings = []
    for path, owners in sorted(_collect(installation, "install_paths").items()):
        unique = sorted(set(owners))
        if len(unique) < 2 or path.endswith("/"):
            continue
        findings.append(Finding(
            code="collision.cyberpunk_install_path",
            severity="warning",
            subject=_pairs(unique),
            summary=f"{len(unique)} mods install a file at the same path {path}",
            detail="The install path is the loader contract in Cyberpunk. Two mods "
                   "shipping the same path cannot both be installed; one overwrites "
                   "the other.",
            evidence_class="extracted",
            targets=[{"kind": "file_path", "id": path}],
            not_established="whether the files are identical, and which was installed "
                            "last",
        ))
    return findings


def stardewvalley(installation: Installation) -> list[Finding]:
    """Content Patcher: Load actions conflict; EditData usually does not."""
    loads: dict[str, list[str]] = defaultdict(list)
    edits: dict[tuple[str, str], list[str]] = defaultdict(list)

    for artifact in installation.artifacts:
        ins = artifact.inspection
        if ins is None:
            continue
        for change in ins.fact("content_changes") or []:
            target = change.get("target")
            if not target:
                continue
            action = str(change.get("action") or "").lower()
            conditioned = bool(change.get("when"))
            if action == "load" and not conditioned:
                loads[target].append(artifact.name)
            elif action in ("editdata", "editimage", "editmap"):
                for entry in (change.get("entries") or []) + (change.get("fields") or []):
                    edits[(target, str(entry))].append(artifact.name)

    findings = []
    for target, owners in sorted(loads.items()):
        unique = sorted(set(owners))
        if len(unique) < 2:
            continue
        findings.append(Finding(
            code="collision.contentpatcher_load",
            severity="error",
            subject=_pairs(unique),
            summary=f"{len(unique)} content packs both Load the asset {target}",
            detail="Only one pack can supply an asset with Load. Content Patcher "
                   "reports this as a conflict and one of the packs will not apply.",
            evidence_class="extracted",
            targets=[{"kind": "asset", "id": target}],
            resolutions=[{"method": "known_patch",
                          "step": "Keep one pack, or use versions that Edit the asset "
                                  "instead of Loading it."}],
            not_established="which pack wins, and whether either provides a patch for "
                            "the other",
        ))
    for (target, entry), owners in sorted(edits.items()):
        unique = sorted(set(owners))
        if len(unique) < 2:
            continue
        findings.append(Finding(
            code="collision.contentpatcher_same_entry",
            severity="warning",
            subject=_pairs(unique),
            summary=f"{len(unique)} content packs edit the same entry {entry!r} of "
                    f"{target}",
            detail="Content packs editing different entries of the same asset coexist "
                   "normally. These edit the same entry, so the later one overrides the "
                   "earlier.",
            evidence_class="extracted",
            targets=[{"kind": "field", "id": entry, "container": target}],
            not_established="whether the two edits set the same value, and the order "
                            "Content Patcher applies them in",
        ))
    return findings


ANALYZERS = {
    "sims4": sims4,
    "projectzomboid": projectzomboid,
    "cyberpunk2077": cyberpunk2077,
    "stardewvalley": stardewvalley,
}

# Ecosystems where co-presence of a same-named thing is NOT evidence of conflict.
NOT_ANALYSED = {
    "minecraft": "two jars containing the same class path is usually a shaded library, "
                 "not a conflict, and we cannot distinguish the cases",
    "skyrimse": "record-level conflicts need a record parser; LOOT rules cover the "
                "documented cases instead",
    "fallout4": "record-level conflicts need a record parser; LOOT rules cover the "
                "documented cases instead",
    "falloutnv": "record-level conflicts need a record parser; LOOT rules cover the "
                 "documented cases instead",
    "bg3": "the .pak file list is LZ4-compressed and is not read, so we cannot see "
           "which resources two mods both contain",
    "rimworld": "Def-level overrides need the Def contents; declared and community "
                "load-order rules cover the documented cases instead",
}


def analyze(installation: Installation) -> list[Finding]:
    analyzer = ANALYZERS.get(installation.game)
    return analyzer(installation) if analyzer else []


def coverage(game: str) -> tuple[list[str], list[str]]:
    if game in ANALYZERS:
        checked = [{
            "sims4": "packages overriding the same DBPF resource key",
            "projectzomboid": "mods shipping the same Lua path under media/",
            "cyberpunk2077": "mods installing a file at the same path",
            "stardewvalley": "content packs loading the same asset, or editing the "
                             "same entry of one",
        }[game]]
        not_checked = ["whether the colliding content actually differs",
                       "the order the game or loader resolves the collision in"]
    else:
        checked = []
        not_checked = [f"same-target collisions: {NOT_ANALYSED.get(game, 'not implemented for this game')}"]
    return checked, not_checked

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
    """Content Patcher: resolve competing Load patches by the documented rule.

    Load patches are exclusive by default, so two packs loading one asset is a
    real failure -- neither applies. But a pack that sets an explicit priority
    has deliberately chosen to compete and win, which is a selection, not a
    conflict, and must not be reported as one. Where the rule's tiebreak needs
    the mod load order, or where conditions decide whether the patches are ever
    live together, the result is reported unresolved with the missing input
    named.

    EditData patches compose, so they are only reported where two packs touch
    the same entry.
    """
    from . import contentpatcher as cp

    findings: list[Finding] = []

    for target, patches in sorted(cp.competing_patches(installation, "Load").items()):
        # Two patches from one pack are that author's own ordering, not a
        # cross-mod interaction.
        if len({p.artifact for p in patches}) < 2:
            continue
        resolution = cp.resolve_load(target, patches)
        finding = _load_finding(resolution)
        if finding is not None:
            findings.append(finding)

    findings += _edit_findings(installation)
    return findings


def _patch_lines(patches) -> str:
    return "; ".join(f"{p.location} priority={p.priority.raw}" for p in patches)


def _load_finding(resolution):
    """Turn one resolved competition into a finding, or None when there is none."""
    from . import contentpatcher as cp

    target = resolution.target
    targets = [{"kind": "asset", "id": target}]
    packs = sorted({p.artifact for p in resolution.patches})

    if resolution.outcome == "no_competition":
        return None

    if resolution.outcome == "exclusive_conflict":
        return Finding(
            code="contentpatcher.exclusive_conflict",
            severity="error",
            subject=", ".join(packs),
            summary=(f"{len(resolution.patches)} patches load {target} at Exclusive "
                     "priority, so Content Patcher applies NONE of them"),
            detail=(f"{cp.GOVERNING_RULE_LOAD} Competing patches: "
                    f"{_patch_lines(resolution.patches)}. Exclusive is the default, so "
                    "a patch that sets no Priority is exclusive."),
            evidence_class="derived",
            targets=targets,
            sources=[cp.RULE_SOURCE],
            resolutions=[
                {"method": "configuration_change", "audience": "creator",
                 "step": ("Changing priority selects which replacement is used. Choose "
                          "the intended replacement, then check whether the other "
                          "pack's remaining patches still work with that asset. "
                          "Selecting a winner is not the same as both packs working: "
                          "Content Patcher's own documentation says Exclusive is the "
                          "default precisely because it cannot know whether a pack "
                          "still works when a different replacement is selected.")},
                {"method": "alternative_extension_point", "audience": "creator",
                 "step": ("If a pack only changes part of the asset, an Edit action may "
                          "be an alternative, since Edit patches compose instead of "
                          "competing. This is not an automatic compatibility "
                          "guarantee: the edit still has to make sense against "
                          "whichever asset ends up loaded.")},
                {"method": "installation_change", "audience": "player",
                 "step": (f"Keep one of these packs installed and remove or disable the "
                          f"other ({', '.join(packs)}). While both are installed neither "
                          f"replacement is applied, so {target} stays as the game ships "
                          "it -- removing one is what makes a replacement appear. The "
                          "packs' own authors can also change this, by agreeing which "
                          "one declares a lower priority; the finding above is worth "
                          "reporting to them.")},
            ],
            not_established=("which pack the player wants to win, and whether either "
                             "pack's asset would look correct layered over the other"),
        )

    if resolution.outcome == "exclusive_supersedes":
        winner = resolution.winner
        return Finding(
            code="contentpatcher.exclusive_supersedes",
            severity="warning",
            subject=", ".join(packs),
            summary=(f"{winner.artifact} loads {target} at Exclusive priority, so the "
                     f"{len(resolution.ignored)} other patch(es) for it are ignored"),
            detail=(f"{cp.GOVERNING_RULE_LOAD} Applied: {winner.location}. Ignored: "
                    f"{_patch_lines(resolution.ignored)}."),
            evidence_class="derived",
            targets=targets,
            sources=[cp.RULE_SOURCE],
            resolutions=[
                {"method": "configuration_change",
                 "step": (f"Nothing outranks Exclusive: SMAPI defines it as int.MaxValue, "
                          f"and a second Exclusive patch does not win, it makes both fail. "
                          f"So if you wanted the other pack's replacement, the options are "
                          f"to remove or disable {winner.artifact}'s competing Load, or for "
                          f"{winner.artifact} to declare an explicit Low/Medium/High "
                          "priority the other patch can outrank. Then check that the "
                          "superseded pack's remaining patches still work against the "
                          "asset that is actually loaded.")},
            ],
            not_established="whether the ignored patches were meant to be overridden",
        )

    if resolution.outcome == "priority_selection":
        winner = resolution.winner
        return Finding(
            code="contentpatcher.priority_selection",
            severity="note",
            subject=", ".join(packs),
            summary=(f"{len(resolution.patches)} patches load {target}; "
                     f"{winner.artifact} wins by declared priority "
                     f"{winner.priority.raw}"),
            detail=(f"{cp.GOVERNING_RULE_LOAD} Applied: {winner.location} "
                    f"(priority {winner.priority.raw}). Not applied: "
                    f"{_patch_lines(resolution.ignored)}. This is the selection "
                    "produced by the declared priorities. It records what the "
                    "declarations resolve to; it does not establish what either "
                    "author intended for cross-mod behaviour, nor that the "
                    "not-applied pack still works."),
            evidence_class="derived",
            targets=targets,
            sources=[cp.RULE_SOURCE],
            not_established=("whether the selected asset is the one the player wants, "
                             "and whether the packs whose Load did not apply still work "
                             "against the asset that did"),
        )

    return Finding(
        code="contentpatcher.competition_unresolved",
        severity="unresolved",
        subject=", ".join(packs),
        summary=(f"{len(resolution.patches)} patches load {target} and which one "
                 "applies cannot be decided from what was supplied"),
        detail=(f"{resolution.unresolved_because}. Competing patches: "
                f"{_patch_lines(resolution.patches)}."),
        evidence_class="unresolved",
        targets=targets,
        conditions=resolution.conditions,
        sources=[cp.RULE_SOURCE],
        not_established="; ".join(resolution.missing_inputs) or "the winning patch",
    )


def _edit_findings(installation: Installation) -> list[Finding]:
    """Edit patches compose; only the same entry in one asset is worth reporting."""
    edits: dict[tuple[str, str], list[str]] = defaultdict(list)
    for artifact in installation.artifacts:
        ins = artifact.inspection
        if ins is None:
            continue
        for change in ins.fact("content_changes") or []:
            action = str(change.get("action") or "").lower()
            if action not in ("editdata", "editimage", "editmap"):
                continue
            for target in change.get("targets") or []:
                for entry in (change.get("entries") or []) + (change.get("fields") or []):
                    edits[(target, str(entry))].append(artifact.name)

    findings = []
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
                   "earlier. Edit order is by Priority (Early/Default/Late), then mod "
                   "load order, then patch order.",
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

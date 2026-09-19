"""Reports: the same evidence, presented to whoever is asking.

A creator's release report and a player's configuration report are built from
the same analyzers over the same knowledge, so an artifact carries its findings
forward: the player does not have to redo the creator's diagnostic work, and
the creator sees what a player will see.

Every report separates what was declared, what was extracted, what our analysis
derived, what was observed in a run, and what is unresolved -- and states what
was not checked. There are no overall scores or pass/fail badges, because
"passed these checks under these conditions" is the only claim the evidence
supports.
"""
from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from .acquire import FetchError, read_cached
from .analyze import collisions, impact as impact_mod, requirements
from .analyze.config import Installation
from .analyze.findings import SEVERITY, Finding, Report
from .inspect import inspect_path
from .store import Store

LOOT_GAMES = {"skyrimse", "fallout4", "falloutnv"}
RIMSORT_SOURCE_ID = "rimsort_community_rules"
SMAPI_SOURCE_ID = "smapi_metadata"


def _loot_findings(installation: Installation, store: Store,
                   report: Report) -> list[Finding]:
    """Apply the LOOT masterlist for the Bethesda games, if we have it cached."""
    if installation.game not in LOOT_GAMES:
        return []
    from .integrations.loot import LootMasterlist, coverage as loot_coverage

    pack = store.pack(installation.game)
    source_id = f"loot_masterlist_{installation.game}"
    source = pack.sources.get(source_id)
    if source is None:
        report.not_checked.append(
            f"LOOT masterlist rules: no source record {source_id!r} in this pack")
        return []
    try:
        raw = read_cached(source)
    except FetchError as exc:
        report.not_checked.append(f"LOOT masterlist rules: {exc}")
        return []

    masterlist = LootMasterlist.cached(raw, game=installation.game, source_id=source_id)
    checked, not_checked = loot_coverage()
    report.checked += checked
    report.not_checked += not_checked
    report.sources_used.append(source_id)
    return masterlist.analyze(installation)


def _rimworld_findings(installation: Installation, store: Store,
                       report: Report) -> list[Finding]:
    """Declared About.xml rules always; the community database when cached."""
    if installation.game != "rimworld":
        return []
    from .integrations import rimworld as rw

    findings = rw.analyze_declared(installation)
    source = store.pack("rimworld").sources.get(RIMSORT_SOURCE_ID)
    with_community = False
    if source is not None:
        try:
            raw = read_cached(source)
        except FetchError as exc:
            report.not_checked.append(f"RimWorld community rules: {exc}")
        else:
            rules = rw.CommunityRules.cached(raw, source_id=RIMSORT_SOURCE_ID)
            findings += rules.analyze(installation)
            report.sources_used.append(RIMSORT_SOURCE_ID)
            with_community = True
    else:
        report.not_checked.append(
            f"RimWorld community rules: no source record {RIMSORT_SOURCE_ID!r}")

    checked, not_checked = rw.coverage(with_community)
    report.checked += checked
    report.not_checked += not_checked
    return findings


def _smapi_findings(installation: Installation, store: Store,
                    report: Report) -> list[Finding]:
    """SMAPI's own per-version compatibility status, when we have it cached."""
    if installation.game != "stardewvalley":
        return []
    from .integrations import smapi as sm

    findings: list[Finding] = []
    available = False
    source = store.pack("stardewvalley").sources.get(SMAPI_SOURCE_ID)
    if source is None:
        report.not_checked.append(
            f"SMAPI compatibility status: no source record {SMAPI_SOURCE_ID!r}")
    else:
        try:
            raw = read_cached(source)
        except FetchError as exc:
            report.not_checked.append(f"SMAPI compatibility status: {exc}")
        else:
            metadata = sm.SmapiMetadata.cached(raw, source_id=SMAPI_SOURCE_ID)
            findings += metadata.analyze(installation)
            report.sources_used.append(SMAPI_SOURCE_ID)
            available = True

    checked, not_checked = sm.coverage(available)
    report.checked += checked
    report.not_checked += not_checked
    return findings


def analyze_installation(installation: Installation, store: Store | None = None) -> Report:
    """The player path: what applies to this exact configuration."""
    store = store or Store()
    report = Report(kind="player_configuration", game=installation.game,
                    subject=installation.as_dict())

    findings = requirements.analyze(installation)
    checked, not_checked = requirements.coverage()
    report.checked += checked
    report.not_checked += not_checked

    findings += _loot_findings(installation, store, report)
    findings += _rimworld_findings(installation, store, report)
    findings += _smapi_findings(installation, store, report)

    findings += collisions.analyze(installation)
    checked, not_checked = collisions.coverage(installation.game)
    report.checked += checked
    report.not_checked += not_checked

    for artifact in installation.artifacts:
        ins = artifact.inspection
        if ins is None:
            continue
        for warning in ins.warnings:
            findings.append(Finding(
                code="inspection.warning",
                severity="warning",
                subject=artifact.name,
                summary=warning,
                evidence_class="extracted",
                not_established="the consequence of this, if any",
            ))
        report.not_checked += [f"{artifact.name}: {item}" for item in ins.not_checked]

    if not installation.files_known_complete:
        findings.append(Finding(
            code="coverage.partial_file_view",
            severity="note",
            summary="this configuration is not a complete file listing, so the absence of "
                    "a file is not evidence that it is not installed",
            evidence_class="unresolved",
            not_established="anything that depends on a file being absent",
        ))

    for finding in findings:
        report.add(finding)
        report.sources_used += finding.sources
    report.sort()
    return report


def analyze_artifact(path: str | Path, game: str | None = None,
                     store: Store | None = None) -> Report:
    """Analyse a single artifact on its own, as a creator would before release."""
    inspection = inspect_path(path, game=game)
    resolved_game = game or inspection.game or "unknown"
    installation = Installation(game=resolved_game, files_known_complete=False)
    from .analyze.config import InstalledArtifact

    installation.artifacts.append(InstalledArtifact(
        name=Path(path).name, path=str(path), sha256=inspection.sha256,
        version=inspection.fact("version"), mod_id=inspection.fact("mod_id"),
        load_index=0, inspection=inspection,
        declared_dependencies=inspection.fact("dependencies") or []))

    report = analyze_installation(installation, store)
    report.kind = "artifact"
    report.subject = {"artifact": inspection.as_dict()}
    return report


def release_report(*, artifact_path: str | Path, game: str, store: Store | None = None,
                   build: dict[str, Any] | None = None,
                   source: dict[str, Any] | None = None,
                   recipes: list[str] | None = None) -> Report:
    """The creator path: one report tying source, build, artifact and checks together.

    The artifact's sha256 is the join key: a player analysing the same bytes
    gets the same findings, plus whatever their own configuration adds.
    """
    report = analyze_artifact(artifact_path, game=game, store=store)
    report.kind = "creator_release"
    inspection = inspect_path(artifact_path, game=game)

    report.subject = {
        "artifact": {
            "path": str(artifact_path),
            "sha256": inspection.sha256,
            "bytes": inspection.bytes,
            "mod_id": inspection.fact("mod_id"),
            "version": inspection.fact("version"),
            "loader": inspection.loader,
        },
        "source": source or {},
        "build": build or {},
        "recipes_applied": recipes or [],
    }
    if build:
        report.checked.append(
            "the artifact was produced by a build that ran here; see subject.build")
        for item in build.get("does_not_establish", []):
            report.not_checked.append(f"build: {item}")
    else:
        report.not_checked.append(
            "no build record: this artifact was not built under ModCheck, so its "
            "relationship to any source tree is unverified")
    report.sort()
    return report


# --- rendering -----------------------------------------------------------
SEVERITY_LABEL = {
    "blocker": "BLOCKER",
    "error": "ERROR",
    "warning": "WARN",
    "note": "note",
    "unresolved": "UNRESOLVED",
}


def render_text(report: Report) -> str:
    lines: list[str] = []
    title = {"player_configuration": "Configuration report",
             "creator_release": "Release report",
             "artifact": "Artifact report"}.get(report.kind, report.kind)
    lines.append(f"{title}  [{report.game}]  {report.generated_at}")

    subject = report.subject or {}
    artifact = subject.get("artifact") or {}
    if artifact.get("sha256"):
        lines.append(f"artifact  {artifact.get('mod_id') or artifact.get('path')} "
                     f"{artifact.get('version') or ''}")
        lines.append(f"sha256    {artifact['sha256']}")
    if subject.get("build"):
        build = subject["build"]
        lines.append(f"build     {'ok' if build.get('ok') else 'FAILED'}  "
                     f"{build.get('toolchain', {})}")
    if subject.get("artifacts"):
        lines.append(f"artifacts {len(subject['artifacts'])}")

    counts = report.counts()
    lines.append("")
    lines.append("findings: " + (", ".join(f"{v} {k}" for k, v in counts.items())
                                 if counts else "none"))

    for severity in SEVERITY:
        group = report.by_severity(severity)
        if not group:
            continue
        lines.append("")
        lines.append(f"-- {SEVERITY_LABEL[severity]} --")
        for finding in group:
            head = f"  [{finding.code}]"
            if finding.subject:
                head += f" {finding.subject}"
            lines.append(head)
            lines.append(f"    {finding.summary}")
            if finding.detail:
                lines.append(f"    {finding.detail}")
            for condition in finding.conditions:
                lines.append(f"    condition: {condition.get('description')}")
            for resolution in finding.resolutions:
                lines.append(f"    resolution ({resolution.get('method')}): "
                             f"{resolution.get('step')}")
            lines.append(f"    evidence: {finding.evidence_class}"
                         + (f"; source {', '.join(finding.sources)}" if finding.sources else ""))
            if finding.not_established:
                lines.append(f"    not established: {finding.not_established}")

    lines.append("")
    lines.append("-- what was checked --")
    for item in dict.fromkeys(report.checked):
        lines.append(f"  + {item}")
    lines.append("")
    lines.append("-- what was NOT checked (a clean result here is not a guarantee) --")
    for item in dict.fromkeys(report.not_checked):
        lines.append(f"  - {item}")
    if report.sources_used:
        lines.append("")
        lines.append("-- sources used --")
        for item in sorted(set(report.sources_used)):
            lines.append(f"  {item}")
    return "\n".join(lines)


def render_json(report: Report) -> str:
    return json.dumps(report.as_dict(), indent=2, default=str)


def impact_of(before: Installation, after: Installation,
              store: Store | None = None) -> impact_mod.Impact:
    """What changes between two configurations, as a diff of findings."""
    store = store or Store()
    before_report = analyze_installation(before, store)
    after_report = analyze_installation(after, store)
    return impact_mod.compare(before, after, before_report.findings, after_report.findings)

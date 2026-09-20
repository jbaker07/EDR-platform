"""Evaluation: does the analysis actually discriminate?

A checker that flags every overlap is worthless, so the suite measures both
directions. Each case is either a **positive** (a finding must be produced) or a
**control** (the right answer is silence), and both count.

Two honesty constraints shape this:

* Cases live apart from the knowledge base and are not used to build detection.
* Applying a rule that upstream already recorded is not the same as predicting a
  failure. A case measures whether ModCheck applied the rule correctly to a
  configuration, and the report says so rather than implying foresight.

Metrics that need real usage -- human correction rate, time to a correct
resolution, creator task completion -- are not invented here. They are listed
as not yet measured.
"""
from __future__ import annotations

import dataclasses
import sys
import time
from pathlib import Path
from typing import Any

from . import yamlio
from .analyze.config import Installation, InstalledArtifact
from .paths import project_root
from .report import analyze_installation
from .store import Store


@dataclasses.dataclass
class Case:
    id: str
    game: str
    kind: str  # "positive" | "control" | "witness"
    description: str
    configuration: dict[str, Any]
    expect_findings: list[str] = dataclasses.field(default_factory=list)
    expect_absent: list[str] = dataclasses.field(default_factory=list)
    max_errors: int | None = None
    measures: str = ""
    path: Path | None = None
    # A witness proves a specific `detectable: yes` failure record is really
    # detected: it names the failure, and asserts not just that a code was
    # emitted but that it was emitted about the right artifact and target.
    witness_for: str | None = None
    expect_subject: str | None = None
    expect_target: str | None = None
    expect_summary_contains: str | None = None
    # Artifacts a case needs built before it can run. Cases carrying these can
    # only run where a builder registry is supplied; elsewhere they are SKIPPED,
    # and a skipped witness never counts as a pass.
    artifacts: list[dict[str, Any]] = dataclasses.field(default_factory=list)

    @classmethod
    def from_dict(cls, data: dict[str, Any], path: Path | None = None) -> "Case":
        expect = data.get("expect") or {}
        return cls(
            id=data["id"], game=data["game"], kind=data.get("kind", "positive"),
            description=data.get("description", ""),
            configuration=data.get("configuration") or {},
            expect_findings=list(expect.get("findings") or []),
            expect_absent=list(expect.get("absent") or []),
            max_errors=expect.get("max_errors"),
            measures=data.get("measures", ""),
            path=path,
            witness_for=data.get("witness_for"),
            expect_subject=expect.get("subject"),
            expect_target=expect.get("target"),
            expect_summary_contains=expect.get("summary_contains"),
            artifacts=list(data.get("artifacts") or []))


@dataclasses.dataclass
class CaseResult:
    case: Case
    passed: bool
    produced: list[str]
    missed: list[str]
    unexpected_present: list[str]
    error_count: int
    unresolved_count: int
    seconds: float
    notes: list[str] = dataclasses.field(default_factory=list)
    skipped: bool = False


def load_cases(directory: Path) -> list[Case]:
    cases = []
    for path in sorted(Path(directory).glob("*.yaml")):
        data = yamlio.load_path(path)
        for entry in (data if isinstance(data, list) else [data]):
            cases.append(Case.from_dict(entry, path))
    return cases


def load_builders() -> tuple[dict[str, Any] | None, str | None]:
    """Load the artifact builders the evaluation cases name.

    The gate is required to build the inputs its cases declare, or to say it
    could not. Returning ``(None, reason)`` is the second of those: the caller
    must report the run blocked rather than let the affected cases quietly skip
    and still exit zero.
    """
    root = project_root()
    candidate = root / "tests"
    if not (candidate / "fixtures" / "build.py").exists():
        return None, (f"artifact builders not found: {candidate / 'fixtures' / 'build.py'} "
                      "does not exist, so cases needing built artifacts cannot run")
    if str(candidate) not in sys.path:
        sys.path.insert(0, str(candidate))
    try:
        from fixtures import build  # type: ignore[import-not-found]
    except Exception as exc:  # pragma: no cover - environment-dependent
        return None, f"artifact builders could not be imported: {exc!r}"
    registry = getattr(build, "BUILDERS", None)
    if not registry:
        return None, "artifact builders module defines no BUILDERS registry"
    return dict(registry), None


def _build_artifacts(case: Case, builders, workdir: Path) -> list[str]:
    """Materialise the artifacts a case declares, using the supplied builders."""
    paths = []
    for spec in case.artifacts:
        builder = builders[spec["builder"]]
        target = workdir / spec["name"]
        builder(target, **(spec.get("args") or {}))
        paths.append(str(target))
    return paths


def _installation(case: Case) -> Installation:
    data = case.configuration
    inst = Installation(
        game=case.game,
        game_version=data.get("game_version"),
        loader=data.get("loader"),
        loader_versions=data.get("loader_versions", {}),
        files=set(data.get("files", [])),  # Installation normalises these
        files_known_complete=data.get("files_known_complete", False),
    )
    for index, raw in enumerate(data.get("artifacts", [])):
        crc = raw.get("crc32")
        if isinstance(crc, str):
            crc = int(crc, 16)
        inst.artifacts.append(InstalledArtifact(
            name=raw["name"], mod_id=raw.get("mod_id"), version=raw.get("version"),
            crc32=crc, sha256=raw.get("sha256"), active=raw.get("active", True),
            load_index=raw.get("load_index", index),
            declared_dependencies=raw.get("dependencies", [])))
    return inst


def run_case(case: Case, store: Store, builders=None,
             workdir: Path | None = None) -> CaseResult:
    if case.artifacts and builders is None:
        # Not runnable here. Reported as skipped and counted separately: a
        # witness that did not run has established nothing.
        return CaseResult(case=case, passed=False, produced=[], missed=[],
                          unexpected_present=[], error_count=0, unresolved_count=0,
                          seconds=0.0, skipped=True,
                          notes=["needs built artifacts; no builder registry supplied"])

    if case.artifacts:
        paths = _build_artifacts(case, builders, workdir)
        installation = Installation.from_paths(
            case.game, paths,
            game_version=case.configuration.get("game_version"),
            files_known_complete=case.configuration.get("files_known_complete", True))
        for key, value in (case.configuration.get("loader_versions") or {}).items():
            installation.loader_versions[key] = value
    else:
        installation = _installation(case)

    begin = time.perf_counter()
    report = analyze_installation(installation, store, deep=case.configuration.get("deep", False))
    seconds = time.perf_counter() - begin

    produced = sorted({f.code for f in report.findings})
    missed = [code for code in case.expect_findings if code not in produced]
    unexpected = [code for code in case.expect_absent if code in produced]
    errors = len(report.by_severity("error")) + len(report.by_severity("blocker"))
    unresolved = len(report.by_severity("unresolved"))

    passed = not missed and not unexpected
    notes = []

    # A witness must show the finding was about the right thing, not merely that
    # the code appeared somewhere in the report.
    if case.witness_for:
        target_code = case.expect_findings[0] if case.expect_findings else None
        matching = [f for f in report.findings if f.code == target_code]
        if not matching:
            passed = False
            notes.append(f"witness for {case.witness_for}: {target_code} was not emitted")
        else:
            if case.expect_subject and not any(
                    case.expect_subject in (f.subject or "") for f in matching):
                passed = False
                notes.append(
                    f"witness for {case.witness_for}: {target_code} was emitted but not "
                    f"about {case.expect_subject!r} (subjects: "
                    f"{[f.subject for f in matching]})")
            if case.expect_target and not any(
                    any(case.expect_target in str(t.get("id", "")) for t in f.targets)
                    for f in matching):
                passed = False
                notes.append(
                    f"witness for {case.witness_for}: no finding targets "
                    f"{case.expect_target!r}")
            if case.expect_summary_contains and not any(
                    case.expect_summary_contains in f.summary for f in matching):
                passed = False
                notes.append(
                    f"witness for {case.witness_for}: no finding explains "
                    f"{case.expect_summary_contains!r}")
    if case.max_errors is not None and errors > case.max_errors:
        passed = False
        notes.append(f"{errors} error-level findings, expected at most {case.max_errors}")
    if case.kind == "control" and errors:
        codes = sorted({f.code for f in report.findings
                        if f.severity in ("error", "blocker")})
        notes.append(f"control produced error-level findings: {codes}")

    return CaseResult(case=case, passed=passed, produced=produced, missed=missed,
                      unexpected_present=unexpected, error_count=errors,
                      unresolved_count=unresolved, seconds=seconds, notes=notes)


def run(directory: Path, store: Store | None = None, game: str | None = None,
        builders=None, workdir: Path | None = None) -> list[CaseResult]:
    store = store or Store()
    cases = [c for c in load_cases(directory) if game is None or c.game == game]
    return [run_case(case, store, builders=builders, workdir=workdir) for case in cases]


def gate_verdict(summary: dict[str, Any], blocked_reason: str | None = None) -> str:
    """The single pass/fail/blocked answer the evaluation gate reports.

    A skipped case establishes nothing, so it can never contribute to a pass.
    Where cases were skipped -- or the builders could not be loaded at all --
    the gate reports itself ``blocked``: not a pass with an asterisk, and not a
    failure of the analysis either, but a run that did not measure what it was
    supposed to measure.
    """
    if blocked_reason:
        return "blocked"
    if summary["failed"]:
        return "failed"
    if summary["skipped"]:
        return "blocked"
    return "passed"


def summarize(results: list[CaseResult]) -> dict[str, Any]:
    positives = [r for r in results if r.case.kind == "positive" and not r.skipped]
    controls = [r for r in results if r.case.kind == "control" and not r.skipped]
    witnesses = [r for r in results if r.case.witness_for]
    expected_total = sum(len(r.case.expect_findings) for r in positives)
    missed_total = sum(len(r.missed) for r in positives)
    false_warnings = sum(len(r.unexpected_present) for r in results)
    control_errors = sum(r.error_count for r in controls)

    return {
        "cases": len(results),
        "passed": sum(1 for r in results if r.passed),
        "failed": sum(1 for r in results if not r.passed and not r.skipped),
        "skipped": sum(1 for r in results if r.skipped),
        "witness_cases": len(witnesses),
        "witnesses_passed": sum(1 for r in witnesses if r.passed),
        "witnesses_skipped": sum(1 for r in witnesses if r.skipped),
        "positive_cases": len(positives),
        "control_cases": len(controls),
        "expected_findings": expected_total,
        "expected_findings_detected": expected_total - missed_total,
        "missed_findings": missed_total,
        "false_warnings": false_warnings,
        "error_findings_on_controls": control_errors,
        "unresolved_findings": sum(r.unresolved_count for r in results),
        "skipped_case_ids": [r.case.id for r in results if r.skipped],
        "total_seconds": round(sum(r.seconds for r in results), 4),
        "slowest_case_seconds": round(max((r.seconds for r in results), default=0), 4),
    }


NOT_YET_MEASURED = [
    "creator tasks completed without human correction -- needs real creator sessions",
    "human correction rate on generated changes -- needs real review data",
    "time to a correct resolution -- needs real user sessions",
    "edit-to-feedback latency in an editor -- no editor integration exists",
    "cost of expensive operations -- only builds are expensive, and they are "
    "measured per build rather than aggregated here",
]


def render(results: list[CaseResult], summary: dict[str, Any],
           blocked_reason: str | None = None) -> str:
    verdict = gate_verdict(summary, blocked_reason)
    lines = [f"Evaluation -- gate {verdict.upper()}", ""]
    if blocked_reason:
        lines.append(f"  gate could not build its inputs: {blocked_reason}")
        lines.append("")
    for result in results:
        mark = "SKIP" if result.skipped else ("PASS" if result.passed else "FAIL")
        lines.append(f"[{mark}] {result.case.kind:8} {result.case.id}")
        lines.append(f"         {result.case.description}")
        if result.missed:
            lines.append(f"         MISSED: {result.missed}")
        if result.unexpected_present:
            lines.append(f"         FALSE WARNING: {result.unexpected_present}")
        for note in result.notes:
            lines.append(f"         {note}")
    lines.append("")
    lines.append("-- summary --")
    lines.append(f"  {'gate':32} {verdict}")
    for key, value in summary.items():
        lines.append(f"  {key:32} {value}")
    lines.append("")
    lines.append("-- what these numbers mean --")
    lines.append("  Detection here means ModCheck correctly applied a rule to a")
    lines.append("  configuration. Where the rule comes from an upstream database, this")
    lines.append("  is not evidence that ModCheck would have predicted the failure")
    lines.append("  before that rule existed.")
    lines.append("")
    lines.append("-- skipped cases establish nothing --")
    skipped = [r for r in results if r.skipped]
    if skipped:
        for result in skipped:
            lines.append(f"  {result.case.id}: {'; '.join(result.notes)}")
        lines.append("  The gate is blocked, not passed: a case that did not run")
        lines.append("  has neither detected nor failed to detect anything.")
    else:
        lines.append("  none skipped in this run")
    lines.append("")
    lines.append("-- not yet measured --")
    for item in NOT_YET_MEASURED:
        lines.append(f"  - {item}")
    return "\n".join(lines)

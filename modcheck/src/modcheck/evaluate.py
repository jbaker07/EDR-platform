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
import time
from pathlib import Path
from typing import Any

from . import yamlio
from .analyze.config import Installation, InstalledArtifact
from .report import analyze_installation
from .store import Store


@dataclasses.dataclass
class Case:
    id: str
    game: str
    kind: str  # "positive" | "control"
    description: str
    configuration: dict[str, Any]
    expect_findings: list[str] = dataclasses.field(default_factory=list)
    expect_absent: list[str] = dataclasses.field(default_factory=list)
    max_errors: int | None = None
    measures: str = ""
    path: Path | None = None

    @classmethod
    def from_dict(cls, data: dict[str, Any], path: Path | None = None) -> "Case":
        expect = data.get("expect") or {}
        return cls(
            id=data["id"], game=data["game"], kind=data.get("kind", "positive"),
            description=data.get("description", ""),
            configuration=data["configuration"],
            expect_findings=list(expect.get("findings") or []),
            expect_absent=list(expect.get("absent") or []),
            max_errors=expect.get("max_errors"),
            measures=data.get("measures", ""),
            path=path)


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


def load_cases(directory: Path) -> list[Case]:
    cases = []
    for path in sorted(Path(directory).glob("*.yaml")):
        data = yamlio.load_path(path)
        for entry in (data if isinstance(data, list) else [data]):
            cases.append(Case.from_dict(entry, path))
    return cases


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


def run_case(case: Case, store: Store) -> CaseResult:
    installation = _installation(case)
    begin = time.perf_counter()
    report = analyze_installation(installation, store)
    seconds = time.perf_counter() - begin

    produced = sorted({f.code for f in report.findings})
    missed = [code for code in case.expect_findings if code not in produced]
    unexpected = [code for code in case.expect_absent if code in produced]
    errors = len(report.by_severity("error")) + len(report.by_severity("blocker"))
    unresolved = len(report.by_severity("unresolved"))

    passed = not missed and not unexpected
    notes = []
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


def run(directory: Path, store: Store | None = None,
        game: str | None = None) -> list[CaseResult]:
    store = store or Store()
    cases = [c for c in load_cases(directory) if game is None or c.game == game]
    return [run_case(case, store) for case in cases]


def summarize(results: list[CaseResult]) -> dict[str, Any]:
    positives = [r for r in results if r.case.kind == "positive"]
    controls = [r for r in results if r.case.kind == "control"]
    expected_total = sum(len(r.case.expect_findings) for r in positives)
    missed_total = sum(len(r.missed) for r in positives)
    false_warnings = sum(len(r.unexpected_present) for r in results)
    control_errors = sum(r.error_count for r in controls)

    return {
        "cases": len(results),
        "passed": sum(1 for r in results if r.passed),
        "failed": sum(1 for r in results if not r.passed),
        "positive_cases": len(positives),
        "control_cases": len(controls),
        "expected_findings": expected_total,
        "expected_findings_detected": expected_total - missed_total,
        "missed_findings": missed_total,
        "false_warnings": false_warnings,
        "error_findings_on_controls": control_errors,
        "unresolved_findings": sum(r.unresolved_count for r in results),
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


def render(results: list[CaseResult], summary: dict[str, Any]) -> str:
    lines = ["Evaluation", ""]
    for result in results:
        mark = "PASS" if result.passed else "FAIL"
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
    for key, value in summary.items():
        lines.append(f"  {key:32} {value}")
    lines.append("")
    lines.append("-- what these numbers mean --")
    lines.append("  Detection here means ModCheck correctly applied a rule to a")
    lines.append("  configuration. Where the rule comes from an upstream database, this")
    lines.append("  is not evidence that ModCheck would have predicted the failure")
    lines.append("  before that rule existed.")
    lines.append("")
    lines.append("-- not yet measured --")
    for item in NOT_YET_MEASURED:
        lines.append(f"  - {item}")
    return "\n".join(lines)

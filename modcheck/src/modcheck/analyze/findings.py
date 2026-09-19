"""Findings and reports.

A finding always says three things: what was observed, how strongly it is
evidenced, and what was *not* established. There are no numeric confidence
scores -- an invented percentage would imply a calibration we do not have.
"""
from __future__ import annotations

import dataclasses
import datetime as dt
from typing import Any

# Ordered from strongest to weakest claim.
SEVERITY = ("blocker", "error", "warning", "note", "unresolved")

EVIDENCE_CLASSES = ("declared", "extracted", "derived", "observed", "unresolved")


@dataclasses.dataclass
class Finding:
    code: str
    severity: str
    summary: str
    evidence_class: str
    subject: str | None = None
    detail: str = ""
    targets: list[dict] = dataclasses.field(default_factory=list)
    conditions: list[dict] = dataclasses.field(default_factory=list)
    sources: list[str] = dataclasses.field(default_factory=list)
    resolutions: list[dict] = dataclasses.field(default_factory=list)
    not_established: str = ""

    def as_dict(self) -> dict[str, Any]:
        return {k: v for k, v in dataclasses.asdict(self).items() if v not in (None, "", [], {})}


@dataclasses.dataclass
class Report:
    """A set of findings plus an explicit statement of what was analysed."""

    kind: str  # "player_configuration" | "creator_release"
    game: str
    subject: dict[str, Any] = dataclasses.field(default_factory=dict)
    findings: list[Finding] = dataclasses.field(default_factory=list)
    checked: list[str] = dataclasses.field(default_factory=list)
    not_checked: list[str] = dataclasses.field(default_factory=list)
    sources_used: list[str] = dataclasses.field(default_factory=list)
    generated_at: str = dataclasses.field(
        default_factory=lambda: dt.datetime.now(dt.timezone.utc).replace(
            microsecond=0).isoformat())

    def add(self, finding: Finding) -> None:
        self.findings.append(finding)

    def by_severity(self, severity: str) -> list[Finding]:
        return [f for f in self.findings if f.severity == severity]

    def by_evidence(self, evidence_class: str) -> list[Finding]:
        return [f for f in self.findings if f.evidence_class == evidence_class]

    def counts(self) -> dict[str, int]:
        return {s: len(self.by_severity(s)) for s in SEVERITY if self.by_severity(s)}

    def as_dict(self) -> dict[str, Any]:
        return {
            "kind": self.kind,
            "game": self.game,
            "generated_at": self.generated_at,
            "subject": self.subject,
            "counts": self.counts(),
            "findings": [f.as_dict() for f in self.findings],
            "coverage": {"checked": self.checked, "not_checked": self.not_checked},
            "sources_used": sorted(set(self.sources_used)),
        }

    def sort(self) -> None:
        order = {s: i for i, s in enumerate(SEVERITY)}
        self.findings.sort(key=lambda f: (order.get(f.severity, 99), f.code, f.subject or ""))

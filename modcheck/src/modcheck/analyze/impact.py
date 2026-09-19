"""What would change.

Both audiences need this. A player asks "what happens if I install this, or
update that?". A creator asks "what does my new release do to people who have
the old one?".

The answer is a diff of findings between two configurations, in three parts:
findings the change *introduces*, findings it *resolves*, and findings it leaves
alone. The third part matters: a change that resolves nothing and introduces
nothing should say so plainly rather than producing a wall of unchanged text.

The important caveat is stated in every impact report: this compares what
ModCheck can see. A change can alter behaviour without altering any finding,
and an unchanged finding list is not evidence that nothing changed.
"""
from __future__ import annotations

import dataclasses
from pathlib import Path
from typing import Any

from .config import Installation, InstalledArtifact
from .findings import Finding


def _key(finding: Finding) -> tuple:
    """Identity of a finding for diffing: code, subject and targets."""
    targets = tuple(sorted(str(t.get("id", "")) for t in finding.targets))
    return (finding.code, finding.subject or "", targets, finding.summary)


@dataclasses.dataclass
class Impact:
    before_count: int
    after_count: int
    introduced: list[Finding]
    resolved: list[Finding]
    unchanged: list[Finding]
    artifacts_added: list[str]
    artifacts_removed: list[str]
    artifacts_changed: list[dict[str, Any]]
    caveats: list[str] = dataclasses.field(default_factory=list)

    @property
    def is_neutral(self) -> bool:
        return not self.introduced and not self.resolved

    def as_dict(self) -> dict[str, Any]:
        return {
            "artifacts": {
                "added": self.artifacts_added,
                "removed": self.artifacts_removed,
                "changed": self.artifacts_changed,
            },
            "findings": {
                "before": self.before_count,
                "after": self.after_count,
                "introduced": [f.as_dict() for f in self.introduced],
                "resolved": [f.as_dict() for f in self.resolved],
                "unchanged_count": len(self.unchanged),
            },
            "caveats": self.caveats,
        }


CAVEATS = [
    "This compares what ModCheck can see. A change can alter behaviour without "
    "altering any finding, so an unchanged finding list is not evidence that "
    "nothing changed.",
    "Findings that disappear because an artifact was removed are listed as "
    "resolved; that does not mean the underlying problem was fixed.",
]


def _artifact_index(installation: Installation) -> dict[str, InstalledArtifact]:
    return {(a.mod_id or a.name).lower(): a for a in installation.artifacts}


def compare(before: Installation, after: Installation,
            before_findings: list[Finding], after_findings: list[Finding]) -> Impact:
    before_map = {_key(f): f for f in before_findings}
    after_map = {_key(f): f for f in after_findings}

    introduced = [f for k, f in after_map.items() if k not in before_map]
    resolved = [f for k, f in before_map.items() if k not in after_map]
    unchanged = [f for k, f in after_map.items() if k in before_map]

    old, new = _artifact_index(before), _artifact_index(after)
    changed = []
    for key in sorted(set(old) & set(new)):
        a, b = old[key], new[key]
        if a.sha256 and b.sha256 and a.sha256 != b.sha256:
            changed.append({"id": key, "from_version": a.version, "to_version": b.version,
                            "from_sha256": a.sha256, "to_sha256": b.sha256})
        elif a.version != b.version:
            changed.append({"id": key, "from_version": a.version, "to_version": b.version,
                            "note": "version changed; artifact hashes were not both known, "
                                    "so this is a declared change, not a verified one"})

    caveats = list(CAVEATS)
    if any("sha256" not in c for c in changed):
        caveats.append(
            "Some artifacts were compared by version string only. Matching version "
            "strings do not establish matching artifacts.")

    return Impact(
        before_count=len(before_findings),
        after_count=len(after_findings),
        introduced=sorted(introduced, key=lambda f: (f.severity, f.code)),
        resolved=sorted(resolved, key=lambda f: (f.severity, f.code)),
        unchanged=unchanged,
        artifacts_added=sorted(set(new) - set(old)),
        artifacts_removed=sorted(set(old) - set(new)),
        artifacts_changed=changed,
        caveats=caveats,
    )


def with_artifact(installation: Installation, path: str | Path,
                  game: str | None = None) -> Installation:
    """A copy of this configuration with one more artifact installed."""
    from ..inspect import inspect_path

    inspection = inspect_path(path, game=game or installation.game)
    extended = dataclasses.replace(
        installation,
        artifacts=list(installation.artifacts),
        files=set(installation.files),
        notes=list(installation.notes),
    )
    extended.artifacts.append(InstalledArtifact(
        name=Path(path).name, path=str(path), sha256=inspection.sha256,
        version=inspection.fact("version"), mod_id=inspection.fact("mod_id"),
        load_index=len(installation.artifacts), inspection=inspection,
        declared_dependencies=inspection.fact("dependencies") or []))
    extended.add_file(Path(path).name)
    return extended


def render(impact: Impact) -> str:
    lines = ["Impact", ""]
    if impact.artifacts_added:
        lines.append(f"  added:    {', '.join(impact.artifacts_added)}")
    if impact.artifacts_removed:
        lines.append(f"  removed:  {', '.join(impact.artifacts_removed)}")
    for change in impact.artifacts_changed:
        lines.append(f"  changed:  {change['id']} "
                     f"{change.get('from_version')} -> {change.get('to_version')}")
        if "note" in change:
            lines.append(f"            {change['note']}")
    lines.append("")
    lines.append(f"  findings before: {impact.before_count}    "
                 f"after: {impact.after_count}")

    if impact.is_neutral:
        lines.append("")
        lines.append("  This change introduces no new findings and resolves none.")
        lines.append("  That is not the same as changing nothing.")
    for title, group in (("introduces", impact.introduced), ("resolves", impact.resolved)):
        if not group:
            continue
        lines.append("")
        lines.append(f"  -- {title} --")
        for finding in group:
            lines.append(f"    [{finding.severity}] {finding.code} "
                         f"{finding.subject or ''}")
            lines.append(f"      {finding.summary}")
    lines.append("")
    lines.append("  -- caveats --")
    for caveat in impact.caveats:
        lines.append(f"    - {caveat}")
    return "\n".join(lines)

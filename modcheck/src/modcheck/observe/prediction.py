"""Predictions recorded before the game runs.

The failure mode this module exists to prevent: run the game, read what
happened, then write an explanation that fits, and call the fit a prediction.
That reads as predictive capability and is not.

So a prediction is a file, committed before the observation, carrying:

* the exact bytes it is about (path, size, sha256) -- a prediction about a
  content pack whose bytes we cannot name is a prediction about nothing;
* the exact commands whose output will judge it, so the observer cannot go
  looking for a kinder question;
* falsifiable claims -- each one is a statement the transcript can contradict;
* an ``observation`` block that starts at ``not_yet_observed`` and is only ever
  filled in by a later commit.

``compare`` is deliberately blunt. It does not score, weight, or partially
credit. Each claim is ``matched``, ``contradicted``, or ``unobserved`` -- that
last one where the transcript simply does not speak to it, which is not a pass.
"""
from __future__ import annotations

import dataclasses
import hashlib
from pathlib import Path
from typing import Any

from .. import yamlio
from .contentpatcher import DumpApplied, DumpOrder, ObservedPatch, Summary

# What a claim asserts. Each maps to one question the transcript can answer.
CLAIM_KINDS = frozenset({
    "patch_applied",       # a named patch is/is not applied
    "patch_loaded",        # a named patch is/is not loaded
    "conditions_match",    # a named patch's conditions do/do not match
    "patch_priority",      # a named patch displays this priority label
    "reason_not_loaded",   # the reported reason contains this text
    "winner_for_target",   # exactly this patch is applied against a target
    "no_patch_applied",    # nothing is applied against a target
    "apply_order",         # these patch fragments appear in this order
    "current_change",      # `Current changes:` lists this target
})

OBSERVATION_STATES = frozenset({"not_yet_observed", "blocked", "observed"})


@dataclasses.dataclass
class Claim:
    kind: str
    # The patch this claim is about, matched as a substring of the name Content
    # Patcher prints. Unused by target-scoped kinds.
    patch: str | None = None
    target: str | None = None
    expect: Any = None
    order: list[str] = dataclasses.field(default_factory=list)
    because: str = ""

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> "Claim":
        kind = data["kind"]
        if kind not in CLAIM_KINDS:
            raise ValueError(f"unknown claim kind {kind!r}; known: {sorted(CLAIM_KINDS)}")
        return cls(kind=kind, patch=data.get("patch"), target=data.get("target"),
                   expect=data.get("expect"), order=list(data.get("order") or []),
                   because=data.get("because", ""))

    def describe(self) -> str:
        subject = self.patch or self.target or "?"
        if self.kind == "apply_order":
            return f"apply order for {self.target}: {' then '.join(self.order)}"
        if self.kind == "no_patch_applied":
            return f"no patch applied against {self.target}"
        if self.kind == "winner_for_target":
            return f"{self.expect} is the only patch applied against {self.target}"
        return f"{subject}: {self.kind} == {self.expect!r}"


@dataclasses.dataclass
class InputFile:
    path: str
    sha256: str
    bytes: int
    note: str = ""

    def verify(self, root: Path) -> str | None:
        """Re-hash the file this prediction claims to be about."""
        actual = root / self.path
        if not actual.exists():
            return f"{self.path}: not present"
        raw = actual.read_bytes()
        digest = hashlib.sha256(raw).hexdigest()
        if digest != self.sha256:
            return (f"{self.path}: sha256 {digest} does not match the recorded "
                    f"{self.sha256} -- the prediction is about different bytes")
        if len(raw) != self.bytes:
            return f"{self.path}: {len(raw)} bytes, recorded {self.bytes}"
        return None


@dataclasses.dataclass
class Prediction:
    id: str
    case: str
    game: str
    question: str
    recorded_at: str
    inputs: list[InputFile] = dataclasses.field(default_factory=list)
    commands: list[str] = dataclasses.field(default_factory=list)
    claims: list[Claim] = dataclasses.field(default_factory=list)
    derived_from: str = ""
    requires: dict[str, Any] = dataclasses.field(default_factory=dict)
    observation_status: str = "not_yet_observed"
    observation_blocked_reason: str = ""
    observation_transcript: str = ""
    path: Path | None = None

    @classmethod
    def from_dict(cls, data: dict[str, Any], path: Path | None = None) -> "Prediction":
        observation = data.get("observation") or {}
        status = observation.get("status", "not_yet_observed")
        if status not in OBSERVATION_STATES:
            raise ValueError(f"{data['id']}: unknown observation status {status!r}")
        return cls(
            id=data["id"], case=data["case"], game=data["game"],
            question=data["question"], recorded_at=str(data["recorded_at"]),
            inputs=[InputFile(**i) for i in (data.get("inputs") or [])],
            commands=list(data.get("commands") or []),
            claims=[Claim.from_dict(c) for c in (data.get("claims") or [])],
            derived_from=data.get("derived_from", ""),
            requires=dict(data.get("requires") or {}),
            observation_status=status,
            observation_blocked_reason=observation.get("blocked_reason", ""),
            observation_transcript=observation.get("transcript", ""),
            path=path)

    @property
    def observed(self) -> bool:
        return self.observation_status == "observed"


def load_predictions(directory: Path) -> list[Prediction]:
    out = []
    for path in sorted(Path(directory).glob("*.yaml")):
        data = yamlio.load_path(path)
        for entry in (data if isinstance(data, list) else [data]):
            out.append(Prediction.from_dict(entry, path))
    return out


@dataclasses.dataclass
class ClaimResult:
    claim: Claim
    verdict: str  # "matched" | "contradicted" | "unobserved"
    observed: str = ""

    @property
    def matched(self) -> bool:
        return self.verdict == "matched"


@dataclasses.dataclass
class Comparison:
    prediction: Prediction
    results: list[ClaimResult] = dataclasses.field(default_factory=list)

    @property
    def verdict(self) -> str:
        if any(r.verdict == "contradicted" for r in self.results):
            return "contradicted"
        if any(r.verdict == "unobserved" for r in self.results):
            return "incomplete"
        return "matched" if self.results else "empty"


def _find(summary: Summary, fragment: str) -> list[ObservedPatch]:
    needle = fragment.lower()
    return [p for p in summary.patches
            if needle in p.name.lower() or needle in p.location.lower()]


def _bool_claim(result_field: str, claim: Claim, summary: Summary) -> ClaimResult:
    matches = _find(summary, claim.patch or "")
    if not matches:
        return ClaimResult(claim, "unobserved",
                           f"no patch matching {claim.patch!r} appears in the summary")
    actual = [getattr(p, result_field) for p in matches]
    if all(a is bool(claim.expect) for a in actual):
        return ClaimResult(claim, "matched", f"{result_field}={actual}")
    return ClaimResult(claim, "contradicted",
                       f"{result_field}={actual} for "
                       f"{[p.name for p in matches]}")


def compare(prediction: Prediction, summary: Summary | None = None,
            applied: DumpApplied | None = None,
            order: DumpOrder | None = None) -> Comparison:
    """Judge a prediction against what the transcripts actually say.

    A claim the supplied transcripts cannot speak to is ``unobserved``, never
    ``matched``. Running fewer commands than the prediction names therefore
    cannot turn into a better result.
    """
    comparison = Comparison(prediction=prediction)
    for claim in prediction.claims:
        comparison.results.append(_compare_one(claim, summary, applied, order))
    return comparison


def _compare_one(claim: Claim, summary: Summary | None,
                 applied: DumpApplied | None, order: DumpOrder | None) -> ClaimResult:
    needs_summary = claim.kind in {
        "patch_applied", "patch_loaded", "conditions_match", "patch_priority",
        "reason_not_loaded", "winner_for_target", "no_patch_applied", "current_change"}
    if needs_summary and summary is None:
        return ClaimResult(claim, "unobserved", "no `patch summary` transcript supplied")
    if claim.kind == "apply_order" and applied is None and order is None:
        return ClaimResult(claim, "unobserved", "no `patch dump` transcript supplied")

    if claim.kind == "patch_applied":
        return _bool_claim("applied", claim, summary)  # type: ignore[arg-type]
    if claim.kind == "patch_loaded":
        return _bool_claim("loaded", claim, summary)  # type: ignore[arg-type]
    if claim.kind == "conditions_match":
        return _bool_claim("conditions_match", claim, summary)  # type: ignore[arg-type]

    if claim.kind == "patch_priority":
        matches = _find(summary, claim.patch or "")  # type: ignore[arg-type]
        if not matches:
            return ClaimResult(claim, "unobserved", f"no patch matching {claim.patch!r}")
        actual = [p.priority for p in matches]
        if all(a == claim.expect for a in actual):
            return ClaimResult(claim, "matched", f"priority={actual}")
        return ClaimResult(claim, "contradicted", f"priority={actual}")

    if claim.kind == "reason_not_loaded":
        matches = _find(summary, claim.patch or "")  # type: ignore[arg-type]
        if not matches:
            return ClaimResult(claim, "unobserved", f"no patch matching {claim.patch!r}")
        reasons = [p.reason_not_loaded for p in matches]
        if any(r and str(claim.expect).lower() in r.lower() for r in reasons):
            return ClaimResult(claim, "matched", f"reason={reasons}")
        return ClaimResult(claim, "contradicted", f"reason={reasons}")

    if claim.kind == "winner_for_target":
        hits = summary.applied_to(claim.target or "")  # type: ignore[union-attr]
        names = [p.location for p in hits]
        if len(hits) == 1 and str(claim.expect).lower() in hits[0].location.lower():
            return ClaimResult(claim, "matched", f"applied={names}")
        return ClaimResult(claim, "contradicted", f"applied={names}")

    if claim.kind == "no_patch_applied":
        hits = summary.applied_to(claim.target or "")  # type: ignore[union-attr]
        if not hits:
            return ClaimResult(claim, "matched", "no applied patch against this target")
        return ClaimResult(claim, "contradicted",
                           f"applied={[p.location for p in hits]}")

    if claim.kind == "current_change":
        for mod in summary.mods:  # type: ignore[union-attr]
            for target, labels in mod.current_changes.items():
                if (claim.target or "").lower() in target.lower():
                    if not claim.expect or str(claim.expect).lower() in "; ".join(labels).lower():
                        return ClaimResult(claim, "matched", f"{target} | {'; '.join(labels)}")
                    return ClaimResult(claim, "contradicted", f"{target} | {'; '.join(labels)}")
        return ClaimResult(claim, "contradicted",
                           f"no current change listed for {claim.target!r}")

    if claim.kind == "apply_order":
        sequence = applied.order_for(claim.target or "") if applied else []
        source = "patch dump applied"
        if not sequence and order is not None:
            sequence = [row[2] for row in order.rows]
            source = "patch dump order"
        if not sequence:
            return ClaimResult(claim, "unobserved", f"no order reported for {claim.target!r}")
        positions = []
        for fragment in claim.order:
            found = next((i for i, name in enumerate(sequence)
                          if fragment.lower() in name.lower()), None)
            if found is None:
                return ClaimResult(claim, "contradicted",
                                   f"{fragment!r} does not appear in {source}: {sequence}")
            positions.append(found)
        if positions == sorted(positions):
            return ClaimResult(claim, "matched", f"{source}: {sequence}")
        return ClaimResult(claim, "contradicted", f"{source}: {sequence}")

    raise AssertionError(f"unhandled claim kind {claim.kind!r}")

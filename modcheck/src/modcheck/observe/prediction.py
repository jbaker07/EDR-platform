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
from .contentpatcher import normalise_asset as cp_normalise

# What a claim asserts. Each maps to one question the transcript can answer.
CLAIM_KINDS = frozenset({
    "patch_applied",       # a named patch is/is not applied
    "patch_loaded",        # a named patch is/is not loaded
    "conditions_match",    # a named patch's conditions do/do not match
    "patch_priority",      # a named patch displays this priority label
    "reason_not_loaded",   # the reported reason contains this text
    "load_winner_for_target",  # exactly this Load supplies the asset
    "no_load_applied",     # no Load supplies the asset (the Exclusive conflict)
    "no_patch_applied",    # nothing at all is applied against the asset
    "apply_order",         # applied patches for an asset, in apply order
    "definition_order",    # `patch dump order` positions -- NOT apply order
    "current_change",      # `Current changes:` lists this target
})

# Claim kinds whose truth is a *negative*: the transcript must affirmatively
# show it enumerated the thing being denied before the claim can be confirmed.
# An empty parse result is otherwise indistinguishable from an empty game.
NEGATIVE_KINDS = frozenset({"no_load_applied", "no_patch_applied"})

# Claim kinds that turn on whether a patch is a Load or an Edit. Content Patcher
# prints a patch's action only in `patch dump applied`, and in `patch summary`
# only when the patch's name does not already contain its target -- so these
# require action-typed evidence and stay unresolved without it.
ROLE_KINDS = frozenset({"load_winner_for_target", "no_load_applied"})

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


def _one(summary: Summary, claim: Claim) -> tuple[ObservedPatch | None, str]:
    """Resolve a claim's patch fragment to exactly one row, or explain why not.

    Ambiguity is not resolved by picking a winner. Two rows matching a fragment
    means we cannot say which patch the prediction meant, and answering about
    either one would answer a different question than the one recorded.
    """
    matches = _find(summary, claim.patch or "")
    if not matches:
        return None, f"no patch matching {claim.patch!r} appears in the summary"
    if len(matches) > 1:
        return None, (f"{claim.patch!r} matches {len(matches)} patches "
                      f"({[p.location for p in matches]}); the claim cannot be "
                      "attributed to one of them")
    return matches[0], ""


def _bool_claim(field: str, claim: Claim, summary: Summary) -> ClaimResult:
    patch, why = _one(summary, claim)
    if patch is None:
        return ClaimResult(claim, "unobserved", why)
    actual = getattr(patch, field)
    verdict = "matched" if actual is bool(claim.expect) else "contradicted"
    return ClaimResult(claim, verdict, f"{patch.location}: {field}={actual}")


def _needs(claim: Claim, summary: Summary | None, applied: DumpApplied | None,
           order: DumpOrder | None) -> ClaimResult | None:
    """Refuse a claim whose evidence was not supplied or is not usable."""
    if claim.kind == "definition_order":
        if order is None:
            return ClaimResult(claim, "unobserved", "no `patch dump order` transcript supplied")
        if not order.usable:
            return ClaimResult(claim, "unobserved",
                               "the `patch dump order` transcript lists no rows")
        return None

    if claim.kind == "apply_order":
        # Definition order is NOT apply order, so there is no fallback here.
        # Upstream says the definition order "affects the order that patches are
        # applied" -- affects, not is -- and it includes patches that never
        # applied at all.
        if applied is None:
            extra = ("; `patch dump order` reports the global definition order, which "
                     "is a different fact and cannot stand in for it"
                     if order is not None else "")
            return ClaimResult(claim, "unobserved",
                               "no `patch dump applied` transcript supplied" + extra)
        if not applied.usable:
            return ClaimResult(claim, "unobserved",
                               applied.why_unusable() or "the dump is not usable")
        return None

    if summary is None:
        return ClaimResult(claim, "unobserved", "no `patch summary` transcript supplied")
    if not summary.coverage.usable:
        return ClaimResult(claim, "unobserved",
                           summary.coverage.why_unusable() or "the summary is not usable")
    if claim.kind in NEGATIVE_KINDS:
        covered, why = summary.coverage.covers_asset(claim.target or "")
        if not covered:
            return ClaimResult(
                claim, "unobserved",
                f"cannot confirm a negative here: {why}")
    return None


def compare(prediction: Prediction, summary: Summary | None = None,
            applied: DumpApplied | None = None,
            order: DumpOrder | None = None) -> Comparison:
    """Judge a prediction against what the transcripts actually say.

    Three rules hold for every claim, and they exist because a verifier that
    overstates its own evidence is worse than no verifier:

    * A claim the supplied transcripts cannot speak to is ``unobserved``, never
      ``matched``. Running fewer commands cannot produce a better result.
    * A *negative* claim needs affirmative evidence that the transcript
      enumerated what is being denied. An empty parse and an empty game look
      identical otherwise.
    * A claim about loads-versus-edits needs evidence of each patch's action.
      Content Patcher does not always print it, and it is never inferred.
    """
    comparison = Comparison(prediction=prediction)
    for claim in prediction.claims:
        comparison.results.append(_compare_one(claim, summary, applied, order))
    return comparison


def _compare_one(claim: Claim, summary: Summary | None,
                 applied: DumpApplied | None, order: DumpOrder | None) -> ClaimResult:
    refusal = _needs(claim, summary, applied, order)
    if refusal is not None:
        return refusal

    if claim.kind == "patch_applied":
        return _bool_claim("applied", claim, summary)  # type: ignore[arg-type]
    if claim.kind == "patch_loaded":
        return _bool_claim("loaded", claim, summary)  # type: ignore[arg-type]
    if claim.kind == "conditions_match":
        return _bool_claim("conditions_match", claim, summary)  # type: ignore[arg-type]

    if claim.kind == "patch_priority":
        patch, why = _one(summary, claim)  # type: ignore[arg-type]
        if patch is None:
            return ClaimResult(claim, "unobserved", why)
        verdict = "matched" if patch.priority == claim.expect else "contradicted"
        return ClaimResult(claim, verdict, f"{patch.location}: priority={patch.priority!r}")

    if claim.kind == "reason_not_loaded":
        patch, why = _one(summary, claim)  # type: ignore[arg-type]
        if patch is None:
            return ClaimResult(claim, "unobserved", why)
        reason = patch.reason_not_loaded
        if reason is None:
            return ClaimResult(claim, "contradicted",
                               f"{patch.location}: no reason reported")
        verdict = "matched" if str(claim.expect).lower() in reason.lower() else "contradicted"
        return ClaimResult(claim, verdict, f"{patch.location}: reason={reason!r}")

    if claim.kind == "load_winner_for_target":
        return _load_winner(claim, summary, applied)  # type: ignore[arg-type]
    if claim.kind == "no_load_applied":
        return _no_load(claim, summary, applied)  # type: ignore[arg-type]

    if claim.kind == "no_patch_applied":
        hits = summary.applied_to(claim.target or "")  # type: ignore[union-attr]
        if hits:
            return ClaimResult(claim, "contradicted",
                               f"applied={[p.location for p in hits]}")
        return ClaimResult(claim, "matched",
                           "no applied patch against this target, in a transcript "
                           "that demonstrably enumerated it")

    if claim.kind == "current_change":
        for mod in summary.mods:  # type: ignore[union-attr]
            for target, labels in mod.current_changes.items():
                if cp_normalise(target) != cp_normalise(claim.target or ""):
                    continue
                joined = "; ".join(labels)
                if not claim.expect or str(claim.expect).lower() in joined.lower():
                    return ClaimResult(claim, "matched", f"{target} | {joined}")
                return ClaimResult(claim, "contradicted", f"{target} | {joined}")
        return ClaimResult(claim, "contradicted",
                           f"no current change listed for {claim.target!r}")

    if claim.kind == "apply_order":
        sequence = applied.order_for(claim.target or "")  # type: ignore[union-attr]
        if sequence is None:
            return ClaimResult(
                claim, "unobserved",
                f"`patch dump applied` reports no group for {claim.target!r}, which "
                "is not the same as reporting an empty one")
        return _ordered(claim, sequence, "patch dump applied (applied rows only)")

    if claim.kind == "definition_order":
        sequence = [row[2] for row in order.rows]  # type: ignore[union-attr]
        return _ordered(claim, sequence, "patch dump order (DEFINITION order)")

    raise AssertionError(f"unhandled claim kind {claim.kind!r}")


def _ordered(claim: Claim, sequence: list[str], source: str) -> ClaimResult:
    positions = []
    for fragment in claim.order:
        hits = [i for i, name in enumerate(sequence) if fragment.lower() in name.lower()]
        if not hits:
            return ClaimResult(claim, "contradicted",
                               f"{fragment!r} does not appear in {source}: {sequence}")
        if len(hits) > 1:
            return ClaimResult(claim, "unobserved",
                               f"{fragment!r} matches {len(hits)} rows in {source}; "
                               "the claim cannot be attributed to one of them")
        positions.append(hits[0])
    verdict = "matched" if positions == sorted(positions) else "contradicted"
    return ClaimResult(claim, verdict, f"{source}: {sequence}")


def _load_winner(claim: Claim, summary: Summary,
                 applied: DumpApplied | None) -> ClaimResult:
    """Exactly one Load supplies the asset, and it is the predicted one.

    Edits that apply afterwards do not contradict this. That distinction is the
    whole point: a replacement winning and an intentional overlay composing on
    top of it is the normal, healthy case, and a verifier that counted the
    overlay as a second winner would report it as a failure.
    """
    target = claim.target or ""
    if applied is not None and applied.usable:
        split = applied.applied_loads_for(target)
        if split is None:
            return ClaimResult(claim, "unobserved",
                               f"`patch dump applied` reports no group for {target!r}")
        loads, unknown = split
        if unknown:
            return ClaimResult(claim, "unobserved",
                               f"{len(unknown)} applied row(s) have an action we do "
                               f"not recognise: {[r.action for r in unknown]}")
        names = [r.path for r in loads]
        source = "patch dump applied"
    else:
        loads, unknown = summary.applied_loads_to(target)
        if unknown:
            return ClaimResult(
                claim, "unobserved",
                f"the summary did not report an action for {len(unknown)} applied "
                f"patch(es) against {target!r} ({[p.location for p in unknown]}); "
                "run `patch dump applied`, which always prints the action")
        names = [p.location for p in loads]
        source = "patch summary"
    if len(loads) != 1:
        return ClaimResult(claim, "contradicted",
                           f"{source}: {len(loads)} Load(s) applied against {target}: {names}")
    if str(claim.expect).lower() not in names[0].lower():
        return ClaimResult(claim, "contradicted",
                           f"{source}: the applied Load is {names[0]!r}, not {claim.expect!r}")
    return ClaimResult(claim, "matched", f"{source}: the applied Load is {names[0]!r}")


def _no_load(claim: Claim, summary: Summary, applied: DumpApplied | None) -> ClaimResult:
    """No Load supplies the asset -- the Exclusive-conflict outcome.

    Stated about loads specifically, because that is what the rule is about.
    Edits still apply in this case; they compose onto the game's original asset,
    and counting them would contradict a prediction that is correct.
    """
    target = claim.target or ""
    if applied is not None and applied.usable:
        split = applied.applied_loads_for(target)
        if split is None:
            return ClaimResult(
                claim, "unobserved",
                f"`patch dump applied` reports no group for {target!r}. That means "
                "the asset was never requested, not that no Load applied -- summon "
                "or open the thing that uses it, then capture again")
        loads, unknown = split
        if unknown:
            return ClaimResult(claim, "unobserved",
                               f"{len(unknown)} applied row(s) have an unrecognised "
                               f"action: {[r.action for r in unknown]}")
        if loads:
            return ClaimResult(claim, "contradicted",
                               f"a Load applied against {target}: {[r.path for r in loads]}")
        return ClaimResult(claim, "matched",
                           f"patch dump applied: no Load applied against {target}")

    loads, unknown = summary.applied_loads_to(target)
    if unknown:
        return ClaimResult(
            claim, "unobserved",
            f"the summary did not report an action for {len(unknown)} applied "
            f"patch(es) against {target!r}; run `patch dump applied`")
    if loads:
        return ClaimResult(claim, "contradicted",
                           f"a Load applied against {target}: {[p.location for p in loads]}")
    if not summary.for_target(target):
        return ClaimResult(
            claim, "unobserved",
            f"the summary reports no patch at all targeting {target!r}. Content "
            "Patcher prints a patch's resolved target only when it differs from "
            "the patch name, so this may be a reporting gap rather than an "
            "absence; run `patch dump applied`, which groups by target")
    return ClaimResult(claim, "matched",
                       f"patch summary: no Load applied against {target}")

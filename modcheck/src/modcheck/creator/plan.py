"""Creator request -> requirement-to-evidence map -> implementation plan.

The gate this implements is task-specific, not global. A request is a set of
named behavioural requirements; each names the capability it needs; each is
resolved against this game's capability records under this game version and
loader. The output says, per requirement, what mechanism was selected, what
establishes it, how it will be validated, and whether it is grounded.

Three statuses, and the middle one is the useful one:

* ``grounded`` -- an eligible capability record supplies an exact mechanism.
  This means the technical premises of the choice are established. It does NOT
  mean the behaviour has been tested; that is what ``validation`` is for, and
  the plan keeps the two apart on purpose.
* ``requires_investigation`` -- the capability is needed and we have no eligible
  record. The map names the exact missing fact and the smallest procedure that
  would settle it, so the rest of the plan can proceed meanwhile.
* ``unsupported`` -- an eligible record exists and its own stated constraints
  contradict the requirement. This is a real answer, not a gap.

What this deliberately does not do: parse English into requirements. The
taxonomy's ``suggest`` proposes capabilities from wording, and a human or the
CLI confirms them in the spec. A pipeline that guessed requirements would
produce a plan nobody agreed to, and every status below would be about the
guess rather than the request.
"""
from __future__ import annotations

import dataclasses
from typing import Any

from .. import yamlio
from ..store import Record, Store
from .capability import BY_ID, IDS
from .guide import _loader_applies, _version_applies

GROUNDED = "grounded"
REQUIRES_INVESTIGATION = "requires_investigation"
UNSUPPORTED = "unsupported"

SIDES = ("server", "client", "either")


@dataclasses.dataclass
class Requirement:
    """One behaviour the creator asked for, in their terms."""
    id: str
    statement: str
    capability: str
    # Where the behaviour has to take effect. Declaring it is what makes a
    # client/server mismatch detectable instead of a runtime surprise.
    side: str = "either"
    acceptance: str = ""
    notes: str = ""
    # Requirements this one consumes a value from. Declaring the data flow is
    # what lets the composition be checked: two requirements can each be
    # perfectly grounded and still not work together.
    reads_from: list[str] = dataclasses.field(default_factory=list)

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> "Requirement":
        if data["capability"] not in IDS:
            raise ValueError(
                f"{data['id']}: unknown capability {data['capability']!r}; "
                f"known: {sorted(IDS)}")
        side = data.get("side", "either")
        if side not in SIDES:
            raise ValueError(f"{data['id']}: side must be one of {SIDES}")
        return cls(id=data["id"], statement=data["statement"],
                   capability=data["capability"], side=side,
                   acceptance=data.get("acceptance", ""), notes=data.get("notes", ""),
                   reads_from=list(data.get("reads_from") or []))


@dataclasses.dataclass
class Request:
    id: str
    game: str
    title: str
    description: str
    requirements: list[Requirement]
    game_version: str | None = None
    loader: str | None = None
    project: str | None = None

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> "Request":
        return cls(id=data["id"], game=data["game"], title=data["title"],
                   description=data.get("description", ""),
                   requirements=[Requirement.from_dict(r) for r in data["requirements"]],
                   game_version=data.get("game_version"), loader=data.get("loader"),
                   project=data.get("project"))

    @classmethod
    def load(cls, path) -> "Request":
        return cls.from_dict(yamlio.load_path(path))


@dataclasses.dataclass
class Resolution:
    """One requirement, resolved against the library."""
    requirement: Requirement
    status: str
    record: Record | None = None
    mechanism: list[dict] = dataclasses.field(default_factory=list)
    evidence: list[dict] = dataclasses.field(default_factory=list)
    evidence_state: str | None = None
    validation: list[dict] = dataclasses.field(default_factory=list)
    missing_fact: str = ""
    procedure: str = ""
    conflict: str = ""
    ineligible: list[str] = dataclasses.field(default_factory=list)

    @property
    def grounded(self) -> bool:
        return self.status == GROUNDED


def _eligible(record: Record, request: Request, ecosystem_loaders: int) -> str | None:
    """Why this record does not apply here, or None if it does."""
    version_ok, why = _version_applies(record, request.game_version)
    if version_ok is False:
        applies = (record.get("applies_to") or {}).get("game_versions")
        return f"declares game_versions {applies!r}, which excludes {request.game_version}"
    loader_ok = _loader_applies(record, request.loader, ecosystem_loaders)
    if loader_ok is False:
        applies = (record.get("applies_to") or {}).get("loader")
        return f"targets loader {applies!r}, not {request.loader!r}"
    if version_ok is None:
        return f"applicability not established: {why}"
    if loader_ok is None:
        return "applicability not established: the loader could not be decided"
    return None


def _side_fit(record: Record, requirement: Requirement) -> str:
    """Whether this mechanism can serve the requirement's side.

    Three-valued, for the same reason applicability is: a record that does not
    declare `runs_on` has not been established to run anywhere in particular,
    and treating silence as a match is how a client-only entrypoint gets
    selected for a server-side requirement -- which compiles, and does nothing.
    """
    runs_on = record.get("runs_on")
    if requirement.side == "either":
        return "fits"
    if not runs_on:
        return "undeclared"
    if runs_on in ("both", "not_applicable") or runs_on == requirement.side:
        return "fits"
    return "conflicts"


def _side_conflict(record: Record, requirement: Requirement) -> str:
    return (f"the requirement needs this on the {requirement.side}, but "
            f"{record.id} runs on the {record.get('runs_on')}. A value produced "
            "on one side is not readable on the other without an explicit "
            "synchronisation step, and a plan that skips it compiles cleanly "
            "and does nothing visible.")


def resolve(request: Request, store: Store | None = None) -> list[Resolution]:
    store = store or Store()
    pack = store.pack(request.game)
    ecosystem_loaders = len(
        ((pack.manifest or {}).get("ecosystem") or {}).get("loaders") or [])
    records = pack.records("capability")

    out: list[Resolution] = []
    for requirement in request.requirements:
        candidates = [r for r in records if r.get("capability") == requirement.capability]
        eligible: list[Record] = []
        ineligible: list[str] = []
        for record in candidates:
            reason = _eligible(record, request, ecosystem_loaders)
            if reason:
                ineligible.append(f"{record.id}: {reason}")
            else:
                eligible.append(record)

        if not eligible:
            capability = BY_ID[requirement.capability]
            if candidates:
                missing = (f"whether any recorded {requirement.capability} mechanism "
                           f"applies to {request.game} {request.game_version} on "
                           f"{request.loader}")
                procedure = (
                    "Check the applies_to of the records listed below against this "
                    "version and loader; where a record's game_versions is prose, "
                    "restate it as a checkable range before relying on it.")
            else:
                missing = (f"a {request.game} mechanism for {requirement.capability} "
                           f"({capability.title}): which exact type, method or file "
                           "format provides it")
                procedure = (
                    f"Read the resolved toolchain artifact for this loader with "
                    f"`modcheck lookup --jar <artifact> --classes <pattern>`, then "
                    f"record the exact signatures as a capability record. Until then "
                    f"this requirement is not implementable from the library.")
            out.append(Resolution(
                requirement=requirement, status=REQUIRES_INVESTIGATION,
                missing_fact=missing, procedure=procedure, ineligible=ineligible))
            continue

        # Side fitness decides WHICH records may be selected; evidence strength
        # only orders the ones that fit. Ranking by evidence first would pick a
        # build-tested client mechanism over a source-confirmed server one for a
        # server-side requirement, which is the stronger evidence for the wrong
        # thing.
        from .coverage import EVIDENCE_ORDER

        def strength(record: Record) -> int:
            state = record.get("evidence_state")
            return EVIDENCE_ORDER.index(state) if state in EVIDENCE_ORDER else -1

        fits = [r for r in eligible if _side_fit(r, requirement) == "fits"]
        undeclared = [r for r in eligible if _side_fit(r, requirement) == "undeclared"]
        conflicting = [r for r in eligible if _side_fit(r, requirement) == "conflicts"]

        if not fits and undeclared:
            names = ", ".join(r.id for r in undeclared)
            out.append(Resolution(
                requirement=requirement, status=REQUIRES_INVESTIGATION,
                ineligible=ineligible + [
                    f"{r.id}: does not declare runs_on, so it cannot be shown to "
                    f"serve a {requirement.side}-side requirement" for r in undeclared],
                missing_fact=(
                    f"which side these {requirement.capability} mechanisms run on: "
                    f"{names}"),
                procedure=(
                    "Read the mechanism's own entry point -- for a Fabric "
                    "entrypoint, which entrypoint list it is registered under; for "
                    "an API class, whether it lives under a client-only package -- "
                    "and record `runs_on` on the capability record. Until then a "
                    f"{requirement.side}-side requirement cannot select it.")))
            continue

        if not fits and conflicting:
            conflicting.sort(key=strength, reverse=True)
            record = conflicting[0]
            out.append(Resolution(
                requirement=requirement, status=UNSUPPORTED, record=record,
                mechanism=list(record.get("mechanism") or []),
                evidence=list(record.get("references") or []),
                evidence_state=record.get("evidence_state"),
                conflict=_side_conflict(record, requirement), ineligible=ineligible))
            continue

        fits.sort(key=strength, reverse=True)
        record = fits[0]
        ineligible = ineligible + [
            f"{r.id}: runs on the {r.get('runs_on') or 'undeclared'} side"
            for r in conflicting + undeclared]

        out.append(Resolution(
            requirement=requirement, status=GROUNDED, record=record,
            mechanism=list(record.get("mechanism") or []),
            evidence=list(record.get("references") or []),
            evidence_state=record.get("evidence_state"),
            validation=list(record.get("validation") or []),
            ineligible=ineligible))
    return out


@dataclasses.dataclass
class CompositionIssue:
    """A gap between two requirements that are each individually fine."""
    consumer: str
    producer: str
    kind: str
    detail: str
    needed_capability: str | None = None
    status: str = REQUIRES_INVESTIGATION


def check_composition(request: Request, resolutions: list[Resolution],
                      store: Store | None = None) -> list[CompositionIssue]:
    """Check the joins, not just the parts.

    Individually grounded requirements can still fail together. The commonest
    case, and the one this catches, is a value produced on one side and read on
    the other: both halves compile, both are backed by exact signatures, and the
    display shows nothing because nothing ever sends the value.

    A data flow is only checked where the request declares it with `reads_from`.
    Inferring flows from wording would invent joins the creator did not ask for.
    """
    store = store or Store()
    by_id = {r.requirement.id: r for r in resolutions}
    issues: list[CompositionIssue] = []

    def side_of(resolution: Resolution) -> str:
        if resolution.record is not None and resolution.record.get("runs_on") in (
                "server", "client"):
            return resolution.record.get("runs_on")
        return resolution.requirement.side

    pack = store.pack(request.game)
    have_sync = [r for r in pack.records("capability")
                 if r.get("capability") == "sync_state"]

    for resolution in resolutions:
        for producer_id in resolution.requirement.reads_from:
            producer = by_id.get(producer_id)
            if producer is None:
                issues.append(CompositionIssue(
                    consumer=resolution.requirement.id, producer=producer_id,
                    kind="unknown_producer",
                    detail=f"reads_from names {producer_id!r}, which is not a "
                           "requirement of this request"))
                continue
            consumer_side, producer_side = side_of(resolution), side_of(producer)
            if consumer_side == producer_side or "either" in (consumer_side, producer_side):
                continue
            detail = (
                f"{resolution.requirement.id} runs on the {consumer_side} and reads a "
                f"value {producer.requirement.id} produces on the {producer_side}. "
                "Both requirements are grounded on their own, and the combination "
                "still does not work: nothing carries the value across. The build "
                "will succeed and the display will show a default forever.")
            if have_sync:
                issues.append(CompositionIssue(
                    consumer=resolution.requirement.id, producer=producer_id,
                    kind="cross_side_read", detail=detail,
                    needed_capability="sync_state", status=GROUNDED))
            else:
                issues.append(CompositionIssue(
                    consumer=resolution.requirement.id, producer=producer_id,
                    kind="cross_side_read",
                    detail=detail + " This request therefore also needs a "
                           "sync_state mechanism, and no capability record for "
                           f"sync_state exists in the {request.game} pack.",
                    needed_capability="sync_state",
                    status=REQUIRES_INVESTIGATION))
    return issues


@dataclasses.dataclass
class Plan:
    request: Request
    resolutions: list[Resolution]
    composition: list[CompositionIssue] = dataclasses.field(default_factory=list)

    @property
    def grounded(self) -> list[Resolution]:
        return [r for r in self.resolutions if r.status == GROUNDED]

    @property
    def blocked(self) -> list[Resolution]:
        return [r for r in self.resolutions if r.status != GROUNDED]

    @property
    def ready(self) -> bool:
        """Every requirement grounded AND every declared join sound.

        Not a claim that anything works -- only that nothing known is missing.
        """
        unresolved_joins = [i for i in self.composition if i.status != GROUNDED]
        return bool(self.resolutions) and not self.blocked and not unresolved_joins

    def weakest_evidence(self) -> str | None:
        from .coverage import EVIDENCE_ORDER
        states = [r.evidence_state for r in self.grounded if r.evidence_state]
        if not states:
            return None
        return min(states, key=lambda s: EVIDENCE_ORDER.index(s)
                   if s in EVIDENCE_ORDER else 0)

    def as_dict(self) -> dict[str, Any]:
        return {
            "request": self.request.id,
            "game": self.request.game,
            "game_version": self.request.game_version,
            "loader": self.request.loader,
            "ready": self.ready,
            "composition": [dataclasses.asdict(i) for i in self.composition],
            "weakest_evidence": self.weakest_evidence(),
            "requirements": [
                {
                    "id": r.requirement.id,
                    "statement": r.requirement.statement,
                    "capability": r.requirement.capability,
                    "side": r.requirement.side,
                    "status": r.status,
                    "selected": r.record.id if r.record else None,
                    "evidence_state": r.evidence_state,
                    "mechanism": r.mechanism,
                    "evidence": r.evidence,
                    "validation": r.validation,
                    "acceptance": r.requirement.acceptance,
                    "missing_fact": r.missing_fact,
                    "procedure": r.procedure,
                    "conflict": r.conflict,
                    "ineligible": r.ineligible,
                }
                for r in self.resolutions
            ],
        }


def build(request: Request, store: Store | None = None) -> Plan:
    store = store or Store()
    resolutions = resolve(request, store)
    return Plan(request=request, resolutions=resolutions,
                composition=check_composition(request, resolutions, store))


_STATUS_MARK = {GROUNDED: "GROUNDED", REQUIRES_INVESTIGATION: "INVESTIGATE",
                UNSUPPORTED: "UNSUPPORTED"}


def render(plan: Plan) -> str:
    request = plan.request
    lines = [f"{request.title}  [{request.game}"
             + (f" {request.game_version}" if request.game_version else "")
             + (f", {request.loader}" if request.loader else "") + "]", ""]
    if request.description:
        lines.append(f"  {request.description.strip()}")
        lines.append("")
    lines.append("-- requirement to evidence --")
    lines.append("")
    for resolution in plan.resolutions:
        requirement = resolution.requirement
        lines.append(f"  [{_STATUS_MARK[resolution.status]}] {requirement.id}")
        lines.append(f"      needs: {requirement.statement}")
        lines.append(f"      capability: {requirement.capability} "
                     f"(on the {requirement.side})")
        if resolution.record is not None:
            lines.append(f"      selected: {resolution.record.id} "
                         f"[{resolution.evidence_state}]")
            for item in resolution.mechanism[:4]:
                signature = item.get("signature") or item.get("label") or ""
                lines.append(f"        {item['kind']:10} {item['id']}")
                if signature:
                    lines.append(f"                   {signature}")
        for item in resolution.evidence[:4]:
            quote = item.get("quote")
            lines.append(f"      evidence: {item.get('source')} "
                         f"({item.get('evidence_class')}) {item.get('locator', '')}")
            if quote:
                lines.append(f"                {quote}")
        for step in resolution.validation[:2]:
            lines.append(f"      validation: {step.get('method')}")
            if step.get("limits"):
                lines.append(f"                  limits: {step['limits'].strip()}")
        if requirement.acceptance:
            lines.append(f"      acceptance: {requirement.acceptance.strip()}")
        if resolution.conflict:
            lines.append(f"      CONFLICT: {resolution.conflict}")
        if resolution.missing_fact:
            lines.append(f"      missing fact: {resolution.missing_fact}")
            lines.append(f"      to settle it: {resolution.procedure}")
        for line in resolution.ineligible:
            lines.append(f"      not eligible: {line}")
        lines.append("")
    if plan.composition:
        lines.append("-- composition: joins between requirements --")
        lines.append("")
        for issue in plan.composition:
            mark = "GROUNDED" if issue.status == GROUNDED else "INVESTIGATE"
            lines.append(f"  [{mark}] {issue.consumer} <- {issue.producer}"
                         f"  ({issue.kind})")
            lines.append(f"      {issue.detail}")
            if issue.needed_capability:
                lines.append(f"      also needs capability: {issue.needed_capability}")
            lines.append("")
    lines.append("-- status --")
    lines.append(f"  {len(plan.grounded)}/{len(plan.resolutions)} requirements grounded")
    joins = [i for i in plan.composition if i.status != GROUNDED]
    if joins:
        lines.append(f"  {len(joins)} declared join(s) not grounded -- individually "
                     "grounded parts do not make a working whole")
    weakest = plan.weakest_evidence()
    if weakest:
        lines.append(f"  weakest evidence under a grounded requirement: {weakest}")
    lines.append("")
    lines.append("  'Grounded' means the technical premises of the choice are")
    lines.append("  established -- the exact mechanism is known and version-matched.")
    lines.append("  It does NOT mean the behaviour has been tested. Those are")
    lines.append("  separate, and the validation lines above say which is which.")
    return "\n".join(lines)

"""Creator request -> plan -> reviewable project changes -> build -> report.

The one workflow. `plan` decides what is grounded; this turns the grounded part
into a diff, runs the project's own build, and reports what that does and does
not establish.

Three boundaries it will not cross, each of which would be a way of claiming
more than was done:

* **It refuses ungrounded work.** A requirement that is `requires_investigation`
  or `unsupported` produces no code. Emitting a plausible-looking file for a
  mechanism we have not established is how an invented API gets into someone's
  project.
* **It marks the seam.** Generated files wire a mechanism and leave the
  creator's own logic explicitly unwritten, with a comment saying so. An empty
  method body presented as a finished feature is the failure mode here.
* **It keeps build success apart from behaviour.** A green build establishes
  that the wiring compiles against the recorded signatures. It says nothing
  about whether the mod does what was asked, and the report says that in those
  words.

No model is invoked. ModCheck has no language-model integration at all, so it
composes recorded mechanisms and stops where composition stops.
"""
from __future__ import annotations

import dataclasses
from pathlib import Path
from typing import Any

from ..store import Store
from .apply import ChangeSet, FileChange
from .build import BuildResult, run_build
from .mechanisms import Emission, for_record
from .plan import GROUNDED, Plan, Request, build as build_plan


@dataclasses.dataclass
class Step:
    """One requirement, and what the pipeline did about it."""
    requirement: str
    capability: str
    status: str
    generator: str | None = None
    files: list[str] = dataclasses.field(default_factory=list)
    cites: list[str] = dataclasses.field(default_factory=list)
    skipped_because: str = ""


@dataclasses.dataclass
class Outcome:
    request: Request
    plan: Plan
    steps: list[Step]
    changeset: ChangeSet | None
    build: BuildResult | None = None
    written: list[str] = dataclasses.field(default_factory=list)

    @property
    def wired(self) -> list[Step]:
        return [s for s in self.steps if s.files]

    @property
    def skipped(self) -> list[Step]:
        return [s for s in self.steps if not s.files]

    def as_dict(self) -> dict[str, Any]:
        return {
            "request": self.request.id,
            "plan_ready": self.plan.ready,
            "steps": [dataclasses.asdict(s) for s in self.steps],
            "written": self.written,
            "build": self.build.as_dict() if self.build else None,
        }


# Arguments each mechanism generator needs beyond the record, derived from the
# request rather than hardcoded per demonstration.
# Which naming slot each capability's emitted class fills, so a later
# generator references a real class rather than a guessed name.
_PROVIDES = {"persist_state": "state_class", "sync_state": "payload_class",
             "add_configuration": "rule_class"}


def _remember(naming: dict[str, str], capability: str, emission, record) -> None:
    slot = _PROVIDES.get(capability)
    if slot and emission.class_name:
        naming[slot] = emission.class_name
    if capability == "persist_state":
        # The consumer's shape depends on the mechanism's scope, not its
        # capability: a level store has one instance to fetch and a block
        # attachment does not.
        naming["state_scope"] = record.get("scope") or "level"


def _arguments(request: Request, requirement_id: str, capability: str,
               naming: dict[str, str], *, instance: int = 0) -> dict[str, Any]:
    name = naming.get("name", request.id.replace("_", " "))
    if instance:
        # A second independent instance of the same capability needs its own
        # class, or the second emission overwrites the first.
        name = f"{name} {requirement_id}"
    args: dict[str, Any] = {"name": name}
    if capability == "add_configuration":
        args["default"] = int(naming.get("default", 20))
        args["minimum"] = int(naming.get("minimum", 1))
        args["maximum"] = int(naming.get("maximum", 100000))
    if capability in ("persist_state", "sync_state"):
        args["field_name"] = naming.get("field", "charge")
    if capability == "subscribe_event":
        # Required, not defaulted: a ticker that references a class nothing
        # emitted is a compile error found late instead of a refusal found now.
        if "state_class" not in naming:
            raise KeyError(
                "subscribe_event is wired after a persist_state mechanism, and "
                "none was emitted. Its handler would reference a class that does "
                "not exist.")
        args["state_class"] = naming["state_class"]
        args["rule_class"] = naming.get("rule_class")
        args["state_scope"] = naming.get("state_scope", "level")
    if capability == "display_information":
        if "payload_class" not in naming:
            raise KeyError(
                "display_information is wired after a sync_state mechanism, and "
                "none was emitted. Its handler would reference a class that does "
                "not exist.")
        args["payload_class"] = naming["payload_class"]
        # The accessor name must be the payload's field, not a word guessed
        # from the request title. Deriving it from the title produced
        # `payload.rain()` for a record whose component is `charge`.
        args["field_name"] = naming.get("field", "charge")
    return args


def run(request: Request, project: Path, *, store: Store | None = None,
        naming: dict[str, str] | None = None, write: bool = False,
        allow_execute: bool = False) -> Outcome:
    store = store or Store()
    project = Path(project)
    plan = build_plan(request, store)
    naming = dict(naming or {})

    # Producers are wired before consumers, and a consumer references the class
    # a producer ACTUALLY emitted rather than one reconstructed from a naming
    # convention. The convention broke as soon as a revision selected a
    # different persist_state mechanism: the ticker kept referring to a class
    # that was no longer being generated, and only the build noticed.
    PRODUCER_FIRST = {"add_configuration": 0, "persist_state": 1, "sync_state": 2,
                      "subscribe_event": 3, "display_information": 4}
    ordered = sorted(plan.resolutions,
                     key=lambda r: PRODUCER_FIRST.get(r.requirement.capability, 9))

    steps: list[Step] = []
    emissions: list[Emission] = []
    # A join the plan found is work the request implies but did not list. The
    # lantern asks for stored charge and a HUD and never mentions networking;
    # the composition check is what noticed, and wiring only the listed
    # requirements would reproduce exactly the gap it found.
    # Runs BEFORE the requirement loop: a join supplies a producer
    # (sync_state feeds display_information), so wiring it afterwards
    # would leave the consumer referencing a class not yet emitted.
    seen: set[str] = set()
    wired_keys: dict[tuple, str] = {}
    for issue in plan.composition:
        capability = issue.needed_capability
        if not capability or capability in seen:
            continue
        if issue.status != GROUNDED:
            steps.append(Step(
                requirement=f"{issue.consumer}<-{issue.producer}",
                capability=capability, status=issue.status,
                skipped_because=issue.detail))
            continue
        # The record the PLANNER chose, not a fresh lookup. Re-resolving here
        # re-ran the choice with none of the eligibility, side or scope checks,
        # so whichever record happened to be first on disk won.
        record = issue.record
        generator = for_record(record) if record is not None else None
        if record is None or generator is None:
            steps.append(Step(
                requirement=f"{issue.consumer}<-{issue.producer}",
                capability=capability, status=issue.status,
                skipped_because=f"no generator wires {capability}"))
            continue
        emission = generator(project, record,
                             **_arguments(request, issue.consumer, capability, naming))
        emissions.append(emission)
        _remember(naming, capability, emission, record)
        seen.add(capability)
        steps.append(Step(
            requirement=f"{issue.consumer}<-{issue.producer}",
            capability=capability, status=issue.status,
            generator=generator.__name__, files=[emission.path],
            cites=emission.cites))


    for resolution in ordered:
        capability = resolution.requirement.capability
        if resolution.status != GROUNDED:
            steps.append(Step(
                requirement=resolution.requirement.id, capability=capability,
                status=resolution.status,
                skipped_because=(
                    resolution.conflict or resolution.missing_fact
                    or "not grounded")))
            continue
        # Deduplicate by the MECHANISM a requirement selected, not by its
        # capability. Two requirements can both need persist_state and mean two
        # independent stores; collapsing them would silently give one value
        # where the request asked for two.
        key = (capability, resolution.record.id if resolution.record else None,
               resolution.requirement.scope)
        if key in wired_keys:
            steps.append(Step(
                requirement=resolution.requirement.id, capability=capability,
                status=resolution.status,
                skipped_because=(
                    f"already wired in this request by {wired_keys[key]}, which "
                    f"selected the same mechanism at the same scope")))
            continue
        generator = for_record(resolution.record)
        if generator is None:
            steps.append(Step(
                requirement=resolution.requirement.id, capability=capability,
                status=resolution.status,
                skipped_because=(
                    f"{capability} is grounded for {resolution.record.id}, but no "
                    f"generator targets {request.game}/"
                    f"{(resolution.record.get('applies_to') or {}).get('loader')}. "
                    "The mechanism is recorded and can be implemented by hand from "
                    "the plan; ModCheck will not emit code for an ecosystem it has "
                    "no generator for, because that output would look plausible and "
                    "be wrong.")))
            continue
        instance = sum(1 for k in wired_keys if k[0] == capability)
        emission = generator(project, resolution.record,
                             **_arguments(request, resolution.requirement.id,
                                          capability, naming, instance=instance))
        emissions.append(emission)
        seen.add(capability)
        wired_keys[key] = resolution.requirement.id
        _remember(naming, capability, emission, resolution.record)
        steps.append(Step(
            requirement=resolution.requirement.id, capability=capability,
            status=resolution.status, generator=generator.__name__,
            files=[emission.path], cites=emission.cites))

    changeset = None
    if emissions:
        changes = []
        for emission in emissions:
            path = project / emission.path
            before = path.read_text(encoding="utf-8") if path.exists() else None
            changes.append(FileChange(path=emission.path, before=before,
                                      after=emission.content))
        changeset = ChangeSet(
            generator="pipeline",
            description=f"wire {len(emissions)} recorded mechanism(s) for {request.id}",
            changes=changes,
            notes=[
                "Every file cites the capability record that justified it, and "
                "through it the artifact hash the signatures came from.",
                "Files carry a marked seam where the creator's own logic goes. "
                "Nothing behind that seam was written, and the build passing "
                "does not mean it was.",
            ],
            follow_up=[
                "review the diff before writing it",
                "fill in the marked seams",
                "run the project's build to confirm the wiring compiles",
            ])

    outcome = Outcome(request=request, plan=plan, steps=steps, changeset=changeset)
    if write and changeset is not None:
        outcome.written = changeset.write(project)
    if allow_execute and outcome.written:
        outcome.build = run_build(project, ["gradle build --no-daemon --console=plain"],
                                  allow_execute=True)
    return outcome


def render(outcome: Outcome) -> str:
    lines = [f"Creator pipeline: {outcome.request.title}", ""]
    lines.append(f"  plan: {len(outcome.plan.grounded)}/"
                 f"{len(outcome.plan.resolutions)} grounded, "
                 f"ready={outcome.plan.ready}")
    lines.append("")
    lines.append("-- what the pipeline did --")
    for step in outcome.steps:
        if step.files:
            lines.append(f"  [wired]   {step.requirement} ({step.capability})")
            lines.append(f"            generator {step.generator}")
            for path in step.files:
                lines.append(f"            {path}")
            for cite in step.cites:
                lines.append(f"            from {cite}")
        else:
            lines.append(f"  [skipped] {step.requirement} ({step.capability})")
            lines.append(f"            {step.skipped_because}")
    lines.append("")
    if outcome.written:
        lines.append(f"-- written --")
        for path in outcome.written:
            lines.append(f"  {path}")
        lines.append("")
    if outcome.build is not None:
        lines.append("-- build --")
        lines.append(f"  ok: {outcome.build.ok}")
        for name, version in outcome.build.toolchain.items():
            lines.append(f"  {name}: {version}")
        for artifact in outcome.build.artifacts:
            lines.append(f"  artifact {Path(artifact.path).name} "
                         f"sha256 {artifact.sha256[:16]}")
        lines.append("")
        lines.append("  This establishes the generated wiring compiles against the")
        lines.append("  recorded signatures. It does NOT establish that the mod does")
        lines.append("  what was asked: the seams are still empty, and no game ran.")
    return "\n".join(lines)

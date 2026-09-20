"""The seam where a coding agent plugs into the creator pipeline.

ModCheck's deterministic generators emit the *wiring* a recorded mechanism
requires and stop at a marked seam. Filling that seam -- writing a creator's
own logic -- is not something a template can do, and this module is where an
agent would do it instead.

What is here: a callable boundary, a real implementation of it against the
Anthropic SDK, a refusing default, and tests that exercise the wiring with a
fake agent. What an agent returns goes through the SAME path as generated code
-- a ChangeSet, reviewed before it is written, then the project's own build --
because code that arrives by a different route still has to be reviewable and
still has to compile.

**Agent execution is unavailable in this environment.** The blocker is exactly
two things, both checkable:

* the `anthropic` package is not installed (it is not a dependency of ModCheck,
  and adding one that the core does not need would be wrong);
* no credential is resolvable -- no `ANTHROPIC_API_KEY`, no
  `ANTHROPIC_AUTH_TOKEN`, no `ant auth login` profile.

`AnthropicCreatorAgent.availability()` reports which of those is missing rather
than failing at call time. Nothing in this module has been executed against the
API from here, and it is not claimed to work; it is the boundary, written so
that supplying the two missing things is the whole remaining step.
"""
from __future__ import annotations

import dataclasses
import json
import os
import shutil
from pathlib import Path
from typing import Protocol

from .apply import ChangeSet, FileChange
from .plan import Plan, Request, Resolution

# Opus 5 unless a caller names another model. Not downgraded for cost: that is
# the operator's decision, and a creator's implementation is exactly the kind of
# work where the difference shows.
DEFAULT_MODEL = "claude-opus-5"


@dataclasses.dataclass
class AgentTask:
    """Everything an agent is given. No more, and nothing implicit.

    The context is deliberately the same material a human reviewer would need:
    the request in the creator's words, the requirement being implemented, the
    exact mechanism the planner selected with the evidence behind it, and the
    files as they currently stand. An agent that has to guess the API is an
    agent that will invent one.
    """
    request: Request
    resolution: Resolution
    project: Path
    # Files already emitted in this run, so an agent filling a seam sees the
    # wiring it is filling rather than an empty directory.
    context_files: dict[str, str] = dataclasses.field(default_factory=dict)
    instruction: str = ""

    def evidence_lines(self) -> list[str]:
        out = []
        for item in self.resolution.mechanism:
            signature = item.get("signature") or item.get("label") or ""
            out.append(f"{item['kind']} {item['id']}"
                       + (f" :: {signature}" if signature else ""))
        for reference in self.resolution.evidence:
            quote = reference.get("quote")
            out.append(f"evidence {reference.get('source')}"
                       + (f": {quote}" if quote else ""))
        return out


class CreatorAgent(Protocol):
    """A coding agent, as the pipeline needs it.

    One method. Whatever is behind it -- an API call, a local model, a human at
    a terminal -- returns a ChangeSet or raises.
    """

    def implement(self, task: AgentTask) -> ChangeSet:
        ...


class AgentUnavailable(RuntimeError):
    """Raised when an agent cannot run, with the specific blocker named."""


@dataclasses.dataclass
class Availability:
    ready: bool
    blockers: list[str]

    def describe(self) -> str:
        if self.ready:
            return "agent execution is available"
        return "agent execution is unavailable: " + "; ".join(self.blockers)


class NullCreatorAgent:
    """The default: refuses, and says exactly why.

    Not a stub that returns an empty ChangeSet. An empty ChangeSet would flow
    through the rest of the pipeline and read as "the agent had nothing to
    add", which is a different claim from "no agent ran".
    """

    def __init__(self, reason: str = "") -> None:
        self.reason = reason or (
            "no coding agent is configured. ModCheck invokes no model by "
            "default; pass one explicitly.")

    def implement(self, task: AgentTask) -> ChangeSet:
        raise AgentUnavailable(self.reason)


class AnthropicCreatorAgent:
    """A coding agent backed by the Anthropic API.

    NOT EXECUTED FROM THIS ENVIRONMENT. Written against the documented SDK
    surface; `availability()` reports what is missing, and `implement` raises
    the same blockers rather than half-working.
    """

    def __init__(self, model: str = DEFAULT_MODEL, *, max_tokens: int = 64000,
                 effort: str = "high") -> None:
        self.model = model
        self.max_tokens = max_tokens
        self.effort = effort

    @staticmethod
    def availability() -> Availability:
        blockers: list[str] = []
        try:
            import anthropic  # noqa: F401
        except ImportError:
            blockers.append(
                "the `anthropic` package is not installed. It is deliberately "
                "not a ModCheck dependency -- the core does not need it -- so "
                "installing it is an explicit choice: `pip install anthropic`")
        # An unset ANTHROPIC_API_KEY does not mean there are no credentials:
        # the SDK also resolves ANTHROPIC_AUTH_TOKEN and an `ant auth login`
        # profile, so all three are checked before reporting none.
        has_env = bool(os.environ.get("ANTHROPIC_API_KEY")
                       or os.environ.get("ANTHROPIC_AUTH_TOKEN"))
        has_profile = bool(shutil.which("ant")) and any(
            (Path.home() / ".config" / "anthropic").glob("*"))
        if not has_env and not has_profile:
            blockers.append(
                "no credential is resolvable: ANTHROPIC_API_KEY and "
                "ANTHROPIC_AUTH_TOKEN are unset and no `ant auth login` "
                "profile was found under ~/.config/anthropic")
        return Availability(ready=not blockers, blockers=blockers)

    def implement(self, task: AgentTask) -> ChangeSet:
        availability = self.availability()
        if not availability.ready:
            raise AgentUnavailable(availability.describe())

        import anthropic

        client = anthropic.Anthropic()
        # Streaming because a file-sized response with a large max_tokens
        # otherwise risks an HTTP timeout.
        with client.beta.messages.stream(
                model=self.model,
                max_tokens=self.max_tokens,
                thinking={"type": "adaptive"},
                output_config={"effort": self.effort},
                betas=["server-side-fallback-2026-07-01"],
                fallbacks="default",
                system=SYSTEM_PROMPT,
                messages=[{"role": "user", "content": render_task(task)}],
        ) as stream:
            message = stream.get_final_message()

        if message.stop_reason == "refusal":
            raise AgentUnavailable(
                "the model declined this request"
                + (f": {message.stop_details.category}" if message.stop_details else ""))

        text = "".join(block.text for block in message.content
                       if block.type == "text")
        return parse_changeset(text, task)


SYSTEM_PROMPT = """You implement one requirement of a Minecraft mod, in Java.

You are given the exact mechanism to use, read from the artifact the project
compiles against, with its signatures. Use those signatures. Do not use an API
you were not given: if the mechanism supplied does not cover what the
requirement needs, say so in a `note` and return no file for it rather than
inventing a method that does not exist.

Return ONLY a JSON object:
{"files": [{"path": "src/main/java/...", "content": "..."}], "notes": ["..."]}

Rules:
- Every file must be complete and compilable; no ellipses, no TODOs standing in
  for required behaviour.
- Do not delete or rewrite code you were not asked to change.
- If a requirement cannot be met with the mechanism given, return no file and
  explain in a note. An implementation that does not work is worse than none.
"""


def render_task(task: AgentTask) -> str:
    """The prompt body: the request, the requirement, the evidence, the files."""
    lines = [
        f"# Request: {task.request.title}",
        task.request.description.strip(),
        "",
        f"# Requirement: {task.resolution.requirement.id}",
        task.resolution.requirement.statement.strip(),
        "",
        f"Acceptance: {task.resolution.requirement.acceptance.strip()}",
        f"Runs on: {task.resolution.requirement.side}",
        "",
        "# The mechanism to use, and what establishes it",
    ]
    lines += [f"- {line}" for line in task.evidence_lines()]
    for key in ("requirements", "ordering", "limits"):
        values = (task.resolution.record.get(key) if task.resolution.record else None) or []
        if values:
            lines.append("")
            lines.append(f"# {key.title()}")
            lines += [f"- {v.strip()}" for v in values]
    if task.context_files:
        lines.append("")
        lines.append("# Files already in the project")
        for path, content in task.context_files.items():
            lines.append(f"## {path}")
            lines.append("```java")
            lines.append(content)
            lines.append("```")
    if task.instruction:
        lines += ["", "# Additional instruction", task.instruction.strip()]
    return "\n".join(lines)


def parse_changeset(text: str, task: AgentTask) -> ChangeSet:
    """Turn a model response into a ChangeSet, or fail loudly.

    Nothing is written here. The ChangeSet goes through the same review and
    build the deterministic generators' output does.
    """
    start, end = text.find("{"), text.rfind("}")
    if start < 0 or end <= start:
        raise AgentUnavailable("the agent returned no JSON object")
    try:
        payload = json.loads(text[start:end + 1])
    except json.JSONDecodeError as exc:
        raise AgentUnavailable(f"the agent's JSON did not parse: {exc}") from exc

    changes: list[FileChange] = []
    for entry in payload.get("files") or []:
        path, content = entry.get("path"), entry.get("content")
        if not path or content is None:
            raise AgentUnavailable(f"an agent file entry is incomplete: {entry!r}")
        if Path(path).is_absolute() or ".." in Path(path).parts:
            # A path that escapes the project is refused rather than sanitised:
            # quietly relocating it would hide what was attempted.
            raise AgentUnavailable(f"refusing a path outside the project: {path}")
        existing = task.project / path
        before = existing.read_text(encoding="utf-8") if existing.exists() else None
        changes.append(FileChange(path=path, before=before, after=content))

    requirement_id = (task.resolution.requirement.id
                      if task.resolution is not None else "unknown")
    mechanism_id = (task.resolution.record.id
                    if task.resolution is not None and task.resolution.record
                    else "unknown")
    return ChangeSet(
        generator=f"agent:{requirement_id}",
        description=f"agent implementation of {requirement_id} using {mechanism_id}",
        changes=changes,
        notes=[*(payload.get("notes") or []),
               "Produced by a coding agent, not by a deterministic generator. "
               "Reviewed and built on the same path as generated code."],
        follow_up=["review the diff before writing it",
                   "run the project's build to confirm it compiles"])

"""The agent boundary: callable, exercised, and honest about being unavailable.

No network. A fake agent stands in for a real one so the wiring is tested --
that an agent is reached only where no generator targets the mechanism, that
what it returns goes through the same ChangeSet path, and that its output is
labelled as its own rather than blended into generated code.
"""
from __future__ import annotations

import json

import pytest

from modcheck.creator.agent import (AgentTask, AgentUnavailable,
                                    AnthropicCreatorAgent, NullCreatorAgent,
                                    parse_changeset, render_task)
from modcheck.creator.apply import ChangeSet, FileChange
from modcheck.creator.pipeline import run
from modcheck.creator.plan import Request, build
from modcheck.paths import project_root
from modcheck.store import Store

PROJECT = project_root() / "reference" / "rainlantern"
needs_project = pytest.mark.skipif(
    not (PROJECT / "src" / "main" / "resources" / "fabric.mod.json").exists(),
    reason="the reference project is not present")


class FakeAgent:
    """Records what it was asked and returns a fixed file."""

    def __init__(self, response: str | None = None, fail: str | None = None):
        self.tasks: list[AgentTask] = []
        self.fail = fail
        self.response = response or json.dumps({
            "files": [{"path": "src/main/java/com/example/Fake.java",
                       "content": "// from the agent\n"}],
            "notes": ["a note from the agent"]})

    def implement(self, task: AgentTask) -> ChangeSet:
        self.tasks.append(task)
        if self.fail:
            raise AgentUnavailable(self.fail)
        return parse_changeset(self.response, task)


# -- availability is reported, not discovered at call time -----------------

def test_availability_names_the_specific_blockers():
    availability = AnthropicCreatorAgent.availability()
    assert not availability.ready, "this environment has neither the SDK nor a key"
    joined = availability.describe()
    assert "anthropic` package is not installed" in joined
    assert "no credential is resolvable" in joined


def test_an_unavailable_agent_raises_rather_than_returning_nothing():
    agent = AnthropicCreatorAgent()
    with pytest.raises(AgentUnavailable, match="unavailable"):
        agent.implement(AgentTask(request=None, resolution=None, project=PROJECT))


def test_the_null_agent_refuses_instead_of_returning_an_empty_changeset():
    """An empty ChangeSet would read as 'the agent had nothing to add'."""
    with pytest.raises(AgentUnavailable, match="no coding agent is configured"):
        NullCreatorAgent().implement(
            AgentTask(request=None, resolution=None, project=PROJECT))


# -- the prompt carries the planner's decision, not a guess ----------------

@needs_project
def test_the_task_carries_the_selected_mechanism_and_its_evidence():
    plan = build(Request.load(project_root() / "evaluation" / "requests"
                              / "rain_charged_lantern_v2.yaml"), Store())
    resolution = next(r for r in plan.resolutions
                      if r.requirement.id == "retain_charge_across_reload")
    task = AgentTask(request=plan.request, resolution=resolution, project=PROJECT)
    lines = "\n".join(task.evidence_lines())
    assert "AttachmentRegistry.createPersistent" in lines
    assert "evidence fabric_data_attachment_api_v1_jar" in lines

    prompt = render_task(task)
    assert resolution.requirement.statement.strip()[:30] in prompt
    assert "createPersistent" in prompt
    assert "Acceptance:" in prompt


# -- what comes back goes through the same path ----------------------------

def test_a_returned_file_becomes_a_reviewable_change(tmp_path):
    task = AgentTask(request=None, resolution=None, project=tmp_path)
    changeset = parse_changeset(
        '{"files":[{"path":"src/A.java","content":"class A {}"}],"notes":["n"]}', task)
    assert len(changeset.changes) == 1
    assert changeset.changes[0].created
    assert "+class A {}" in changeset.diff()
    assert any("not by a deterministic generator" in n for n in changeset.notes)


def test_a_path_escaping_the_project_is_refused_not_sanitised(tmp_path):
    task = AgentTask(request=None, resolution=None, project=tmp_path)
    for bad in ("/etc/passwd", "../outside.java", "src/../../x.java"):
        with pytest.raises(AgentUnavailable, match="outside the project"):
            parse_changeset(json.dumps({"files": [{"path": bad, "content": "x"}]}), task)


def test_unparsable_output_fails_loudly(tmp_path):
    task = AgentTask(request=None, resolution=None, project=tmp_path)
    with pytest.raises(AgentUnavailable, match="no JSON object"):
        parse_changeset("I could not do that.", task)
    with pytest.raises(AgentUnavailable, match="did not parse"):
        parse_changeset("{not json}", task)


def test_an_incomplete_file_entry_is_refused(tmp_path):
    task = AgentTask(request=None, resolution=None, project=tmp_path)
    with pytest.raises(AgentUnavailable, match="incomplete"):
        parse_changeset('{"files":[{"path":"src/A.java"}]}', task)


# -- the pipeline reaches an agent only where it must ----------------------

@needs_project
def test_no_agent_is_called_where_a_generator_exists():
    agent = FakeAgent()
    outcome = run(Request.load(project_root() / "evaluation" / "requests"
                               / "rain_charged_lantern_v2.yaml"),
                  PROJECT, naming={"name": "probe"}, agent=agent)
    assert agent.tasks == [], "every mechanism here has a deterministic generator"
    assert all(s.produced_by == "generator" for s in outcome.wired)


@needs_project
def test_an_agent_is_called_where_no_generator_targets_the_mechanism():
    """Project Zomboid records have no Java generator."""
    agent = FakeAgent()
    outcome = run(Request.load(project_root() / "evaluation" / "requests"
                               / "pz_generator_sandbox.yaml"),
                  PROJECT, naming={"name": "probe"}, agent=agent)
    assert agent.tasks, "the agent must be offered the mechanisms nothing wires"
    assert any(s.produced_by == "agent" for s in outcome.wired)


@needs_project
def test_agent_output_is_labelled_separately_from_generated_output():
    agent = FakeAgent()
    outcome = run(Request.load(project_root() / "evaluation" / "requests"
                               / "pz_generator_sandbox.yaml"),
                  PROJECT, naming={"name": "probe"}, agent=agent)
    payload = outcome.as_dict()
    assert payload["produced_by"]["agent"], "agent work must be attributable"
    assert not set(payload["produced_by"]["agent"]) & set(
        payload["produced_by"]["generator"]), "the two must not overlap"


@needs_project
def test_an_agent_that_declines_is_reported_not_silently_skipped():
    agent = FakeAgent(fail="the mechanism supplied does not cover this")
    outcome = run(Request.load(project_root() / "evaluation" / "requests"
                               / "pz_generator_sandbox.yaml"),
                  PROJECT, naming={"name": "probe"}, agent=agent)
    declined = [s for s in outcome.skipped if s.produced_by == "agent"]
    assert declined
    assert "does not cover this" in declined[0].skipped_because


@needs_project
def test_without_an_agent_nothing_changes():
    outcome = run(Request.load(project_root() / "evaluation" / "requests"
                               / "pz_generator_sandbox.yaml"),
                  PROJECT, naming={"name": "probe"})
    assert outcome.wired == []
    assert all(s.produced_by == "generator" for s in outcome.steps)

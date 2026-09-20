"""The task-specific evidence gate, and the composition check.

Two things are being held here.

First, per requirement: a mechanism is selected only when an eligible record
supplies exact targets, and selection is governed by fitness before evidence
strength. Picking the build-tested client mechanism for a server-side
requirement is the stronger evidence for the wrong thing.

Second, across requirements: individually grounded parts do not make a working
whole. A value produced on the server and read on the client needs something
to carry it, and both halves compile perfectly without it.
"""
from __future__ import annotations

import pytest

from modcheck.creator.plan import (GROUNDED, REQUIRES_INVESTIGATION, UNSUPPORTED,
                                   Request, build, render)
from modcheck.paths import project_root
from modcheck.store import Store

REQUESTS = project_root() / "evaluation" / "requests"


def _request(**overrides):
    data = {
        "id": "t", "game": "minecraft", "title": "T", "game_version": "26.3",
        "loader": "fabric",
        "requirements": [{"id": "r1", "statement": "s", "capability": "persist_state",
                          "side": "server", "scope": "level"}],
    }
    data.update(overrides)
    return Request.from_dict(data)


def _only(request):
    return build(request, Store()).resolutions[0]


# -- the gate ---------------------------------------------------------------

def test_a_grounded_requirement_names_an_exact_mechanism():
    resolution = _only(_request())
    assert resolution.status == GROUNDED
    assert resolution.record.id == "persist_state.fabric_saveddata"
    ids = [m["id"] for m in resolution.mechanism]
    assert "net.minecraft.world.level.storage.SavedDataStorage.computeIfAbsent" in ids
    assert resolution.evidence, "a grounded choice must cite what establishes it"


def test_an_unknown_capability_is_rejected_rather_than_ignored():
    with pytest.raises(ValueError, match="unknown capability"):
        _request(requirements=[{"id": "r", "statement": "s",
                                "capability": "teleport_the_player"}])


def test_a_capability_with_no_record_names_the_missing_fact_and_a_procedure():
    resolution = _only(_request(requirements=[
        {"id": "r", "statement": "s", "capability": "compose_asset"}]))
    assert resolution.status == REQUIRES_INVESTIGATION
    assert "compose_asset" in resolution.missing_fact
    assert "modcheck lookup" in resolution.procedure, (
        "a missing fact must come with the smallest step that would settle it")


def test_a_wrong_version_makes_a_record_ineligible_not_lower_ranked():
    resolution = _only(_request(game_version="1.16.5"))
    assert resolution.status == REQUIRES_INVESTIGATION
    assert any("excludes 1.16.5" in line for line in resolution.ineligible)


def test_a_wrong_loader_makes_a_record_ineligible():
    resolution = _only(_request(loader="forge"))
    assert resolution.status == REQUIRES_INVESTIGATION
    assert any("not 'forge'" in line for line in resolution.ineligible)


# -- side fitness governs selection ----------------------------------------

def test_side_fitness_beats_evidence_strength_in_selection():
    """The defect this closed: a client entrypoint chosen for server work.

    subscribe_event has two eligible records here -- a build_tested client
    entrypoint and a source_confirmed server tick event. For a server-side
    requirement the weaker-evidence one is the right one.
    """
    resolution = _only(_request(requirements=[
        {"id": "r", "statement": "s", "capability": "subscribe_event",
         "side": "server"}]))
    assert resolution.status == GROUNDED
    assert resolution.record.id == "subscribe_event.fabric_server_tick"
    assert resolution.evidence_state == "source_confirmed"
    assert any("client" in line for line in resolution.ineligible), (
        "the rejected client mechanism must be reported, not silently dropped")


def test_the_client_mechanism_is_still_selected_for_client_work():
    resolution = _only(_request(requirements=[
        {"id": "r", "statement": "s", "capability": "subscribe_event",
         "side": "client"}]))
    assert resolution.record.id == "subscribe_event.fabric_add_client_entrypoint"


def test_an_undeclared_side_cannot_satisfy_a_side_specific_requirement(tmp_path):
    """Silence about runs_on is not a match."""
    resolution = _only(_request(requirements=[
        {"id": "r", "statement": "s", "capability": "modify_behaviour",
         "side": "client"}]))
    # The mixin record declares no runs_on.
    assert resolution.status == REQUIRES_INVESTIGATION
    assert "which side" in resolution.missing_fact
    assert "runs_on" in resolution.procedure


def test_a_side_agnostic_requirement_accepts_an_undeclared_record():
    resolution = _only(_request(requirements=[
        {"id": "r", "statement": "s", "capability": "modify_behaviour",
         "side": "either"}]))
    assert resolution.status == GROUNDED


def test_a_genuine_side_conflict_is_unsupported_not_investigate():
    """A declared mismatch is an answer, not a gap."""
    resolution = _only(_request(requirements=[
        {"id": "r", "statement": "s", "capability": "persist_state",
         "side": "client"}]))
    assert resolution.status == UNSUPPORTED
    assert "runs on the server" in resolution.conflict
    assert "synchronisation" in resolution.conflict


# -- composition ------------------------------------------------------------

def _cross_side_request(**overrides):
    return _request(requirements=[
        {"id": "store", "statement": "keep it", "capability": "persist_state",
         "side": "server", "scope": "level"},
        {"id": "show", "statement": "show it", "capability": "display_information",
         "side": "client", "reads_from": ["store"]},
    ], **overrides)


def test_a_cross_side_read_is_caught_even_when_both_parts_are_grounded():
    plan = build(_cross_side_request(), Store())
    assert all(r.status == GROUNDED for r in plan.resolutions), (
        "both halves must be individually fine -- that is the point")
    assert plan.composition
    issue = plan.composition[0]
    assert issue.kind == "cross_side_read"
    assert issue.needed_capability == "sync_state"
    assert "nothing carries the value across" in issue.detail


def test_a_flow_is_only_checked_where_the_request_declares_it():
    """Inferring joins from wording would invent ones nobody asked for."""
    plan = build(_request(requirements=[
        {"id": "store", "statement": "keep it", "capability": "persist_state",
         "side": "server", "scope": "level"},
        {"id": "show", "statement": "show it", "capability": "display_information",
         "side": "client"},
    ]), Store())
    assert plan.composition == []


def test_a_reads_from_naming_nothing_is_reported():
    plan = build(_request(requirements=[
        {"id": "show", "statement": "s", "capability": "display_information",
         "side": "client", "reads_from": ["does_not_exist"]}]), Store())
    assert plan.composition[0].kind == "unknown_producer"


def test_same_side_flows_raise_no_issue():
    plan = build(_request(requirements=[
        {"id": "tick", "statement": "t", "capability": "subscribe_event",
         "side": "server"},
        {"id": "store", "statement": "s", "capability": "persist_state",
         "side": "server", "scope": "level", "reads_from": ["tick"]}]), Store())
    assert plan.composition == []


# -- the real request -------------------------------------------------------

def test_the_lantern_request_is_fully_grounded_including_its_join():
    """The request that had no matching recipe in the library."""
    plan = build(Request.load(REQUESTS / "rain_charged_lantern.yaml"), Store())
    assert len(plan.resolutions) == 4
    assert all(r.status == GROUNDED for r in plan.resolutions), [
        (r.requirement.id, r.status) for r in plan.blocked]
    assert plan.composition and all(i.status == GROUNDED for i in plan.composition)
    assert plan.ready


def test_ready_requires_the_joins_too_not_just_the_parts():
    from modcheck.creator.plan import CompositionIssue, Plan
    plan = build(_request(), Store())
    assert plan.ready
    plan.composition.append(CompositionIssue(
        consumer="a", producer="b", kind="cross_side_read", detail="x",
        needed_capability="sync_state", status=REQUIRES_INVESTIGATION))
    assert not plan.ready, "an unresolved join must block readiness"


def test_render_separates_grounded_from_tested():
    text = render(build(Request.load(REQUESTS / "rain_charged_lantern.yaml"), Store()))
    assert "does NOT mean the behaviour has been tested" in text
    assert "requirement to evidence" in text


# -- scope: the revision's lesson ------------------------------------------
#
# A revision changed an accepted requirement from one charge per world to one
# per lantern. Both are persist_state, and the gate could not tell them apart:
# it selected the level-scoped SavedData and reported the plan ready. Code
# written against it compiles, and every lantern reads the same number.

def test_capability_alone_cannot_distinguish_world_from_per_object_state():
    """Without scope, the wrong mechanism is selected and nothing objects."""
    store = Store()
    records = [r for r in store.pack("minecraft").records("capability")
               if r.get("capability") == "persist_state"]
    assert len(records) >= 2, "there must be more than one persist_state mechanism"
    scopes = {r.get("scope") for r in records}
    assert "level" in scopes and "block" in scopes, (
        "the two mechanisms differ by scope, not by capability")


def test_a_block_scoped_requirement_rejects_the_level_scoped_mechanism():
    resolution = _only(_request(requirements=[
        {"id": "r", "statement": "each lantern keeps its own", "side": "server",
         "capability": "persist_state", "scope": "level"}]))
    assert resolution.record.id == "persist_state.fabric_saveddata"

    resolution = _only(_request(requirements=[
        {"id": "r", "statement": "each lantern keeps its own", "side": "server",
         "capability": "persist_state", "scope": "block"}]))
    assert resolution.record.id == "persist_state.fabric_attachment"


def test_an_unspecified_scope_over_differently_scoped_mechanisms_is_ambiguous():
    """Two equally established mechanisms that are not interchangeable.

    Picking one by declaration order would answer a question the creator never
    answered -- and recording the block-scoped attachment would retroactively
    have changed what the first lantern request planned against.
    """
    resolution = _only(_request(requirements=[
        {"id": "r", "statement": "s", "capability": "persist_state",
         "side": "server"}]))
    assert resolution.status == REQUIRES_INVESTIGATION
    assert "not\n                    \"interchangeable" in resolution.missing_fact or \
           "interchangeable" in resolution.missing_fact
    assert len(resolution.alternatives) == 2
    assert "Declare `scope:`" in resolution.procedure


def test_a_scope_conflict_explains_the_symptom_not_just_the_mismatch():
    from modcheck.creator.plan import _scope_conflict
    store = Store()
    record = store.pack("minecraft").record("capability",
                                            "persist_state.fabric_saveddata")
    from modcheck.creator.plan import Requirement
    requirement = Requirement(id="r", statement="s", capability="persist_state",
                              side="server", scope="entity")
    message = _scope_conflict(record, requirement)
    assert "one value per entity" in message and "one per level" in message
    assert "every entity reads the same number" in message


def test_the_revision_plans_against_the_new_mechanism():
    plan = build(Request.load(REQUESTS / "rain_charged_lantern_v2.yaml"), Store())
    assert plan.ready
    storage = next(r for r in plan.resolutions
                   if r.requirement.id == "retain_charge_across_reload")
    assert storage.record.id == "persist_state.fabric_attachment"
    assert storage.requirement.scope == "block"


def test_the_first_version_still_plans_against_the_old_one():
    """The revision must not retroactively change what v1 asked for."""
    plan = build(Request.load(REQUESTS / "rain_charged_lantern.yaml"), Store())
    storage = next(r for r in plan.resolutions
                   if r.requirement.id == "retain_charge_across_reload")
    assert storage.record.id == "persist_state.fabric_saveddata"


# -- the planner's decision must survive into emission ----------------------

def test_a_join_carries_the_record_the_planner_selected():
    """Re-looking-up at emission time re-ran the choice without the checks."""
    plan = build(Request.load(REQUESTS / "rain_charged_lantern_v2.yaml"), Store())
    issue = next(i for i in plan.composition if i.needed_capability)
    assert issue.record is not None
    assert issue.record.id == "sync_state.fabric_custom_payload"
    assert issue.record.get("capability") == "sync_state"


def test_a_join_cannot_reach_a_record_a_requirement_could_not():
    """Same eligibility for an implied capability as for a declared one."""
    request = Request.from_dict({
        "id": "t", "game": "minecraft", "title": "T",
        "game_version": "1.16.5", "loader": "fabric",
        "requirements": [
            {"id": "store", "statement": "s", "capability": "persist_state",
             "side": "server", "scope": "level"},
            {"id": "show", "statement": "s", "capability": "display_information",
             "side": "client", "reads_from": ["store"]}]})
    plan = build(request, Store())
    issue = next((i for i in plan.composition if i.needed_capability), None)
    if issue is not None:
        assert issue.record is None, (
            "a version the requirements are ineligible for must not be reachable "
            "through a join")
        assert issue.status == REQUIRES_INVESTIGATION


def test_a_requirement_can_pin_an_exact_mechanism():
    resolution = _only(_request(requirements=[
        {"id": "r", "statement": "s", "capability": "persist_state",
         "side": "server", "mechanism": "persist_state.fabric_attachment"}]))
    assert resolution.status == GROUNDED
    assert resolution.record.id == "persist_state.fabric_attachment"


def test_pinning_a_mechanism_that_does_not_exist_is_reported():
    resolution = _only(_request(requirements=[
        {"id": "r", "statement": "s", "capability": "persist_state",
         "side": "server", "mechanism": "persist_state.invented"}]))
    assert resolution.status == REQUIRES_INVESTIGATION
    assert "pins mechanism" in resolution.missing_fact


def test_two_independent_state_requirements_do_not_collapse():
    """Two stores asked for, two stores emitted."""
    from modcheck.creator.pipeline import run
    from modcheck.paths import project_root
    project = project_root() / "build_workspaces" / "lantern"
    if not (project / "src" / "main" / "resources" / "fabric.mod.json").exists():
        pytest.skip("scaffolded project not present")
    request = Request.from_dict({
        "id": "two", "game": "minecraft", "title": "Two", "game_version": "26.3",
        "loader": "fabric",
        "requirements": [
            {"id": "a", "statement": "first", "capability": "persist_state",
             "side": "server", "scope": "level"},
            {"id": "b", "statement": "second", "capability": "persist_state",
             "side": "server", "scope": "block"}]})
    outcome = run(request, project, naming={"name": "twostore"})
    paths = [f for step in outcome.wired for f in step.files]
    assert len(paths) == 2, paths
    assert len(set(paths)) == 2, "the second emission must not overwrite the first"

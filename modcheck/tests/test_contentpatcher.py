"""T4: explaining which Content Patcher patch wins, and why.

These exercise the analysis on actual content-pack instructions -- Priority,
When, LogName and patch order out of content.json -- not on manifests. The
governing rule is Content Patcher's own, recorded as source
`content_patcher_load`.

Evidence tier: these packs are synthetic fixtures carrying instructions in the
documented shape. The rule they are checked against is upstream's published
documentation (tier: published metadata / reference-source), and the asset name
used is Content Patcher's own example. No published content pack was obtained.
"""
from __future__ import annotations

import pytest

from fixtures import build
from modcheck.analyze import collisions
from modcheck.analyze import contentpatcher as cp
from modcheck.analyze.config import Installation, InstalledArtifact
from modcheck.inspect import inspect_path
from modcheck.report import analyze_installation, render_creator, render_player

ASSET = "Portraits/Abigail"     # Content Patcher's own documentation example


def pack(tmp_path, name, changes):
    return build.content_patcher_pack_with_changes(tmp_path / f"{name}.zip", name, changes)


def load_patch(target=ASSET, priority=None, log_name=None, when=None,
               from_file="assets/x.png"):
    change = {"Action": "Load", "Target": target, "FromFile": from_file}
    if priority is not None:
        change["Priority"] = priority
    if log_name:
        change["LogName"] = log_name
    if when:
        change["When"] = when
    return change


def install(*paths, game="stardewvalley") -> Installation:
    inst = Installation(game=game, files_known_complete=True)
    for index, path in enumerate(paths):
        ins = inspect_path(path, game=game)
        inst.artifacts.append(InstalledArtifact(
            name=path.name, path=str(path), sha256=ins.sha256, load_index=index,
            mod_id=ins.fact("mod_id"), version=ins.fact("version"), inspection=ins,
            declared_dependencies=ins.fact("dependencies") or []))
    return inst


def cp_findings(inst):
    return [f for f in collisions.analyze(inst) if f.code.startswith("contentpatcher.")]


# --- priority grammar, from the published JSON schema --------------------
@pytest.mark.parametrize("raw,kind,value", [
    (None, "exclusive", None),          # the documented default
    ("Exclusive", "exclusive", None),
    ("exclusive", "exclusive", None),   # capitalisation does not matter
    ("Low", "numeric", -1000),
    ("Medium", "numeric", 0),
    ("High", "numeric", 1000),
    ("High + 2", "numeric", 1002),
    ("Medium - 10", "numeric", -10),
    ("nonsense", "unparsed", None),
])
def test_load_priority_grammar(raw, kind, value):
    parsed = cp.parse_load_priority(raw)
    assert parsed.kind == kind
    assert parsed.value == value


# --- case 1: a confirmed conflict ---------------------------------------
def test_two_exclusive_loads_are_a_confirmed_conflict_with_locations(tmp_path):
    a = pack(tmp_path, "Aria.Portraits", [load_patch(log_name="Abigail portrait")])
    b = pack(tmp_path, "Bex.Seasonal", [
        load_patch(target="Portraits/Penny"),
        load_patch(log_name="Abigail winter")])
    findings = cp_findings(install(a, b))

    assert [f.code for f in findings] == ["contentpatcher.exclusive_conflict"]
    finding = findings[0]
    assert finding.severity == "error"
    assert finding.targets[0]["id"] == ASSET
    assert finding.sources == ["content_patcher_load"]
    # Exact patch locations, including the patch index inside each content.json.
    assert "Aria.Portraits/content.json #0 (Abigail portrait)" in finding.detail
    assert "Bex.Seasonal/content.json #1 (Abigail winter)" in finding.detail
    # The consequence, stated: neither applies.
    assert "NONE of them" in finding.summary


def test_conflict_recommendations_preserve_intent_rather_than_suppress(tmp_path):
    """A fix that just silences one author is not a fix."""
    a = pack(tmp_path, "Aria.Portraits", [load_patch()])
    b = pack(tmp_path, "Bex.Portraits", [load_patch()])
    finding = cp_findings(install(a, b))[0]

    steps = " ".join(r["step"] for r in finding.resolutions)
    assert "Priority" in steps, "must offer the priority route"
    assert "Edit action instead of Load" in steps, "must offer the composing route"
    assert "both authors' intent survives" in steps
    assert not any(word in steps.lower() for word in ("delete the", "uninstall"))


def test_default_priority_is_exclusive_so_an_unset_priority_still_conflicts(tmp_path):
    """The default is what makes this the common real-world failure."""
    a = pack(tmp_path, "Aria.X", [load_patch(priority=None)])
    b = pack(tmp_path, "Bex.X", [load_patch(priority="Exclusive")])
    findings = cp_findings(install(a, b))
    assert [f.code for f in findings] == ["contentpatcher.exclusive_conflict"]
    assert "Exclusive is the default" in findings[0].detail


# --- case 2: an intentional priority selection --------------------------
def test_declared_priorities_are_a_selection_not_a_conflict(tmp_path):
    a = pack(tmp_path, "Aria.Base", [load_patch(priority="Medium", log_name="base")])
    b = pack(tmp_path, "Bex.Override", [load_patch(priority="High", log_name="hd")])
    findings = cp_findings(install(a, b))

    assert [f.code for f in findings] == ["contentpatcher.priority_selection"]
    finding = findings[0]
    assert finding.severity == "note", "a deliberate priority must not read as a failure"
    assert "Bex.Override" in finding.summary and "High" in finding.summary
    assert "not a\n" in finding.detail or "not a conflict" in finding.detail
    assert finding.resolutions == [], "a working selection needs no remedy"


def test_offset_priority_decides_the_winner(tmp_path):
    a = pack(tmp_path, "Aria.X", [load_patch(priority="High")])
    b = pack(tmp_path, "Bex.X", [load_patch(priority="High + 2")])
    finding = cp_findings(install(a, b))[0]
    assert finding.code == "contentpatcher.priority_selection"
    assert "Bex.X" in finding.summary


def test_one_exclusive_against_a_prioritised_patch_supersedes_it(tmp_path):
    """Documented: one Exclusive patch applies and all other loads are ignored."""
    a = pack(tmp_path, "Aria.Exclusive", [load_patch(log_name="exclusive one")])
    b = pack(tmp_path, "Bex.Polite", [load_patch(priority="High", log_name="polite")])
    findings = cp_findings(install(a, b))

    assert [f.code for f in findings] == ["contentpatcher.exclusive_supersedes"]
    finding = findings[0]
    assert finding.severity == "warning"
    assert "ignored" in finding.summary
    # The player is told which pack silently does nothing.
    assert "Bex.Polite" in finding.detail


# --- case 3: unresolved, with the missing input named -------------------
def test_equal_priority_is_unresolved_and_names_the_missing_input(tmp_path):
    a = pack(tmp_path, "Aria.X", [load_patch(priority="High", log_name="a")])
    b = pack(tmp_path, "Bex.X", [load_patch(priority="High", log_name="b")])
    findings = cp_findings(install(a, b))

    assert [f.code for f in findings] == ["contentpatcher.competition_unresolved"]
    finding = findings[0]
    assert finding.severity == "unresolved"
    assert finding.evidence_class == "unresolved"
    assert "load order" in finding.not_established
    assert "dependency-sorted" in finding.not_established
    # It still shows what it does know: both locations.
    assert "Aria.X/content.json #0" in finding.detail
    assert "Bex.X/content.json #0" in finding.detail


def test_conditional_patches_are_unresolved_not_assumed_to_collide(tmp_path):
    """Two patches guarded by conditions may never be live together."""
    a = pack(tmp_path, "Aria.X", [load_patch(when={"Season": "winter"}, log_name="w")])
    b = pack(tmp_path, "Bex.X", [load_patch(when={"Season": "summer"}, log_name="s")])
    findings = cp_findings(install(a, b))

    assert [f.code for f in findings] == ["contentpatcher.competition_unresolved"]
    finding = findings[0]
    assert finding.conditions, "the conditions themselves must be reported"
    assert any("Season" in str(c) for c in finding.conditions)
    assert "game state" in finding.unresolved_because if hasattr(finding, "unresolved_because") \
        else "game state" in finding.detail


def test_unparsable_priority_is_unresolved_rather_than_treated_as_default(tmp_path):
    a = pack(tmp_path, "Aria.X", [load_patch(priority="Highest")])
    b = pack(tmp_path, "Bex.X", [load_patch(priority="High")])
    finding = cp_findings(install(a, b))[0]
    assert finding.code == "contentpatcher.competition_unresolved"
    assert "cannot parse" in finding.detail


# --- controls -----------------------------------------------------------
def test_control_different_assets_never_compete(tmp_path):
    a = pack(tmp_path, "Aria.X", [load_patch(target="Portraits/Abigail")])
    b = pack(tmp_path, "Bex.X", [load_patch(target="Portraits/Penny")])
    assert cp_findings(install(a, b)) == []


def test_control_two_patches_in_one_pack_are_not_a_cross_mod_interaction(tmp_path):
    """An author's own ordering inside their pack is their business."""
    a = pack(tmp_path, "Aria.X", [load_patch(log_name="one"), load_patch(log_name="two")])
    assert cp_findings(install(a)) == []


def test_control_a_single_pack_loading_an_asset_is_silent(tmp_path):
    a = pack(tmp_path, "Aria.X", [load_patch()])
    assert cp_findings(install(a)) == []


def test_control_edit_patches_do_not_produce_load_findings(tmp_path):
    a = pack(tmp_path, "Aria.X", [{"Action": "EditData", "Target": "Data/Objects",
                                   "Fields": {"128": {"Price": 100}}}])
    b = pack(tmp_path, "Bex.X", [{"Action": "EditData", "Target": "Data/Objects",
                                  "Fields": {"129": {"Price": 200}}}])
    assert cp_findings(install(a, b)) == []


# --- the two presentations ----------------------------------------------
def test_player_and_creator_views_come_from_the_same_findings(tmp_path):
    a = pack(tmp_path, "Aria.X", [load_patch(log_name="mine")])
    b = pack(tmp_path, "Bex.X", [load_patch(log_name="theirs")])
    report = analyze_installation(install(a, b))

    player = render_player(report)
    creator = render_creator(report)

    # Same conclusion, different foreground.
    assert "applies NONE of them" in player and "applies NONE of them" in creator
    assert "your options:" in player
    assert "content.json #0 (mine)" in creator, "creator view locates the patch"
    assert "content.json #0 (mine)" not in player, "player view does not need indices"
    assert "implementation alternatives:" in creator
    assert "not a guarantee" in player


def test_player_view_labels_an_unresolved_case_as_cannot_tell(tmp_path):
    a = pack(tmp_path, "Aria.X", [load_patch(priority="High")])
    b = pack(tmp_path, "Bex.X", [load_patch(priority="High")])
    player = render_player(analyze_installation(install(a, b)))
    assert "[Cannot tell]" in player
    assert "not established" in player

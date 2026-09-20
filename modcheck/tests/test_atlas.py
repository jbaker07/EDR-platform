"""The Minecraft engineering atlas: extracted facts, authored records, vault, validator.

These tests read the committed extraction outputs under atlas/extracted and the
committed vault. They do not run javap (that needs the toolchain and Gradle
caches); the extractors' own unit-level behaviour is covered by parsing fixtures.
"""
from __future__ import annotations

import importlib.util
import json
import re
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
ATLAS = ROOT / "atlas"
EXTRACTED = ATLAS / "extracted"


def _load(name: str):
    spec = importlib.util.spec_from_file_location(f"atlas_{name}", ATLAS / "extract" / f"{name}.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


needs_extraction = pytest.mark.skipif(not (EXTRACTED / "edges.json").exists(), reason="atlas not extracted")
needs_vault = pytest.mark.skipif(not (ATLAS / "vault" / "_index.md").exists(), reason="vault not built")


# -- extractor parsing, on fixtures (no javap) ------------------------------------
JAVAP_MULTI = """Compiled from "Level.java"
public abstract class net.minecraft.world.level.Level implements net.minecraft.world.level.LevelAccessor {
  public boolean isRaining();
  public boolean isRainingAt(net.minecraft.core.BlockPos);
  protected final java.util.List<java.lang.Object> blockEntityTickers;
}
Compiled from "ServerLevel.java"
public class net.minecraft.server.level.ServerLevel extends net.minecraft.world.level.Level {
  public net.minecraft.world.level.gamerules.GameRules getGameRules();
  public void tick(java.util.function.BooleanSupplier);
}
"""


def test_vanilla_members_parses_multi_class_javap_output():
    vm = _load("vanilla_members")
    blocks = vm.parse_blocks(JAVAP_MULTI)
    assert set(blocks) == {"net.minecraft.world.level.Level", "net.minecraft.server.level.ServerLevel"}
    assert blocks["net.minecraft.world.level.Level"]["kind"] == "abstract_class"
    assert "public boolean isRaining()" in blocks["net.minecraft.world.level.Level"]["members"]
    names = vm.member_names(blocks["net.minecraft.server.level.ServerLevel"]["members"])
    assert names == {"getGameRules", "tick"}
    # a field is named by its last token, a method by the token before '('
    assert "blockEntityTickers" in vm.member_names(blocks["net.minecraft.world.level.Level"]["members"])


def test_reference_pattern_does_not_swallow_sentence_period():
    validate = _load("validate")
    m = validate.REF_RE.search("see question:q.payload_receiver_thread. Then")
    assert m.group(2) == "q.payload_receiver_thread"
    m = validate.REF_RE.search("event:net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_LEVEL_TICK,")
    assert m.group(2).endswith("END_LEVEL_TICK")


def test_certainty_regex_flags_positive_claims_only():
    validate = _load("validate")
    assert validate.CERTAINTY_RE.search("this was game-tested on a server")
    assert validate.NEGATION_RE.search("nothing here is game-tested")
    assert not validate.CERTAINTY_RE.search("compiles against the pinned corpus")
    assert validate.NUMERIC_CONFIDENCE_RE.search("we are 90% confident")


# -- extracted facts ----------------------------------------------------------------
@needs_extraction
def test_every_edge_cites_a_corpus_artifact_hash_and_no_edge_is_observed():
    corpus = json.loads((EXTRACTED / "corpus.json").read_text())
    hashes = {a["sha256"] for a in corpus["artifacts"]}
    edges = json.loads((EXTRACTED / "edges.json").read_text())
    assert edges["edges"], "no edges extracted"
    for e in edges["edges"]:
        assert e["provenance"]["artifact_sha256"] in hashes, e["id"]
        assert e["applies_to"]["game_versions"] == "26.3"
    assert edges["evidence_classes"].get("observed", 0) == 0
    assert set(edges["evidence_classes"]) <= {"direct_reference", "static_inference", "declared", "documented"}


@needs_extraction
def test_fabric_api_publishes_no_overwrite_and_edges_reflect_it():
    edges = json.loads((EXTRACTED / "edges.json").read_text())
    assert edges["counts"].get("replaces", 0) == 0
    assert edges["counts"]["injects_into"] > 400
    assert edges["counts"]["callback_of"] > 150
    fabric = json.loads((EXTRACTED / "fabric_api.json").read_text())
    injectors = {inj["injector"] for m in fabric["modules"] for mx in m["mixins"] for inj in mx["injections"]}
    assert "Overwrite" not in injectors


@needs_extraction
def test_known_publication_sites_are_exact():
    """The lantern's tick event and the server lifecycle events resolve to the sites the contracts cite."""
    edges = json.loads((EXTRACTED / "edges.json").read_text())["edges"]
    handlers = {(e["from"].get("owner"), e["from"]["id"]): e for e in edges if e["relation"] in ("injects_into", "wraps")}
    sites = {}
    for e in edges:
        if e["relation"] == "publishes_event":
            h = handlers.get((e["from"]["owner"], e["from"]["id"]))
            if h:
                sites.setdefault(e["to"]["id"].rsplit(".", 2)[-2] + "." + e["to"]["id"].rsplit(".", 1)[-1],
                                 (h["to"]["owner"], h["to"]["id"], h["operation"]))
    assert sites["ServerTickEvents.END_LEVEL_TICK"][:2] == ("net.minecraft.server.level.ServerLevel", "tick")
    assert "TAIL" in sites["ServerTickEvents.END_LEVEL_TICK"][2]
    assert sites["ServerTickEvents.END_SERVER_TICK"][:2] == ("net.minecraft.server.MinecraftServer", "tickServer")
    assert sites["ServerLifecycleEvents.SERVER_STOPPING"][:2] == ("net.minecraft.server.MinecraftServer", "stopServer")
    assert sites["ClientTickEvents.END_CLIENT_TICK"][:2] == ("net.minecraft.client.Minecraft", "tick")


@needs_extraction
def test_member_extraction_cross_checks_edge_targets():
    members = json.loads((EXTRACTED / "minecraft_members.json").read_text())
    assert members["types_extracted"] == members["types_requested"] - len(members["types_missing"])
    et = members["edge_targets"]
    assert et["resolved_on_type"] > 1000
    assert len(et["unresolved"]) < 100, "unresolved edge targets grew; see q.edge_targets_unresolved"
    level = members["types"]["net.minecraft.world.level.Level"]["members"]
    assert "public boolean isRaining()" in level and "public boolean isRainingAt(net.minecraft.core.BlockPos)" in level
    server_level = members["types"]["net.minecraft.server.level.ServerLevel"]["members"]
    assert any(m.endswith("getGameRules()") for m in server_level)


# -- validator and vault ------------------------------------------------------------
@needs_extraction
@needs_vault
def test_atlas_validates_with_no_errors():
    validate = _load("validate")
    rep = validate.run()
    assert rep.errors == [], "\n".join(rep.errors[:20])
    assert rep.counts["authored_workflow"] >= 24
    assert rep.counts["authored_question"] >= 30
    assert rep.counts["authored_request"] >= 6
    assert rep.counts["authored_contract"] >= 12
    assert rep.counts["notes"] > 1500 and rep.counts["wikilinks"] > 5000


@needs_vault
def test_every_note_has_frontmatter_then_exactly_one_banner():
    vault = ATLAS / "vault"
    gen = auth = 0
    for path in vault.rglob("*.md"):
        text = path.read_text()
        assert text.startswith("---\n"), path
        body = text.split("\n---\n", 1)[1]
        g, a = "> [!info] Generated" in body[:400], "> [!warning] Analyst-authored" in body[:400]
        assert g != a, path
        gen += g
        auth += a
    assert gen > 1000 and auth > 50


@needs_vault
def test_workflow_index_covers_all_eight_areas():
    idx = (ATLAS / "vault" / "10-Workflows" / "_index.md").read_text()
    for area in ("content", "world", "behaviour", "presentation", "state", "multiplayer", "integration", "engineering"):
        assert re.search(rf"^\| {area} \|", idx, re.M), area


@needs_vault
def test_coverage_note_reports_zero_runtime_evidence_and_denominators():
    cov = (ATLAS / "vault" / "90-Coverage" / "_index.md").read_text()
    assert "observed edges: 0. No game has run." in cov
    assert re.search(r"vanilla packages with at least one hook: \d+ / \d+", cov)
    assert re.search(r"events with an analyst-stated subscriber contract: \d+ / \d+", cov)
    assert "NOT implemented" in cov

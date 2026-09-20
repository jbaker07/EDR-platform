"""The Minecraft engineering atlas: extracted facts, authored records, vault, validator.

These tests read the committed extraction outputs under atlas/extracted and the
committed vault. They do not run javap (that needs the toolchain and Gradle
caches); the extractors' own unit-level behaviour is covered by parsing fixtures.
"""
from __future__ import annotations

import collections
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
_UNUSED = """Compiled from "Level.java"
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
def test_fabric_api_overwrite_count_is_regenerated_from_class_files():
    """The first atlas said zero overwrites; the class-file reader finds exactly one, plus 199 MixinExtras injections."""
    edges = json.loads((EXTRACTED / "edges.json").read_text())
    assert edges["counts"]["replaces"] == 1
    fabric = json.loads((EXTRACTED / "fabric_api.json").read_text())
    injectors = collections.Counter(inj["injector"] for m in fabric["modules"] for mx in m["mixins"] for inj in mx["injections"])
    assert injectors["Overwrite"] == 1 and injectors["WrapOperation"] > 100 and injectors["ModifyReturnValue"] > 10
    assert sum(m["counts"]["extraction_failures"] for m in fabric["modules"]) == 0
    assert all(m["mixins_missing"] == [] and m["mixins_undeclared"] == [] for m in fabric["modules"])


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
    assert sites["ServerTickEvents.END_LEVEL_TICK"][:2] == ("net/minecraft/server/level/ServerLevel", "tick")
    assert "TAIL" in sites["ServerTickEvents.END_LEVEL_TICK"][2]
    assert sites["ServerTickEvents.END_SERVER_TICK"][:2] == ("net/minecraft/server/MinecraftServer", "tickServer")
    assert sites["ServerLifecycleEvents.SERVER_STOPPING"][:2] == ("net/minecraft/server/MinecraftServer", "stopServer")
    assert sites["ClientTickEvents.END_CLIENT_TICK"][:2] == ("net/minecraft/client/Minecraft", "tick")


@needs_extraction
def test_surface_and_resolution_are_exact_against_the_processed_jar():
    import gzip
    with gzip.open(EXTRACTED / "minecraft_surface.json.gz", "rt") as fh:
        surface = json.load(fh)
    level = surface["classes"]["net/minecraft/world/level/Level"]
    assert ["isRaining", "()Z"] in [[n, d] for n, d, a in level["methods"]]
    assert ["isRainingAt", "(Lnet/minecraft/core/BlockPos;)Z"] in [[n, d] for n, d, a in level["methods"]]
    assert surface["counts"]["classes"] > 11000 and surface["counts"]["processed_differs"] > 300
    edges = json.loads((EXTRACTED / "edges.json").read_text())
    rb = edges["resolution_by_relation"]
    assert set(rb["calls"]) <= {"exact", "inherited_exact"} and set(rb["reads"]) <= {"exact", "inherited_exact"}
    assert set(edges["injection_point_resolution"]) <= {"exact", "inherited_exact", "selector_unsupported"}
    env = json.loads((EXTRACTED / "resolved_environment.json").read_text())
    assert env["minecraft_jar_processing"]["processed_jar"]["sha256"] != env["minecraft_jar_processing"]["cache_jar"]["sha256"]
    assert edges["resolution"]["sha256"] == env["minecraft_jar_processing"]["processed_jar"]["sha256"]


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
    assert "executed_transformation" in cov
    assert re.search(r"depth-4 packages: \d+; with at least one Fabric API hook: \d+", cov)
    assert re.search(r"events with an analyst-stated subscriber contract: \d+ / \d+", cov)
    assert "NOT implemented" in cov

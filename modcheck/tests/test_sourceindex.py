"""Exact retrieval, with byte-level provenance.

Two properties this must hold, because everything downstream cites it:

* A hit names bytes. A symbol hit carries the artifact's hash AND the class
  entry's hash; a text hit carries the hash of the file searched. A citation
  that names only a title cannot be re-checked.
* No fuzziness. An exact match in a named file at a named offset, or no hit.
  Nothing here scores relevance, and a near-miss is a miss.
"""
from __future__ import annotations

import hashlib
import zipfile

import pytest

from modcheck.creator.toolchain import find_java
from modcheck.sourceindex import EvidenceIndex, JarIndex
from modcheck.store import Store

MINECRAFT_JAR = "/root/.gradle/caches/fabric-loom/26.3/minecraft-merged.jar"


def _javap():
    java = find_java()
    return (java.path / "bin" / "javap") if java.path else None


@pytest.fixture(scope="module")
def jar(tmp_path_factory):
    """A tiny jar we build here, so the structural tests need no toolchain."""
    path = tmp_path_factory.mktemp("jars") / "sample.jar"
    with zipfile.ZipFile(path, "w") as zf:
        zf.writestr("com/example/Alpha.class", b"\xca\xfe\xba\xbe not-real")
        zf.writestr("com/example/Beta.class", b"\xca\xfe\xba\xbe also-not-real")
        zf.writestr("com/example/Beta$Inner.class", b"nested")
        zf.writestr("README.txt", b"not a class")
    return path


def test_class_listing_comes_from_the_archive_not_a_manifest(jar):
    index = JarIndex(jar)
    assert set(index.classes) == {"com.example.Alpha", "com.example.Beta"}, (
        "nested classes and non-class entries must not be listed as types")


def test_class_search_is_exact_regex_not_fuzzy(jar):
    index = JarIndex(jar)
    assert index.find_classes("Alpha") == ["com.example.Alpha"]
    assert index.find_classes("^com\\.example\\.B") == ["com.example.Beta"]
    assert index.find_classes("Gamma") == [], "a near-miss is a miss"


def test_the_artifact_hash_is_of_the_actual_bytes(jar):
    index = JarIndex(jar)
    assert index.sha256 == hashlib.sha256(jar.read_bytes()).hexdigest()


def test_a_missing_artifact_fails_loudly(tmp_path):
    with pytest.raises(FileNotFoundError):
        JarIndex(tmp_path / "nope.jar")


def test_an_unknown_class_returns_nothing_rather_than_guessing(jar):
    assert JarIndex(jar).symbol("com.example.Nope") == []


# -- against the real artifact we compile against --------------------------

real = pytest.mark.skipif(
    not __import__("pathlib").Path(MINECRAFT_JAR).is_file() or _javap() is None,
    reason="the resolved Minecraft artifact or a JDK is not present here")


@real
def test_signatures_come_from_the_artifact_the_build_compiles_against():
    """Documentation would have said something different, and been wrong."""
    index = JarIndex(MINECRAFT_JAR, javap=_javap())
    hits = index.symbol("net.minecraft.world.level.saveddata.SavedDataType")
    signatures = [h.signature for h in hits]
    constructor = next(s for s in signatures if "SavedDataType(" in s)
    # The 26.3 API is Codec-based. A record written from memory of the older
    # load/save(CompoundTag) API would compile against nothing.
    for expected in ("Identifier", "Supplier", "Codec", "DataFixTypes"):
        assert expected in constructor, constructor


@real
def test_the_accessor_is_the_one_the_jar_declares():
    index = JarIndex(MINECRAFT_JAR, javap=_javap())
    hits = index.member("net.minecraft.server.level.ServerLevel", "getDataStorage")
    assert hits, "ServerLevel must declare the accessor we plan to call"
    assert "SavedDataStorage" in hits[0].signature
    assert "DimensionDataStorage" not in hits[0].signature, (
        "the older name is not in this version; citing it would not compile")


@real
def test_every_symbol_hit_names_both_hashes():
    index = JarIndex(MINECRAFT_JAR, javap=_javap())
    for hit in index.symbol("net.minecraft.world.level.saveddata.SavedData"):
        assert len(hit.artifact_sha256) == 64
        assert len(hit.entry_sha256) == 64
        assert hit.entry.endswith(".class")
        assert hit.artifact_sha256[:12] in hit.citation()


# -- text retrieval --------------------------------------------------------

def test_a_text_hit_names_the_bytes_it_came_from():
    hits = EvidenceIndex(Store()).search("entrypoints", game="minecraft", limit=3)
    assert hits, "the Fabric manifest spec is in the cache"
    for hit in hits:
        assert len(hit.sha256) == 64
        assert hit.line > 0
        assert hit.source_id


def test_text_search_can_be_restricted_to_one_game():
    index = EvidenceIndex(Store())
    hits = index.search("entrypoints", game="rimworld", limit=5)
    assert all(h.source_id for h in hits)
    store = Store()
    rimworld_sources = set(store.pack("rimworld").sources)
    assert all(h.source_id in rimworld_sources for h in hits)


def test_a_phrase_that_is_not_in_the_cache_returns_nothing():
    hits = EvidenceIndex(Store()).search("zzz-not-in-any-source-zzz", limit=5)
    assert hits == []


def test_a_literal_search_does_not_interpret_regex_metacharacters():
    index = EvidenceIndex(Store())
    assert index.search("entry.oints", game="minecraft", limit=1) == []
    assert index.search("entry.oints", game="minecraft", limit=1, regex=True)

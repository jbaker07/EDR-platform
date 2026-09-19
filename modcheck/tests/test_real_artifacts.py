"""Checks against a really published mod, not a fixture we wrote ourselves.

Synthetic fixtures only prove a parser reads the layout it was written against.
This test pulls an actual release from its upstream maven -- Fabric API, which
is Apache-2.0 and freely redistributable -- and asserts the inspector reads it.

It is skipped, not failed, when the network is unavailable, so the suite still
runs offline. The artifact is cached under evidence_cache/ and never committed.
"""
from __future__ import annotations

import os

import pytest

from modcheck.acquire import FetchError, fetch
from modcheck.inspect import inspect_path

FABRIC_API_VERSION = "0.161.0+26.3"
FABRIC_API_URL = (
    "https://maven.fabricmc.net/net/fabricmc/fabric-api/fabric-api/"
    f"{FABRIC_API_VERSION.replace('+', '%2B')}/"
    f"fabric-api-{FABRIC_API_VERSION.replace('+', '%2B')}.jar"
)

offline = pytest.mark.skipif(
    os.environ.get("MODCHECK_OFFLINE") == "1",
    reason="MODCHECK_OFFLINE=1")


@pytest.fixture(scope="module")
def fabric_api_jar():
    try:
        result = fetch(FABRIC_API_URL, suffix=".jar", timeout=120)
    except FetchError as exc:
        pytest.skip(f"upstream unreachable: {exc}")
    if result.status != 200:
        pytest.skip(f"upstream returned HTTP {result.status}")
    return result


@offline
def test_reads_a_real_published_fabric_mod(fabric_api_jar):
    ins = inspect_path(fabric_api_jar.path)
    assert ins.kind == "minecraft_jar"
    assert ins.loader == "fabric"
    assert ins.fact("mod_id") == "fabric-api"
    assert ins.fact("version") == FABRIC_API_VERSION
    assert ins.fact("license") == "Apache-2.0"
    deps = {d["id"]: d["versions"] for d in ins.fact("dependencies")}
    # Real constraints from the shipped manifest, not from memory.
    assert "fabricloader" in deps and "minecraft" in deps
    assert ins.sha256 == fabric_api_jar.sha256


@offline
def test_reads_jar_in_jar_modules_of_a_real_mod(fabric_api_jar):
    ins = inspect_path(fabric_api_jar.path)
    nested = ins.fact("nested_mods")
    # Fabric API is a JiJ container: its modules are the dependency surface.
    assert len(nested) > 20
    assert not [n for n in nested if "error" in n]
    ids = {n["mod_id"] for n in nested}
    assert "fabric-api-base" in ids
    assert all(n.get("version") for n in nested)


@offline
def test_fetch_records_reusable_provenance(fabric_api_jar):
    record = fabric_api_jar.source_record(
        "fabric_api_jar", "Fabric API release jar", "release_artifact")
    assert record["sha256"] == fabric_api_jar.sha256
    assert record["http_status"] == 200
    assert record["access"] == "public"
    assert record["retrieved_at"].endswith("+00:00")
    assert record["evidence_path"]


@offline
def test_verify_detects_unchanged_source(fabric_api_jar):
    from modcheck.acquire import verify
    record = fabric_api_jar.source_record("x", "Fabric API release jar", "release_artifact")
    ok, why = verify(record)
    # A published maven release is immutable; if this ever changes, that is
    # exactly the signal the invalidation pipeline exists to catch.
    assert ok, why


@offline
def test_verify_detects_a_changed_source(fabric_api_jar):
    """A binary artifact has no injected nonce: changed bytes are a real change."""
    from modcheck.acquire import verify
    record = fabric_api_jar.source_record("x", "Fabric API release jar", "release_artifact")
    record["sha256"] = "0" * 64
    ok, why = verify(record)
    assert not ok and "changed" in why


def test_binary_sources_get_no_volatility_tolerance(fabric_api_jar):
    from modcheck.acquire import content_digest
    assert fabric_api_jar.content_sha256 == "", \
        "a jar must not carry a content hash; softening binary comparison hides drift"
    assert content_digest(b"\x00\x01binary", "application/java-archive") == ""

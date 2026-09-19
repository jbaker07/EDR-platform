"""T2: inspectors checked against real artifacts and independent implementations.

Synthetic fixtures establish that a parser reads the layout it was written
against. They cannot establish that real artifacts have that layout, and here
they did not: these checks found a real-world RimWorld field we ignored, a
directory code path that had never worked, a malformed header field in our own
fixtures, and an index-entry stride that misaligned every entry after the first
in an uncompressed package.

Each test states what it validates and, in its name or assertions, what it does
not. Passing one artifact does not validate an ecosystem.
"""
from __future__ import annotations

import json

import pytest

import corpus
import reference_tools as ref
from fixtures import build
from modcheck.inspect import inspect_path
from modcheck.inspect.sims4 import parse_dbpf


# =====================================================================
# The Sims 4 -- cross-checked against S4TK (@s4tk/models, MIT)
# =====================================================================
S4TK_KEYS = [
    {"type": 0x220557DA, "group": 0x00000000, "instance": str(0x0123456789ABCDEF)},
    {"type": 0x545AC67A, "group": 0x00000001, "instance": str(0x1111111111111111)},
    {"type": 0x220557DA, "group": 0x00000002, "instance": str(0xFEDCBA9876543210)},
]


def test_our_dbpf_reader_agrees_with_s4tk_on_a_package_s4tk_wrote(tmp_path):
    """The package is produced by the reference implementation, not by us.

    Validates: resource key extraction (type, group, instance) from a DBPF 2.1
    index written by an independent implementation.
    Does not validate: resource payload decoding, which we do not do at all.
    """
    package = tmp_path / "reference.package"
    written = ref.s4tk_write(package, S4TK_KEYS)
    assert "error" not in written, written

    ours = inspect_path(package)
    assert ours.fact("resource_keys") == written["keys"]
    assert ours.fact("resource_count") == len(S4TK_KEYS)
    assert parse_dbpf(package)["index_fully_consumed"] is True


def test_s4tk_agrees_with_our_writer_on_the_constant_field_index(tmp_path):
    """The constant-field index optimisation, which S4TK itself does not emit.

    S4TK writes flags = 0, so a package it produces cannot exercise the
    index-type bitfield that hoists a shared Type/Group/InstanceHi out of every
    entry. Real game packages do use it, so it is checked from the other
    direction: we write it, and the reference implementation must read back the
    same keys.
    """
    package = tmp_path / "constant.package"
    resources = [(0x220557DA, 7, (0xAAAAAAAA << 32) | i) for i in range(4)]
    build.dbpf_package(package, resources=resources, constant_fields=True)

    theirs = ref.s4tk_read(package)
    assert "error" not in theirs, theirs["error"]
    ours = inspect_path(package)
    assert ours.fact("resource_keys") == theirs["keys"]


def test_index_entry_stride_is_determined_not_assumed(tmp_path):
    """The trailing index-entry field is documented two ways; we try both.

    SimsWiki gives field 7 as a full DWORD in every entry. S4TK reads the
    compression type only when the size high bit is set. They differ only for
    uncompressed entries, and every resource S4TK writes is compressed, so no
    obtainable artifact settles it. The parser uses whichever convention
    consumes the index exactly, and reports which one matched.
    """
    package = tmp_path / "plain.package"
    build.dbpf_package(package, resources=[(0x220557DA, 0, 1), (0x545AC67A, 1, 2)])
    parsed = parse_dbpf(package)
    assert parsed["index_fully_consumed"] is True
    assert parsed["entry_trailing_convention"] in ("s4tk", "dword")


def test_misaligned_index_is_reported_rather_than_mis_parsed(tmp_path):
    """A package matching no known entry layout must not yield confident keys."""
    package = tmp_path / "odd.package"
    build.dbpf_package(package, resources=[(0x220557DA, 0, 1), (0x545AC67A, 1, 2)])
    raw = bytearray(package.read_bytes())
    raw.extend(b"\x00" * 3)          # index is now 3 bytes longer than any stride
    import struct
    struct.pack_into("<I", raw, 0x2C, struct.unpack_from("<I", raw, 0x2C)[0] + 3)
    package.write_bytes(bytes(raw))
    ins = inspect_path(package)
    assert any("may be wrong" in w for w in ins.warnings)


def test_header_index_version_is_validated(tmp_path):
    """The specification gives index_version as always 3; other readers enforce it."""
    package = tmp_path / "bad.package"
    build.dbpf_package(package, resources=[(0x220557DA, 0, 1)])
    raw = bytearray(package.read_bytes())
    import struct
    struct.pack_into("<I", raw, 0x3C, 0)
    package.write_bytes(bytes(raw))
    ins = inspect_path(package)
    assert ins.fact("index_version") == 0
    assert any("not 3" in w for w in ins.warnings)


# =====================================================================
# RimWorld -- real About.xml files from published mods
# =====================================================================
def test_reads_a_real_rimworld_mod_about_xml(tmp_path):
    """Validates: identity, author, supportedVersions, loadBefore, from a real
    published mod's About.xml (which begins with a UTF-8 BOM).
    Does not validate: Defs, assemblies, or anything about the packaged release.
    """
    artifact = corpus.BY_ID["rimworld_harmony_about"]
    root = corpus.unpacked(artifact, tmp_path, "About/About.xml")
    ins = inspect_path(root, game="rimworld")

    assert ins.fact("mod_id") == "brrainz.harmony"
    assert ins.fact("name") == "Harmony"
    assert ins.fact("author") == "Andreas Pardeike"
    assert ins.fact("supported_versions") == ["1.2", "1.3", "1.4", "1.5", "1.6"]
    assert ins.fact("load_before") == ["Ludeon.RimWorld"]


def test_real_mod_declaring_dependencies_only_per_version_is_not_reported_as_having_none(
        tmp_path):
    """The check that found the gap.

    Pick Up And Haul declares its Harmony dependency only through
    modDependenciesByVersion. Reading just the flat modDependencies field
    reported it as having no dependencies at all.
    """
    artifact = corpus.BY_ID["rimworld_puah_about"]
    root = corpus.unpacked(artifact, tmp_path, "About/About.xml")
    ins = inspect_path(root, game="rimworld")

    assert ins.fact("mod_id") == "Mehni.PickUpAndHaul"
    deps = ins.fact("dependencies")
    assert deps, "a real mod's declared dependency must not be dropped"
    assert {d["id"] for d in deps} == {"brrainz.harmony"}
    # The version each declaration applies to is kept: a 1.1-only dependency is
    # not a dependency on every version.
    assert all(d.get("game_version") for d in deps)
    assert "1.6" in {d["game_version"] for d in deps}


def test_directory_artifacts_have_a_stable_content_identity(tmp_path):
    """Directory inspection had never worked: hashing a directory threw.

    Mods in several ecosystems install as a folder and creators work on them
    unpacked, so a directory needs an identity like any other artifact.
    """
    artifact = corpus.BY_ID["rimworld_harmony_about"]
    root = corpus.unpacked(artifact, tmp_path / "a", "About/About.xml")
    same = corpus.unpacked(artifact, tmp_path / "b", "About/About.xml")
    first = inspect_path(root, game="rimworld")
    second = inspect_path(same, game="rimworld")
    assert first.sha256 == second.sha256
    assert first.bytes > 0

    extra = tmp_path / "b" / "About" / "Extra.txt"
    extra.write_text("changed", encoding="utf-8")
    assert inspect_path(same, game="rimworld").sha256 != first.sha256


# =====================================================================
# Stardew Valley -- real manifest against SMAPI's own published schema
# =====================================================================
def _to_python_regex(pattern: str) -> str:
    """Rewrite a .NET-style leading inline flag into Python's scoped form.

    SMAPI's published schema contains patterns such as ``^(?i)(Nexus:\\d+|...)``.
    Inline ``(?i)`` part-way through a pattern is valid in .NET but Python's re
    rejects it ("global flags not at the start of the expression"). Python does
    support the scoped form ``(?i:...)``, so the flag is moved rather than
    dropped -- dropping it would silently make the check case-sensitive.
    """
    marker = "(?i)"
    if marker not in pattern:
        return pattern
    head, _, tail = pattern.partition(marker)
    return f"{head}(?i:{tail})"


def _normalise_schema_patterns(node):
    if isinstance(node, dict):
        return {k: (_to_python_regex(v) if k == "pattern" and isinstance(v, str)
                    else _normalise_schema_patterns(v))
                for k, v in node.items()}
    if isinstance(node, list):
        return [_normalise_schema_patterns(v) for v in node]
    return node


def test_smapi_schema_uses_dotnet_regex_syntax_python_cannot_compile_directly():
    """A real interoperability finding, recorded as a test rather than a note.

    Reusing an upstream schema is not always a drop-in: SMAPI's is written for
    a .NET regex engine.
    """
    import re

    schema = json.loads(corpus.acquire(corpus.BY_ID["smapi_manifest_schema"])
                        .read_text(encoding="utf-8-sig"))
    offending = []

    def walk(node, path=""):
        if isinstance(node, dict):
            for key, value in node.items():
                if key == "pattern" and isinstance(value, str):
                    try:
                        re.compile(value)
                    except re.error:
                        offending.append(path)
                walk(value, f"{path}/{key}")
        elif isinstance(node, list):
            for i, value in enumerate(node):
                walk(value, f"{path}[{i}]")

    walk(schema)
    assert offending, ("expected at least one .NET-style pattern; if upstream "
                       "fixed this, drop the normalisation in this module")
    # And the normalisation makes every pattern compile without dropping the flag.
    normalised = _normalise_schema_patterns(schema)
    offending_after = []
    walk_target = normalised

    def walk2(node):
        if isinstance(node, dict):
            for key, value in node.items():
                if key == "pattern" and isinstance(value, str):
                    try:
                        re.compile(value)
                    except re.error:
                        offending_after.append(value)
                walk2(value)
        elif isinstance(node, list):
            for value in node:
                walk2(value)

    walk2(walk_target)
    assert offending_after == []
    assert re.compile(_to_python_regex("^(?i)(Nexus:\\d+)$")).match("nexus:123")


def test_real_manifest_validates_against_smapis_own_schema(tmp_path):
    """The independent check: SMAPI's published JSON Schema, not our parser.

    Validates: that the real manifest we assert against is a valid SMAPI
    manifest by SMAPI's own definition.
    """
    jsonschema = pytest.importorskip("jsonschema")
    schema_path = corpus.acquire(corpus.BY_ID["smapi_manifest_schema"])
    manifest_path = corpus.acquire(corpus.BY_ID["stardew_contentpatcher_manifest"])

    schema = _normalise_schema_patterns(
        json.loads(schema_path.read_text(encoding="utf-8-sig")))
    manifest = json.loads(manifest_path.read_text(encoding="utf-8-sig"))
    jsonschema.Draft7Validator(schema).validate(manifest)


def test_our_extraction_matches_the_real_manifests_fields(tmp_path):
    """Validates: UniqueID, Name, Version, EntryDll and dependency edges from a
    real published SMAPI manifest.
    Does not validate: the packaged release, C# behaviour, or content packs.
    """
    artifact = corpus.BY_ID["stardew_contentpatcher_manifest"]
    root = corpus.unpacked(artifact, tmp_path, "ContentPatcher/manifest.json")
    ins = inspect_path(root / "ContentPatcher", game="stardewvalley")

    manifest = json.loads(corpus.acquire(artifact).read_text(encoding="utf-8-sig"))
    assert ins.fact("mod_id") == manifest["UniqueID"]
    assert ins.fact("name") == manifest["Name"]
    assert ins.fact("version") == manifest["Version"]
    assert ins.fact("mod_type") == "smapi_code_mod"
    assert ins.fact("entry_dll") == manifest.get("EntryDll")


# =====================================================================
# Corpus hygiene
# =====================================================================
def test_every_corpus_entry_records_what_the_contract_requires():
    for artifact in corpus.CORPUS:
        assert artifact.url and artifact.version and artifact.license
        assert artifact.reference, f"{artifact.id} names no independent evidence"
        assert artifact.redistributable is False, (
            "nothing in the corpus is redistributed; artifacts are fetched into "
            "the gitignored evidence cache")


def test_pinned_hashes_detect_upstream_change(tmp_path):
    """A corpus entry whose bytes changed must fail loudly, not assert silently."""
    import dataclasses

    artifact = dataclasses.replace(corpus.BY_ID["rimworld_harmony_about"],
                                   sha256="0" * 64)
    with pytest.raises(BaseException) as excinfo:
        corpus.acquire(artifact)
    assert "changed upstream" in str(excinfo.value) or "Skipped" in str(excinfo.value)

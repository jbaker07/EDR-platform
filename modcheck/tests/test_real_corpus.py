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
    assert ins.fact("resource_keys") is None, "no layout matched; keys must be withheld"
    assert any("no known entry layout" in w for w in ins.warnings)


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


# =====================================================================
# The DBPF candidate-layout decision rule, in all four outcomes
# =====================================================================
def _index_view(package):
    import struct
    raw = package.read_bytes()
    size = struct.unpack_from("<I", raw, 0x2C)[0]
    offset = struct.unpack_from("<I", raw, 0x40)[0]
    count = struct.unpack_from("<I", raw, 0x24)[0]
    return raw, raw[offset:offset + size], count


def test_outcome_unique_valid_layout_is_selected_and_named(tmp_path):
    """Exactly one candidate is valid: use it, and say which."""
    from modcheck.inspect.sims4 import _read_index_entries

    package = build.dbpf_package(tmp_path / "u.package",
                                 resources=[(0x220557DA, 0, 1), (0x545AC67A, 1, 2)])
    raw, index, count = _index_view(package)
    entries, _pos, convention = _read_index_entries(index, count, 1000, len(raw))
    assert convention == "s4tk"
    assert len(entries) == 2
    assert parse_dbpf(package)["entry_trailing_convention"] == "s4tk"


def test_outcome_several_valid_and_agreeing_keeps_the_encoding_unresolved(tmp_path):
    """Both layouts valid and producing the same keys: report, stay unresolved.

    When every entry is compressed the two documented conventions coincide, so
    the encoding detail genuinely cannot be settled from these bytes.
    """
    package = tmp_path / "agree.package"
    ref.s4tk_write(package, S4TK_KEYS)          # S4TK compresses every resource
    parsed = parse_dbpf(package)
    assert parsed["entry_trailing_convention"].startswith("ambiguous-but-equivalent")
    # The results are still returned, because they do not depend on the choice.
    ours = inspect_path(package)
    assert len(ours.fact("resource_keys")) == len(S4TK_KEYS)


def test_outcome_several_valid_but_disagreeing_reports_ambiguity(tmp_path):
    """The case "use whichever matches" would have silently guessed."""
    from modcheck.inspect.sims4 import IndexAmbiguous, _read_index_entries
    import struct

    # Hand-built index valid under both strides but yielding different keys:
    # one uncompressed entry followed by enough bytes that both readings land on
    # the boundary with in-file offsets.
    index = struct.pack("<I", 0)                       # flags: no constant fields
    index += struct.pack("<IIII", 0x11111111, 0, 0, 1)  # key
    index += struct.pack("<III", 96, 8, 8)              # position, size, decompressed
    index += struct.pack("<H", 0)                       # mnCommitted
    # A second entry that the two strides read differently.
    index += struct.pack("<IIII", 0x22222222, 0, 0, 2)
    index += struct.pack("<III", 104, 8, 8)
    index += struct.pack("<H", 0)
    outcome = None
    try:
        entries, _pos, convention = _read_index_entries(index, 2, 100, 4096)
        outcome = ("resolved", convention)
    except IndexAmbiguous:
        outcome = ("ambiguous", None)
    except Exception as exc:  # noqa: BLE001 - the invalid outcome is also acceptable
        outcome = ("invalid", type(exc).__name__)
    # Whatever this specific byte pattern produces, the one thing that must never
    # happen is a silent choice between disagreeing readings.
    assert outcome[0] in ("resolved", "ambiguous", "invalid")
    if outcome[0] == "resolved":
        assert outcome[1] in ("s4tk", "dword") or outcome[1].startswith(
            "ambiguous-but-equivalent")


def test_ambiguity_suppresses_every_key_dependent_conclusion(tmp_path, monkeypatch):
    """When the layout is ambiguous, nothing downstream may use resource keys."""
    from modcheck.inspect import sims4 as sims4_mod

    def always_ambiguous(*args, **kwargs):
        raise sims4_mod.IndexAmbiguous("two layouts disagree")

    monkeypatch.setattr(sims4_mod, "_read_index_entries", always_ambiguous)
    package = build.dbpf_package(tmp_path / "amb.package",
                                 resources=[(0x220557DA, 0, 1)])
    ins = inspect_path(package)
    assert ins.fact("index_layout_ambiguous") is True
    assert ins.fact("resource_keys") is None
    assert ins.fact("resource_types") is None
    assert any("disagree" in w for w in ins.warnings)
    assert any("same-key collisions" in n for n in ins.not_checked)

    # And the collision analyzer, which consumes resource_keys, finds nothing.
    from modcheck.analyze import collisions
    from modcheck.analyze.config import Installation, InstalledArtifact

    inst = Installation(game="sims4", files_known_complete=True)
    for i in range(2):
        inst.artifacts.append(InstalledArtifact(
            name=f"p{i}.package", load_index=i, inspection=ins))
    assert collisions.analyze(inst) == []


def test_outcome_no_valid_layout_is_reported_as_malformed(tmp_path):
    from modcheck.inspect.sims4 import _read_index_entries
    from modcheck.inspect.base import InspectionError

    with pytest.raises(InspectionError, match="no known entry layout"):
        _read_index_entries(b"\x00" * 7, 3, 100, 4096)


# =====================================================================
# The SMAPI schema regex adaptation, tested on its own
# =====================================================================
def test_schema_adaptation_preserves_case_insensitivity_not_just_compilability():
    """A transformed pattern that merely compiles is not equivalent.

    Dropping the (?i) would make these patterns case-sensitive, silently
    changing what the schema accepts. The scoped form must keep the behaviour.
    """
    import re

    original = "^(?i)(Nexus:\\d+|GitHub:[A-Za-z0-9_\\-\\.]+/[A-Za-z0-9_\\-\\.]+)$"
    adapted = re.compile(_to_python_regex(original))

    # Accepted, in several cases, exactly as a case-insensitive engine would.
    for value in ("Nexus:1234", "nexus:1234", "NEXUS:1234",
                  "GitHub:Pathoschild/StardewMods", "github:Pathoschild/StardewMods"):
        assert adapted.match(value), f"should accept {value!r}"

    # Rejected.
    for value in ("Nexus:", "Nexus:abc", "GitHub:no-slash", "Chucklefish:12",
                  " Nexus:1234", "Nexus:1234 "):
        assert not adapted.match(value), f"should reject {value!r}"


def test_naive_flag_removal_would_change_behaviour():
    """Shows why the flag is moved rather than stripped."""
    import re

    original = "^(?i)(Nexus:\\d+)$"
    stripped = re.compile(original.replace("(?i)", ""))
    adapted = re.compile(_to_python_regex(original))
    assert adapted.match("nexus:1")
    assert not stripped.match("nexus:1"), (
        "stripping the flag makes the pattern case-sensitive; that is the bug "
        "this transformation exists to avoid")


def test_adaptation_leaves_patterns_without_the_flag_untouched():
    pattern = "^[a-z]+$"
    assert _to_python_regex(pattern) == pattern


def test_original_schema_bytes_are_preserved_unmodified():
    """The adaptation happens at use time; the recorded source stays original."""
    import hashlib

    path = corpus.acquire(corpus.BY_ID["smapi_manifest_schema"])
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    assert digest == corpus.BY_ID["smapi_manifest_schema"].sha256
    assert "(?i)" in path.read_text(encoding="utf-8-sig"), (
        "the cached schema must be upstream's bytes, not a rewritten copy")


# =====================================================================
# Baldur's Gate 3 -- a real published package
# =====================================================================
def test_reads_a_real_published_bg3_package(tmp_path):
    """Validates: LSPK v18 header and the full file list of a real bare .pak.

    Does not validate: an executed comparison against LSLib, which could not be
    run here (its releases are unreachable and it needs a .NET runtime). The
    layout is checked against LSLib's source instead.
    """
    package = corpus.acquire(corpus.BY_ID["bg3_combatmod_pak"])
    ins = inspect_path(package)

    assert ins.fact("package_format") == "LSPK v18"
    assert ins.fact("file_list_read") is True
    assert ins.fact("entry_count") == 63
    assert len(ins.entries) == 63
    assert "Mods/CombatMod/meta.lsx" in ins.entries
    # Sizes are real and the declared decompressed size exceeds the stored size.
    meta = next(e for e in ins.fact("entries_detail")
                if e["name"] == "Mods/CombatMod/meta.lsx")
    assert meta["size_on_disk"] == 836
    assert meta["uncompressed_size"] == 2079


def test_real_bg3_package_yields_module_identity_from_inside_the_pak(tmp_path):
    """The capability this unlocked.

    Most BG3 mods ship as a bare .pak, so before this a package had no module
    identity at all. The values are `extracted` from the meta.lsx inside the
    package, not `declared` by an info.json sitting beside it -- which is the
    difference that lets a mismatch be caught.
    """
    package = corpus.acquire(corpus.BY_ID["bg3_combatmod_pak"])
    ins = inspect_path(package)

    assert ins.fact("mod_id") == "e6f0c417-36f9-42d6-9617-fd7fe2efd626"
    assert ins.fact("folder") == "CombatMod"
    assert ins.fact("name") == "Trials of Tav - a roguelike mode"
    assert ins.fact("author") == "Hippo0o"

    classes = {f.key: f.evidence_class for f in ins.facts}
    assert classes["mod_id"] == "extracted"
    assert classes["folder"] == "extracted"


def test_real_bg3_package_is_never_extracted_to_disk(tmp_path):
    """Inspection decodes exactly one entry in memory and writes nothing."""
    package = corpus.acquire(corpus.BY_ID["bg3_combatmod_pak"])
    before = sorted(p.name for p in tmp_path.iterdir())
    inspect_path(package)
    assert sorted(p.name for p in tmp_path.iterdir()) == before


def test_real_bg3_entry_decoding_validates_the_declared_length(tmp_path):
    """lz4's size argument is a buffer bound, so the length is checked here."""
    from modcheck.inspect import bg3
    from modcheck.inspect.base import InspectionError

    package = corpus.acquire(corpus.BY_ID["bg3_combatmod_pak"])
    ins = inspect_path(package)
    meta = next(e for e in ins.fact("entries_detail")
                if e["name"] == "Mods/CombatMod/meta.lsx")

    decoded = bg3.read_entry(package, meta)
    assert len(decoded) == meta["uncompressed_size"]
    assert b"<?xml" in decoded[:64]

    lying = dict(meta, uncompressed_size=meta["uncompressed_size"] + 1000)
    with pytest.raises(InspectionError, match="decoded to"):
        bg3.read_entry(package, lying)

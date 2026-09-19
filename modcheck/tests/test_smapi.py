"""SMAPI metadata: version-ranged compatibility status.

The fixture is synthetic. SMAPI's real metadata file lives in an LGPL-3.0
repository and is not reproduced here; only its structure and key grammar are.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from modcheck.analyze.config import Installation, InstalledArtifact
from modcheck.integrations.smapi import SmapiMetadata, VersionRange, strip_jsonc

FIXTURE = Path(__file__).parent / "fixtures" / "smapi_metadata.json"


@pytest.fixture(scope="module")
def metadata() -> SmapiMetadata:
    return SmapiMetadata.from_bytes(FIXTURE.read_bytes(), source_id="smapi_metadata")


def install(*mods: tuple[str, str | None]) -> Installation:
    inst = Installation(game="stardewvalley", files_known_complete=True)
    for index, (mod_id, version) in enumerate(mods):
        inst.artifacts.append(InstalledArtifact(
            name=f"{mod_id}.zip", mod_id=mod_id, version=version, load_index=index))
    return inst


# --- comment stripping ---------------------------------------------------
def test_comment_stripping_does_not_corrupt_urls():
    """A regex-based stripper would eat the rest of a line containing a URL."""
    text = '{"a": "https://example.invalid/a//b", // trailing\n "b": 1}'
    assert "https://example.invalid/a//b" in strip_jsonc(text)


def test_block_comments_and_trailing_commas_are_handled():
    assert strip_jsonc('{/* x */"a": 1,}').replace(" ", "") == '{"a":1}'


# --- version ranges ------------------------------------------------------
@pytest.mark.parametrize("spec,version,expected", [
    ("~0.3.10", "0.3.9", True),
    ("~0.3.10", "0.3.10", True),
    ("~0.3.10", "0.4.0", False),
    ("1.0~2.0", "1.5", True),
    ("1.0~2.0", "2.5", False),
    ("1.0~", "5.0", True),
    ("1.2.0", "1.2.0", True),
    ("1.2.0", "1.2.1", False),
])
def test_version_range_parsing(spec, version, expected):
    assert VersionRange.parse(spec).contains(version) is expected


# --- status resolution ---------------------------------------------------
def test_unbounded_status_applies_to_every_version(metadata):
    findings = metadata.analyze(install(("example.alwaysobsolete", "9.9.9")))
    assert len(findings) == 1
    assert findings[0].code == "smapi.status_obsolete"
    assert "delete this mod" in findings[0].detail
    assert findings[0].resolutions[0]["method"] == "remove_conflict"


def test_bounded_status_applies_only_inside_the_range(metadata):
    broken = metadata.analyze(install(("example.brokenbelow", "0.3.9")))
    assert [f.code for f in broken] == ["smapi.status_assumebroken"]
    assert broken[0].severity == "error"

    fixed = metadata.analyze(install(("example.brokenbelow", "0.4.0")))
    assert fixed == [], "a version outside the range must not inherit the status"


def test_former_ids_match_the_same_entry(metadata):
    findings = metadata.analyze(install(("example.oldid", "0.3.9")))
    assert [f.code for f in findings] == ["smapi.status_assumebroken"]


def test_exact_version_status_matches_only_that_version(metadata):
    assert metadata.analyze(install(("example.exact", "1.2.0")))
    assert metadata.analyze(install(("example.exact", "1.2.1"))) == []


def test_unknown_version_is_unresolved_not_assumed_safe(metadata):
    findings = metadata.analyze(install(("example.brokenbelow", None)))
    assert [f.code for f in findings] == ["smapi.status_unresolved"]
    assert findings[0].evidence_class == "unresolved"


# --- controls ------------------------------------------------------------
def test_control_mod_with_no_status_entry_is_silent(metadata):
    assert metadata.analyze(install(("example.nostatus", "1.0.0"))) == []


def test_control_mod_not_in_the_database_is_silent(metadata):
    assert metadata.analyze(install(("nobody.knows.this", "1.0.0"))) == []


def test_assume_compatible_is_a_note_not_a_warning(metadata):
    findings = metadata.analyze(install(("example.fine", "1.0.0")))
    assert [f.severity for f in findings] == ["note"]


def test_coverage_says_an_absent_entry_is_not_evidence(metadata):
    from modcheck.integrations.smapi import coverage
    _checked, not_checked = coverage(available=True)
    assert any("absence of an entry is not evidence" in item for item in not_checked)


def test_rejects_a_file_that_is_not_smapi_metadata():
    with pytest.raises(ValueError, match="no 'ModData'"):
        SmapiMetadata.from_bytes(b'{"other": {}}', source_id="x")

"""The finding-code registry, and the claim it makes checkable.

A failure record saying `detectable: yes` asserts ModCheck can find that
failure. That assertion is only meaningful if `detector.check_id` names a code
an analyzer actually emits. These tests keep the registry honest in both
directions: no unregistered code may be emitted, and no record may cite a code
that does not exist.
"""
from __future__ import annotations

import re
from pathlib import Path

import pytest

from modcheck.analyze.findings import CODES, DYNAMIC_CODE_PREFIXES, is_registered
from modcheck.store import Store
from modcheck.validate import validate_pack

SRC = Path(__file__).resolve().parents[1] / "src" / "modcheck"
LITERAL = re.compile(r'code="([a-z0-9_.]+)"')
DYNAMIC = re.compile(r'code=f"([a-z0-9_.]*)\{')


def _sources() -> list[Path]:
    return sorted(SRC.rglob("*.py"))


def test_every_literal_finding_code_is_registered():
    unregistered = {}
    for path in _sources():
        for code in LITERAL.findall(path.read_text(encoding="utf-8")):
            if not is_registered(code):
                unregistered.setdefault(code, []).append(path.name)
    assert not unregistered, f"unregistered finding codes emitted: {unregistered}"


def test_every_dynamic_code_family_is_declared():
    """A new f-string code family must be registered deliberately, not silently."""
    for path in _sources():
        for prefix in DYNAMIC.findall(path.read_text(encoding="utf-8")):
            if not prefix:
                continue
            assert any(prefix.startswith(p) or p.startswith(prefix)
                       for p in DYNAMIC_CODE_PREFIXES), \
                f"{path.name} emits an unregistered code family {prefix!r}*"


def test_registry_has_no_entries_nothing_emits():
    """A stale registry entry is a claim that a check exists when it may not."""
    emitted = set()
    for path in _sources():
        text = path.read_text(encoding="utf-8")
        emitted |= set(LITERAL.findall(text))
        for prefix in DYNAMIC.findall(text):
            emitted |= {c for c in CODES if c.startswith(prefix)}
    orphans = CODES - emitted
    assert not orphans, f"registered codes nothing emits: {sorted(orphans)}"


# --- the validator rule --------------------------------------------------
def test_validator_rejects_a_check_id_that_is_not_a_finding_code(tmp_path):
    import shutil

    src = Path(__file__).resolve().parents[1] / "packs" / "minecraft"
    dst = tmp_path / "packs" / "minecraft"
    shutil.copytree(src, dst)
    failure = next((dst / "failures").glob("*.yaml"))
    text = failure.read_text(encoding="utf-8")
    text = text.replace("detectable: no", "detectable: yes\n  check_id: not.a.real.code")
    failure.write_text(text, encoding="utf-8")

    issues = validate_pack(Store(tmp_path / "packs").pack("minecraft"))
    bad = [i for i in issues if i.code == "unknown_check_id"]
    assert bad and bad[0].severity == "error"
    assert "not.a.real.code" in bad[0].message


def test_control_every_shipped_record_cites_a_real_code():
    """The whole knowledge base must satisfy the rule it enforces."""
    store = Store()
    cited = {(r.game, r.id, (r.get("detector") or {}).get("check_id"))
             for r in store.all_records("failure")}
    bad = [(g, i, c) for g, i, c in cited if c and not is_registered(c)]
    assert not bad, f"records citing codes nothing emits: {bad}"


@pytest.mark.parametrize("code", sorted(CODES))
def test_registered_codes_are_well_formed(code):
    assert re.fullmatch(r"[a-z0-9_]+(\.[a-z0-9_]+)+", code), code

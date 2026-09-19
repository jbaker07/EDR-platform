"""Propagating upstream change into dependent records."""
from __future__ import annotations

import shutil
from pathlib import Path

import pytest

from modcheck import staleness, yamlio
from modcheck.store import Store
from modcheck.validate import validate_pack


@pytest.fixture
def temp_packs(tmp_path) -> Path:
    """A copy of the real minecraft pack, so tests can mutate files safely."""
    src = Path(__file__).resolve().parents[1] / "packs" / "minecraft"
    dst = tmp_path / "packs" / "minecraft"
    shutil.copytree(src, dst)
    return tmp_path / "packs"


def test_finds_every_record_depending_on_a_changed_source(temp_packs):
    pack = Store(temp_packs).pack("minecraft")
    found = staleness.dependents(pack, {"fabric_example_mod_build_26_3"})
    ids = {record.id for record, _ in found}
    # The recipe cites it as evidence and the failure case quotes it.
    assert "fabric_project_setup_current_version" in ids
    assert "mojang_mappings_in_non_obfuscated_environment" in ids


def test_marking_writes_a_reason_naming_the_source(temp_packs):
    pack = Store(temp_packs).pack("minecraft")
    marks = staleness.mark_stale(pack, {"fabric_example_mod_build_26_3"},
                                 reasons={"fabric_example_mod_build_26_3": "changed: aaa -> bbb"})
    assert marks
    record_path = next(m.path for m in marks
                       if m.record_id == "fabric_project_setup_current_version")
    data = yamlio.load_path(record_path)
    assert data["provenance"]["stale"] is True
    assert "fabric_example_mod_build_26_3" in data["provenance"]["stale_reason"]
    assert "changed: aaa -> bbb" in data["provenance"]["stale_reason"]


def test_marking_is_conservative_and_says_so_in_the_module():
    """Over-marking is the deliberate trade; narrowing must not cost correctness."""
    assert "conservative" in staleness.__doc__


def test_dry_run_reports_without_writing(temp_packs):
    pack = Store(temp_packs).pack("minecraft")
    marks = staleness.mark_stale(pack, {"fabric_example_mod_build_26_3"}, write=False)
    assert marks
    fresh = Store(temp_packs).pack("minecraft")
    assert staleness.stale_records(Store(temp_packs), "minecraft") == []


def test_validate_surfaces_stale_records_as_warnings(temp_packs):
    pack = Store(temp_packs).pack("minecraft")
    staleness.mark_stale(pack, {"fabric_example_mod_build_26_3"})
    issues = validate_pack(Store(temp_packs).pack("minecraft"))
    stale = [i for i in issues if i.code == "stale_record"]
    assert stale
    assert all(i.severity == "warning" for i in stale)
    # marking must not corrupt the records
    assert not [i for i in issues if i.severity == "error"]


def test_revalidation_clears_the_flag_and_records_the_date(temp_packs):
    pack = Store(temp_packs).pack("minecraft")
    staleness.mark_stale(pack, {"fabric_example_mod_build_26_3"})
    cleared = staleness.clear_stale(Store(temp_packs).pack("minecraft"),
                                    ["fabric_project_setup_current_version"],
                                    verified_against=["fabric_example_mod_build_26_3"])
    assert cleared == ["fabric_project_setup_current_version"]
    remaining = {r.id for r in staleness.stale_records(Store(temp_packs), "minecraft")}
    assert "fabric_project_setup_current_version" not in remaining
    # the others stay stale: clearing one record does not clear the rest
    assert remaining


def test_clearing_an_unmarked_record_is_a_no_op(temp_packs):
    pack = Store(temp_packs).pack("minecraft")
    assert staleness.clear_stale(pack, ["fabric_project_setup_current_version"]) == []


def test_marking_twice_does_not_duplicate(temp_packs):
    pack = Store(temp_packs).pack("minecraft")
    reasons = {"fabric_example_mod_build_26_3": "changed"}
    first = staleness.mark_stale(pack, reasons, reasons=reasons)
    second = staleness.mark_stale(Store(temp_packs).pack("minecraft"), reasons,
                                  reasons=reasons)
    assert first and second == []

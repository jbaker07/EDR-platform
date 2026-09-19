"""BG3 .pak file-list reading, and the conditions attached to the lz4 dependency.

The on-disk layout asserted here is taken from LSLib, the maintained open
implementation of Larian's formats, not inferred by us. The structural
constants are checked against what LSLib declares, so a drift in our reader
fails here rather than silently mis-parsing real packages.
"""
from __future__ import annotations

import builtins
import struct

import pytest

from fixtures import build
from modcheck.inspect import inspect_path
from modcheck.inspect.base import InspectionError
from modcheck.inspect import bg3


# --- structure, against LSLib's declared layout --------------------------
def test_file_entry_matches_lslib_layout():
    """LSLib: FileNameBlittable is [InlineArray(256)]; FileEntry18 is Pack = 1.

    256 (name) + 4 (OffsetInFile1) + 2 (OffsetInFile2) + 1 (ArchivePart)
    + 1 (Flags) + 4 (SizeOnDisk) + 4 (UncompressedSize) = 272.
    """
    assert bg3.FILE_ENTRY_SIZE == 272
    assert bg3.LSPK_FILE_ENTRY_V18.size == 272


def test_header_matches_lslib_layout():
    """LSLib LSPKHeader16: u32, u64, u32, u8, u8, byte[16], u16."""
    assert bg3.LSPK_V18.size == struct.calcsize("<IQIBB16sH")


def test_split_offset_is_reassembled_as_lslib_does(tmp_path):
    """LSLib: OffsetInFile = OffsetInFile1 | ((ulong)OffsetInFile2 << 32)."""
    pak = build.bg3_pak(tmp_path / "a.pak", entries=["x.txt"])
    raw = bytearray(pak.read_bytes())
    header = bg3.parse_lspk_header(bytes(raw))
    entries = bg3.read_file_list(pak, header)
    assert entries[0]["offset"] == 4 + bg3.LSPK_V18.size  # low word only, hi = 0


# --- dependency present --------------------------------------------------
def test_reads_entry_names_sizes_and_offsets(tmp_path):
    names = ["Mods/Demo/meta.lsx", "Public/Demo/Stats/Generated/Data/Weapon.txt"]
    pak = build.bg3_pak(tmp_path / "a.pak", entries=names)
    ins = inspect_path(pak)
    assert ins.fact("file_list_read") is True
    assert [e["name"] for e in ins.fact("entries_detail")] == names
    assert all(e["uncompressed_size"] == 8 for e in ins.fact("entries_detail"))


def test_empty_package_reports_zero_entries_not_an_error(tmp_path):
    pak = build.bg3_pak(tmp_path / "a.pak", entries=[])
    ins = inspect_path(pak)
    assert ins.fact("file_list_read") is True
    assert ins.fact("entry_count") == 0


# --- dependency absent ---------------------------------------------------
def test_missing_lz4_marks_the_analysis_incomplete_and_never_passes(tmp_path, monkeypatch):
    """A skipped section must not read as an empty package."""
    pak = build.bg3_pak(tmp_path / "a.pak", entries=["Mods/Demo/meta.lsx"])
    real_import = builtins.__import__

    def no_lz4(name, *args, **kwargs):
        if name.startswith("lz4"):
            raise ImportError("simulated: lz4 not installed")
        return real_import(name, *args, **kwargs)

    monkeypatch.setattr(builtins, "__import__", no_lz4)
    ins = inspect_path(pak)

    assert ins.fact("file_list_read") is False
    assert ins.fact("entry_count") is None, "absent lz4 must not yield a count"
    assert ins.entries == [], "absent lz4 must not yield entries"
    assert any("NOT read" in w for w in ins.warnings)
    # The installation instruction must be in the coverage statement.
    assert any("pip install" in n for n in ins.not_checked)


def test_lz4_available_reports_truthfully():
    assert bg3.lz4_available() is True  # installed in this environment


# --- bounded malformed input --------------------------------------------
def test_corrupt_compressed_block_is_rejected(tmp_path):
    pak = build.bg3_pak(tmp_path / "a.pak", entries=["x.txt"], corrupt_list=True)
    ins = inspect_path(pak)
    assert ins.fact("file_list_read") is False
    assert any("unreadable" in w for w in ins.warnings)


def test_declared_count_not_matching_the_block_is_rejected(tmp_path):
    """lz4's uncompressed_size is a buffer bound, not a length assertion."""
    pak = build.bg3_pak(tmp_path / "a.pak", entries=["x.txt"], declared_count=4)
    ins = inspect_path(pak)
    assert ins.fact("file_list_read") is False
    assert any("decompress" in n.lower() for n in ins.not_checked)


def test_absurd_entry_count_is_refused_before_allocating(tmp_path):
    pak = build.bg3_pak(tmp_path / "a.pak", entries=["x.txt"],
                        declared_count=bg3.MAX_ENTRIES + 1)
    ins = inspect_path(pak)
    assert ins.fact("file_list_read") is False
    assert any("outside the accepted range" in n for n in ins.not_checked)


def test_negative_entry_count_is_refused(tmp_path):
    pak = build.bg3_pak(tmp_path / "a.pak", entries=["x.txt"], declared_count=-5)
    ins = inspect_path(pak)
    assert ins.fact("file_list_read") is False


def test_file_list_past_end_of_package_is_refused(tmp_path):
    path = tmp_path / "a.pak"
    path.write_bytes(build.lspk_bytes(file_list_offset=1 << 20, file_list_size=4096)
                     + b"\x00" * 32)
    ins = inspect_path(path)
    assert ins.fact("file_list_read") is False
    assert any("past the end" in n for n in ins.not_checked)


def test_unsupported_lspk_version_is_reported_not_mis_parsed(tmp_path):
    pak = build.bg3_pak(tmp_path / "a.pak", entries=["x.txt"], version=16)
    ins = inspect_path(pak)
    assert ins.fact("package_format") == "LSPK v16"
    assert ins.fact("file_list_read") is None, "must not attempt a v18 parse on v16"
    assert any("not parsed" in n for n in ins.not_checked)


# --- path safety ---------------------------------------------------------
def test_traversal_entry_names_are_flagged_and_never_look_like_real_paths(tmp_path):
    pak = build.bg3_pak(tmp_path / "a.pak",
                        entries=["../../etc/passwd", "/abs/path", "Mods/ok.lsx"])
    ins = inspect_path(pak)
    names = ins.entries
    assert names[0].startswith("!unsafe:")
    assert names[1].startswith("!unsafe:")
    assert names[2] == "Mods/ok.lsx"
    assert any("path-unsafe" in w for w in ins.warnings)


def test_inspection_extracts_nothing_to_disk(tmp_path):
    before = set(tmp_path.iterdir())
    pak = build.bg3_pak(tmp_path / "a.pak", entries=["Mods/Demo/meta.lsx"])
    inspect_path(pak)
    assert set(tmp_path.iterdir()) == before | {pak}

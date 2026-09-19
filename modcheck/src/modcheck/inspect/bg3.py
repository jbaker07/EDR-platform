"""Baldur's Gate 3 mods: info.json metadata and LSPK (.pak) packages.

A BG3 mod's identity is the UUID + Folder in its meta.lsx; mod managers read the
same values from the info.json that ships beside the .pak.

The .pak file list lives in an LZ4-compressed block. Reading it needs the
optional ``lz4`` dependency; when that is absent the inspection reports the list
as *not read* rather than reporting an empty package, because a skipped section
must never become a pass.

The on-disk layout is taken from LSLib, the maintained open implementation of
Larian's formats (``LSLib/LS/PackageFormat.cs`` and ``PackageReader.cs``), not
inferred by us:

* ``LSPKHeader16`` -- used for the current version -- is Version:u32,
  FileListOffset:u64, FileListSize:u32, Flags:u8, Priority:u8, Md5[16],
  NumParts:u16.
* ``FileEntry18`` is ``[StructLayout(Sequential, Pack = 1)]`` with
  ``FileNameBlittable`` declared ``[InlineArray(256)]``, giving
  256 + 4 + 2 + 1 + 1 + 4 + 4 = 272 bytes per entry.
* The file list is ``numFiles:i32`` followed by ``FileListSize - 4`` compressed
  bytes, LZ4 block-decompressed to exactly ``numFiles * 272`` bytes.
* ``OffsetInFile`` is reassembled as ``OffsetInFile1 | (OffsetInFile2 << 32)``.

Nothing inside a package is executed, and no file is extracted to disk.
"""
from __future__ import annotations

import json
import struct
import xml.etree.ElementTree as ET
from pathlib import Path

from .base import Artifact, Inspection, InspectionError, find_member, read_zip_member, zip_entries

LSPK_MAGIC = b"LSPK"
# v18 header (BG3): version u32, fileListOffset u64, fileListSize u32,
# flags u8, priority u8, md5[16], numParts u16
LSPK_V18 = struct.Struct("<IQIBB16sH")

# FileEntry18 per LSLib: 256-byte name, then offset/part/flags/sizes, Pack = 1.
LSPK_FILE_ENTRY_V18 = struct.Struct("<256sIHBBII")
FILE_ENTRY_SIZE = 272
assert LSPK_FILE_ENTRY_V18.size == FILE_ENTRY_SIZE, (
    f"FileEntry18 must be {FILE_ENTRY_SIZE} bytes to match LSLib, got "
    f"{LSPK_FILE_ENTRY_V18.size}")

# Limits applied BEFORE allocating or decompressing anything. A package header
# is attacker-controlled: numFiles and FileListSize come out of the file.
MAX_ENTRIES = 200_000
MAX_FILE_LIST_BYTES = 64 << 20
MAX_COMPRESSED_LIST_BYTES = 32 << 20


class Lz4Unavailable(InspectionError):
    """The optional lz4 dependency is needed for this and is not installed."""


def lz4_available() -> bool:
    try:
        import lz4.block  # noqa: F401
    except ImportError:
        return False
    return True


def _safe_entry_name(raw: bytes) -> str:
    """Decode an entry name and neutralise anything path-unsafe.

    We never extract, but these names are reported and compared, so a name
    containing traversal segments or an absolute root must not pass through
    looking like a legitimate relative path.
    """
    name = raw.split(b"\x00", 1)[0].decode("utf-8", errors="replace")
    name = name.replace("\\", "/")
    parts = [p for p in name.split("/") if p not in ("", ".")]
    if any(p == ".." for p in parts) or name.startswith("/"):
        return "!unsafe:" + "/".join(parts)
    return "/".join(parts)


def read_file_list(path: Path, header: dict) -> list[dict]:
    """Read the package's file list. Requires the optional lz4 dependency.

    Raises Lz4Unavailable when lz4 is missing, so the caller reports the list as
    not read rather than as empty.
    """
    try:
        import lz4.block
    except ImportError as exc:  # pragma: no cover - exercised via lz4_available
        raise Lz4Unavailable(
            "reading a .pak file list needs the optional 'lz4' dependency: "
            "pip install 'modcheck[bg3]'") from exc

    offset = header["file_list_offset"]
    list_size = header["file_list_size"]
    if list_size < 4:
        raise InspectionError(f"file list size {list_size} is too small to hold a count")
    if list_size > MAX_COMPRESSED_LIST_BYTES:
        raise InspectionError(
            f"file list is {list_size} bytes, over the {MAX_COMPRESSED_LIST_BYTES} "
            "byte inspection limit")

    file_size = path.stat().st_size
    if offset + list_size > file_size:
        raise InspectionError(
            f"file list at {offset}+{list_size} runs past the end of the package "
            f"({file_size} bytes)")

    with path.open("rb") as fh:
        fh.seek(offset)
        count_raw = fh.read(4)
        if len(count_raw) < 4:
            raise InspectionError("truncated file list header")
        num_files = struct.unpack("<i", count_raw)[0]
        if num_files < 0 or num_files > MAX_ENTRIES:
            raise InspectionError(
                f"package declares {num_files} entries, outside the accepted range "
                f"0..{MAX_ENTRIES}")
        expected = num_files * FILE_ENTRY_SIZE
        if expected > MAX_FILE_LIST_BYTES:
            raise InspectionError(
                f"file list would decompress to {expected} bytes, over the "
                f"{MAX_FILE_LIST_BYTES} byte limit")
        compressed = fh.read(list_size - 4)

    if num_files == 0:
        return []

    # LSPK stores no size prefix, so uncompressed_size must be supplied. It is a
    # BUFFER UPPER BOUND, not an assertion about output length -- lz4 returns
    # fewer bytes without complaint -- so the exact length is checked here.
    try:
        raw = lz4.block.decompress(compressed, uncompressed_size=expected)
    except Exception as exc:  # lz4.block.LZ4BlockError and friends
        raise InspectionError(f"file list did not decompress: {exc}") from exc
    if len(raw) != expected:
        raise InspectionError(
            f"file list decompressed to {len(raw)} bytes, expected exactly "
            f"{expected} ({num_files} entries x {FILE_ENTRY_SIZE})")

    entries = []
    for index in range(num_files):
        (name, offset_lo, offset_hi, archive_part, flags, size_on_disk,
         uncompressed_size) = LSPK_FILE_ENTRY_V18.unpack_from(
            raw, index * FILE_ENTRY_SIZE)
        entries.append({
            "name": _safe_entry_name(name),
            "offset": offset_lo | (offset_hi << 32),
            "archive_part": archive_part,
            "flags": flags,
            "size_on_disk": size_on_disk,
            "uncompressed_size": uncompressed_size,
        })
    return entries


def parse_lspk_header(raw: bytes) -> dict:
    if raw[:4] != LSPK_MAGIC:
        raise InspectionError(f"not an LSPK package: magic is {raw[:4]!r}")
    if len(raw) < 4 + LSPK_V18.size:
        raise InspectionError("LSPK header truncated")
    version, file_list_offset, file_list_size, flags, priority, md5, num_parts = \
        LSPK_V18.unpack_from(raw, 4)
    out = {"version": version, "flags": flags, "priority": priority,
           "num_parts": num_parts, "md5": md5.hex(),
           "file_list_offset": file_list_offset, "file_list_size": file_list_size}
    if version != 18:
        # v16 and earlier lay the header out differently; report the version and
        # stop rather than mis-parsing.
        out["unsupported_layout"] = True
    return out


def parse_meta_lsx(raw: bytes) -> dict:
    """Read ModuleInfo attributes out of a meta.lsx."""
    try:
        root = ET.fromstring(raw)
    except ET.ParseError as exc:
        raise InspectionError(f"meta.lsx is not well-formed XML: {exc}") from exc
    info: dict = {"dependencies": []}
    for node in root.iter("node"):
        if node.get("id") == "ModuleInfo":
            for attr in node.findall("attribute"):
                key = attr.get("id")
                if key:
                    info[key] = attr.get("value")
        elif node.get("id") == "ModuleShortDesc":
            dep = {attr.get("id"): attr.get("value") for attr in node.findall("attribute")}
            if dep.get("UUID"):
                info["dependencies"].append(dep)
    return info


def inspect(artifact: Artifact) -> Inspection:
    ins = artifact.base_inspection("bg3_mod")
    ins.game = "bg3"
    ins.loader = "bg3_modmanager"

    if artifact.path.suffix.lower() == ".pak":
        with artifact.path.open("rb") as fh:
            head = fh.read(4 + LSPK_V18.size)
        header = parse_lspk_header(head)
        ins.add("package_format", f"LSPK v{header['version']}", "extracted", "LSPK header")
        ins.add("package_priority", header["priority"], "extracted", "LSPK header")
        ins.add("package_parts", header["num_parts"], "extracted", "LSPK header")
        ins.checked = ["LSPK package header"]
        ins.not_checked = [
            "meta.lsx module identity inside the package",
            "stats, scripts and asset contents",
        ]

        if header.get("unsupported_layout"):
            ins.not_checked.append(
                f"package file list: LSPK v{header['version']} lays the header out "
                "differently and is not parsed")
            ins.warnings.append(
                f"LSPK v{header['version']} is not the version this reader handles; "
                "only the version field is reported")
            return ins

        try:
            entries = read_file_list(artifact.path, header)
        except Lz4Unavailable as exc:
            # A skipped section must never read as an empty package.
            ins.add("file_list_read", False, "extracted", "LSPK file list")
            ins.not_checked.append(f"package file list: {exc}")
            ins.warnings.append(
                "the package file list was NOT read, so this inspection says nothing "
                "about the package's contents")
        except InspectionError as exc:
            ins.add("file_list_read", False, "extracted", "LSPK file list")
            ins.not_checked.append(f"package file list: {exc}")
            ins.warnings.append(f"package file list unreadable: {exc}")
        else:
            ins.add("file_list_read", True, "extracted", "LSPK file list")
            ins.add("entry_count", len(entries), "extracted", "LSPK file list")
            ins.add("entries_detail", entries[:5000], "extracted", "LSPK file list")
            ins.entries = [e["name"] for e in entries]
            ins.checked.append("package file list (LZ4 block, entry count and sizes "
                               "validated against the header)")
            unsafe = [e["name"] for e in entries if e["name"].startswith("!unsafe:")]
            if unsafe:
                ins.warnings.append(
                    f"{len(unsafe)} entry name(s) are path-unsafe and were flagged: "
                    f"{unsafe[:3]}")

        ins.warnings.append(
            "inspected a bare .pak: module UUID and dependencies live in its meta.lsx, "
            "which is not parsed")
        return ins

    entries = zip_entries(artifact.path)
    ins.entries = entries
    info_member = find_member(entries, "info.json")
    meta_member = find_member(entries, "meta.lsx")
    paks = [e for e in entries if e.lower().endswith(".pak")]
    ins.add("pak_files", paks, "extracted", "archive layout")

    if info_member:
        try:
            info = json.loads(read_zip_member(artifact.path, info_member).decode("utf-8-sig"))
        except (UnicodeDecodeError, json.JSONDecodeError) as exc:
            raise InspectionError(f"{info_member} is not valid JSON: {exc}") from exc
        mods = info.get("Mods") or []
        primary = mods[0] if mods else {}
        loc = info_member
        ins.add("mod_id", primary.get("UUID"), "declared", loc)
        ins.add("name", primary.get("Name"), "declared", loc)
        ins.add("folder", primary.get("Folder"), "declared", loc)
        ins.add("version", primary.get("Version") or primary.get("Version64"), "declared", loc)
        ins.add("author", primary.get("Author"), "declared", loc)
        ins.add("dependencies",
                [{"id": d.get("UUID"), "name": d.get("Name"), "required": True}
                 for d in (primary.get("Dependencies") or [])],
                "declared", loc)
        ins.add("declared_md5", info.get("MD5"), "declared", loc)
        if len(mods) > 1:
            ins.add("additional_modules", [m.get("UUID") for m in mods[1:]], "declared", loc)
    elif meta_member:
        info = parse_meta_lsx(read_zip_member(artifact.path, meta_member))
        loc = meta_member
        ins.add("mod_id", info.get("UUID"), "extracted", loc)
        ins.add("name", info.get("Name"), "extracted", loc)
        ins.add("folder", info.get("Folder"), "extracted", loc)
        ins.add("version", info.get("Version64") or info.get("Version"), "extracted", loc)
        ins.add("dependencies",
                [{"id": d.get("UUID"), "name": d.get("Name"), "required": True}
                 for d in info.get("dependencies", [])],
                "extracted", loc)
    else:
        raise InspectionError("no info.json or meta.lsx; cannot establish module identity")

    ins.checked = [m for m in (info_member, meta_member) if m] + ["archive layout"]
    ins.not_checked = [
        "contents of the .pak files (stats entries, scripts, assets)",
        "whether the declared MD5 matches the shipped .pak",
        "load order interaction with other modules",
    ]
    return ins


def detect(path: Path) -> bool:
    if path.is_dir():
        return False
    if path.suffix.lower() == ".pak":
        try:
            with path.open("rb") as fh:
                return fh.read(4) == LSPK_MAGIC
        except OSError:
            return False
    try:
        entries = zip_entries(path)
    except InspectionError:
        return False
    return bool(find_member(entries, "meta.lsx")) or (
        bool(find_member(entries, "info.json")) and any(e.lower().endswith(".pak") for e in entries))

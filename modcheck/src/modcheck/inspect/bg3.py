"""Baldur's Gate 3 mods: info.json metadata and LSPK (.pak) package headers.

A BG3 mod's identity is the UUID + Folder in its meta.lsx; mod managers read the
same values from the info.json that ships beside the .pak. The .pak file list
lives in an LZ4-compressed block; we read it when the optional ``lz4`` package is
available and say so plainly when it is not, rather than guessing at contents.
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
            "package file list (LZ4-compressed; needs the optional lz4 package)",
            "meta.lsx module identity inside the package",
            "stats, scripts and asset contents",
        ]
        ins.warnings.append(
            "inspected a bare .pak: module UUID and dependencies live inside it and were "
            "not read")
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

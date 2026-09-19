"""Bethesda plugin files (.esp/.esm/.esl) for Skyrim SE, Fallout 4 and Fallout: New Vegas.

This reads the **TES4 header record only**: masters, flags, author, record count.
That is the minimum needed to establish a plugin's real identity and its master
dependency edges, which is what load-order and missing-master analysis needs.

It deliberately does not reimplement a record-level parser. For record- and
field-level conflict detection the ecosystem already has xEdit and esplugin; we
call those rather than duplicate them (see the game pack's toolchain list).

Format reference: the TES4 header record is a 24-byte record header followed by
subrecords of a 4-byte signature and a 2-byte length. See the pack sources for
the retrieved specification.
"""
from __future__ import annotations

import struct
from pathlib import Path

from .base import Artifact, Inspection, InspectionError

RECORD_HEADER = struct.Struct("<4sIIIII")  # type, dataSize, flags, formId, vcInfo, version
SUBRECORD_HEADER = struct.Struct("<4sH")

FLAG_MASTER = 0x00000001
FLAG_LIGHT = 0x00000200  # "light master" / ESL, Skyrim SE 1.5.39+ and Fallout 4
FLAG_LOCALIZED = 0x00000080

PLUGIN_SUFFIXES = {".esp", ".esm", ".esl"}


def _zstring(data: bytes) -> str:
    return data.split(b"\x00", 1)[0].decode("cp1252", errors="replace")


def parse_header(raw: bytes) -> dict:
    """Parse the TES4 header record. Raises InspectionError if it is not one."""
    if len(raw) < RECORD_HEADER.size:
        raise InspectionError("file is shorter than a record header")
    sig, data_size, flags, form_id, _vc, _ver = RECORD_HEADER.unpack_from(raw, 0)
    if sig == b"TES3":
        raise InspectionError("TES3 (Morrowind) plugins are not in scope")
    if sig != b"TES4":
        raise InspectionError(f"not a TES4 plugin: leading record is {sig!r}")

    body = raw[RECORD_HEADER.size:RECORD_HEADER.size + data_size]
    if len(body) < data_size:
        raise InspectionError(
            f"header record claims {data_size} bytes but only {len(body)} are present")

    out: dict = {
        "signature": "TES4",
        "flags_raw": flags,
        "is_master": bool(flags & FLAG_MASTER),
        "is_light": bool(flags & FLAG_LIGHT),
        "is_localized": bool(flags & FLAG_LOCALIZED),
        "form_id": form_id,
        "masters": [],
        "author": None,
        "description": None,
        "header_version": None,
        "record_count": None,
        "next_object_id": None,
        "overridden_forms": 0,
    }

    offset = 0
    while offset + SUBRECORD_HEADER.size <= len(body):
        sub_sig, sub_size = SUBRECORD_HEADER.unpack_from(body, offset)
        offset += SUBRECORD_HEADER.size
        chunk = body[offset:offset + sub_size]
        if len(chunk) < sub_size:
            raise InspectionError(f"truncated subrecord {sub_sig!r}")
        offset += sub_size

        if sub_sig == b"HEDR" and sub_size >= 12:
            version, count, next_id = struct.unpack_from("<fiI", chunk, 0)
            out["header_version"] = round(version, 4)
            out["record_count"] = count
            out["next_object_id"] = next_id
        elif sub_sig == b"CNAM":
            out["author"] = _zstring(chunk)
        elif sub_sig == b"SNAM":
            out["description"] = _zstring(chunk)
        elif sub_sig == b"MAST":
            out["masters"].append(_zstring(chunk))
        elif sub_sig == b"ONAM":
            out["overridden_forms"] = sub_size // 4
    return out


def inspect(artifact: Artifact, game: str | None = None) -> Inspection:
    with artifact.path.open("rb") as fh:
        raw = fh.read(1 << 16)
    header = parse_header(raw)

    ins = artifact.base_inspection("bethesda_plugin")
    ins.game = game
    ins.loader = "bethesda_plugin"
    loc = "TES4 header record"

    suffix = artifact.path.suffix.lower()
    ins.add("plugin_name", artifact.path.name, "extracted", loc)
    ins.add("mod_id", artifact.path.name, "extracted", loc)
    ins.add("author", header["author"], "declared", loc)
    ins.add("description", header["description"], "declared", loc)
    ins.add("header_version", header["header_version"], "extracted", loc)
    ins.add("record_count", header["record_count"], "declared", loc)
    ins.add("next_object_id", header["next_object_id"], "extracted", loc)
    ins.add("masters", header["masters"], "extracted", loc)
    ins.add("dependencies",
            [{"id": m, "kind": "content_mod", "required": True} for m in header["masters"]],
            "extracted", loc)
    ins.add("is_master", header["is_master"], "extracted", loc)
    ins.add("is_localized", header["is_localized"], "extracted", loc)
    ins.add("overridden_form_count", header["overridden_forms"], "extracted", loc)

    # An .esl is treated as light by the engine irrespective of the header flag;
    # the flag is what makes an .esp light. Report both, do not collapse them.
    ins.add("light_flag_set", header["is_light"], "extracted", loc)
    ins.add("extension", suffix, "extracted", "filename")
    ins.add("loads_in_light_slot", header["is_light"] or suffix == ".esl", "extracted",
            "header flag 0x200 or .esl extension")

    if header["is_light"] and header["next_object_id"] and header["next_object_id"] > 0xFFF:
        ins.warnings.append(
            "light plugin with nextObjectID above 0xFFF: new FormIDs may fall outside "
            "the light FormID range")

    ins.checked = ["TES4 header record: flags, masters, author, HEDR counts"]
    ins.not_checked = [
        "every record and field below the header (use xEdit / esplugin for that)",
        "whether the masters are present or at compatible versions",
        "script (.pex) and mesh/texture assets shipped alongside the plugin",
        "whether record contents conflict with another plugin",
    ]
    return ins


def detect(path: Path) -> bool:
    if path.is_dir() or path.suffix.lower() not in PLUGIN_SUFFIXES:
        return False
    try:
        with path.open("rb") as fh:
            return fh.read(4) == b"TES4"
    except OSError:
        return False

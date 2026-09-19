"""Cyberpunk 2077 mods.

Cyberpunk mods are layered: REDmod modules under ``mods/``, raw ``.archive``
files under ``archive/pc/mod/``, redscript under ``r6/scripts/``, CET Lua under
``bin/x64/plugins/cyber_engine_tweaks/mods/`` and TweakXL/ArchiveXL YAML under
``r6/tweaks/``. The install path *is* the loader contract, so the layout is the
compatibility surface: two mods shipping the same path collide on install.
"""
from __future__ import annotations

import json
from pathlib import Path

from .base import Artifact, Inspection, InspectionError, read_zip_member, zip_entries

ARCHIVE_MAGIC = b"RDAR"

LAYERS = {
    "redmod": "mods/",
    "archive": "archive/pc/mod/",
    "archive_patch": "archive/pc/patch/",
    "redscript": "r6/scripts/",
    "tweakxl": "r6/tweaks/",
    "cet": "bin/x64/plugins/cyber_engine_tweaks/mods/",
    "red4ext": "red4ext/plugins/",
    "input": "r6/input/",
    "config": "r6/config/",
}


def _norm(entry: str) -> str:
    return entry.replace("\\", "/").lstrip("./").lower()


def inspect(artifact: Artifact) -> Inspection:
    entries = zip_entries(artifact.path)
    ins = artifact.base_inspection("cyberpunk_mod")
    ins.game = "cyberpunk2077"
    ins.entries = entries

    normalised = [_norm(e) for e in entries if not e.endswith("/")]
    layers = sorted({name for name, prefix in LAYERS.items()
                     if any(n.startswith(prefix) for n in normalised)})
    ins.add("layers", layers, "extracted", "archive layout")
    ins.loader = layers[0] if layers else None

    ins.add("archive_files", [n for n in normalised if n.endswith(".archive")], "extracted",
            "archive layout")
    ins.add("redscript_files", [n for n in normalised if n.endswith(".reds")], "extracted",
            "archive layout")
    ins.add("tweak_files", [n for n in normalised
                            if n.startswith(LAYERS["tweakxl"]) and n.endswith((".yaml", ".yml"))],
            "extracted", "archive layout")
    ins.add("install_paths", normalised[:2000], "extracted", "archive layout")

    # REDmod modules declare themselves in info.json.
    redmod_infos = [e for e in entries if _norm(e).startswith("mods/") and _norm(e).endswith("/info.json")]
    modules = []
    for member in redmod_infos:
        try:
            info = json.loads(read_zip_member(artifact.path, member).decode("utf-8-sig"))
        except (InspectionError, UnicodeDecodeError, json.JSONDecodeError) as exc:
            ins.warnings.append(f"{member} unreadable: {exc}")
            continue
        modules.append({
            "folder": _norm(member).split("/")[1],
            "name": info.get("name"),
            "version": info.get("version"),
            "description": info.get("description"),
            "member": member,
        })
    if modules:
        ins.add("redmod_modules", modules, "declared", "mods/*/info.json")
        ins.add("mod_id", modules[0]["folder"], "extracted", "mods/ folder name")
        ins.add("name", modules[0].get("name"), "declared", modules[0]["member"])
        ins.add("version", modules[0].get("version"), "declared", modules[0]["member"])
    else:
        stem = artifact.path.stem
        ins.add("mod_id", stem, "extracted", "filename")
        ins.warnings.append(
            "no REDmod info.json: this mod has no declared id, so identity is the install "
            "paths it ships")

    if not layers:
        raise InspectionError("no recognised Cyberpunk install path in this archive")

    ins.checked = ["archive layout against known Cyberpunk install paths",
                   "REDmod info.json where present"]
    ins.not_checked = [
        "contents of .archive files (which game resources they replace)",
        "redscript @replaceMethod/@wrapMethod targets",
        "TweakXL record targets inside the YAML",
        "whether the mod matches the installed game build",
    ]
    return ins


def detect(path: Path) -> bool:
    if path.is_dir() or path.suffix.lower() == ".archive":
        if path.suffix.lower() == ".archive":
            try:
                with path.open("rb") as fh:
                    return fh.read(4) == ARCHIVE_MAGIC
            except OSError:
                return False
        return False
    try:
        entries = [_norm(e) for e in zip_entries(path)]
    except InspectionError:
        return False
    return any(any(e.startswith(p) for p in LAYERS.values()) for e in entries)

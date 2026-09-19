"""Project Zomboid mods: mod.info plus the Lua/media overlap surface.

PZ loads ``media/lua/...`` files by path. Two mods shipping the same relative
path is the ecosystem's actual last-one-wins collision, so the file list is the
signal that matters, not the metadata.
"""
from __future__ import annotations

from pathlib import Path

from .base import Artifact, Inspection, InspectionError, find_member, read_zip_member, zip_entries

KNOWN_KEYS = {"name", "id", "description", "poster", "icon", "require", "url",
              "pack", "tiledef", "modversion", "versionmin", "versionmax", "category",
              "author", "loadmoddata"}


def parse_mod_info(raw: bytes) -> tuple[dict[str, list[str]], list[str]]:
    """mod.info is line-based ``key=value``. Repeated keys accumulate."""
    fields: dict[str, list[str]] = {}
    unknown: list[str] = []
    for line in raw.decode("utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, _, value = line.partition("=")
        key = key.strip().lower()
        fields.setdefault(key, []).append(value.strip())
        if key not in KNOWN_KEYS:
            unknown.append(key)
    return fields, unknown


def inspect(artifact: Artifact) -> Inspection:
    is_dir = artifact.path.is_dir()
    if is_dir:
        info_path = next((p for p in artifact.path.rglob("mod.info")), None)
        if info_path is None:
            raise InspectionError("no mod.info")
        raw = info_path.read_bytes()
        loc = str(info_path.relative_to(artifact.path))
        names = [str(p.relative_to(artifact.path)) for p in artifact.path.rglob("*") if p.is_file()]
    else:
        names = zip_entries(artifact.path)
        member = find_member(names, "mod.info")
        if not member:
            raise InspectionError("no mod.info in archive")
        raw = read_zip_member(artifact.path, member)
        loc = member

    fields, unknown = parse_mod_info(raw)
    ins = artifact.base_inspection("projectzomboid_mod")
    ins.game = "projectzomboid"
    ins.loader = "projectzomboid"
    ins.entries = names

    first = lambda k: fields.get(k, [None])[0]  # noqa: E731
    ins.add("mod_id", first("id"), "declared", loc)
    ins.add("name", first("name"), "declared", loc)
    ins.add("version", first("modversion"), "declared", loc)
    ins.add("pz_version_min", first("versionmin"), "declared", loc)
    ins.add("pz_version_max", first("versionmax"), "declared", loc)
    ins.add("dependencies",
            [{"id": r, "kind": "content_mod", "required": True} for r in fields.get("require", [])
             for r in [x.strip() for x in r.split(";") if x.strip()]],
            "declared", loc)
    ins.add("tile_definitions", fields.get("tiledef", []), "declared", loc)
    ins.add("packs", fields.get("pack", []), "declared", loc)
    if unknown:
        ins.warnings.append(f"unrecognised mod.info keys: {sorted(set(unknown))}")

    lua = sorted(n for n in names if n.lower().endswith(".lua"))
    # The path under media/ is what the game keys on, so normalise to that.
    def media_rel(n: str) -> str:
        idx = n.lower().find("media/")
        return n[idx:] if idx >= 0 else n

    ins.add("lua_files", [media_rel(n) for n in lua][:2000], "extracted", "archive layout")
    ins.add("lua_scopes",
            sorted({media_rel(n).split("/")[2] for n in lua
                    if len(media_rel(n).split("/")) > 2 and media_rel(n).startswith("media/lua/")}),
            "extracted", "archive layout")
    ins.add("script_files",
            [media_rel(n) for n in names if n.lower().endswith(".txt") and "/scripts/" in n.lower()][:2000],
            "extracted", "archive layout")
    ins.add("has_b42_version_folders",
            any(p.split("/")[0].isdigit() for p in names if "/" in p),
            "extracted", "archive layout")

    ins.checked = ["mod.info fields", "Lua and script file layout under media/"]
    ins.not_checked = [
        "Lua contents: which functions or events each file overrides",
        "whether required mod ids are installed",
        "sandbox options and tiledef contents",
    ]
    return ins


def detect(path: Path) -> bool:
    if path.is_dir():
        return any(path.rglob("mod.info"))
    try:
        return find_member(zip_entries(path), "mod.info") is not None
    except InspectionError:
        return False

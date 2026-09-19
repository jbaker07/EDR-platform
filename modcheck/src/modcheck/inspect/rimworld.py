"""RimWorld mods: About/About.xml, LoadFolders.xml and the Defs surface.

RimWorld composes mods by XML inheritance and patch operations. Two mods
touching the same Def is normal; what matters is whether they *replace* the
same Def or *patch* the same xpath.
"""
from __future__ import annotations

import re
import xml.etree.ElementTree as ET
from pathlib import Path

from .base import Artifact, Inspection, InspectionError, find_member, read_zip_member, zip_entries


def _text(node: ET.Element | None) -> str | None:
    return node.text.strip() if node is not None and node.text else None


def _li_list(parent: ET.Element | None, child: str | None = None) -> list[str]:
    if parent is None:
        return []
    out = []
    for li in parent.findall("li"):
        if child is not None:
            value = _text(li.find(child))
        else:
            value = _text(li)
        if value:
            out.append(value)
    return out


def inspect(artifact: Artifact) -> Inspection:
    is_dir = artifact.path.is_dir()
    entries = None if is_dir else zip_entries(artifact.path)

    if is_dir:
        about = next((p for p in artifact.path.rglob("About.xml")), None)
        if about is None:
            raise InspectionError("no About/About.xml")
        raw = about.read_bytes()
        loc = str(about.relative_to(artifact.path))
        all_names = [str(p.relative_to(artifact.path)) for p in artifact.path.rglob("*") if p.is_file()]
    else:
        member = find_member(entries, "About.xml")
        if not member:
            raise InspectionError("no About.xml in archive")
        raw = read_zip_member(artifact.path, member)
        loc = member
        all_names = entries

    try:
        root = ET.fromstring(raw)
    except ET.ParseError as exc:
        raise InspectionError(f"About.xml is not well-formed XML: {exc}") from exc

    ins = artifact.base_inspection("rimworld_mod")
    ins.game = "rimworld"
    ins.loader = "rimworld"
    ins.entries = all_names

    ins.add("mod_id", _text(root.find("packageId")), "declared", loc)
    ins.add("name", _text(root.find("name")), "declared", loc)
    ins.add("author", _text(root.find("author")), "declared", loc)
    ins.add("supported_versions", _li_list(root.find("supportedVersions")), "declared", loc)
    ins.add("mod_version", _text(root.find("modVersion")), "declared", loc)

    deps = [{"id": pid, "required": True}
            for pid in _li_list(root.find("modDependencies"), "packageId")]
    ins.add("dependencies", deps, "declared", loc)
    ins.add("load_after", _li_list(root.find("loadAfter")), "declared", loc)
    ins.add("load_before", _li_list(root.find("loadBefore")), "declared", loc)
    ins.add("incompatible_with", _li_list(root.find("incompatibleWith")), "declared", loc)

    # Extracted structure: version folders, assemblies, Defs and PatchOperations.
    version_dirs = sorted({m.split("/")[0] for m in all_names
                           if re.match(r"^\d+\.\d+/", m)})
    ins.add("version_folders", version_dirs, "extracted", "archive layout")
    ins.add("has_assemblies", any(m.lower().endswith(".dll") for m in all_names),
            "extracted", "archive layout")
    ins.add("def_files", [m for m in all_names if "/Defs/" in m and m.endswith(".xml")][:500],
            "extracted", "archive layout")
    ins.add("patch_files", [m for m in all_names if "/Patches/" in m and m.endswith(".xml")][:500],
            "extracted", "archive layout")
    ins.add("has_load_folders", any(m.split("/")[-1] == "LoadFolders.xml" for m in all_names),
            "extracted", "archive layout")

    ins.checked = ["About.xml metadata and declared load rules", "file layout",
                   "presence of Defs, Patches, assemblies and version folders"]
    ins.not_checked = [
        "Def contents and xpath targets of individual PatchOperations",
        "Harmony patches inside any shipped assembly",
        "whether supportedVersions matches what the code actually does",
    ]
    return ins


def detect(path: Path) -> bool:
    if path.is_dir():
        return any(path.rglob("About.xml"))
    try:
        return find_member(zip_entries(path), "About.xml") is not None
    except InspectionError:
        return False

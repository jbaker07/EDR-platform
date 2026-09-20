"""Stardew Valley: SMAPI mods and Content Patcher content packs.

SMAPI reads ``manifest.json``. Content packs additionally carry ``content.json``,
whose ``Changes`` entries name the exact assets they touch -- which is the real
compatibility surface, and is extracted, not declared.
"""
from __future__ import annotations

import json
from pathlib import Path

from .base import Artifact, Inspection, InspectionError, find_member, read_zip_member, zip_entries


def _load_json(raw: bytes, what: str) -> dict:
    try:
        return json.loads(raw.decode("utf-8-sig"))
    except (UnicodeDecodeError, json.JSONDecodeError) as exc:
        raise InspectionError(f"{what} is not valid JSON: {exc}") from exc


def _read(artifact: Artifact, entries: list[str] | None, member: str) -> bytes:
    if entries is None:  # unpacked directory
        return (artifact.path / member).read_bytes()
    return read_zip_member(artifact.path, member)


def inspect(artifact: Artifact) -> Inspection:
    is_dir = artifact.path.is_dir()
    entries = None if is_dir else zip_entries(artifact.path)
    if is_dir:
        manifest_path = artifact.path / "manifest.json"
        if not manifest_path.exists():
            raise InspectionError("no manifest.json")
        manifest_member = "manifest.json"
    else:
        manifest_member = find_member(entries, "manifest.json")
        if not manifest_member:
            raise InspectionError("no manifest.json in archive")

    ins = artifact.base_inspection("smapi_mod")
    ins.game = "stardewvalley"
    ins.loader = "smapi"
    ins.entries = entries or []
    manifest = _load_json(_read(artifact, entries, manifest_member), manifest_member)
    loc = manifest_member

    ins.add("mod_id", manifest.get("UniqueID"), "declared", loc)
    ins.add("name", manifest.get("Name"), "declared", loc)
    ins.add("version", manifest.get("Version"), "declared", loc)
    ins.add("author", manifest.get("Author"), "declared", loc)
    ins.add("minimum_api_version", manifest.get("MinimumApiVersion"), "declared", loc)
    ins.add("entry_dll", manifest.get("EntryDll"), "declared", loc)
    ins.add("update_keys", manifest.get("UpdateKeys", []), "declared", loc)

    content_pack_for = manifest.get("ContentPackFor")
    ins.add("content_pack_for", content_pack_for, "declared", loc)
    is_content_pack = bool(content_pack_for)
    ins.add("mod_type", "content_pack" if is_content_pack else "smapi_code_mod",
            "extracted", loc)

    deps = []
    for dep in manifest.get("Dependencies", []) or []:
        deps.append({
            "id": dep.get("UniqueID"),
            "versions": f">={dep['MinimumVersion']}" if dep.get("MinimumVersion") else "",
            "required": dep.get("IsRequired", True),
        })
    if is_content_pack and content_pack_for.get("UniqueID"):
        deps.append({
            "id": content_pack_for["UniqueID"],
            "versions": f">={content_pack_for['MinimumVersion']}"
            if content_pack_for.get("MinimumVersion") else "",
            "required": True,
            "declared_by": "ContentPackFor",
        })
    ins.add("dependencies", deps, "declared", loc)

    # Content Patcher: the Changes list is the precise asset-level surface.
    cp_member = "content.json" if is_dir else find_member(entries or [], "content.json")
    if cp_member and (is_dir and (artifact.path / cp_member).exists() or not is_dir):
        try:
            content = _load_json(_read(artifact, entries, cp_member), cp_member)
        except InspectionError as exc:
            ins.warnings.append(str(exc))
        else:
            ins.add("content_format", content.get("Format"), "declared", cp_member)
            changes = []
            for i, change in enumerate(content.get("Changes", []) or []):
                if not isinstance(change, dict):
                    continue
                # Target may be a comma-separated list of assets.
                targets = [t.strip() for t in str(change.get("Target") or "").split(",")
                           if t.strip()]
                changes.append({
                    "action": change.get("Action"),
                    "target": targets[0] if targets else None,
                    "targets": targets,
                    "fields": sorted((change.get("Fields") or {}).keys()) or None,
                    "entries": sorted((change.get("Entries") or {}).keys())[:50] or None,
                    "when": change.get("When") or None,
                    # Priority decides which patch wins when several touch one
                    # asset, so it is the field the whole analysis turns on.
                    "priority": change.get("Priority"),
                    "log_name": change.get("LogName"),
                    "from_file": change.get("FromFile"),
                    "target_locale": change.get("TargetLocale"),
                    "update": change.get("Update"),
                    # Where a creator finds this patch again.
                    "source_file": cp_member,
                    "index": i,
                })
            ins.add("content_changes", changes, "extracted", cp_member)
            ins.add("targets",
                    sorted({t for c in changes for t in (c.get("targets") or [])}),
                    "extracted", cp_member)

    ins.checked = ["manifest.json", "declared dependencies"] + (
        ["content.json Changes: action, targets, Priority, When conditions, LogName "
         "and patch order"] if cp_member else [])
    ins.not_checked = [
        "C# assembly behaviour and Harmony patches in EntryDll",
        "whether declared targets exist in the installed game version",
        "token values that only resolve at runtime",
    ]
    return ins


def detect(path: Path) -> bool:
    if path.is_dir():
        return (path / "manifest.json").exists()
    if path.suffix.lower() != ".zip":
        return False
    try:
        return find_member(zip_entries(path), "manifest.json") is not None
    except InspectionError:
        return False

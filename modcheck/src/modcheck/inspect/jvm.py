"""Minecraft mod jars: Fabric, Quilt, Forge and NeoForge manifests.

Reads only manifests that the loaders themselves read, so every fact here is an
author *declaration*, not a claim about what the bytecode does. The entries list
is extracted and is what later analysis uses for real file-level overlap.
"""
from __future__ import annotations

import json
import tomllib
import zipfile
from pathlib import Path

from .base import Artifact, Inspection, InspectionError, read_zip_member, zip_entries

FABRIC_MANIFEST = "fabric.mod.json"
QUILT_MANIFEST = "quilt.mod.json"
FORGE_MANIFEST = "META-INF/mods.toml"
NEOFORGE_MANIFEST = "META-INF/neoforge.mods.toml"


def _dep_list(raw: dict | None, required: bool) -> list[dict]:
    out = []
    for dep_id, constraint in (raw or {}).items():
        if isinstance(constraint, list):
            constraint = " || ".join(str(c) for c in constraint)
        out.append({"id": dep_id, "versions": str(constraint), "required": required})
    return out


def inspect_fabric(artifact: Artifact, entries: list[str], manifest_name: str = FABRIC_MANIFEST) -> Inspection:
    ins = artifact.base_inspection("minecraft_jar")
    ins.game = "minecraft"
    ins.loader = "quilt" if manifest_name == QUILT_MANIFEST else "fabric"
    ins.entries = entries
    raw = read_zip_member(artifact.path, manifest_name)
    try:
        manifest = json.loads(raw.decode("utf-8-sig"))
    except (UnicodeDecodeError, json.JSONDecodeError) as exc:
        raise InspectionError(f"{manifest_name} is not valid JSON: {exc}") from exc

    loc = manifest_name
    ins.add("mod_id", manifest.get("id"), "declared", loc)
    ins.add("version", manifest.get("version"), "declared", loc)
    ins.add("name", manifest.get("name"), "declared", loc)
    ins.add("license", manifest.get("license"), "declared", loc)
    ins.add("environment", manifest.get("environment", "*"), "declared", loc)
    ins.add("schema_version", manifest.get("schemaVersion"), "declared", loc)

    deps = _dep_list(manifest.get("depends"), True) + _dep_list(manifest.get("recommends"), False)
    ins.add("dependencies", deps, "declared", loc)
    ins.add("breaks", _dep_list(manifest.get("breaks"), False), "declared", loc)
    ins.add("conflicts", _dep_list(manifest.get("conflicts"), False), "declared", loc)
    ins.add("provides", manifest.get("provides", []), "declared", loc)

    entrypoints = manifest.get("entrypoints") or {}
    ins.add("entrypoints", {k: list(v) if isinstance(v, list) else [v] for k, v in entrypoints.items()},
            "declared", loc)

    # Mixins are the real overlap surface on Fabric: they name the classes and
    # methods a mod rewrites. Read the mixin configs themselves, not just the list.
    mixin_configs = manifest.get("mixins") or []
    names = [m["config"] if isinstance(m, dict) else m for m in mixin_configs]
    ins.add("mixin_configs", names, "declared", loc)
    targets, missing = [], []
    for cfg in names:
        if cfg not in entries:
            missing.append(cfg)
            continue
        try:
            data = json.loads(read_zip_member(artifact.path, cfg).decode("utf-8-sig"))
        except (InspectionError, UnicodeDecodeError, json.JSONDecodeError) as exc:
            ins.warnings.append(f"mixin config {cfg} unreadable: {exc}")
            continue
        package = data.get("package", "")
        for section in ("mixins", "client", "server"):
            for cls in data.get(section, []) or []:
                targets.append({"config": cfg, "side": section,
                                "class": f"{package}.{cls}" if package else cls})
    if missing:
        ins.warnings.append(f"mixin configs declared but absent from the jar: {missing}")
    ins.add("mixin_classes", targets, "extracted", "mixin configs")
    ins.add("access_widener", manifest.get("accessWidener"), "declared", loc)

    # Jar-in-jar: a Fabric mod can ship other mods inside META-INF/jars/. Those
    # nested mods satisfy dependencies at runtime, so resolution that ignores
    # them reports missing dependencies that are in fact present.
    nested = read_nested_jars(artifact.path, entries)
    ins.add("nested_mods", nested, "extracted", "META-INF/jars/")

    ins.checked = [manifest_name, "declared dependencies", "mixin configs listed in the manifest",
                   "jar entry list", "nested jar-in-jar manifests (one level)"]
    ins.not_checked = [
        "bytecode: which methods the mixins actually rewrite and how",
        "runtime behaviour of entrypoints",
        "whether declared versions match the code",
        "nested jars beyond the first level",
    ]
    return ins


def read_nested_jars(path: Path, entries: list[str], limit: int = 200) -> list[dict]:
    """Read the manifest of each jar under META-INF/jars/ (one level deep).

    Reading a member of a member needs the nested bytes in memory; nested mod
    jars are small, and oversized ones are skipped rather than inflated.
    """
    nested_names = [e for e in entries if e.startswith("META-INF/jars/") and e.endswith(".jar")]
    out: list[dict] = []
    if not nested_names:
        return out
    with zipfile.ZipFile(path) as outer:
        for name in nested_names[:limit]:
            try:
                info = outer.getinfo(name)
                if info.file_size > (16 << 20):
                    out.append({"jar": name, "error": "nested jar too large to inspect"})
                    continue
                with outer.open(info) as fh:
                    import io
                    nested_bytes = io.BytesIO(fh.read())
                with zipfile.ZipFile(nested_bytes) as inner:
                    names = inner.namelist()
                    manifest_name = next(
                        (m for m in (FABRIC_MANIFEST, QUILT_MANIFEST, NEOFORGE_MANIFEST,
                                     FORGE_MANIFEST) if m in names), None)
                    if manifest_name is None:
                        out.append({"jar": name, "error": "no loader manifest"})
                        continue
                    raw = inner.read(manifest_name)
                if manifest_name in (FABRIC_MANIFEST, QUILT_MANIFEST):
                    data = json.loads(raw.decode("utf-8-sig"))
                    out.append({"jar": name, "mod_id": data.get("id"),
                                "version": data.get("version"),
                                "provides": data.get("provides", [])})
                else:
                    data = tomllib.loads(raw.decode("utf-8-sig"))
                    mods = data.get("mods") or [{}]
                    out.append({"jar": name, "mod_id": mods[0].get("modId"),
                                "version": mods[0].get("version"), "provides": []})
            except (KeyError, OSError, zipfile.BadZipFile, UnicodeDecodeError,
                    json.JSONDecodeError, tomllib.TOMLDecodeError) as exc:
                out.append({"jar": name, "error": str(exc)})
    return out


def inspect_forge(artifact: Artifact, entries: list[str], manifest_name: str) -> Inspection:
    loader = "neoforge" if manifest_name == NEOFORGE_MANIFEST else "forge"
    ins = artifact.base_inspection("minecraft_jar")
    ins.game = "minecraft"
    ins.loader = loader
    ins.entries = entries
    raw = read_zip_member(artifact.path, manifest_name)
    try:
        manifest = tomllib.loads(raw.decode("utf-8-sig"))
    except (UnicodeDecodeError, tomllib.TOMLDecodeError) as exc:
        raise InspectionError(f"{manifest_name} is not valid TOML: {exc}") from exc

    loc = manifest_name
    ins.add("mod_loader", manifest.get("modLoader"), "declared", loc)
    ins.add("loader_version_range", manifest.get("loaderVersion"), "declared", loc)
    ins.add("license", manifest.get("license"), "declared", loc)

    mods = manifest.get("mods") or []
    if not mods:
        raise InspectionError(f"{manifest_name} declares no [[mods]] block")
    primary = mods[0]
    ins.add("mod_id", primary.get("modId"), "declared", loc)
    ins.add("version", primary.get("version"), "declared", loc)
    ins.add("name", primary.get("displayName"), "declared", loc)
    if len(mods) > 1:
        ins.add("additional_mod_ids", [m.get("modId") for m in mods[1:]], "declared", loc)

    deps = []
    for mod_id, entries_ in (manifest.get("dependencies") or {}).items():
        for dep in entries_ if isinstance(entries_, list) else [entries_]:
            # Forge uses `mandatory = true`; NeoForge uses `type = "required"`.
            required = dep.get("mandatory")
            if required is None:
                required = str(dep.get("type", "required")).lower() == "required"
            deps.append({
                "id": dep.get("modId"),
                "versions": dep.get("versionRange", ""),
                "required": bool(required),
                "ordering": dep.get("ordering", "NONE"),
                "side": dep.get("side", "BOTH"),
                "declared_by": mod_id,
            })
    ins.add("dependencies", deps, "declared", loc)

    mixin_cfgs = [m.get("config") for m in (manifest.get("mixins") or []) if isinstance(m, dict)]
    ins.add("mixin_configs", mixin_cfgs, "declared", loc)
    ins.add("access_transformers",
            [a.get("file") for a in (manifest.get("accessTransformers") or []) if isinstance(a, dict)],
            "declared", loc)

    nested = read_nested_jars(artifact.path, entries)
    ins.add("nested_mods", nested, "extracted", "META-INF/jars/")

    ins.checked = [manifest_name, "declared dependencies and ordering", "jar entry list",
                   "nested jar-in-jar manifests (one level)"]
    ins.not_checked = [
        "bytecode and coremod behaviour",
        "whether the version range matches the shipped code",
        "nested jars beyond the first level",
    ]
    return ins


def detect(path: Path) -> tuple[str, list[str]] | None:
    """Return (manifest_name, entries) if this jar is a known Minecraft mod."""
    try:
        entries = zip_entries(path)
    except InspectionError:
        return None
    for manifest in (FABRIC_MANIFEST, QUILT_MANIFEST, NEOFORGE_MANIFEST, FORGE_MANIFEST):
        if manifest in entries:
            return manifest, entries
    return None


def inspect(artifact: Artifact) -> Inspection:
    found = detect(artifact.path)
    if not found:
        raise InspectionError("no Fabric/Quilt/Forge/NeoForge manifest in this jar")
    manifest, entries = found
    if manifest in (FABRIC_MANIFEST, QUILT_MANIFEST):
        return inspect_fabric(artifact, entries, manifest)
    return inspect_forge(artifact, entries, manifest)

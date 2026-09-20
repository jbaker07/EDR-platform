"""Builders for synthetic mod artifacts.

These are written from each format's documented layout so the inspectors can be
tested without shipping game files. They establish that a parser reads the
layout it was written against -- they do NOT establish that real-world artifacts
look like this. ``test_real_artifacts.py`` covers that separately against an
actual published mod jar.
"""
from __future__ import annotations

import json
import struct
import zipfile
from pathlib import Path


def _zip(path: Path, members: dict[str, bytes | str]) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as zf:
        for name, content in members.items():
            zf.writestr(name, content if isinstance(content, bytes) else content.encode("utf-8"))
    return path


# --- Minecraft -----------------------------------------------------------
def fabric_jar(path: Path, mod_id: str = "examplemod", version: str = "1.0.0",
               depends: dict | None = None, mixin_targets: list[str] | None = None,
               breaks: dict | None = None) -> Path:
    manifest = {
        "schemaVersion": 1,
        "id": mod_id,
        "version": version,
        "name": "Example Mod",
        "license": "MIT",
        "environment": "*",
        "entrypoints": {"main": [f"com.example.{mod_id}.ExampleMod"]},
        "mixins": [f"{mod_id}.mixins.json"],
        "depends": depends if depends is not None else {
            "fabricloader": ">=0.15.0", "minecraft": "~1.21", "java": ">=21"},
    }
    if breaks:
        manifest["breaks"] = breaks
    mixin_cfg = {
        "required": True,
        "package": f"com.example.{mod_id}.mixin",
        "compatibilityLevel": "JAVA_21",
        "mixins": mixin_targets if mixin_targets is not None else ["ExampleMixin"],
        "injectors": {"defaultRequire": 1},
    }
    return _zip(path, {
        "fabric.mod.json": json.dumps(manifest, indent=2),
        f"{mod_id}.mixins.json": json.dumps(mixin_cfg, indent=2),
        f"com/example/{mod_id}/ExampleMod.class": b"\xca\xfe\xba\xbe",
    })


def forge_jar(path: Path, mod_id: str = "examplemod", neoforge: bool = False) -> Path:
    name = "META-INF/neoforge.mods.toml" if neoforge else "META-INF/mods.toml"
    dep_kind = 'type = "required"' if neoforge else "mandatory = true"
    toml = f"""
modLoader = "{'javafml' if not neoforge else 'javafml'}"
loaderVersion = "[47,)"
license = "MIT"

[[mods]]
modId = "{mod_id}"
version = "1.0.0"
displayName = "Example Mod"

[[dependencies.{mod_id}]]
modId = "minecraft"
{dep_kind}
versionRange = "[1.21,1.22)"
ordering = "NONE"
side = "BOTH"

[[dependencies.{mod_id}]]
modId = "jei"
{'type = "optional"' if neoforge else 'mandatory = false'}
versionRange = "[15.0,)"
ordering = "AFTER"
side = "CLIENT"
""".strip()
    return _zip(path, {name: toml, f"com/example/{mod_id}/Mod.class": b"\xca\xfe\xba\xbe"})


# --- Stardew Valley ------------------------------------------------------
def smapi_mod(path: Path, unique_id: str = "Example.CodeMod", deps: list[dict] | None = None,
              version: str = "1.2.0") -> Path:
    manifest = {
        "Name": "Example Code Mod",
        "Author": "example",
        "Version": version,
        "Description": "Example.",
        "UniqueID": unique_id,
        "EntryDll": "ExampleMod.dll",
        "MinimumApiVersion": "4.0.0",
        "Dependencies": deps or [],
        "UpdateKeys": ["Nexus:1234"],
    }
    return _zip(path, {f"{unique_id}/manifest.json": json.dumps(manifest, indent=2),
                       f"{unique_id}/ExampleMod.dll": b"MZ\x90\x00"})


def content_patcher_pack_with_changes(path: Path, unique_id: str,
                                      changes: list[dict]) -> Path:
    """A content pack carrying exact patch instructions.

    Used where the analysis turns on the content.json itself -- Priority, When,
    LogName, patch order -- rather than on the manifest.
    """
    manifest = {
        "Name": unique_id, "Author": "example", "Version": "1.0.0",
        "Description": "Example.", "UniqueID": unique_id,
        "ContentPackFor": {"UniqueID": "Pathoschild.ContentPatcher",
                           "MinimumVersion": "2.0.0"},
    }
    content = {"Format": "2.9.0", "Changes": changes}
    return _zip(path, {f"{unique_id}/manifest.json": json.dumps(manifest, indent=2),
                       f"{unique_id}/content.json": json.dumps(content, indent=2)})


def content_patcher_pack(path: Path, unique_id: str = "Example.CPPack",
                         targets: list[str] | None = None, fields: dict | None = None,
                         when: dict | None = None) -> Path:
    manifest = {
        "Name": "Example CP Pack",
        "Author": "example",
        "Version": "1.0.0",
        "Description": "Example.",
        "UniqueID": unique_id,
        "ContentPackFor": {"UniqueID": "Pathoschild.ContentPatcher", "MinimumVersion": "2.0.0"},
    }
    change: dict = {
        "Action": "EditData",
        "Target": (targets or ["Data/Objects"])[0],
        "Fields": fields or {"128": {"Price": 100}},
    }
    if when:
        change["When"] = when
    changes = [change]
    for extra in (targets or [])[1:]:
        load: dict = {"Action": "Load", "Target": extra, "FromFile": "assets/x.png"}
        if when:
            load["When"] = when
        changes.append(load)
    content = {"Format": "2.0.0", "Changes": changes}
    return _zip(path, {f"{unique_id}/manifest.json": json.dumps(manifest, indent=2),
                       f"{unique_id}/content.json": json.dumps(content, indent=2)})


# --- RimWorld ------------------------------------------------------------
def rimworld_mod(path: Path, package_id: str = "example.mod",
                 versions: list[str] | None = None, load_after: list[str] | None = None,
                 defs: list[str] | None = None, patches: list[str] | None = None) -> Path:
    versions = versions or ["1.5", "1.6"]
    about = ["<?xml version='1.0' encoding='utf-8'?>", "<ModMetaData>",
             "  <name>Example Mod</name>", "  <author>example</author>",
             f"  <packageId>{package_id}</packageId>",
             "  <supportedVersions>"]
    about += [f"    <li>{v}</li>" for v in versions]
    about += ["  </supportedVersions>"]
    if load_after:
        about += ["  <loadAfter>"] + [f"    <li>{p}</li>" for p in load_after] + ["  </loadAfter>"]
    about += ["  <modDependencies>", "    <li>", "      <packageId>brrainz.harmony</packageId>",
              "    </li>", "  </modDependencies>", "</ModMetaData>"]
    members: dict[str, bytes | str] = {"About/About.xml": "\n".join(about)}
    for d in defs or [f"{versions[-1]}/Defs/ThingDefs/Example.xml"]:
        members[d] = "<Defs><ThingDef><defName>Example</defName></ThingDef></Defs>"
    for p in patches or []:
        members[p] = "<Patch><Operation Class='PatchOperationReplace'/></Patch>"
    return _zip(path, members)


# --- Project Zomboid -----------------------------------------------------
def pz_mod(path: Path, mod_id: str = "ExampleMod", lua: list[str] | None = None,
           require: list[str] | None = None) -> Path:
    info = [f"name=Example Mod", f"id={mod_id}", "description=Example.",
            "poster=poster.png", "modversion=1.0.0", "versionMin=41.78"]
    if require:
        info.append("require=" + ";".join(require))
    members: dict[str, bytes | str] = {f"{mod_id}/mod.info": "\n".join(info)}
    for rel in lua or [f"{mod_id}/media/lua/client/Example.lua"]:
        members[rel] = "-- example\n"
    return _zip(path, members)


# --- Baldur's Gate 3 -----------------------------------------------------
def bg3_zip(path: Path, uuid: str = "11111111-2222-3333-4444-555555555555",
            folder: str = "ExampleMod", deps: list[dict] | None = None) -> Path:
    info = {"Mods": [{"Author": "example", "Name": "Example Mod", "Folder": folder,
                      "Version": "1.0.0.0", "UUID": uuid, "Created": "2026-01-01",
                      "Dependencies": deps or [], "Group": ""}],
            "MD5": "0" * 32}
    return _zip(path, {"info.json": json.dumps(info, indent=2),
                       f"{folder}.pak": lspk_bytes()})


def lspk_bytes(version: int = 18, file_list_offset: int = 4096,
               file_list_size: int = 128) -> bytes:
    return b"LSPK" + struct.pack("<IQIBB16sH", version, file_list_offset,
                                 file_list_size, 0, 0, b"\x00" * 16, 1)


FILE_ENTRY = struct.Struct("<256sIHBBII")  # FileEntry18, Pack = 1, 272 bytes


def bg3_pak(path: Path, entries: list[str] | None = None,
            version: int = 18, corrupt_list: bool = False,
            declared_count: int | None = None) -> Path:
    """Build a valid LSPK v18 package with a real LZ4-compressed file list.

    The layout follows LSLib's FileEntry18 / PackageReader: a 256-byte
    null-terminated name, then offset/part/flags/sizes, Pack = 1, with the list
    stored as numFiles:i32 followed by an LZ4 block.
    """
    import lz4.block

    entries = entries if entries is not None else [
        "Mods/ExampleMod/meta.lsx", "Public/ExampleMod/Stats/Generated/Data/Weapon.txt"]
    header_size = 4 + struct.calcsize("<IQIBB16sH")
    payload = b"\x00" * 64
    blob = b""
    for index, name in enumerate(entries):
        blob += FILE_ENTRY.pack(name.encode("utf-8"), header_size + index * 8, 0, 0, 0,
                                8, 8)
    compressed = lz4.block.compress(blob, store_size=False)
    if corrupt_list:
        compressed = b"\x00" * len(compressed)
    count = declared_count if declared_count is not None else len(entries)
    # From LSPK version 14 the entry count is followed by an explicit compressed
    # size and the block starts at offset + 8 (LSLib ReadCompressedFileList).
    if version > 13:
        list_block = struct.pack("<ii", count, len(compressed)) + compressed
    else:
        list_block = struct.pack("<i", count) + compressed

    file_list_offset = header_size + len(payload)
    body = lspk_bytes(version=version, file_list_offset=file_list_offset,
                      file_list_size=len(list_block)) + payload + list_block
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(body)
    return path


# --- Cyberpunk 2077 ------------------------------------------------------
def cyberpunk_zip(path: Path, mod_id: str = "examplemod", layers: tuple[str, ...] = ("archive",),
                  archive_name: str = "example.archive") -> Path:
    members: dict[str, bytes | str] = {}
    if "archive" in layers:
        members[f"archive/pc/mod/{archive_name}"] = b"RDAR" + b"\x00" * 32
    if "redmod" in layers:
        members[f"mods/{mod_id}/info.json"] = json.dumps(
            {"name": mod_id, "description": "Example.", "version": "1.0.0"})
        members[f"mods/{mod_id}/archives/{archive_name}"] = b"RDAR" + b"\x00" * 32
    if "redscript" in layers:
        members["r6/scripts/example/example.reds"] = "@wrapMethod(PlayerPuppet) func X() {}"
    if "tweakxl" in layers:
        members["r6/tweaks/example.yaml"] = "Items.Preset_X:\n  damage: 1\n"
    if "cet" in layers:
        members["bin/x64/plugins/cyber_engine_tweaks/mods/example/init.lua"] = "return {}"
    return _zip(path, members)


# --- The Sims 4 ----------------------------------------------------------
def dbpf_package(path: Path, resources: list[tuple[int, int, int]] | None = None,
                 constant_fields: bool = False) -> Path:
    """Write a minimal DBPF v2.1 package.

    ``constant_fields`` sets the index-type bitfield that hoists a shared Type,
    Group and InstanceHi out of every entry -- an optimisation real game
    packages use and that S4TK does not emit, so it is exercised from this side
    instead.
    """
    resources = resources or [(0x220557DA, 0x00000000, 0x0123456789ABCDEF)]
    if constant_fields:
        types = {r[0] for r in resources}
        groups = {r[1] for r in resources}
        his = {(r[2] >> 32) & 0xFFFFFFFF for r in resources}
        if not (len(types) == 1 and len(groups) == 1 and len(his) == 1):
            raise ValueError("constant_fields needs one shared type, group and "
                             "instance-high across all resources")
        index = struct.pack("<I", 0x1 | 0x2 | 0x4)
        index += struct.pack("<III", types.pop(), groups.pop(), his.pop())
        for _type_id, _group, instance in resources:
            index += struct.pack("<I", instance & 0xFFFFFFFF)
            index += struct.pack("<III", 96, 16, 16)
            index += struct.pack("<H", 0)  # mnCommitted, always present
        header = bytearray(96)
        header[0:4] = b"DBPF"
        struct.pack_into("<II", header, 4, 2, 1)
        struct.pack_into("<I", header, 0x24, len(resources))
        struct.pack_into("<I", header, 0x2C, len(index))
        struct.pack_into("<I", header, 0x3C, 3)  # index_version, "always 3"
        payload = b"\x00" * 16 * len(resources)
        struct.pack_into("<I", header, 0x40, 96 + len(payload))
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(bytes(header) + payload + index)
        return path

    index = struct.pack("<I", 0)  # flags: no constant fields
    for type_id, group, instance in resources:
        index += struct.pack("<IIII", type_id, group, (instance >> 32) & 0xFFFFFFFF,
                             instance & 0xFFFFFFFF)
        index += struct.pack("<III", 96, 16, 16)  # position, size, decompressed size
        index += struct.pack("<H", 0)  # mnCommitted, always present
    header = bytearray(96)
    header[0:4] = b"DBPF"
    struct.pack_into("<II", header, 4, 2, 1)
    struct.pack_into("<I", header, 0x24, len(resources))
    struct.pack_into("<I", header, 0x2C, len(index))
    struct.pack_into("<I", header, 0x3C, 3)  # index_version, "always 3"
    payload = b"\x00" * 16 * len(resources)
    index_offset = 96 + len(payload)
    struct.pack_into("<I", header, 0x40, index_offset)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(bytes(header) + payload + index)
    return path


def ts4script(path: Path, magic: int = 3394, count: int = 2) -> Path:
    """``magic`` is the CPython bytecode magic; 3394 is Python 3.7."""
    members = {f"examplemod/mod_{i}.pyc": struct.pack("<H", magic) + b"\r\n" + b"\x00" * 12
               for i in range(count)}
    members["examplemod/__init__.py"] = "\n"
    return _zip(path, members)


# --- Bethesda ------------------------------------------------------------
def bethesda_plugin_with_record(path: Path, *, light: bool = True,
                                object_index: int = 0x000123,
                                masters: list[str] | None = None) -> Path:
    """A plugin carrying one real GLOB record, so record-level checks can run.

    esplugin returns "valid as light" for any plugin whose records were not
    parsed (RecordIds::None), so a light-plugin FormID-range check is only
    meaningful against a plugin that actually contains a record. `object_index`
    is the low 24 bits of the record's FormID: a light plugin may only use
    0x000-0xFFF, so a larger value makes the plugin invalid as light.
    """
    masters = masters if masters is not None else ["Skyrim.esm"]

    def zstr(text: str) -> bytes:
        return text.encode("cp1252") + b"\x00"

    subs = struct.pack("<4sH", b"HEDR", 12) + struct.pack("<fiI", 1.71, 1, 0x800)
    subs += struct.pack("<4sH", b"CNAM", len(zstr("modcheck"))) + zstr("modcheck")
    for master in masters:
        subs += struct.pack("<4sH", b"MAST", len(zstr(master))) + zstr(master)
        subs += struct.pack("<4sH", b"DATA", 8) + struct.pack("<Q", 0)
    header = struct.pack("<4sIIIII", b"TES4", len(subs),
                         0x200 if light else 0, 0, 0, 0) + subs

    form_id = (len(masters) << 24) | (object_index & 0xFFFFFF)
    data = struct.pack("<4sH", b"EDID", len(zstr("mcTest"))) + zstr("mcTest")
    data += struct.pack("<4sH", b"FNAM", 1) + b"s"
    data += struct.pack("<4sH", b"FLTV", 4) + struct.pack("<f", 1.0)
    record = struct.pack("<4sIIIII", b"GLOB", len(data), 0, form_id, 0, 0) + data
    group = struct.pack("<4sI4sIII", b"GRUP", 24 + len(record), b"GLOB", 0, 0, 0) + record

    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(header + group)
    return path


def bethesda_plugin(path: Path, masters: list[str] | None = None, light: bool = False,
                    master_flag: bool = False, author: str = "modcheck",
                    record_count: int = 42, next_object_id: int = 0x800,
                    header_version: float = 1.71) -> Path:
    def zstr(s: str) -> bytes:
        return s.encode("cp1252") + b"\x00"

    subs = struct.pack("<4sH", b"HEDR", 12) + struct.pack(
        "<fiI", header_version, record_count, next_object_id)
    subs += struct.pack("<4sH", b"CNAM", len(zstr(author))) + zstr(author)
    for m in masters if masters is not None else ["Skyrim.esm", "Update.esm"]:
        subs += struct.pack("<4sH", b"MAST", len(zstr(m))) + zstr(m)
        subs += struct.pack("<4sH", b"DATA", 8) + struct.pack("<Q", 0)
    flags = (0x200 if light else 0) | (0x01 if master_flag else 0)
    record = struct.pack("<4sIIIII", b"TES4", len(subs), flags, 0, 0, 0) + subs
    path.parent.mkdir(parents=True, exist_ok=True)
    # No trailing bytes: a plugin is the TES4 header record followed by valid
    # groups, so padding would make the file header-valid but not a valid
    # plugin. esplugin's full parse rejects that, which is how we found it.
    path.write_bytes(record)
    return path

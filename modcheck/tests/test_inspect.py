"""Inspector tests.

These establish that each inspector reads the layout it was written against and
reports the right evidence class and coverage. Fixtures are synthetic; see
test_real_artifacts.py for a check against a really published mod.
"""
from __future__ import annotations

import pytest

from fixtures import build
from modcheck.inspect import detect_kind, inspect_path
from modcheck.inspect.base import InspectionError
from modcheck.inspect import bethesda, sims4


# --- Minecraft -----------------------------------------------------------
def test_fabric_jar_reads_manifest_and_mixin_targets(tmp_path):
    jar = build.fabric_jar(tmp_path / "example.jar", mixin_targets=["ItemMixin", "BlockMixin"])
    assert detect_kind(jar) == "minecraft_jar"
    ins = inspect_path(jar)
    assert ins.game == "minecraft" and ins.loader == "fabric"
    assert ins.fact("mod_id") == "examplemod"
    assert {d["id"] for d in ins.fact("dependencies")} == {"fabricloader", "minecraft", "java"}
    classes = [m["class"] for m in ins.fact("mixin_classes")]
    assert classes == ["com.example.examplemod.mixin.ItemMixin",
                       "com.example.examplemod.mixin.BlockMixin"]
    # Manifest claims are declared; the mixin class list is read out of the jar.
    kinds = {f.key: f.evidence_class for f in ins.facts}
    assert kinds["mod_id"] == "declared"
    assert kinds["mixin_classes"] == "extracted"
    assert any("bytecode" in n for n in ins.not_checked)


def test_fabric_jar_flags_missing_mixin_config(tmp_path):
    jar = build.fabric_jar(tmp_path / "broken.jar")
    # Rewrite the manifest to point at a config that is not in the jar.
    import json, zipfile
    with zipfile.ZipFile(jar) as zf:
        members = {n: zf.read(n) for n in zf.namelist()}
    manifest = json.loads(members["fabric.mod.json"])
    manifest["mixins"] = ["missing.mixins.json"]
    members["fabric.mod.json"] = json.dumps(manifest).encode()
    with zipfile.ZipFile(jar, "w") as zf:
        for name, data in members.items():
            zf.writestr(name, data)
    ins = inspect_path(jar)
    assert any("absent from the jar" in w for w in ins.warnings)


@pytest.mark.parametrize("neoforge", [False, True])
def test_forge_and_neoforge_dependency_requiredness(tmp_path, neoforge):
    jar = build.forge_jar(tmp_path / f"forge{neoforge}.jar", neoforge=neoforge)
    ins = inspect_path(jar)
    assert ins.loader == ("neoforge" if neoforge else "forge")
    deps = {d["id"]: d for d in ins.fact("dependencies")}
    # Forge says `mandatory`, NeoForge says `type`. Both must resolve the same way.
    assert deps["minecraft"]["required"] is True
    assert deps["jei"]["required"] is False
    assert deps["jei"]["ordering"] == "AFTER"


# --- Stardew Valley ------------------------------------------------------
def test_smapi_code_mod(tmp_path):
    mod = build.smapi_mod(tmp_path / "mod.zip",
                          deps=[{"UniqueID": "Pathoschild.ContentPatcher",
                                 "MinimumVersion": "2.0.0", "IsRequired": True}])
    ins = inspect_path(mod)
    assert ins.game == "stardewvalley" and ins.fact("mod_type") == "smapi_code_mod"
    assert ins.fact("dependencies")[0]["versions"] == ">=2.0.0"


def test_content_patcher_targets_are_extracted(tmp_path):
    pack = build.content_patcher_pack(
        tmp_path / "cp.zip", targets=["Data/Objects", "Portraits/Abigail"],
        when={"Season": "spring"})
    ins = inspect_path(pack)
    assert ins.fact("mod_type") == "content_pack"
    assert ins.fact("targets") == ["Data/Objects", "Portraits/Abigail"]
    change = ins.fact("content_changes")[0]
    assert change["action"] == "EditData" and change["when"] == {"Season": "spring"}
    # ContentPackFor becomes a real dependency edge.
    assert any(d["id"] == "Pathoschild.ContentPatcher" for d in ins.fact("dependencies"))


# --- RimWorld ------------------------------------------------------------
def test_rimworld_about_and_layout(tmp_path):
    mod = build.rimworld_mod(tmp_path / "rw.zip", load_after=["ludeon.rimworld"],
                             defs=["1.6/Defs/ThingDefs/Example.xml"],
                             patches=["1.6/Patches/Example.xml"])
    ins = inspect_path(mod)
    assert ins.game == "rimworld"
    assert ins.fact("mod_id") == "example.mod"
    assert ins.fact("supported_versions") == ["1.5", "1.6"]
    assert ins.fact("load_after") == ["ludeon.rimworld"]
    assert ins.fact("dependencies")[0]["id"] == "brrainz.harmony"
    assert ins.fact("version_folders") == ["1.6"]
    assert ins.fact("patch_files") == ["1.6/Patches/Example.xml"]


# --- Project Zomboid -----------------------------------------------------
def test_pz_mod_info_and_lua_paths(tmp_path):
    mod = build.pz_mod(tmp_path / "pz.zip", require=["OtherMod", "ThirdMod"],
                       lua=["ExampleMod/media/lua/client/A.lua",
                            "ExampleMod/media/lua/server/B.lua"])
    ins = inspect_path(mod)
    assert ins.game == "projectzomboid"
    assert ins.fact("mod_id") == "ExampleMod"
    assert [d["id"] for d in ins.fact("dependencies")] == ["OtherMod", "ThirdMod"]
    assert ins.fact("lua_files") == ["media/lua/client/A.lua", "media/lua/server/B.lua"]
    assert ins.fact("lua_scopes") == ["client", "server"]


# --- Baldur's Gate 3 -----------------------------------------------------
def test_bg3_zip_identity(tmp_path):
    mod = build.bg3_zip(tmp_path / "bg3.zip",
                        deps=[{"UUID": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee", "Name": "Dep"}])
    ins = inspect_path(mod)
    assert ins.game == "bg3"
    assert ins.fact("mod_id") == "11111111-2222-3333-4444-555555555555"
    assert ins.fact("folder") == "ExampleMod"
    assert ins.fact("dependencies")[0]["id"] == "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"


def test_bg3_bare_pak_says_what_it_could_not_read(tmp_path):
    pak = build.bg3_pak(tmp_path / "Example.pak")
    ins = inspect_path(pak)
    assert ins.fact("package_format") == "LSPK v18"
    assert any("not read" in w for w in ins.warnings)
    assert any("LZ4" in n for n in ins.not_checked)


# --- Cyberpunk 2077 ------------------------------------------------------
def test_cyberpunk_layers(tmp_path):
    mod = build.cyberpunk_zip(tmp_path / "cp77.zip",
                              layers=("archive", "redscript", "tweakxl"))
    ins = inspect_path(mod)
    assert ins.game == "cyberpunk2077"
    assert ins.fact("layers") == ["archive", "redscript", "tweakxl"]
    assert ins.fact("archive_files") == ["archive/pc/mod/example.archive"]
    assert any("no declared id" in w for w in ins.warnings)


def test_cyberpunk_redmod_declares_identity(tmp_path):
    mod = build.cyberpunk_zip(tmp_path / "redmod.zip", layers=("redmod",))
    ins = inspect_path(mod)
    assert ins.fact("mod_id") == "examplemod"
    assert ins.fact("redmod_modules")[0]["version"] == "1.0.0"


# --- The Sims 4 ----------------------------------------------------------
def test_dbpf_resource_keys(tmp_path):
    pkg = build.dbpf_package(tmp_path / "x.package",
                             resources=[(0x220557DA, 0, 0x0123456789ABCDEF),
                                        (0x545AC67A, 1, 0x1111111111111111)])
    ins = inspect_path(pkg)
    assert ins.game == "sims4"
    assert ins.fact("resource_count") == 2
    assert ins.fact("resource_keys")[0] == "220557DA:00000000:0123456789ABCDEF"
    assert ins.fact("resource_types") == {"220557DA": 1, "545AC67A": 1}


def test_ts4script_bytecode_version(tmp_path):
    script = build.ts4script(tmp_path / "mod.ts4script", magic=3394)
    ins = inspect_path(script)
    assert ins.fact("bytecode_python_versions") == {"3.7": 2}


def test_ts4script_unknown_magic_is_reported_not_guessed(tmp_path):
    script = build.ts4script(tmp_path / "weird.ts4script", magic=9999)
    ins = inspect_path(script)
    assert ins.fact("bytecode_python_versions") == {}
    assert any("unrecognised .pyc magic" in w for w in ins.warnings)


# --- Bethesda ------------------------------------------------------------
def test_bethesda_masters_and_light_flag(tmp_path):
    plugin = build.bethesda_plugin(tmp_path / "Example.esp", light=True)
    ins = inspect_path(plugin)
    assert ins.fact("masters") == ["Skyrim.esm", "Update.esm"]
    assert ins.fact("light_flag_set") is True
    assert ins.fact("loads_in_light_slot") is True
    assert ins.game == "skyrimse"  # inferred from the master, and said so
    assert ins.fact("game_inferred_from") == "skyrim.esm"


def test_bethesda_esl_extension_loads_light_without_the_flag(tmp_path):
    plugin = build.bethesda_plugin(tmp_path / "Example.esl", light=False)
    ins = inspect_path(plugin)
    assert ins.fact("light_flag_set") is False
    assert ins.fact("loads_in_light_slot") is True


def test_bethesda_game_inferred_per_master(tmp_path):
    fo4 = build.bethesda_plugin(tmp_path / "F.esp", masters=["Fallout4.esm"])
    fnv = build.bethesda_plugin(tmp_path / "N.esp", masters=["FalloutNV.esm"])
    assert inspect_path(fo4).game == "fallout4"
    assert inspect_path(fnv).game == "falloutnv"


def test_bethesda_unknown_master_is_not_guessed(tmp_path):
    plugin = build.bethesda_plugin(tmp_path / "U.esp", masters=["Unknown.esm"])
    ins = inspect_path(plugin)
    assert ins.game is None
    assert any("undetermined" in w for w in ins.warnings)


def test_bethesda_light_plugin_form_range_warning(tmp_path):
    plugin = build.bethesda_plugin(tmp_path / "L.esp", light=True, next_object_id=0x5000)
    ins = inspect_path(plugin)
    assert any("light FormID range" in w for w in ins.warnings)


def test_bethesda_rejects_non_plugin(tmp_path):
    p = tmp_path / "not.esp"
    p.write_bytes(b"NOPE" + b"\x00" * 64)
    ins = inspect_path(p)  # falls back rather than crashing
    assert ins.kind == "unknown"
    with pytest.raises(InspectionError):
        bethesda.parse_header(b"NOPE" + b"\x00" * 64)


def test_truncated_plugin_header_raises(tmp_path):
    with pytest.raises(InspectionError):
        bethesda.parse_header(b"TES4" + b"\x00" * 8)


# --- fallback ------------------------------------------------------------
def test_unknown_artifact_reports_only_identity(tmp_path):
    p = tmp_path / "mystery.bin"
    p.write_bytes(b"\x00" * 100)
    ins = inspect_path(p)
    assert ins.kind == "unknown"
    assert ins.sha256 and ins.bytes == 100
    assert any("no ModCheck inspector recognised" in w for w in ins.warnings)


def test_every_inspection_reports_coverage(tmp_path):
    artifacts = [
        build.fabric_jar(tmp_path / "a.jar"),
        build.smapi_mod(tmp_path / "b.zip"),
        build.rimworld_mod(tmp_path / "c.zip"),
        build.pz_mod(tmp_path / "d.zip"),
        build.bg3_zip(tmp_path / "e.zip"),
        build.cyberpunk_zip(tmp_path / "f.zip"),
        build.dbpf_package(tmp_path / "g.package"),
        build.bethesda_plugin(tmp_path / "h.esp"),
    ]
    for art in artifacts:
        ins = inspect_path(art)
        assert ins.checked, f"{art.name} reported no coverage"
        assert ins.not_checked, f"{art.name} claims it checked everything"
        assert ins.sha256

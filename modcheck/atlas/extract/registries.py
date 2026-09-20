"""The vanilla registration surface: what a mod can register into, by name.

Read from BuiltInRegistries (the static registries) and Registries (the
ResourceKeys, which also cover the data-driven registries a datapack can add
to). Element types come from the field descriptors, so "ITEM holds Item" is
established from the artifact, not from a wiki.
"""
from __future__ import annotations

import datetime as dt
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from extract import jvm  # noqa: E402

CORPUS = Path(__file__).resolve().parents[1] / "extracted" / "corpus.json"
OUT = Path(__file__).resolve().parents[1] / "extracted" / "minecraft_registries.json"
JAVAP = Path(__file__).resolve().parents[2] / "toolchains" / "jdk-25.0.4.1+1" / "bin" / "javap"

_FIELD = re.compile(r"public static final (?P<type>\S+(?:<.*>)?) (?P<name>[A-Z_0-9]+)$")


def fields(jar: Path, fqcn: str) -> list[dict]:
    out = []
    for member in jvm.public_surface(JAVAP, jar, fqcn)["members"]:
        m = _FIELD.match(member.strip())
        if m:
            out.append({"field": m.group("name"), "type": m.group("type")})
    return out


def main() -> int:
    corpus = json.loads(CORPUS.read_text())
    merged = next(e for e in corpus["artifacts"] if e["name"] == "minecraft-merged")
    jar = Path(merged["path"])
    built_in = fields(jar, "net.minecraft.core.registries.BuiltInRegistries")
    keys = fields(jar, "net.minecraft.core.registries.Registries")
    data_types = corpus["minecraft"]["data_entries_by_type"]
    out = {
        "generated_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat(),
        "artifact": merged["name"], "sha256": merged["sha256"], "version": merged["version"],
        "built_in_registries": built_in,
        "registry_keys": keys,
        "datapack_registry_entry_counts": data_types,
        "note": "built_in_registries are populated in code at startup and are what "
                "Registry.register targets. registry_keys include dynamic registries "
                "loaded from data packs; the datapack counts are the vanilla entries "
                "shipped inside the jar under data/minecraft/<type>/.",
    }
    OUT.write_text(json.dumps(out, indent=1))
    print(f"wrote {OUT}: {len(built_in)} built-in registries, {len(keys)} registry keys, "
          f"{len(data_types)} datapack entry types")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

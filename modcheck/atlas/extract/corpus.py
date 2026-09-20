"""Inventory the artifacts the project actually resolved, and nothing else.

"The corpus" means the concrete jars on this machine that a Fabric 26.3 build
compiles and runs against, plus the build tooling Loom pulled in. Each is
classified, hashed and counted. A jar that is not here is not in the corpus,
whatever documentation says about it -- and the atlas records that boundary
rather than papering over it.
"""
from __future__ import annotations

import collections
import datetime as dt
import hashlib
import json
import re
import sys
import zipfile
from pathlib import Path

GRADLE = Path("/root/.gradle/caches")
OUT = Path(__file__).resolve().parents[1] / "extracted" / "corpus.json"

TOOLING = {"tiny-remapper", "stitch", "mercury", "mercurymixin", "mapping-io",
           "tiny-mappings-parser", "access-widener", "fabric-loom", "lorenz-tiny",
           "name-proposal", "cfr", "fernflower", "fabric-fernflower", "unpick",
           "vineflower", "fabric-mixin-compile-extensions", "java-syntax", "enigma",
           "dev-launch-injector", "fabric-log4j-util", "fabric-loom-native",
           "unpick-format-utils"}
# Runtime dependencies of the loader that are neither the loader nor a mod.
LOADER_RUNTIME = {"class-tweaker"}


# Paths on the reference build's resolved classpaths (atlas/extract/resolved_env.py input).
# A Gradle-cache artifact that is on the runtime classpath but is neither Fabric nor
# tooling is one of Minecraft's own libraries (Brigadier, DataFixerUpper, Guava, ...).
_RESOLVED = Path(__file__).resolve().parents[1] / "extracted" / "resolved_classpaths.tsv"
RESOLVED_PATHS = {l.split("\t", 1)[1].strip() for l in _RESOLVED.read_text().splitlines() if "\t" in l} \
    if _RESOLVED.exists() else set()


def classify(path: Path) -> tuple[str, str, str]:
    """(group, artifact_name, version)."""
    parts = path.parts
    if "loom-cache" in parts and "minecraftMaven" in parts:
        # the Loom-PROCESSED jar a project compiles against (access widened, interfaces injected)
        return "minecraft_processed", path.stem, "26.3"
    if "fabric-loom" in parts:
        version = parts[parts.index("fabric-loom") + 1] if parts[-2] != "26.3" else "26.3"
        return "minecraft", path.stem, "26.3"
    try:
        idx = parts.index("files-2.1")
    except ValueError:
        return "unclassified", path.stem, "?"
    group, name, version = parts[idx + 1], parts[idx + 2], parts[idx + 3]
    if group == "io.github.llamalad7" and name.startswith("mixinextras"):
        return "mixin_runtime_extras", name, version
    if not group.startswith("net.fabricmc") and str(path) in RESOLVED_PATHS:
        return "minecraft_library", f"{group}:{name}", version
    if group == "net.fabricmc.fabric-api":
        return "fabric_api_module", name, version
    if group == "net.fabricmc":
        if name == "fabric-loader":
            return "fabric_loader", name, version
        if name == "sponge-mixin":
            return "mixin_runtime", name, version
        if name in TOOLING:
            return "build_tooling", name, version
        if name in LOADER_RUNTIME:
            return "loader_runtime_dep", name, version
        return "fabric_other", name, version
    if group == "net.fabricmc.unpick":
        return "build_tooling", f"{group}:{name}", version
    return "unclassified", f"{group}:{name}", version


def count_classes(path: Path) -> tuple[int, int]:
    top = nested = 0
    with zipfile.ZipFile(path) as zf:
        for n in zf.namelist():
            if n.endswith(".class"):
                if "$" in n:
                    nested += 1
                else:
                    top += 1
    return top, nested


def minecraft_breakdown(jar: Path) -> dict:
    pkg3: collections.Counter = collections.Counter()
    pkg4: collections.Counter = collections.Counter()
    data: collections.Counter = collections.Counter()
    assets: collections.Counter = collections.Counter()
    other_roots: collections.Counter = collections.Counter()
    version_json = None
    with zipfile.ZipFile(jar) as zf:
        for n in zf.namelist():
            if n.endswith("/"):
                continue
            if n == "version.json":
                version_json = json.loads(zf.read(n))
                continue
            parts = n.split("/")
            if n.endswith(".class"):
                if parts[:2] == ["net", "minecraft"] and len(parts) > 3:
                    pkg3[".".join(parts[:3])] += 1
                    pkg4[".".join(parts[:4]) if len(parts) > 4 else ".".join(parts[:3])] += 1
                else:
                    other_roots[".".join(parts[:2])] += 1
            elif parts[0] == "data" and len(parts) > 2:
                data[f"{parts[1]}/{parts[2]}"] += 1
            elif parts[0] == "assets" and len(parts) > 2:
                assets[f"{parts[1]}/{parts[2]}"] += 1
    return {"version_json": version_json,
            "classes_by_package_depth3": dict(sorted(pkg3.items())),
            "classes_by_package_depth4": dict(sorted(pkg4.items())),
            "non_minecraft_class_roots": dict(sorted(other_roots.items())),
            "data_entries_by_type": dict(sorted(data.items())),
            "asset_entries_by_type": dict(sorted(assets.items()))}


def main(jar_list: Path) -> int:
    jars = [Path(l.strip()) for l in jar_list.read_text().splitlines() if l.strip()]
    entries = []
    for jar in sorted(jars):
        if not jar.is_file():
            continue
        group, name, version = classify(jar)
        top, nested = count_classes(jar)
        entries.append({
            "group": group, "name": name, "version": version,
            "path": str(jar), "bytes": jar.stat().st_size,
            "sha256": hashlib.sha256(jar.read_bytes()).hexdigest(),
            "classes_top_level": top, "classes_nested": nested,
        })
    merged = next((e for e in entries if e["name"] == "minecraft-merged"), None)
    summary = collections.Counter(e["group"] for e in entries)
    out = {
        "generated_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat(),
        "source_of_truth": "jars resolved into the Gradle cache by the project's own build "
                           "(build_workspaces/demo and reference/rainlantern); nothing fetched "
                           "for this inventory",
        "groups": dict(summary),
        "artifacts": entries,
        "minecraft": minecraft_breakdown(Path(merged["path"])) if merged else None,
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(out, indent=1))
    print(f"wrote {OUT}: {len(entries)} artifacts; groups={dict(summary)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(Path(sys.argv[1]) if len(sys.argv) > 1 else Path("/tmp/corpus_jars.txt")))

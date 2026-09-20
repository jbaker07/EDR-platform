"""Record the actually resolved build environment, as distinct from the cache inventory.

corpus.json lists what is in the Gradle caches. This records what the reference
project's Gradle build actually put on its compile, runtime and test classpaths,
hashes every entry, and diffs the Loom-processed Minecraft jar the project
compiles against (access widened, interfaces injected by Fabric) against the
unprocessed cache jar. The processed jar is what a mod's code is resolved
against; the cache jar is what Mojang shipped.

Input: a Gradle listing produced by the init script in atlas/README.md
(``<configuration>\\t<path>`` per line). Output: atlas/extracted/resolved_environment.json
"""
from __future__ import annotations

import hashlib
import json
import sys
import zipfile
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from classfile import ClassFileError, read_class  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
EXTRACTED = ROOT / "extracted"


def sha256(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def jar_diff(cache_jar: Path, processed_jar: Path) -> dict:
    """Per-class differences: member access widened, class access changed, hierarchy changed."""
    za, zb = zipfile.ZipFile(cache_jar), zipfile.ZipFile(processed_jar)
    na, nb = set(za.namelist()), set(zb.namelist())
    changed, member_access, class_access, hierarchy = [], [], [], []
    for n in sorted(na & nb):
        if not n.endswith(".class"):
            continue
        ba, bb = za.read(n), zb.read(n)
        if ba == bb:
            continue
        changed.append(n[:-6])
        try:
            a, b = read_class(ba), read_class(bb)
        except ClassFileError:
            continue
        if a.access != b.access:
            class_access.append({"class": a.name, "from": a.access, "to": b.access})
        ma = {(m.name, m.desc): m.access for m in a.fields + a.methods}
        mb = {(m.name, m.desc): m.access for m in b.fields + b.methods}
        for k, acc in ma.items():
            if k in mb and mb[k] != acc:
                member_access.append({"class": a.name, "member": k[0], "desc": k[1], "from": acc, "to": mb[k]})
        if a.super_name != b.super_name or a.interfaces != b.interfaces:
            hierarchy.append({"class": a.name, "super": [a.super_name, b.super_name],
                              "interfaces_added": sorted(set(b.interfaces) - set(a.interfaces)),
                              "interfaces_removed": sorted(set(a.interfaces) - set(b.interfaces))})
    return {"entries_cache": len(na), "entries_processed": len(nb), "only_in_cache": sorted(na - nb)[:50],
            "only_in_processed": sorted(nb - na)[:50], "classes_with_different_bytes": len(changed),
            "member_access_changes": member_access, "class_access_changes": class_access,
            "hierarchy_changes": hierarchy}


def main(listing: Path) -> int:
    corpus = json.loads((EXTRACTED / "corpus.json").read_text())
    by_path = {a["path"]: a for a in corpus["artifacts"]}
    configs: dict[str, list[dict]] = {}
    hashes: dict[str, str] = {}
    for line in listing.read_text().splitlines():
        if "\t" not in line:
            continue
        cfg, path = line.split("\t", 1)
        p = Path(path.strip())
        if str(p) not in hashes:
            hashes[str(p)] = sha256(p) if p.exists() else None
        configs.setdefault(cfg, []).append({"path": str(p), "name": p.name, "sha256": hashes[str(p)],
                                            "in_corpus": str(p) in by_path,
                                            "corpus_group": by_path.get(str(p), {}).get("group")})
    processed = next((e for e in configs.get("compileClasspath", []) if "minecraft-merged-" in e["name"]), None)
    cache_jar = next((a for a in corpus["artifacts"] if a["name"] == "minecraft-merged"), None)
    diff = None
    if processed and cache_jar:
        diff = jar_diff(Path(cache_jar["path"]), Path(processed["path"]))
        diff["cache_jar"] = {"path": cache_jar["path"], "sha256": cache_jar["sha256"]}
        diff["processed_jar"] = {"path": processed["path"], "sha256": processed["sha256"]}
    runtime_only = [e for e in configs.get("runtimeClasspath", []) if not e["in_corpus"]]
    out = {
        "generated_at": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "project": "reference/rainlantern", "gradle": "9.5.1 (toolchains/gradle-9.5.1)", "jdk": "25.0.4.1",
        "method": "gradle --offline printResolvedClasspaths (init script in atlas/README.md); every entry hashed",
        "counts": {cfg: len(v) for cfg, v in configs.items()},
        "corpus_artifacts_on_runtime_classpath": sum(1 for e in configs.get("runtimeClasspath", []) if e["in_corpus"]),
        "corpus_artifacts_total": len(corpus["artifacts"]),
        "runtime_entries_not_in_corpus": len(runtime_only),
        "runtime_entries_not_in_corpus_by_kind": dict(Counter(
            "minecraft_library" if not any(k in e["name"] for k in ("fabric", "mixin", "asm-", "lz4", "log4j")) else
            "loader_or_toolchain" for e in runtime_only)),
        "configurations": configs,
        "minecraft_jar_processing": diff,
    }
    (EXTRACTED / "resolved_environment.json").write_text(json.dumps(out, indent=1))
    print(f"wrote resolved_environment.json: {out['counts']}; runtime entries not in corpus {len(runtime_only)}; "
          f"processed-vs-cache classes differing {diff['classes_with_different_bytes'] if diff else 'n/a'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(Path(sys.argv[1]) if len(sys.argv) > 1 else Path("/tmp/resolved_cp.txt")))

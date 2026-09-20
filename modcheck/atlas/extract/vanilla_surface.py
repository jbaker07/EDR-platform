"""The declared surface of every class in the Minecraft jar a mod compiles against.

Reads all classes (top-level and nested) of the Loom-processed merged jar named
in resolved_environment.json -- the jar the reference build actually resolves
against -- and, for each class, whether its bytes differ from the unprocessed
cache jar (access widened or interfaces injected by Fabric's processing).

Output: atlas/extracted/minecraft_surface.json.gz
  {"classes": {internal: {"kind","access","super","interfaces","outer","processed_differs",
                          "fields": [[name, desc, access], ...], "methods": [[name, desc, access], ...]}},
   "packages": {package: {"classes": n, "top_level": n}}, ...}
Nothing is filtered by whether Fabric API hooks it.
"""
from __future__ import annotations

import gzip
import json
import sys
import time
import zipfile
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import classfile as cf  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
EXTRACTED = ROOT / "extracted"


def main() -> int:
    env = json.loads((EXTRACTED / "resolved_environment.json").read_text())
    proc = env["minecraft_jar_processing"]
    processed, cache = Path(proc["processed_jar"]["path"]), Path(proc["cache_jar"]["path"])
    t0 = time.time()
    zc = zipfile.ZipFile(cache)
    cache_names = set(zc.namelist())
    classes: dict[str, dict] = {}
    errors = []
    for internal, parsed in cf.iter_jar_classes(processed):
        if isinstance(parsed, cf.ClassFileError):
            errors.append(str(parsed))
            continue
        entry = internal + ".class"
        differs = entry in cache_names and zc.read(entry) != zipfile.ZipFile(processed).read(entry) if False else None
        classes[internal] = {
            "kind": parsed.kind, "access": parsed.access, "super": parsed.super_name, "interfaces": parsed.interfaces,
            "outer": next((i["outer"] for i in parsed.inner_classes if i["inner"] == internal), None),
            "fields": [[f.name, f.desc, f.access] for f in parsed.fields],
            "methods": [[m.name, m.desc, m.access] for m in parsed.methods],
        }
    # bytes comparison in one pass (cheap: zip reads only)
    zp = zipfile.ZipFile(processed)
    for internal in classes:
        entry = internal + ".class"
        classes[internal]["processed_differs"] = entry in cache_names and zc.read(entry) != zp.read(entry)
    packages: dict[str, dict] = defaultdict(lambda: {"classes": 0, "top_level": 0, "differs": 0})
    for internal, c in classes.items():
        pkg = internal.rsplit("/", 1)[0].replace("/", ".") if "/" in internal else "(default)"
        packages[pkg]["classes"] += 1
        packages[pkg]["top_level"] += "$" not in internal
        packages[pkg]["differs"] += bool(c["processed_differs"])
    out = {
        "generated_at": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "jar": {"path": str(processed), "sha256": proc["processed_jar"]["sha256"], "role": "compileClasspath of reference/rainlantern (Loom-processed)"},
        "cache_jar": {"path": str(cache), "sha256": proc["cache_jar"]["sha256"], "role": "unprocessed merged jar in the Loom cache"},
        "method": "class-file reader over every .class entry; processed_differs = entry bytes differ from the cache jar",
        "counts": {"classes": len(classes), "top_level": sum(1 for k in classes if "$" not in k),
                   "members": sum(len(c["fields"]) + len(c["methods"]) for c in classes.values()),
                   "packages": len(packages), "processed_differs": sum(1 for c in classes.values() if c["processed_differs"]),
                   "read_errors": len(errors)},
        "read_errors": errors[:50],
        "packages": dict(sorted(packages.items())),
        "classes": classes,
    }
    with gzip.open(EXTRACTED / "minecraft_surface.json.gz", "wt", compresslevel=6) as fh:
        json.dump(out, fh, separators=(",", ":"))
    print(f"wrote minecraft_surface.json.gz: {out['counts']} in {time.time() - t0:.0f}s")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

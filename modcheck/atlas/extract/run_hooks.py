"""Extract every Fabric API module's declared and mechanical surface.

Per module: what fabric.mod.json declares; every mixin config; every @Mixin
class with its vanilla targets and injection points; every event-fire site in
mixin and impl classes; every reference from mixin/impl code into net.minecraft
(the module's dependency on vanilla); and the public API surface a mod compiles
against. Plus the loader's own API.

Output is keyed by module id and carries each jar's sha256, so a fact can be
traced to the exact bytes it was read from.
"""
from __future__ import annotations

import dataclasses
import datetime as dt
import json
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from extract import jvm  # noqa: E402

CORPUS = Path(__file__).resolve().parents[1] / "extracted" / "corpus.json"
OUT = Path(__file__).resolve().parents[1] / "extracted" / "fabric_api.json"
JAVAP = Path(__file__).resolve().parents[2] / "toolchains" / "jdk-25.0.4.1+1" / "bin" / "javap"


def module(entry: dict) -> dict:
    jar = Path(entry["path"])
    manifest = jvm.read_json_entry(jar, "fabric.mod.json") or {}
    mixin_config_names = []
    for m in manifest.get("mixins") or []:
        mixin_config_names.append(m if isinstance(m, str) else m.get("config"))
    configs = {}
    for name in mixin_config_names:
        cfg = jvm.read_json_entry(jar, name)
        if cfg:
            configs[name] = {k: cfg.get(k) for k in
                             ("package", "mixins", "client", "server", "injectors",
                              "compatibilityLevel", "minVersion", "required")}

    mixins, impl_refs, api = [], [], []
    all_classes = jvm.classes_in(jar, include_nested=True)
    mixin_classes = [c for c in all_classes if ".mixin." in c and "$" not in c]
    impl_classes = [c for c in all_classes if ".impl." in c and "$" not in c]
    api_classes = [c for c in all_classes if ".api." in c and "$" not in c
                   and not c.endswith("package-info")]

    for c in mixin_classes:
        facts = jvm.mixin_facts(JAVAP, jar, c)
        refs = jvm.class_refs(JAVAP, jar, c)
        if facts is None:
            continue
        mixins.append({
            **dataclasses.asdict(facts),
            "vanilla_refs": {
                "methods": [r for r in refs.methods if r.startswith("net/minecraft/")],
                "fields": [r for r in refs.fields if r.startswith("net/minecraft/")]},
            "event_fires": refs.event_fires,
        })
    for c in impl_classes:
        refs = jvm.class_refs(JAVAP, jar, c)
        vanilla_m = [r for r in refs.methods if r.startswith("net/minecraft/")]
        vanilla_f = [r for r in refs.fields if r.startswith("net/minecraft/")]
        if refs.event_fires or vanilla_m or vanilla_f:
            impl_refs.append({"fqcn": c, "event_fires": refs.event_fires,
                              "vanilla_refs": {"methods": vanilla_m, "fields": vanilla_f}})
    for c in api_classes:
        surface = jvm.public_surface(JAVAP, jar, c)
        if surface["members"]:
            api.append(surface)
    # nested API types matter too (callback interfaces are nested): include their
    # public members under the enclosing class.
    nested_api = [c for c in all_classes if ".api." in c and "$" in c
                  and not c.endswith("package-info")]
    nested = []
    for c in nested_api:
        surface = jvm.public_surface(JAVAP, jar, c)
        if surface["members"]:
            nested.append(surface)

    return {
        "id": manifest.get("id", entry["name"]),
        "artifact": entry["name"], "version": entry["version"],
        "sha256": entry["sha256"],
        "declared": {k: manifest.get(k) for k in
                     ("id", "version", "environment", "entrypoints", "mixins",
                      "depends", "breaks", "provides", "accessWidener", "custom")},
        "mixin_configs": configs,
        "counts": {"classes": len(all_classes), "mixin_classes": len(mixin_classes),
                   "impl_classes": len(impl_classes), "api_classes": len(api_classes)},
        "mixins": mixins,
        "impl": impl_refs,
        "api": api,
        "api_nested": nested,
    }


def loader(entry: dict) -> dict:
    jar = Path(entry["path"])
    classes = jvm.classes_in(jar, include_nested=False)
    api = [c for c in classes if c.startswith(("net.fabricmc.api.", "net.fabricmc.loader.api."))
           and not c.endswith("package-info")]
    return {"artifact": entry["name"], "version": entry["version"], "sha256": entry["sha256"],
            "api": [s for s in (jvm.public_surface(JAVAP, jar, c) for c in api) if s["members"]]}


def main() -> int:
    corpus = json.loads(CORPUS.read_text())
    started = time.time()
    modules = []
    for entry in corpus["artifacts"]:
        if entry["group"] != "fabric_api_module":
            continue
        t = time.time()
        modules.append(module(entry))
        print(f"  {entry['name']:42} {time.time() - t:5.1f}s  mixins={modules[-1]['counts']['mixin_classes']}",
              flush=True)
    loader_entry = next(e for e in corpus["artifacts"] if e["group"] == "fabric_loader")
    out = {
        "generated_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat(),
        "javap": str(JAVAP),
        "method": "javap -v -p for annotations; javap -c -p for references; fabric.mod.json "
                  "and mixin configs read as JSON. Event-fire pairing is a static pattern "
                  "(getstatic Event field followed by invokeinterface on its callback in "
                  "the same method) and is labelled inferred.",
        "modules": modules,
        "loader": loader(loader_entry),
    }
    OUT.write_text(json.dumps(out, indent=1))
    print(f"wrote {OUT} in {time.time() - started:.0f}s: {len(modules)} modules")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

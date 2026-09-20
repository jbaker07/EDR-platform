"""Extract Fabric API module facts into atlas/extracted/fabric_api.json.

Every fact comes from reading class files (atlas/extract/classfile.py) of the
jars listed in corpus.json, hashed there. A mixin is any class carrying
@org.spongepowered.asm.mixin.Mixin, found by annotation rather than by package
name, and every injector annotation from the Mixin and MixinExtras namespaces is
recorded; forms the reader does not understand go into ``extraction_failures``
instead of being dropped.

References are recorded per method with instruction offset and opcode for every
class in a module, restricted to owners under net/minecraft, com/mojang and
net/fabricmc to keep the file bounded. Event fires (getstatic of an Event field,
Event.invoker(), checkcast to the callback type, then the callback invocation)
are recorded wherever they occur.
"""
from __future__ import annotations

import datetime as dt
import json
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import classfile as cf  # noqa: E402
import jvm  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
CORPUS = ROOT / "extracted" / "corpus.json"
OUT = ROOT / "extracted" / "fabric_api.json"
KEEP_OWNERS = ("net/minecraft/", "com/mojang/", "net/fabricmc/")


def compact_refs(refs: dict) -> dict:
    """class_refs -> compact per-method lists, restricted to interesting owners."""
    methods = []
    for m in refs["methods"]:
        kept = [[r["offset"], r["opcode"], r["kind"], r.get("owner"), r.get("name"), r.get("desc"), r.get("access")]
                for r in m["refs"] if (r.get("owner") or "").startswith(KEEP_OWNERS)]
        if kept or m["event_fires"]:
            methods.append({"name": m["name"], "desc": m["desc"], "static": m["static"],
                            "refs": kept, "event_fires": m["event_fires"]})
    return {"fqcn": refs["fqcn"], "internal": refs["internal"], "super": refs["super"],
            "interfaces": refs["interfaces"], "methods": methods}


def module(entry: dict) -> dict:
    jar = Path(entry["path"])
    manifest = jvm.read_json_entry(jar, "fabric.mod.json") or {}
    config_names = [m if isinstance(m, str) else m.get("config") for m in manifest.get("mixins") or []]
    config_env = {m.get("config"): m.get("environment") for m in manifest.get("mixins") or [] if isinstance(m, dict)}
    configs, declared_mixins = {}, {}
    for name in config_names:
        cfg = jvm.read_json_entry(jar, name)
        if not cfg:
            continue
        configs[name] = {k: cfg.get(k) for k in ("package", "mixins", "client", "server", "injectors",
                                                  "compatibilityLevel", "minVersion", "required", "priority")}
        configs[name]["environment"] = config_env.get(name)
        pkg = cfg.get("package", "")
        for side, lst in (("both", cfg.get("mixins")), ("client", cfg.get("client")), ("server", cfg.get("server"))):
            for c in lst or []:
                declared_mixins[f"{pkg}.{c}".replace("/", ".")] = {"config": name, "side": side}

    mixins, classes, api, api_nested, failures = [], [], [], [], []
    n_classes = 0
    for internal, parsed in cf.iter_jar_classes(jar):
        n_classes += 1
        if isinstance(parsed, cf.ClassFileError):
            failures.append({"class": internal.replace("/", "."), "error": str(parsed)})
            continue
        dotted = internal.replace("/", ".")
        refs = compact_refs(jvm.class_refs(parsed))
        if refs["methods"]:
            classes.append(refs)
        facts = jvm.mixin_facts(parsed)
        if facts is not None:
            decl = declared_mixins.get(dotted)
            facts["declared_in_config"] = decl["config"] if decl else None
            facts["config_side"] = decl["side"] if decl else None
            env = facts["environment"] or (config_env.get(decl["config"]) if decl else None)
            if decl and decl["side"] != "both":
                env = decl["side"]
            facts["effective_environment"] = env or "both"
            mixins.append(facts)
            for f in facts["extraction_failures"]:
                failures.append({"class": dotted, **f})
        if ".api." in dotted and not dotted.endswith("package-info"):
            s = jvm.surface(parsed, public_only=True)
            s["members"] = jvm.surface_lines(s)
            (api_nested if "$" in dotted else api).append(s)
    found = {m["mixin_class"] for m in mixins}
    undeclared = sorted(found - set(declared_mixins))
    missing = sorted(set(declared_mixins) - found)
    return {
        "id": manifest.get("id"), "version": manifest.get("version"), "artifact": entry["name"],
        "sha256": entry["sha256"], "path": entry["path"],
        "declared": {k: manifest.get(k) for k in ("id", "version", "environment", "entrypoints", "mixins",
                                                    "depends", "breaks", "provides", "accessWidener", "custom")},
        "mixin_configs": configs,
        "counts": {"classes": n_classes, "mixin_classes_declared": len(declared_mixins),
                   "mixin_classes_found": len(mixins), "api_classes": len(api), "api_nested": len(api_nested),
                   "classes_with_refs": len(classes),
                   "injections": sum(len(m["injections"]) for m in mixins),
                   "extraction_failures": len(failures)},
        "mixins_undeclared": undeclared, "mixins_missing": missing,
        "mixins": mixins, "classes": classes, "api": api, "api_nested": api_nested,
        "extraction_failures": failures,
    }


def loader(entry: dict) -> dict:
    jar = Path(entry["path"])
    api = []
    for internal, parsed in cf.iter_jar_classes(jar, "net/fabricmc/"):
        if isinstance(parsed, cf.ClassFileError):
            continue
        dotted = internal.replace("/", ".")
        if dotted.startswith(("net.fabricmc.api.", "net.fabricmc.loader.api.")) and not dotted.endswith("package-info"):
            s = jvm.surface(parsed, public_only=True)
            s["members"] = jvm.surface_lines(s)
            api.append(s)
    return {"version": entry["version"], "artifact": entry["name"], "sha256": entry["sha256"], "api": api}


def mixin_runtime(entry: dict) -> dict:
    jar = Path(entry["path"])
    ann = cf.read_class_from_jar(jar, "org/spongepowered/asm/mixin/Mixin")
    inj = cf.read_class_from_jar(jar, "org/spongepowered/asm/mixin/injection/Inject")
    return {"version": entry["version"], "artifact": entry["name"], "sha256": entry["sha256"],
            "annotation_defaults": {
                "Mixin": {m.name: m.annotation_default for m in ann.methods if m.annotation_default is not None},
                "Inject": {m.name: m.annotation_default for m in inj.methods if m.annotation_default is not None}}}


def main() -> int:
    corpus = json.loads(CORPUS.read_text())
    t0 = time.time()
    modules = []
    for entry in sorted(corpus["artifacts"], key=lambda a: a["name"]):
        if entry["group"] != "fabric_api_module":
            continue
        t1 = time.time()
        modules.append(module(entry))
        m = modules[-1]
        print(f"  {m['id']:<44} {time.time() - t1:5.1f}s mixins={m['counts']['mixin_classes_found']} "
              f"inj={m['counts']['injections']} fail={m['counts']['extraction_failures']}", flush=True)
    loader_entry = next(a for a in corpus["artifacts"] if a["group"] == "fabric_loader")
    mixin_entry = next(a for a in corpus["artifacts"] if a["group"] == "mixin_runtime")
    extras = next((a for a in corpus["artifacts"] if a["name"].startswith("mixinextras")), None)
    out = {
        "generated_at": dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds"),
        "method": "class-file reader (atlas/extract/classfile.py): annotations from RuntimeVisible/Invisible "
                  "Annotations attributes; references from Code instructions with offsets; members from the "
                  "field and method tables. No javap.",
        "loader": loader(loader_entry),
        "mixin_runtime": mixin_runtime(mixin_entry),
        "mixinextras": {k: extras[k] for k in ("version", "name", "sha256")} if extras else None,
        "modules": modules,
    }
    OUT.write_text(json.dumps(out, indent=None, separators=(",", ":")))
    inj = sum(m["counts"]["injections"] for m in modules)
    fails = sum(m["counts"]["extraction_failures"] for m in modules)
    print(f"wrote {OUT} in {time.time() - t0:.0f}s: {len(modules)} modules, {inj} injections, {fails} extraction failures")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

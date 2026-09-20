"""Build atlas/extracted/edges.json: typed relationships with exact endpoints.

Sources: fabric_api.json (mixins, per-method references, API surfaces),
minecraft_registries.json, resolved_environment.json (the processed Minecraft
jar a mod resolves against), corpus.json (hashes). Every vanilla endpoint is
resolved with resolve.py against the processed jar, Minecraft's libraries and
the JDK, and carries its resolution state; nothing is assumed to exist.

Relations and their evidence classes:
* injects_into / wraps / replaces -- one edge per (injection, selector), from the
  handler method to the resolved target method; direct_reference (annotation).
  `points` carries each @At with its own resolved call or field target.
* calls          -- an invoke instruction: caller method + offset -> callee; direct_reference.
* reads / writes -- get*/put* instructions, separated by opcode; direct_reference.
* publishes_event -- getstatic Event field, Event.invoker(), checkcast callback,
  callback invoke, all in one method; static_inference (a bytecode pattern).
* callback_of    -- Event<Callback> field in the API surface; declared (generic signature).
* registers_into -- BuiltInRegistries field element types; direct_reference.

Also emitted: `shared_targets` -- vanilla methods touched by more than one
module's injections. That is an index of POTENTIAL interactions: whether two
entries actually conflict depends on exact resolution of both, applicability
(environment) and the transformer's composition rule for that pair of
injectors (see mixin_transformation_tests.json). No conflict verdict is made here.
"""
from __future__ import annotations

import json
import re
import sys
from collections import Counter, defaultdict
from datetime import datetime, timezone
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from resolve import Resolver  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
EXTRACTED = ROOT / "extracted"
JDK_CLASSES = ROOT.parents[0] / ".cache" / "jdk_classes"
VANILLA = ("net/minecraft/", "com/mojang/")
EVENT_DESC = "Lnet/fabricmc/fabric/api/event/Event;"
RELATION_FOR_EFFECT = {
    "additive": "injects_into", "modify_call_argument": "injects_into", "modify_call_arguments": "injects_into",
    "modify_constant": "injects_into", "modify_local": "injects_into", "modify_expression_value": "injects_into",
    "modify_return_value": "injects_into", "modify_call_receiver": "wraps", "replace_call_site": "wraps",
    "wrap_call_site": "wraps", "wrap_call_site_conditionally": "wraps", "wrap_method": "wraps",
    "replace_method_body": "replaces",
}


def _res(r: dict) -> dict:
    out = {"state": r["state"]}
    if r.get("declared_in"):
        out["declared_in"] = r["declared_in"]
    if r.get("candidates"):
        out["candidates"] = r["candidates"]
    if r.get("reason"):
        out["reason"] = r["reason"]
    return out


def main() -> int:
    corpus = json.loads((EXTRACTED / "corpus.json").read_text())
    fabric = json.loads((EXTRACTED / "fabric_api.json").read_text())
    registries = json.loads((EXTRACTED / "minecraft_registries.json").read_text())
    env = json.loads((EXTRACTED / "resolved_environment.json").read_text())
    processed = env["minecraft_jar_processing"]["processed_jar"]
    by_group = defaultdict(list)
    for a in corpus["artifacts"]:
        by_group[a["group"]].append(a)
    jars = [Path(processed["path"])] + [Path(a["path"]) for a in by_group["minecraft_library"]] \
        + [Path(a["path"]) for a in by_group["fabric_loader"]] + [Path(a["path"]) for a in by_group["fabric_api_module"]]
    resolver = Resolver(jars, JDK_CLASSES if JDK_CLASSES.exists() else None)
    default_priority = fabric["mixin_runtime"]["annotation_defaults"]["Mixin"]["priority"]
    applies = {"game_versions": "26.3", "loader": "fabric", "loader_versions": fabric["loader"]["version"]}
    resolution_note = {"against": processed["path"], "sha256": processed["sha256"],
                       "note": "the Loom-processed compile jar; see resolved_environment.json for how it differs from the cache jar"}
    edges: list[dict] = []
    shared: dict[tuple, list[dict]] = defaultdict(list)

    def add(**e):
        e["id"] = f"e{len(edges) + 1:06d}"
        edges.append(e)
        return e

    for module in fabric["modules"]:
        prov = {"artifact": module["artifact"], "artifact_sha256": module["sha256"],
                "method": "class-file annotations (RuntimeVisible/InvisibleAnnotations)",
                "extracted_at": fabric["generated_at"], "resolved_against": resolution_note["sha256"]}
        for mx in module["mixins"]:
            envt = mx["effective_environment"]
            prio = mx["priority"] if mx["priority"] is not None else default_priority
            targets = [t["name"] for t in mx["targets"]]
            handler_owner = mx["mixin_class"].replace(".", "/")
            for inj in mx["injections"]:
                relation = RELATION_FOR_EFFECT.get(inj["effect"], "injects_into")
                for target in targets:
                    for sel in inj["selectors"]:
                        r = resolver.resolve_selector(sel, target)
                        to = {"kind": "method", "owner": r.get("owner") or target, "id": r.get("name") or sel.get("raw", "?"),
                              "resolution": _res(r)}
                        if r.get("desc"):
                            to["descriptor"] = r["desc"]
                        points = []
                        for at in inj["at"]:
                            pt = {"value": at.get("value")}
                            if at.get("target"):
                                pt["target"] = at["target"]
                                tgt = at["target"].strip()
                                type_form = re.fullmatch(r"L[\w/$]+;", tgt) or re.fullmatch(r"[\w/$]+", tgt)
                                ctor = re.fullmatch(r"\((?P<args>.*)\)L(?P<cls>[\w/$]+);", tgt) if at.get("value") == "NEW" else None
                                if ctor:
                                    # NEW with a constructor descriptor: "(args)Lowner;" names owner.<init>(args)V
                                    r_new = resolver.resolve(ctor.group("cls"), "<init>", f"({ctor.group('args')})V", "method")
                                    pt["resolution"] = _res(r_new)
                                    pt["resolution"]["kind"] = "constructor"
                                elif at.get("value") == "NEW" or (type_form and "(" not in tgt and ":" not in tgt):
                                    # a type target (NEW, or a bare class name): resolved as a class, not a member
                                    internal = tgt[1:-1] if tgt.startswith("L") else tgt
                                    pt["resolution"] = {"state": "exact" if resolver.cls(internal) else "owner_missing",
                                                        "kind": "type"}
                                else:
                                    pt["resolution"] = _res(resolver.resolve_target(at["target"], target))
                            if at.get("desc"):
                                pt["target"] = f"@Desc {at['desc']}"
                                pt["resolution"] = _res(resolver.resolve_selector(at["desc"], target))
                            for k in ("ordinal", "shift", "by", "opcode"):
                                if k in at:
                                    pt[k] = at[k]
                            if inj.get("expressions"):
                                pt["resolution"] = {"state": "selector_unsupported", "reason": "MixinExtras expression injection point"}
                            points.append(pt)
                        e = add(**{
                            "from": {"kind": "method", "owner": handler_owner, "id": inj["handler"]["name"],
                                     "descriptor": inj["handler"]["desc"], "static": inj["handler"]["static"]},
                            "to": to, "relation": relation, "injector": inj["injector"], "effect": inj["effect"],
                            "selector": sel.get("raw") or f"@Desc {sel.get('name')}",
                            "points": points, "priority": prio,
                            "priority_source": "annotation" if mx["priority"] is not None else "default",
                            "operation": f"@{inj['injector']}" + (f" at {[p['value'] for p in points]}" if points else ""),
                            "activation": f"mixin config {mx['declared_in_config']} applied for environment={envt}",
                            "evidence_class": "direct_reference",
                            "applies_to": {**applies, "environment": envt},
                            "limits": ["names the target and points as declared; whether the transformer applies "
                                       "this injection alongside others is a composition question "
                                       "(mixin_transformation_tests.json), and whether it runs needs the game"],
                            "provenance": {**prov, "locator": f"{mx['mixin_class']}#{inj['handler']['name']}{inj['handler']['desc']}"},
                            "module": module["id"],
                        })
                        if inj.get("cancellable"):
                            e["fields"] = ["cancellable"]
                        key = (to["owner"], to["id"], to.get("descriptor"))
                        shared[key].append({"module": module["id"], "mixin": mx["mixin_class"], "injector": inj["injector"],
                                            "effect": inj["effect"], "relation": relation, "priority": prio,
                                            "environment": envt, "resolution": to["resolution"]["state"],
                                            "points": [p.get("value") for p in points], "edge": e["id"]})
            for sh in mx["shadows"]:
                for target in targets:
                    kind = "method" if sh["desc"].startswith("(") else "field"
                    r = resolver.resolve(target, sh["name"], sh["desc"], kind)
                    add(**{"from": {"kind": "type", "id": mx["mixin_class"]},
                           "to": {"kind": kind, "owner": target, "id": sh["name"], "descriptor": sh["desc"], "resolution": _res(r)},
                           "relation": "reads" if kind == "field" else "calls", "operation": "@Shadow declaration",
                           "evidence_class": "declared", "applies_to": {**applies, "environment": envt},
                           "limits": ["a @Shadow declares access to the member; reads and writes through it are "
                                      "counted by the per-method reference edges of the mixin class"],
                           "provenance": {**prov, "locator": f"{mx['mixin_class']}#{sh['name']}"}, "module": module["id"]})
        # per-method references from every class of the module
        for cls in module["classes"]:
            for m in cls["methods"]:
                caller = {"kind": "method", "owner": cls["internal"], "id": m["name"], "descriptor": m["desc"], "static": m["static"]}
                for offset, opcode, kind, owner, name, desc, access in m["refs"]:
                    if not owner or not owner.startswith(VANILLA) or kind not in ("field", "method"):
                        continue  # `type` refs (new/anewarray/instanceof) are counted, not edges
                    r = resolver.resolve(owner, name, desc, kind)
                    relation = "calls" if kind == "method" else ("writes" if access == "write" else "reads")
                    add(**{"from": caller,
                           "to": {"kind": "field" if kind == "field" else "method", "owner": owner, "id": name,
                                  "descriptor": desc, "resolution": _res(r)},
                           "relation": relation, "site": {"offset": offset, "opcode": opcode},
                           "evidence_class": "direct_reference",
                           "applies_to": {**applies, "environment": "unknown"},
                           "limits": ["an instruction in the constant pool path; not a proof the instruction executes"],
                           "provenance": {**prov, "method": "class-file Code attribute (instruction + constant pool)",
                                          "locator": f"{cls['fqcn']}#{m['name']}{m['desc']}@{offset}"},
                           "module": module["id"]})
                for fire in m["event_fires"]:
                    add(**{"from": caller,
                           "to": {"kind": "event", "id": f"{fire['event_owner'].replace('/', '.')}.{fire['event_field']}"},
                           "relation": "publishes_event",
                           "site": {"offset": fire["offset"], "opcode": "invokeinterface"},
                           "operation": "getstatic Event field; Event.invoker(); checkcast callback; callback invoke",
                           "callback": f"{fire['callback_owner'].replace('/', '.')}.{fire['callback_name']}{fire['callback_desc']}",
                           "evidence_class": "static_inference",
                           "applies_to": {**applies, "environment": "unknown"},
                           "limits": ["a bytecode pattern inside one method; the field, invoker and callback are exact, "
                                      "but that the pattern is reached at runtime is not established"],
                           "provenance": {**prov, "method": "class-file Code attribute pattern",
                                          "locator": f"{cls['fqcn']}#{m['name']}{m['desc']}@{fire['offset']}"},
                           "module": module["id"]})
        # callback_of from the API surface's Event fields (generic signature)
        for api in module["api"] + module["api_nested"]:
            for f in api["fields"]:
                if f["desc"] != EVENT_DESC or not f.get("signature"):
                    continue
                m = re.match(r"Lnet/fabricmc/fabric/api/event/Event<L([^;<]+)[;<]", f["signature"])
                if not m:
                    continue
                add(**{"from": {"kind": "type", "id": m.group(1).replace("/", ".")},
                       "to": {"kind": "event", "id": f"{api['fqcn']}.{f['name']}"},
                       "relation": "callback_of", "evidence_class": "declared",
                       "applies_to": {**applies, "environment": {"*": "both", None: "both"}.get(module["declared"].get("environment"), module["declared"].get("environment"))},
                       "provenance": {**prov, "method": "field Signature attribute", "locator": f"{api['fqcn']}#{f['name']}"},
                       "module": module["id"]})
    # registers_into from BuiltInRegistries
    for r in registries["built_in_registries"]:
        elem = re.match(r".*<(.+)>$", r["type"])
        elem_type = (elem.group(1) if elem else r["type"]).strip().removeprefix("? extends ").split("<", 1)[0].strip()
        add(**{"from": {"kind": "type", "id": elem_type},
               "to": {"kind": "registry", "id": f"BuiltInRegistries.{r['field']}", "owner": "net.minecraft.core.registries.BuiltInRegistries"},
               "relation": "registers_into", "operation": "Registry.register(registry, id, value)",
               "evidence_class": "direct_reference", "applies_to": {"game_versions": "26.3", "environment": "both"},
               "provenance": {"artifact": registries["artifact"], "artifact_sha256": registries["sha256"],
                              "method": "field declarations of BuiltInRegistries", "extracted_at": registries["generated_at"],
                              "locator": "net.minecraft.core.registries.BuiltInRegistries"}})
    resolver.close()

    counts = Counter(e["relation"] for e in edges)
    evidence = Counter(e["evidence_class"] for e in edges)
    res_by_rel: dict[str, Counter] = defaultdict(Counter)
    for e in edges:
        st = e["to"].get("resolution", {}).get("state")
        if st:
            res_by_rel[e["relation"]][st] += 1
    point_states = Counter(p["resolution"]["state"] for e in edges for p in e.get("points", []) if "resolution" in p)
    shared_out = []
    for (owner, name, desc), entries in sorted(shared.items()):
        mods = sorted({x["module"] for x in entries})
        if len(mods) > 1:
            shared_out.append({"owner": owner, "method": name, "descriptor": desc, "modules": mods, "entries": entries,
                               "exactly_resolved": all(x["resolution"] in ("exact", "inherited_exact", "name_only") for x in entries)})
    out = {
        "generated_at": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "resolution": resolution_note,
        "counts": dict(counts), "evidence_classes": dict(evidence),
        "resolution_by_relation": {k: dict(v) for k, v in res_by_rel.items()},
        "injection_point_resolution": dict(point_states),
        "shared_targets": shared_out,
        "edges": edges,
    }
    (EXTRACTED / "edges.json").write_text(json.dumps(out, separators=(",", ":")))
    print(f"wrote edges.json: {len(edges)} edges; {dict(counts)}")
    print("resolution by relation:", {k: dict(v) for k, v in res_by_rel.items()})
    print("injection points:", dict(point_states), "| shared targets:", len(shared_out))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

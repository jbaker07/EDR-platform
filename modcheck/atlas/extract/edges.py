"""Typed interaction edges, derived from the extracted facts.

Every edge carries: exact endpoints, the relation, the evidence class, where it
applies, and the artifact hash it came from. The relation vocabulary is the
edge schema's; nothing is emitted as "related to".

Derivations, and what each is worth:

* injects_into  -- @Mixin class, @Inject/@Redirect/... member -> vanilla method.
                   direct_reference: the target is named in the annotation.
                   Live only when the module's mixin config applies for the
                   environment (client/server), which `activation` records.
* replaces      -- @Overwrite. direct_reference. This is the relation that
                   makes two mods incompatible: only one @Overwrite of a method
                   can win, and the loser is silently discarded.
* publishes_event -- a mixin/impl method fires an Event field.
                   static_inference: the getstatic/invokeinterface pairing is a
                   pattern, and is labelled so.
* callback_of   -- callback interface method <- event field. declared by the
                   API's own types (the field's generic parameter names it).
* reads/writes  -- field refs from mixin/impl code into net.minecraft.
                   direct_reference to the member; whether it is a read or a
                   write comes from the opcode (getfield/putfield).
* calls         -- method refs from mixin/impl code into net.minecraft.
                   direct_reference. Says the member is named, not that it runs.
* registers_into -- BuiltInRegistries fields: each is a registration target.
                   direct_reference to the field; element type from descriptor.
"""
from __future__ import annotations

import datetime as dt
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FABRIC = ROOT / "extracted" / "fabric_api.json"
REG = ROOT / "extracted" / "minecraft_registries.json"
OUT = ROOT / "extracted" / "edges.json"

_DESC = re.compile(r"^(?P<owner>[^;]+);(?P<name>[^(]+)(?P<desc>\(.*)$")


def _member_endpoint(target: str) -> dict:
    """'Lnet/minecraft/a/B;m(Ldesc;)V' -> method endpoint."""
    text = target.lstrip("L")
    m = _DESC.match(text)
    if m:
        return {"kind": "method", "owner": m.group("owner").replace("/", "."),
                "id": m.group("name"), "descriptor": m.group("desc")}
    if ":" in text:
        owner_name, desc = text.split(":", 1)
        owner, name = owner_name.rsplit(";", 1) if ";" in owner_name else ("", owner_name)
        return {"kind": "field", "owner": owner.replace("/", "."), "id": name, "descriptor": desc}
    return {"kind": "type", "id": text.rstrip(";").replace("/", ".")}


def _ref_endpoint(ref: str, kind: str) -> dict:
    owner, rest = ref.split(".", 1) if "." in ref else ("", ref)
    name, desc = rest.split(":", 1) if ":" in rest else (rest, "")
    return {"kind": kind, "owner": owner.replace("/", "."), "id": name, "descriptor": desc}


def build() -> dict:
    fabric = json.loads(FABRIC.read_text())
    reg = json.loads(REG.read_text())
    edges: list[dict] = []
    seq = 0

    def add(**edge):
        nonlocal seq
        seq += 1
        edge.setdefault("id", f"e{seq:06d}")
        edges.append(edge)

    for module in fabric["modules"]:
        applies = {"game_versions": "26.3", "loader": "fabric",
                   "loader_versions": (module["declared"].get("depends") or {}).get("fabricloader", "?")}
        prov = {"artifact": module["artifact"], "artifact_sha256": module["sha256"],
                "method": "javap -v -p annotations", "extracted_at": fabric["generated_at"]}
        # Which environment each mixin config applies in.
        env_of_config: dict[str, str] = {}
        for raw in module["declared"].get("mixins") or []:
            if isinstance(raw, dict):
                env_of_config[raw["config"]] = raw.get("environment", "both")
            else:
                env_of_config[raw] = "both"
        mixin_env: dict[str, str] = {}
        for name, cfg in module["mixin_configs"].items():
            pkg = cfg.get("package") or ""
            env = env_of_config.get(name, "both")
            for group, forced in (("mixins", None), ("client", "client"), ("server", "server")):
                for cls in cfg.get(group) or []:
                    mixin_env[f"{pkg}.{cls}"] = forced or env

        for mixin in module["mixins"]:
            env = mixin.get("environment") or mixin_env.get(mixin["mixin_class"], "both")
            mixin_ep = {"kind": "type", "id": mixin["mixin_class"]}
            for target in mixin["targets"]:
                for inj in mixin["injections"]:
                    relation = "replaces" if inj["injector"] == "Overwrite" else \
                               "wraps" if inj["injector"] in ("Redirect", "WrapOperation", "WrapMethod") else \
                               "injects_into"
                    handler = re.search(r"([\w$<>]+)\(", inj.get("member") or "")
                    handler_ep = ({"kind": "method", "owner": mixin["mixin_class"], "id": handler.group(1)}
                                  if handler else mixin_ep)
                    for method in (inj["method"] or ([inj["member"].split("(")[0].split()[-1]]
                                                     if inj["injector"] == "Overwrite" and inj["member"] else ["?"])):
                        add(**{
                            "from": handler_ep,
                            "to": {"kind": "method", "owner": target, "id": method},
                            "relation": relation,
                            "operation": f"@{inj['injector']}"
                                         + (f" at {inj['at']}" if inj.get("at") else "")
                                         + (f" {inj['target']}" if inj.get("target") else ""),
                            "activation": f"mixin config applied for environment={env}",
                            "evidence_class": "direct_reference",
                            "applies_to": {**applies, "environment": env},
                            "limits": ["names the injection target; does not establish the "
                                       "injected code runs, or its effect on other injections "
                                       "into the same method"],
                            "provenance": {**prov, "locator": mixin["mixin_class"]},
                            "module": module["id"],
                        })
            for fire in mixin["event_fires"]:
                add(**{
                    "from": {"kind": "method", "owner": mixin["mixin_class"], "id": fire["method"]},
                    "to": {"kind": "event", "id": fire["event_field"].replace("/", ".")},
                    "relation": "publishes_event",
                    "operation": "invoker() call on the Event field",
                    "activation": f"mixin config applied for environment={env}",
                    "evidence_class": "static_inference",
                    "applies_to": {**applies, "environment": env},
                    "limits": ["getstatic/invokeinterface pairing within one method; a "
                               "second fire site in the same method may be attributed to "
                               "the wrong field"],
                    "provenance": {**prov, "method": "javap -c -p pattern",
                                   "locator": mixin["mixin_class"]},
                    "module": module["id"],
                    "callback": fire["callback"].replace("/", "."),
                })
            for ref in mixin["vanilla_refs"]["methods"]:
                add(**{"from": mixin_ep, "to": _ref_endpoint(ref, "method"), "relation": "calls",
                       "evidence_class": "direct_reference",
                       "applies_to": {**applies, "environment": env},
                       "limits": ["constant-pool reference; not a proof of execution"],
                       "provenance": {**prov, "method": "javap -c -p Methodref",
                                      "locator": mixin["mixin_class"]},
                       "module": module["id"]})
            for ref in mixin["vanilla_refs"]["fields"]:
                add(**{"from": mixin_ep, "to": _ref_endpoint(ref, "field"), "relation": "reads",
                       "evidence_class": "direct_reference",
                       "applies_to": {**applies, "environment": env},
                       "limits": ["Fieldref; read/write direction not separated in this pass"],
                       "provenance": {**prov, "method": "javap -c -p Fieldref",
                                      "locator": mixin["mixin_class"]},
                       "module": module["id"]})
        for impl in module["impl"]:
            impl_ep = {"kind": "type", "id": impl["fqcn"]}
            for fire in impl["event_fires"]:
                add(**{"from": {"kind": "method", "owner": impl["fqcn"], "id": fire["method"]},
                       "to": {"kind": "event", "id": fire["event_field"].replace("/", ".")},
                       "relation": "publishes_event", "evidence_class": "static_inference",
                       "applies_to": {**applies, "environment": "unknown"},
                       "limits": ["pattern-inferred"],
                       "provenance": {**prov, "method": "javap -c -p pattern", "locator": impl["fqcn"]},
                       "module": module["id"], "callback": fire["callback"].replace("/", ".")})
            for ref in impl["vanilla_refs"]["methods"]:
                add(**{"from": impl_ep, "to": _ref_endpoint(ref, "method"), "relation": "calls",
                       "evidence_class": "direct_reference",
                       "applies_to": {**applies, "environment": "unknown"},
                       "limits": ["constant-pool reference"],
                       "provenance": {**prov, "method": "javap -c -p Methodref", "locator": impl["fqcn"]},
                       "module": module["id"]})
        # callback_of: event fields in the API surface, from their generic type.
        for api in module["api"]:
            for member in api["members"]:
                m = re.match(r"public static final net\.fabricmc\.fabric\.api\.event\.Event<(?P<cb>[^>]+)> (?P<field>[A-Z_0-9]+)$", member)
                if m:
                    add(**{"from": {"kind": "type", "id": m.group("cb")},
                           "to": {"kind": "event", "id": f"{api['fqcn']}.{m.group('field')}"},
                           "relation": "callback_of", "evidence_class": "declared",
                           "applies_to": applies,
                           "provenance": {**prov, "method": "javap public surface",
                                          "locator": api["fqcn"]},
                           "module": module["id"]})

    mc_prov = {"artifact": reg["artifact"], "artifact_sha256": reg["sha256"],
               "method": "javap public surface", "extracted_at": reg["generated_at"],
               "locator": "net.minecraft.core.registries.BuiltInRegistries"}
    for r in reg["built_in_registries"]:
        elem = re.search(r"<(.+)>$", r["type"])
        elem_type = (elem.group(1) if elem else r["type"]).strip()
        elem_type = elem_type.removeprefix("? extends ").split("<", 1)[0].strip()
        add(**{"from": {"kind": "type", "id": elem_type},
               "to": {"kind": "registry", "id": f"BuiltInRegistries.{r['field']}",
                      "owner": "net.minecraft.core.registries.BuiltInRegistries"},
               "relation": "registers_into", "operation": "Registry.register(registry, id, value)",
               "evidence_class": "direct_reference",
               "applies_to": {"game_versions": "26.3", "environment": "both"},
               "provenance": mc_prov})

    from collections import Counter
    return {
        "generated_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat(),
        "inputs": {"fabric_api": fabric["generated_at"], "minecraft_registries": reg["generated_at"]},
        "counts": dict(Counter(e["relation"] for e in edges)),
        "evidence_classes": dict(Counter(e["evidence_class"] for e in edges)),
        "edges": edges,
    }


def main() -> int:
    out = build()
    OUT.write_text(json.dumps(out, indent=1))
    print(f"wrote {OUT}: {len(out['edges'])} edges; {out['counts']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

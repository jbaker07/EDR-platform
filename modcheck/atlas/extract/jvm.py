"""Facts about JVM classes for the atlas, read from class files (see classfile.py).

Three readers, all pure functions of one ``ClassFile``:

* ``mixin_facts``   -- @Mixin targets, every injector annotation from the Mixin
                       and MixinExtras namespaces (including no-argument markers
                       such as @Overwrite), selectors and injection points as
                       structured data, and explicit ``extraction_failures`` for
                       any annotation form the reader does not understand.
* ``class_refs``    -- every constant-pool reference the code makes, per method,
                       with the instruction offset and opcode, so a field access
                       is a read or a write, and a call has a caller.
* ``surface``       -- declared members with exact descriptors and access flags.

Nothing here resolves a reference against another class; that is
``resolve.py``'s job, and it reports what it could not resolve.
"""
from __future__ import annotations

import hashlib
import json
import re
import zipfile
from pathlib import Path

from classfile import ClassFile, Member, member_to_java  # noqa: F401  (re-exported for callers)

MIXIN_NS = "Lorg/spongepowered/asm/mixin/"
EXTRAS_NS = "Lcom/llamalad7/mixinextras/"
FABRIC_ENV = "Lnet/fabricmc/api/Environment;"
EVENT_DESC = "Lnet/fabricmc/fabric/api/event/Event;"
EVENT_CLASS = "net/fabricmc/fabric/api/event/Event"

# annotation type descriptor -> injector name. Anything else in the two
# namespaces that sits on a method is an extraction failure, not a skip.
INJECTOR_TYPES = {
    MIXIN_NS + "injection/Inject;": "Inject",
    MIXIN_NS + "injection/Redirect;": "Redirect",
    MIXIN_NS + "injection/ModifyArg;": "ModifyArg",
    MIXIN_NS + "injection/ModifyArgs;": "ModifyArgs",
    MIXIN_NS + "injection/ModifyConstant;": "ModifyConstant",
    MIXIN_NS + "injection/ModifyVariable;": "ModifyVariable",
    MIXIN_NS + "Overwrite;": "Overwrite",
    EXTRAS_NS + "injector/ModifyExpressionValue;": "ModifyExpressionValue",
    EXTRAS_NS + "injector/ModifyReceiver;": "ModifyReceiver",
    EXTRAS_NS + "injector/ModifyReturnValue;": "ModifyReturnValue",
    EXTRAS_NS + "injector/WrapWithCondition;": "WrapWithCondition(v1)",
    EXTRAS_NS + "injector/v2/WrapWithCondition;": "WrapWithCondition",
    EXTRAS_NS + "injector/wrapoperation/WrapOperation;": "WrapOperation",
    EXTRAS_NS + "injector/wrapmethod/WrapMethod;": "WrapMethod",
}
# the composition class of each injector: what it does to the target method
INJECTOR_EFFECT = {
    "Inject": "additive", "ModifyArg": "modify_call_argument", "ModifyArgs": "modify_call_arguments",
    "ModifyConstant": "modify_constant", "ModifyVariable": "modify_local",
    "Redirect": "replace_call_site", "Overwrite": "replace_method_body",
    "ModifyExpressionValue": "modify_expression_value", "ModifyReceiver": "modify_call_receiver",
    "ModifyReturnValue": "modify_return_value", "WrapWithCondition(v1)": "wrap_call_site_conditionally",
    "WrapWithCondition": "wrap_call_site_conditionally", "WrapOperation": "wrap_call_site",
    "WrapMethod": "wrap_method",
}
# MixinExtras expression injection points: @Expression / @Definition(s) qualify an
# @At("MIXINEXTRAS:EXPRESSION") on the same handler. Recorded on the injection;
# never resolvable to one exact member, since an expression matches code shape.
EXPRESSION_TYPES = {EXTRAS_NS + "expression/Expression;": "Expression",
                    EXTRAS_NS + "expression/Definition;": "Definition",
                    EXTRAS_NS + "expression/Definitions;": "Definitions"}
MARKER_TYPES = {
    MIXIN_NS + "Shadow;": "Shadow", MIXIN_NS + "Unique;": "Unique", MIXIN_NS + "Final;": "Final",
    MIXIN_NS + "Mutable;": "Mutable", MIXIN_NS + "Dynamic;": "Dynamic", MIXIN_NS + "Intrinsic;": "Intrinsic",
    MIXIN_NS + "SoftOverride;": "SoftOverride", MIXIN_NS + "Debug;": "Debug",
    MIXIN_NS + "gen/Accessor;": "Accessor", MIXIN_NS + "gen/Invoker;": "Invoker",
    MIXIN_NS + "injection/Surrogate;": "Surrogate", MIXIN_NS + "injection/Group;": "Group",
}
CLASS_LEVEL_TYPES = {MIXIN_NS + "Mixin;": "Mixin", MIXIN_NS + "Pseudo;": "Pseudo",
                     MIXIN_NS + "Implements;": "Implements", MIXIN_NS + "Debug;": "Debug",
                     MIXIN_NS + "Unique;": "Unique", MIXIN_NS + "Dynamic;": "Dynamic"}
# parameter-level sugar we recognise (recorded, never a failure)
PARAM_TYPES = {MIXIN_NS + "injection/Coerce;", EXTRAS_NS + "sugar/Local;", EXTRAS_NS + "sugar/Share;",
               EXTRAS_NS + "sugar/Cancellable;"}

# Mixin's MemberInfo selector grammar, the subset we resolve exactly:
#   [Lowner;]name[(args)ret]      or      [owner.]name[(args)ret]
# A quantifier (name*, name+, name{n,m}) or a regex selector (/.../) is recorded as
# unsupported for exact resolution -- it can match many members.
_SELECTOR = re.compile(r"^(?P<owner>L[^;]+;)?(?P<name>[^\s(*+{]+)(?P<quant>\*|\+|\{[^}]*\})?(?P<desc>\(.*\).+)?$")


def sha256_of(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def classes_in(jar: Path, prefix: str = "", include_nested: bool = True) -> list[str]:
    out = []
    with zipfile.ZipFile(jar) as zf:
        for name in zf.namelist():
            if not name.endswith(".class") or not name.startswith(prefix):
                continue
            if not include_nested and "$" in name:
                continue
            out.append(name[:-6].replace("/", "."))
    return sorted(out)


def read_json_entry(jar: Path, entry: str) -> dict | list | None:
    with zipfile.ZipFile(jar) as zf:
        if entry not in zf.namelist():
            return None
        raw = zf.read(entry)
    return json.loads(raw.decode("utf-8-sig"))


# --- helpers over annotation values ---------------------------------------------
def _as_list(v):
    if v is None:
        return []
    return v if isinstance(v, list) else [v]


def _class_internal(v) -> str | None:
    """{'class': 'Lfoo/Bar;'} -> 'foo/Bar'"""
    if isinstance(v, dict) and "class" in v:
        d = v["class"]
        return d[1:-1] if d.startswith("L") and d.endswith(";") else d
    return None


def parse_selector(raw: str) -> dict:
    """A Mixin member selector string -> structured selector."""
    sel = {"kind": "string", "raw": raw, "owner": None, "name": None, "desc": None, "exact_resolvable": False}
    if raw.startswith("/") and raw.endswith("/"):
        sel["unsupported"] = "regex selector"
        return sel
    if raw == "*":
        sel["name"], sel["quantifier"], sel["unsupported"] = "*", "*", "wildcard selector matches every method"
        return sel
    m = _SELECTOR.match(raw)
    if not m:
        sel["unsupported"] = "selector did not parse"
        return sel
    owner = m.group("owner")
    name = m.group("name")
    if owner:
        sel["owner"] = owner[1:-1]
    elif "." in name and not name.startswith("<"):
        sel["owner"], name = name.rsplit(".", 1)
        sel["owner"] = sel["owner"].replace(".", "/")
    sel["name"] = name
    sel["desc"] = m.group("desc")
    if m.group("quant"):
        sel["quantifier"] = m.group("quant")
        sel["unsupported"] = "quantified selector matches many members"
    else:
        sel["exact_resolvable"] = True
    return sel


def _desc_selector(ann: dict) -> dict:
    v = ann.get("values", {})
    args = [_class_internal(a) or a for a in _as_list(v.get("args"))]
    ret = _class_internal(v.get("ret")) if v.get("ret") else None
    return {"kind": "desc", "owner": _class_internal(v.get("owner")), "name": v.get("value"),
            "args": args, "ret": ret, "id": v.get("id"), "exact_resolvable": bool(v.get("value"))}


def _at(ann: dict) -> dict:
    v = ann.get("values", {})
    out = {"value": v.get("value"), "target": v.get("target"), "ordinal": v.get("ordinal"),
           "opcode": v.get("opcode"), "shift": (v.get("shift") or {}).get("value") if isinstance(v.get("shift"), dict) else v.get("shift"),
           "by": v.get("by"), "args": _as_list(v.get("args")), "slice": v.get("slice"), "id": v.get("id"),
           "remap": v.get("remap"), "unsafe": v.get("unsafe")}
    if isinstance(v.get("desc"), dict):
        out["desc"] = _desc_selector(v["desc"])
    return {k: val for k, val in out.items() if val not in (None, [])}


def _slice(ann: dict) -> dict:
    v = ann.get("values", {})
    return {"id": v.get("id"), "from": _at(v["from"]) if isinstance(v.get("from"), dict) else None,
            "to": _at(v["to"]) if isinstance(v.get("to"), dict) else None}


def _injection(injector: str, ann: dict, handler: Member) -> dict:
    v = ann.get("values", {})
    selectors = [parse_selector(s) for s in _as_list(v.get("method"))]
    selectors += [_desc_selector(d) for d in _as_list(v.get("target")) if isinstance(d, dict)]
    if injector == "Overwrite":
        # an overwrite targets the method with the handler's own name and descriptor
        selectors = [{"kind": "handler", "owner": None, "name": handler.name, "desc": handler.desc,
                      "exact_resolvable": True}]
    inj = {
        "injector": injector, "annotation": ann["type"], "effect": INJECTOR_EFFECT.get(injector, "unknown"),
        "handler": {"name": handler.name, "desc": handler.desc, "static": handler.is_static},
        "selectors": selectors,
        "at": [_at(a) for a in _as_list(v.get("at")) if isinstance(a, dict)],
        "slice": [_slice(s) for s in _as_list(v.get("slice")) if isinstance(s, dict)],
    }
    for key in ("cancellable", "require", "expect", "allow", "remap", "locals", "index", "ordinal", "argsOnly",
                "print", "id", "slice"):
        if key in v and key not in inj:
            val = v[key]
            inj[key] = val.get("value") if isinstance(val, dict) and "enum" in val else val
    if injector == "ModifyConstant":
        inj["constant"] = [c.get("values", {}) for c in _as_list(v.get("constant")) if isinstance(c, dict)]
    unsupported = [s["unsupported"] for s in selectors if s.get("unsupported")]
    if not selectors:
        unsupported.append("no selector (neither method nor target given)")
    if unsupported:
        inj["unsupported"] = unsupported
    return inj


def mixin_facts(cf: ClassFile) -> dict | None:
    """Structured facts for one mixin class, or None if the class is not a mixin."""
    mixin_ann = next((a for a in cf.annotations if a["type"] == MIXIN_NS + "Mixin;"), None)
    if mixin_ann is None:
        return None
    v = mixin_ann.get("values", {})
    facts = {
        "mixin_class": cf.name.replace("/", "."),
        "kind": cf.kind,
        "targets": [{"kind": "class", "name": n} for n in (_class_internal(x) for x in _as_list(v.get("value"))) if n]
                   + [{"kind": "string", "name": s.replace(".", "/")} for s in _as_list(v.get("targets"))],
        "priority": v.get("priority"),     # None = annotation default (read from the Mixin jar separately)
        "remap": v.get("remap"),
        "pseudo": any(a["type"] == MIXIN_NS + "Pseudo;" for a in cf.annotations),
        "environment": None,
        "injections": [], "shadows": [], "accessors": [], "uniques": [], "implements": [],
        "extraction_failures": [],
    }
    for a in cf.annotations:
        if a["type"] == FABRIC_ENV:
            ev = a.get("values", {}).get("value")
            facts["environment"] = ev.get("value", "").lower() if isinstance(ev, dict) else None
        elif a["type"] == MIXIN_NS + "Implements;":
            facts["implements"] = [(i.get("values", {}).get("iface") or {}) for i in _as_list(a["values"].get("value"))]
        elif (a["type"].startswith(MIXIN_NS) or a["type"].startswith(EXTRAS_NS)) and a["type"] not in CLASS_LEVEL_TYPES:
            facts["extraction_failures"].append({"where": "class", "annotation": a["type"],
                                                 "reason": "unsupported class-level annotation"})
    for member in cf.fields + cf.methods:
        is_method = member.desc.startswith("(")
        expressions = [{"kind": EXPRESSION_TYPES[a["type"]], "values": a.get("values", {})}
                       for a in member.annotations if a["type"] in EXPRESSION_TYPES]
        for a in member.annotations:
            t = a["type"]
            if t in INJECTOR_TYPES and is_method:
                inj = _injection(INJECTOR_TYPES[t], a, member)
                if expressions:
                    inj["expressions"] = expressions
                    inj.setdefault("unsupported", []).append(
                        "MixinExtras expression injection point: matches code shape, not one member")
                facts["injections"].append(inj)
            elif t in EXPRESSION_TYPES:
                continue
            elif t in MARKER_TYPES:
                name = MARKER_TYPES[t]
                rec = {"name": member.name, "desc": member.desc, "static": member.is_static}
                if name == "Shadow":
                    rec["aliases"] = _as_list(a.get("values", {}).get("aliases"))
                    rec["prefix"] = a.get("values", {}).get("prefix")
                    facts["shadows"].append(rec)
                elif name in ("Accessor", "Invoker"):
                    rec["kind"] = name
                    rec["target"] = a.get("values", {}).get("value")
                    facts["accessors"].append(rec)
                elif name == "Unique":
                    facts["uniques"].append(rec)
                # Final/Mutable/Dynamic/Intrinsic/SoftOverride/Debug/Surrogate/Group: recognised, no edge
            elif t.startswith(MIXIN_NS) or t.startswith(EXTRAS_NS):
                facts["extraction_failures"].append({"where": f"{member.name}{member.desc}", "annotation": t,
                                                     "reason": "unsupported member annotation"})
        for params in member.param_annotations:
            for a in params:
                if (a["type"].startswith(MIXIN_NS) or a["type"].startswith(EXTRAS_NS)) and a["type"] not in PARAM_TYPES:
                    facts["extraction_failures"].append({"where": f"{member.name}{member.desc}", "annotation": a["type"],
                                                         "reason": "unsupported parameter annotation"})
    return facts


# --- references per method, with opcodes and offsets ---------------------------------
_ACCESS = {"getstatic": "read", "getfield": "read", "putstatic": "write", "putfield": "write"}
_INVOKE = {"invokevirtual", "invokespecial", "invokestatic", "invokeinterface"}


def class_refs(cf: ClassFile) -> dict:
    methods = []
    for m in cf.methods:
        if m.code is None:
            continue
        refs, fires = [], []
        pending = None      # (offset, owner, field) after getstatic of an Event field
        invoker_seen = False
        callback_type = None
        for ins in m.code:
            r = ins.ref
            if not r:
                continue
            if ins.mnemonic in _ACCESS and r["kind"] == "Fieldref":
                refs.append({"offset": ins.offset, "opcode": ins.mnemonic, "access": _ACCESS[ins.mnemonic],
                             "kind": "field", "owner": r["owner"], "name": r["name"], "desc": r["desc"]})
                if ins.mnemonic == "getstatic" and r["desc"] == EVENT_DESC:
                    pending, invoker_seen, callback_type = (ins.offset, r["owner"], r["name"]), False, None
            elif ins.mnemonic in _INVOKE and r["kind"] in ("Methodref", "InterfaceMethodref"):
                refs.append({"offset": ins.offset, "opcode": ins.mnemonic, "kind": "method",
                             "interface": r["kind"] == "InterfaceMethodref",
                             "owner": r["owner"], "name": r["name"], "desc": r["desc"]})
                if pending and r["owner"] == EVENT_CLASS and r["name"] == "invoker":
                    invoker_seen = True
                elif pending and invoker_seen and callback_type and r["owner"] == callback_type:
                    fires.append({"offset": ins.offset, "event_owner": pending[1], "event_field": pending[2],
                                  "getstatic_offset": pending[0], "callback_owner": r["owner"],
                                  "callback_name": r["name"], "callback_desc": r["desc"]})
                    pending, invoker_seen, callback_type = None, False, None
            elif ins.mnemonic == "checkcast" and r["kind"] == "Class":
                if pending and invoker_seen and callback_type is None:
                    callback_type = r["name"]
            elif ins.mnemonic == "invokedynamic":
                refs.append({"offset": ins.offset, "opcode": "invokedynamic", "kind": "indy",
                             "name": r["name"], "desc": r["desc"]})
            elif ins.mnemonic in ("new", "anewarray", "instanceof") and r["kind"] == "Class":
                refs.append({"offset": ins.offset, "opcode": ins.mnemonic, "kind": "type", "owner": r["name"]})
        methods.append({"name": m.name, "desc": m.desc, "static": m.is_static, "refs": refs, "event_fires": fires})
    return {"fqcn": cf.name.replace("/", "."), "internal": cf.name, "super": cf.super_name,
            "interfaces": cf.interfaces, "methods": methods}


# --- declared surface ------------------------------------------------------------------
def surface(cf: ClassFile, public_only: bool = False) -> dict:
    def keep(m: Member) -> bool:
        return not public_only or bool(m.access & 0x0001) or bool(m.access & 0x0004)
    return {
        "fqcn": cf.name.replace("/", "."), "internal": cf.name, "kind": cf.kind, "access": cf.access,
        "super": cf.super_name, "interfaces": cf.interfaces, "signature": cf.signature,
        "fields": [{"name": f.name, "desc": f.desc, "access": f.access, "signature": f.signature}
                   for f in cf.fields if keep(f)],
        "methods": [{"name": m.name, "desc": m.desc, "access": m.access, "signature": m.signature,
                     "has_code": m.code is not None} for m in cf.methods if keep(m)],
    }


def surface_lines(s: dict) -> list[str]:
    """Display strings for a surface, javap-like."""
    out = []
    for f in s["fields"]:
        out.append(member_to_java(Member(f["name"], f["desc"], f["access"])))
    for m in s["methods"]:
        out.append(member_to_java(Member(m["name"], m["desc"], m["access"])))
    return out

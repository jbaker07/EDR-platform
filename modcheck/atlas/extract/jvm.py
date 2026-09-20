"""Mechanical facts from JVM artifacts, via the JDK's own ``javap``.

Reused rather than reimplemented: a class-file parser we wrote could disagree
with the compiler, and these facts feed compatibility conclusions. ``javap -v``
prints annotations with their arguments, which is where Mixin targets live;
``javap -c`` prints resolved method and field references, which is where a
class's direct dependencies live.

Four kinds of fact come out, and they are kept apart because they mean
different things:

* ``direct_reference`` -- a Methodref/Fieldref/InterfaceMethodref in the
  constant pool. Established from the artifact. Says the code *names* the
  member, not that it runs.
* ``mixin_injection`` -- a ``@Mixin`` class with ``@Inject``/``@Redirect``/
  ``@Overwrite``/... members. Established from the artifact. Says the loader
  will rewrite that vanilla method at load time -- IF the mixin config is
  applied for this environment.
* ``event_fire`` -- a ``getstatic <Events>.<FIELD>`` followed by an
  ``invokeinterface`` on that event's callback interface in the same method.
  Inferred by static pattern; the pairing is a heuristic and is labelled so.
* ``declared`` -- fabric.mod.json contents. Declared by the module author.

Nothing here observes runtime behaviour.
"""
from __future__ import annotations

import dataclasses
import hashlib
import json
import re
import subprocess
import zipfile
from pathlib import Path
from typing import Iterable

INJECTORS = ("Inject", "Redirect", "ModifyVariable", "ModifyArg", "ModifyArgs",
             "ModifyConstant", "ModifyReturnValue", "ModifyExpressionValue",
             "WrapOperation", "WrapWithCondition", "WrapMethod", "Overwrite")
MEMBER_MARKERS = ("Shadow", "Accessor", "Invoker", "Unique", "Mutable", "Final")

_MEMBER_DECL = re.compile(r"^  (?:public |private |protected |static |final |abstract |synchronized |native |default )*"
                          r"(?P<decl>[^\s].*\S)\s*;$")
_REF = re.compile(r"//\s+(?P<kind>Method|InterfaceMethod|Field)\s+(?P<ref>\S+)")
_METHOD_HEAD = re.compile(r"^  (?:[a-z ]+ )?[^\s(]+\s+(?P<name>[^\s(]+)\((?P<args>[^)]*)\)")


def sha256_of(path: Path) -> str:
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def javap(javap_bin: Path, jar: Path, fqcn: str, *flags: str) -> str:
    proc = subprocess.run(
        [str(javap_bin), *flags, "-cp", str(jar), fqcn],
        capture_output=True, text=True, timeout=180,
        env={"PATH": "/usr/bin:/bin", "JAVA_TOOL_OPTIONS": ""})
    return proc.stdout if proc.returncode == 0 else ""


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
    # fabric.mod.json and mixin configs are plain JSON; tolerate a BOM.
    return json.loads(raw.decode("utf-8-sig"))


# --- annotations (javap -v) -------------------------------------------------

def _annotation_blocks(text: str) -> list[tuple[str | None, str, str]]:
    """(member declaration or None for class-level, annotation name, block text)."""
    lines = text.splitlines()
    blocks: list[tuple[str | None, str, str]] = []
    member: str | None = None
    in_class_body = False
    i = 0
    while i < len(lines):
        line = lines[i]
        if not in_class_body:
            if line.startswith("{"):
                in_class_body = True
            i += 1
            continue
        decl = _MEMBER_DECL.match(line)
        if decl and not line.startswith("    "):
            member = decl.group("decl")
        stripped = line.strip()
        if stripped.startswith("org.spongepowered.asm.mixin") and stripped.endswith("("):
            name = stripped[: -1]
            indent = len(line) - len(line.lstrip())
            j = i + 1
            body = []
            while j < len(lines):
                cur = lines[j]
                if cur.strip() == ")" and len(cur) - len(cur.lstrip()) == indent:
                    break
                body.append(cur)
                j += 1
            blocks.append((member if line.startswith("    ") or member else None,
                           name, "\n".join(body)))
            i = j
        i += 1
    return blocks


def _list_field(block: str, field: str) -> list[str]:
    m = re.search(rf"\b{field}=\[(.*?)\]", block, re.S)
    if not m:
        return []
    inner = m.group(1)
    return [s.strip().strip('"') for s in re.findall(r'"[^"]*"|class [^\s,\]]+', inner)]


def _str_field(block: str, field: str) -> str | None:
    m = re.search(rf'\b{field}="([^"]*)"', block)
    return m.group(1) if m else None


def _class_list(values: Iterable[str]) -> list[str]:
    out = []
    for v in values:
        v = v.strip()
        if v.startswith("class "):
            v = v[6:]
        v = v.strip("L;").replace("/", ".")
        out.append(v)
    return out


@dataclasses.dataclass
class Injection:
    injector: str
    method: list[str]
    at: str | None
    target: str | None
    cancellable: bool
    member: str | None


@dataclasses.dataclass
class MixinFacts:
    mixin_class: str
    targets: list[str]
    injections: list[Injection]
    shadows: list[str]
    accessors: list[str]
    overwrites: list[str]
    environment: str | None


def mixin_facts(javap_bin: Path, jar: Path, fqcn: str) -> MixinFacts | None:
    text = javap(javap_bin, jar, fqcn, "-v", "-p")
    if "org.spongepowered.asm.mixin.Mixin(" not in text and "asm/mixin/Mixin;" not in text:
        return None
    targets: list[str] = []
    injections: list[Injection] = []
    shadows: list[str] = []
    accessors: list[str] = []
    overwrites: list[str] = []
    for member, name, block in _annotation_blocks(text):
        short = name.rsplit(".", 1)[-1]
        if short == "Mixin":
            targets = _class_list(_list_field(block, "value")) + _list_field(block, "targets")
        elif short in INJECTORS:
            at_block = re.search(r"at=\[?@org\.spongepowered\.asm\.mixin\.injection\.At\((.*?)\n\s*\)", block, re.S)
            at_text = at_block.group(1) if at_block else block
            injections.append(Injection(
                injector=short,
                method=_list_field(block, "method"),
                at=_str_field(at_text, "value") if at_block else None,
                target=_str_field(at_text, "target") if at_block else None,
                cancellable="cancellable=true" in block,
                member=member))
            if short == "Overwrite" and member:
                overwrites.append(member)
        elif short == "Shadow" and member:
            shadows.append(member)
        elif short in ("Accessor", "Invoker") and member:
            accessors.append(member)
    env = None
    m = re.search(r"net\.fabricmc\.api\.Environment\(\s*value=(?:.*?)\.(CLIENT|SERVER)", text, re.S)
    if m:
        env = m.group(1).lower()
    return MixinFacts(mixin_class=fqcn, targets=targets, injections=injections,
                      shadows=shadows, accessors=accessors, overwrites=overwrites,
                      environment=env)


# --- references and event fires (javap -c) ----------------------------------

@dataclasses.dataclass
class ClassRefs:
    fqcn: str
    methods: list[str]                 # owner.name:desc (owner may be the class itself)
    fields: list[str]
    event_fires: list[dict]            # {method, event_field, callback}
    method_names: list[str]


def class_refs(javap_bin: Path, jar: Path, fqcn: str) -> ClassRefs:
    text = javap(javap_bin, jar, fqcn, "-c", "-p")
    methods: set[str] = set()
    fields: set[str] = set()
    fires: list[dict] = []
    names: list[str] = []
    current: str | None = None
    pending_event: str | None = None
    for line in text.splitlines():
        head = _METHOD_HEAD.match(line)
        if head and not line.startswith("    "):
            current = head.group("name")
            names.append(current)
            pending_event = None
        m = _REF.search(line)
        if not m:
            continue
        kind, ref = m.group("kind"), m.group("ref")
        if kind == "Field":
            fields.add(ref)
            if ref.endswith(":Lnet/fabricmc/fabric/api/event/Event;") and "getstatic" in line:
                pending_event = ref.split(":")[0]
        else:
            methods.add(ref)
            if pending_event and kind == "InterfaceMethod":
                owner = ref.split(".")[0]
                event_owner = pending_event.rsplit(".", 1)[0]
                if owner.startswith(event_owner):
                    fires.append({"method": current, "event_field": pending_event,
                                  "callback": ref})
                    pending_event = None
    return ClassRefs(fqcn=fqcn, methods=sorted(methods), fields=sorted(fields),
                     event_fires=fires, method_names=names)


# --- public API surface (javap, no -p) ---------------------------------------

def public_surface(javap_bin: Path, jar: Path, fqcn: str) -> dict:
    text = javap(javap_bin, jar, fqcn)
    members = []
    kind = "class"
    for line in text.splitlines():
        s = line.strip()
        if s.startswith("Compiled from"):
            continue
        if s.endswith("{") and not members:
            head = s[:-1].strip()
            if " interface " in f" {head} ":
                kind = "interface"
            elif " enum " in f" {head} ":
                kind = "enum"
            elif " record " in f" {head}" or "extends java.lang.Record" in head:
                kind = "record"
            elif "abstract class" in head:
                kind = "abstract_class"
            continue
        if s.endswith(";") and s not in ("}",):
            members.append(s[:-1])
    return {"fqcn": fqcn, "kind": kind, "members": members}

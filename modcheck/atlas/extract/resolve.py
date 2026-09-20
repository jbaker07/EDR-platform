"""Exact member resolution against a set of jars.

A reference (owner, name, descriptor) or a Mixin selector resolves to one of:

* ``exact``            -- declared on the owner with that exact descriptor;
* ``inherited_exact``  -- declared with that descriptor on a superclass or an
                          interface reached by walking the hierarchy (cycle-safe);
* ``name_only``        -- no descriptor was given and exactly one member of that
                          name exists in the walk (the descriptor is filled in);
* ``ambiguous``        -- no descriptor was given and several members share the
                          name (candidates listed);
* ``unresolved``       -- the owner exists but no such member, on it or above it;
* ``owner_missing``    -- the owner class is in none of the jars;
* ``selector_unsupported`` -- a wildcard, quantified or regex selector: can match
                          many members, so it is never resolved exactly.

Constructors are never inherited: ``<init>`` resolves only on the owner itself.
``<clinit>`` resolves iff the owner declares a static initialiser.
"""
from __future__ import annotations

import re
import zipfile
from collections import deque
from pathlib import Path

from classfile import ClassFile, ClassFileError, read_class

# Lowner;name(args)ret   |   Lowner;name:desc   |   owner.name(args)ret
_TARGET = re.compile(r"^(?:L(?P<owner>[^;]+);|(?P<downer>[\w$/.]+)\.)?(?P<name>[^\s(:]+)(?:(?P<mdesc>\(.*\).+)|:(?P<fdesc>.+))?$")


def parse_target(target: str) -> dict | None:
    """An @At target string -> {owner, name, desc, kind: method|field}."""
    m = _TARGET.match(target.strip())
    if not m:
        return None
    owner = m.group("owner") or (m.group("downer").replace(".", "/") if m.group("downer") else None)
    desc = m.group("mdesc") or m.group("fdesc")
    kind = "field" if m.group("fdesc") else ("method" if m.group("mdesc") else "unknown")
    return {"owner": owner, "name": m.group("name"), "desc": desc, "kind": kind}


class Resolver:
    def __init__(self, jars: list[Path], jdk_jmods: Path | None = None) -> None:
        """``jars`` are searched in order; ``jdk_jmods`` (a JDK's jmods/ dir) supplies java.* types last."""
        self.zips = [zipfile.ZipFile(j) for j in jars]
        self.prefixes = [""] * len(jars)
        self.jar_names = [str(j) for j in jars]
        self.dirs: list[Path] = []
        if jdk_jmods and jdk_jmods.exists():
            if jdk_jmods.is_dir() and any(jdk_jmods.glob("*.jmod")):
                for jm in sorted(jdk_jmods.glob("*.jmod")):
                    self.zips.append(zipfile.ZipFile(jm))
                    self.prefixes.append("classes/")
                    self.jar_names.append(str(jm))
            else:
                # a directory of extracted classes (``jimage extract``), e.g. <dir>/java.base/java/lang/Object.class
                self.dirs = [jdk_jmods] + [d for d in sorted(jdk_jmods.iterdir()) if d.is_dir()]
        self.cache: dict[str, ClassFile | None] = {}
        self.where: dict[str, int] = {}

    def close(self) -> None:
        for z in self.zips:
            z.close()

    def cls(self, internal: str) -> ClassFile | None:
        if internal in self.cache:
            return self.cache[internal]
        for i, z in enumerate(self.zips):
            entry = self.prefixes[i] + internal + ".class"
            try:
                data = z.read(entry)
            except KeyError:
                continue
            try:
                cf = read_class(data)
            except (ClassFileError, Exception) as exc:  # noqa: BLE001 - recorded, never raised
                self.cache[internal] = None
                self.where[internal] = i
                return None
            self.cache[internal] = cf
            self.where[internal] = i
            return cf
        for d in self.dirs:
            f = d / (internal + ".class")
            if f.exists():
                try:
                    cf = read_class(f.read_bytes())
                except (ClassFileError, Exception):  # noqa: BLE001
                    cf = None
                self.cache[internal] = cf
                return cf
        self.cache[internal] = None
        return None

    def hierarchy(self, internal: str, limit: int = 200) -> list[str]:
        """Owner first, then superclasses and interfaces breadth-first, each once."""
        out, seen, queue = [], set(), deque([internal])
        while queue and len(out) < limit:
            cur = queue.popleft()
            if cur in seen:
                continue
            seen.add(cur)
            cf = self.cls(cur)
            if cf is None:
                continue
            out.append(cur)
            if cf.super_name:
                queue.append(cf.super_name)
            queue.extend(cf.interfaces)
        return out

    def _members(self, cf: ClassFile, kind: str):
        return cf.methods if kind == "method" else cf.fields

    def resolve(self, owner: str, name: str, desc: str | None, kind: str = "method") -> dict:
        res = {"owner": owner, "name": name, "desc": desc, "kind": kind, "state": "unresolved",
               "declared_in": None, "candidates": []}
        top = self.cls(owner)
        if top is None:
            res["state"] = "owner_missing"
            return res
        if name == "<clinit>":
            if any(m.name == "<clinit>" for m in top.methods):
                res["state"], res["declared_in"], res["desc"] = "exact", owner, "()V"
            return res
        if name == "<init>":
            cands = [m for m in top.methods if m.name == name]
            if desc:
                if any(m.desc == desc for m in cands):
                    res["state"], res["declared_in"] = "exact", owner
            elif len(cands) == 1:
                res["state"], res["declared_in"], res["desc"] = "name_only", owner, cands[0].desc
            elif cands:
                res["state"], res["candidates"] = "ambiguous", [m.desc for m in cands]
            return res
        walk = self.hierarchy(owner)
        res["walk_depth"] = len(walk)
        if desc:
            for cur in walk:
                cf = self.cls(cur)
                if cf and any(m.name == name and m.desc == desc for m in self._members(cf, kind)):
                    res["state"] = "exact" if cur == owner else "inherited_exact"
                    res["declared_in"] = cur
                    return res
            return res
        found = []
        for cur in walk:
            cf = self.cls(cur)
            if cf:
                found += [(cur, m.desc) for m in self._members(cf, kind) if m.name == name]
        # the same (name, desc) declared on several levels is one member (an override)
        uniq: dict[str, str] = {}
        for cur, d in found:
            uniq.setdefault(d, cur)
        if len(uniq) == 1:
            d, cur = next(iter(uniq.items()))
            res["state"], res["declared_in"], res["desc"] = "name_only", cur, d
        elif uniq:
            res["state"], res["candidates"] = "ambiguous", sorted(uniq)
        return res

    def resolve_selector(self, selector: dict, default_owner: str) -> dict:
        if selector.get("unsupported"):
            return {"state": "selector_unsupported", "reason": selector["unsupported"], "owner": default_owner,
                    "name": selector.get("name"), "desc": selector.get("desc"), "kind": "method",
                    "declared_in": None, "candidates": []}
        owner = selector.get("owner") or default_owner
        if selector["kind"] == "desc":
            desc = None
            if selector.get("args") is not None and selector.get("ret") is not None:
                desc = "(" + "".join(_to_desc(a) for a in selector["args"]) + ")" + _to_desc(selector["ret"])
            return self.resolve(owner, selector["name"], desc, "method")
        return self.resolve(owner, selector["name"], selector.get("desc"), "method")

    def resolve_target(self, target: str, default_owner: str) -> dict:
        t = parse_target(target)
        if not t:
            return {"state": "selector_unsupported", "reason": "target string did not parse", "owner": default_owner,
                    "name": None, "desc": None, "kind": "unknown", "declared_in": None, "candidates": []}
        kind = t["kind"] if t["kind"] != "unknown" else "method"
        return self.resolve(t["owner"] or default_owner, t["name"], t["desc"], kind)


def _to_desc(t: str) -> str:
    """A @Desc class value ('I', 'foo/Bar', '[Lfoo/Bar;') -> a descriptor."""
    if len(t) == 1 or t.startswith("[") or (t.startswith("L") and t.endswith(";")):
        return t
    return "L" + t + ";"

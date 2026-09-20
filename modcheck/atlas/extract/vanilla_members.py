"""Extract the members of every vanilla type that Fabric API hooks.

Closes the `incomplete_extraction` gap left by the first pass: the edge graph
names vanilla methods as injection targets, but the vanilla side of each edge
was only a name. This reads the declared members (all visibilities, `javap -p`)
of each hooked type from the merged 26.3 jar and cross-checks every edge target
against them, so a target that does not exist in the jar is reported instead of
assumed.

Output: atlas/extracted/minecraft_members.json
"""
from __future__ import annotations

import json
import re
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXTRACTED = ROOT / "extracted"
JAVAP = ROOT.parents[0] / "toolchains" / "jdk-25.0.4.1+1" / "bin" / "javap"
HEADER_RE = re.compile(r"\b(class|interface|enum|record|@interface)\s+([\w.$]+)")


def javap_many(jar: Path, fqcns: list[str]) -> str:
    proc = subprocess.run([str(JAVAP), "-p", "-cp", str(jar), *fqcns], capture_output=True, text=True,
                          env={"PATH": "/usr/bin:/bin", "JAVA_TOOL_OPTIONS": ""}, timeout=600)
    return proc.stdout


def parse_blocks(text: str) -> dict[str, dict]:
    out: dict[str, dict] = {}
    current = None
    for line in text.splitlines():
        if not line.startswith(" ") and line.rstrip().endswith("{"):
            head = line.rstrip()[:-1].strip()
            m = HEADER_RE.search(head)
            if not m:
                continue
            kind = m.group(1)
            if kind == "class" and "abstract" in head.split(m.group(2))[0]:
                kind = "abstract_class"
            current = {"kind": kind, "header": head, "members": []}
            out[m.group(2)] = current
        elif current is not None and line.startswith(" ") and line.strip().endswith(";"):
            current["members"].append(line.strip()[:-1])
        elif line.startswith("}"):
            current = None
    return out


def member_names(members: list[str]) -> set[str]:
    names = set()
    for mem in members:
        # method: "... name(args)"  field: "... type name"
        m = re.search(r"([\w$<>]+)\(", mem)
        if m:
            names.add(m.group(1))
        else:
            names.add(mem.split()[-1])
    return names


def main() -> int:
    corpus = json.loads((EXTRACTED / "corpus.json").read_text())
    edges = json.loads((EXTRACTED / "edges.json").read_text())
    jar_rec = next(a for a in corpus["artifacts"] if a["name"] == "minecraft-merged")
    jar = Path(jar_rec["path"])
    targets: dict[str, set[str]] = {}
    for e in edges["edges"]:
        to = e["to"]
        if to["kind"] == "registry":
            continue
        owner = to.get("owner") or (to["id"] if to["kind"] == "type" else None)
        if owner and owner.startswith(("net.minecraft.", "com.mojang.")):
            targets.setdefault(owner, set())
            if to["kind"] in ("method", "field"):
                targets[owner].add(to["id"])
    fqcns = sorted(targets)
    t0 = time.time()
    types: dict[str, dict] = {}
    for i in range(0, len(fqcns), 60):
        batch = fqcns[i:i + 60]
        types.update(parse_blocks(javap_many(jar, batch)))
        print(f"  {min(i + 60, len(fqcns))}/{len(fqcns)} types, {time.time() - t0:.0f}s", flush=True)
    missing = [f for f in fqcns if f not in types]
    # cross-check every edge target against the extracted member names. A name
    # not declared on the type itself may be inherited: walk `extends` chains
    # (javap headers) up to six levels, extracting supertypes on demand.
    supers: dict[str, dict] = {}

    def parent_of(fqcn: str) -> str | None:
        blk = types.get(fqcn) or supers.get(fqcn)
        if blk is None:
            got = parse_blocks(javap_many(jar, [fqcn]))
            if fqcn not in got:
                return None
            supers[fqcn] = blk = got[fqcn]
        m = re.search(r"\bextends\s+([\w.$]+)", blk["header"])
        return m.group(1) if m and m.group(1) != "java.lang.Object" else None

    def declared_in(owner: str, base: str) -> str | None:
        cur, depth = owner, 0
        while cur and depth < 6:
            blk = types.get(cur) or supers.get(cur)
            if blk is None and parent_of(cur) is None and cur not in supers:
                return None
            blk = types.get(cur) or supers.get(cur)
            if blk and base in member_names(blk["members"]):
                return cur
            cur, depth = parent_of(cur), depth + 1
        return None

    resolved, inherited, unresolved = 0, [], []
    for owner, names in sorted(targets.items()):
        for n in sorted(names):
            base = n.split("(")[0].strip('"')
            if base in ("<init>", "<clinit>"):
                resolved += 1
                continue
            where = declared_in(owner, base)
            if where == owner:
                resolved += 1
            elif where:
                inherited.append(f"{owner}.{base} <- {where}")
            else:
                unresolved.append(f"{owner}.{n}")
    out = {
        "generated_at": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "artifact": jar_rec["name"], "path": str(jar), "sha256": jar_rec["sha256"],
        "method": "javap -p on every vanilla type that is an endpoint of an edge in edges.json; "
                  "member names cross-checked against edge targets",
        "types_requested": len(fqcns), "types_extracted": len(types), "types_missing": missing,
        "edge_targets": {"resolved_on_type": resolved, "resolved_by_inheritance": inherited,
                         "unresolved": sorted(unresolved)},
        "supertypes_consulted": sorted(supers),
        "types": {k: v for k, v in types.items() if k in targets},
    }
    (EXTRACTED / "minecraft_members.json").write_text(json.dumps(out, indent=1))
    print(f"wrote minecraft_members.json: {len(types)}/{len(fqcns)} types, targets on type {resolved}, "
          f"inherited {len(inherited)}, unresolved {len(unresolved)}, missing types {len(missing)} "
          f"in {time.time() - t0:.0f}s")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

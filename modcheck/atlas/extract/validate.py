"""Validate the atlas: schemas, provenance, references, wikilinks, certainty.

Exit status 1 on any error. Warnings are printed but do not fail. Run after
``vault.py``; the data checks also work before the vault exists.

What is checked
* every edge in ``extracted/edges.json`` validates against ``schema/edge.schema.json``,
  cites an artifact sha256 that is in ``extracted/corpus.json``, and carries an
  evidence class that its provenance can support (``observed`` needs runtime
  evidence, and there is none);
* every authored workflow / question / system / request / contract validates
  against its schema, has a unique id, and every typed reference it makes
  (``mechanism:``, ``event:``, ``type:``, ``source:``, ``record:``,
  ``extracted:``, ``question:``, ``workflow:``, ``request:``) resolves to
  something that exists in the store or the extracted facts;
* every ``[[wikilink]]`` in the vault resolves to a note (and a heading, if one
  is named); every note starts with frontmatter followed by exactly one of the
  two banners;
* no authored or generated text asserts runtime certainty ("game-tested",
  "observed at runtime", ...) without a negation, and none states a numeric
  confidence.
"""
from __future__ import annotations

import collections
import json
import re
import sys
from pathlib import Path

import jsonschema

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT.parents[0] / "src"))
from modcheck import yamlio  # noqa: E402
from modcheck.store import Store  # noqa: E402

EXTRACTED = ROOT / "extracted"
SCHEMA = ROOT / "schema"
AUTHORED = ROOT / "authored"
VAULT = ROOT / "vault"

EVIDENCE_CLASSES = {"direct_reference", "static_inference", "declared", "documented", "observed"}
REF_RE = re.compile(r"\b(mechanism|question|workflow|event|type|source|extracted|record|request):([A-Za-z0-9_/#$-]+(?:\.[A-Za-z0-9_/#$-]+)*)")
WIKI_RE = re.compile(r"\[\[([^\]|#]+)(#[^\]|]+)?(\|[^\]]*)?\]\]")
CERTAINTY_RE = re.compile(r"(game[- ]tested|verified in[- ]game|observed (at|in) runtime|confirmed (at|in) (runtime|the game)|runtime[- ]verified|tested in[- ]game)", re.I)
NEGATION_RE = re.compile(r"\b(not|no|never|nothing|none|would|needs|unavailable|untested|cannot|has not|is not|until|before)\b", re.I)
NUMERIC_CONFIDENCE_RE = re.compile(r"(\b\d{1,3}\s?%\s?(sure|confident|confidence|certain)|\bconfidence\s*[:=]\s*\d|\bp\s*=\s*0\.\d)", re.I)


class Report:
    def __init__(self) -> None:
        self.errors: list[str] = []
        self.warnings: list[str] = []
        self.counts: dict[str, int] = collections.Counter()

    def err(self, msg: str) -> None:
        self.errors.append(msg)

    def warn(self, msg: str) -> None:
        self.warnings.append(msg)


def _load_schema(name: str):
    return jsonschema.Draft202012Validator(json.loads((SCHEMA / f"{name}.schema.json").read_text()))


def _authored(kind: str) -> list[tuple[Path, dict]]:
    out = []
    for path in sorted((AUTHORED / kind).glob("*.yaml")) if (AUTHORED / kind).exists() else []:
        data = yamlio.load_path(path)
        for entry in (data if isinstance(data, list) else [data]):
            out.append((path, entry))
    return out


def _strings(obj):
    if isinstance(obj, str):
        yield obj
    elif isinstance(obj, dict):
        for v in obj.values():
            yield from _strings(v)
    elif isinstance(obj, list):
        for v in obj:
            yield from _strings(v)


class Universe:
    """Everything a typed reference may resolve to."""

    def __init__(self) -> None:
        self.corpus = json.loads((EXTRACTED / "corpus.json").read_text()) if (EXTRACTED / "corpus.json").exists() else None
        self.fabric = json.loads((EXTRACTED / "fabric_api.json").read_text()) if (EXTRACTED / "fabric_api.json").exists() else None
        self.edges = json.loads((EXTRACTED / "edges.json").read_text()) if (EXTRACTED / "edges.json").exists() else None
        self.members = json.loads((EXTRACTED / "minecraft_members.json").read_text()) if (EXTRACTED / "minecraft_members.json").exists() else None
        self.store = Store()
        self.pack = self.store.pack("minecraft")
        src = self.pack.sources
        self.sources = set((src() if callable(src) else src).keys())
        self.modules = {m["id"] for m in self.fabric["modules"]} if self.fabric else set()
        self.mechanisms = self.modules | {"Registries", "Mixin"}
        self.events: set[str] = set()
        self.types: set[str] = set()
        if self.edges:
            for e in self.edges["edges"]:
                if e["to"]["kind"] == "event":
                    self.events.add(e["to"]["id"])
                owner = e["to"].get("owner") or (e["to"]["id"] if e["to"]["kind"] == "type" else None)
                if owner and e["to"]["kind"] != "registry":
                    self.types.add(owner)
        if self.fabric:
            for m in self.fabric["modules"]:
                self.types.update(a["fqcn"] for a in m["api"] + m["api_nested"])
            self.types.update(a["fqcn"] for a in self.fabric["loader"]["api"])
        self.artifact_hashes = {a["sha256"] for a in self.corpus["artifacts"]} if self.corpus else set()
        self.packages = (set(self.corpus["minecraft"]["classes_by_package_depth4"])
                         | set(self.corpus["minecraft"]["classes_by_package_depth3"])) if self.corpus else set()
        self.ids: dict[str, set[str]] = {k: set() for k in ("workflow", "question", "request", "system", "contract")}

    def resolve(self, kind: str, ident: str) -> str | None:
        """Return an error string, or None if the reference resolves."""
        if kind == "mechanism":
            return None if ident in self.mechanisms else f"unknown mechanism {ident}"
        if kind in ("question", "workflow", "request"):
            return None if ident in self.ids[kind] else f"unknown {kind} {ident}"
        if kind == "event":
            if not self.edges:
                return None
            return None if ident in self.events else f"unknown event {ident} (not in edges.json)"
        if kind == "type":
            if not self.edges and not self.fabric:
                return None
            return None if ident in self.types else f"type {ident} has no interface note (not a hooked vanilla type or API class)"
        if kind == "source":
            return None if ident in self.sources else f"unknown source {ident}"
        if kind == "extracted":
            path, _, frag = ident.partition("#")
            if not (EXTRACTED / path).exists():
                return f"extracted file {path} does not exist"
            if frag and path == "fabric_api.json" and self.fabric and frag not in self.modules:
                return f"fabric_api.json has no module {frag}"
            if frag and path == "edges.json" and self.edges and not (frag in self.edges["counts"] or re.match(r"^e\d{6}$", frag)):
                return f"edges.json fragment {frag} is neither a relation nor an edge id"
            return None
        if kind == "record":
            rk, _, rid = ident.partition("/")
            return None if self.pack.record(rk, rid) is not None else f"unknown store record {ident}"
        return f"unknown reference kind {kind}"


def check_edges(u: Universe, rep: Report) -> None:
    if not u.edges:
        rep.warn("edges.json missing; skipping edge checks")
        return
    validator = _load_schema("edge")
    seen = set()
    for e in u.edges["edges"]:
        rep.counts["edges"] += 1
        for err in validator.iter_errors(e):
            rep.err(f"edge {e.get('id')}: schema: {err.message[:160]}")
        if e["id"] in seen:
            rep.err(f"edge {e['id']}: duplicate id")
        seen.add(e["id"])
        prov = e.get("provenance", {})
        if prov.get("artifact_sha256") not in u.artifact_hashes:
            rep.err(f"edge {e['id']}: provenance sha256 {prov.get('artifact_sha256', '?')[:12]} is not a corpus artifact")
        if e.get("evidence_class") not in EVIDENCE_CLASSES:
            rep.err(f"edge {e['id']}: evidence class {e.get('evidence_class')}")
        if e.get("evidence_class") == "observed" and not str(prov.get("method", "")).startswith("runtime:"):
            rep.err(f"edge {e['id']}: 'observed' without runtime provenance")
        if not e.get("applies_to", {}).get("game_versions"):
            rep.err(f"edge {e['id']}: applies_to.game_versions missing")
    rep.counts["edges_observed"] = u.edges["evidence_classes"].get("observed", 0)


def check_authored(u: Universe, rep: Report) -> list[tuple[str, Path, dict]]:
    kinds = (("workflow", "workflows"), ("question", "questions"), ("system", "systems"),
             ("request", "requests"), ("contract", "contracts"))
    loaded: list[tuple[str, Path, dict]] = []
    for kind, folder in kinds:
        validator = _load_schema(kind)
        for path, entry in _authored(folder):
            loaded.append((kind, path, entry))
            rep.counts[f"authored_{kind}"] += 1
            for err in validator.iter_errors(entry):
                rep.err(f"{path.name} ({entry.get('id', '?')}): schema: {err.message[:200]}")
            ident = entry.get("id")
            if ident in u.ids[kind]:
                rep.err(f"{path.name}: duplicate {kind} id {ident}")
            u.ids[kind].add(ident)
            if kind == "system" and u.packages and ident not in u.packages:
                rep.err(f"{path.name}: system {ident} is not a depth-4 package in corpus.json")
    # second pass: references (ids are now all known)
    for kind, path, entry in loaded:
        for text in _strings(entry):
            for m in REF_RE.finditer(text):
                rep.counts["refs"] += 1
                problem = u.resolve(m.group(1), m.group(2))
                if problem:
                    rep.err(f"{path.name} ({entry.get('id')}): {problem}")
            if CERTAINTY_RE.search(text) and not NEGATION_RE.search(text):
                rep.err(f"{path.name} ({entry.get('id')}): asserts runtime certainty without evidence: {text[:100]!r}")
            if NUMERIC_CONFIDENCE_RE.search(text):
                rep.err(f"{path.name} ({entry.get('id')}): numeric confidence: {text[:100]!r}")
        if kind == "contract":
            for a in entry.get("asserted", []):
                if a["basis"] == "observed":
                    rep.err(f"{path.name}: contract asserts observed basis")
        if kind == "workflow":
            s = entry["status"]
            if s.get("implemented_in_modcheck") and not any(t.startswith("record:") for t in entry.get("evidence", [])):
                rep.err(f"{path.name} ({entry['id']}): implemented_in_modcheck without a record: citation")
        if kind == "question" and entry.get("status") == "resolved" and not entry.get("resolved_by"):
            rep.err(f"{path.name} ({entry['id']}): resolved without a resolved_by field")
    # authored markdown
    for path in AUTHORED.rglob("*.md"):
        text = path.read_text()
        for line in text.splitlines():
            for m in REF_RE.finditer(line):
                rep.counts["refs"] += 1
                problem = u.resolve(m.group(1), m.group(2))
                if problem:
                    rep.err(f"{path.relative_to(ROOT)}: {problem}")
            if CERTAINTY_RE.search(line) and not NEGATION_RE.search(line):
                rep.err(f"{path.relative_to(ROOT)}: asserts runtime certainty: {line[:100]!r}")
            if NUMERIC_CONFIDENCE_RE.search(line):
                rep.err(f"{path.relative_to(ROOT)}: numeric confidence: {line[:100]!r}")
    return loaded


def check_vault(rep: Report) -> None:
    if not VAULT.exists():
        rep.warn("vault not built; skipping wikilink checks")
        return
    notes = {p.relative_to(VAULT).with_suffix("").as_posix(): p for p in VAULT.rglob("*.md")}
    rep.counts["notes"] = len(notes)
    headings: dict[str, set[str]] = {}
    for rel, path in notes.items():
        text = path.read_text()
        if not text.startswith("---\n"):
            rep.err(f"{rel}: no frontmatter")
        body = text.split("\n---\n", 1)[1] if "\n---\n" in text else text
        gen, auth = "> [!info] Generated" in body[:400], "> [!warning] Analyst-authored" in body[:400]
        if gen == auth:
            rep.err(f"{rel}: must carry exactly one banner (generated={gen}, authored={auth})")
        rep.counts["notes_generated" if gen else "notes_authored"] += 1
        headings[rel] = {h.strip().lower() for h in re.findall(r"^#+\s+(.*)$", body, re.M)}
    for rel, path in notes.items():
        text = path.read_text()
        for m in WIKI_RE.finditer(text):
            rep.counts["wikilinks"] += 1
            target = m.group(1).strip()
            if target not in notes:
                rep.err(f"{rel}: dangling link [[{target}]]")
                continue
            if m.group(2):
                h = m.group(2)[1:].strip().lower()
                if h not in headings.get(target, set()):
                    rep.err(f"{rel}: link [[{target}#{m.group(2)[1:]}]] names a heading that does not exist")
        for line in text.splitlines():
            if CERTAINTY_RE.search(line) and not NEGATION_RE.search(line):
                rep.err(f"{rel}: asserts runtime certainty: {line[:100]!r}")
            if NUMERIC_CONFIDENCE_RE.search(line):
                rep.err(f"{rel}: numeric confidence: {line[:100]!r}")


def run() -> Report:
    rep = Report()
    u = Universe()
    check_edges(u, rep)
    check_authored(u, rep)
    check_vault(rep)
    return rep


def main(argv: list[str]) -> int:
    rep = run()
    if "--json" in argv:
        print(json.dumps({"errors": rep.errors, "warnings": rep.warnings, "counts": dict(rep.counts)}, indent=1))
    else:
        for w in rep.warnings:
            print(f"warning: {w}")
        for e in rep.errors:
            print(f"ERROR: {e}")
        print("counts:", json.dumps(dict(rep.counts)))
        print(f"{len(rep.errors)} errors, {len(rep.warnings)} warnings")
    return 1 if rep.errors else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))

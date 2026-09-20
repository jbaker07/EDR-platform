"""Generate the Obsidian-compatible vault from records and extracted facts.

Everything under ``atlas/vault/`` except ``_authored/`` is generated: delete the
directory and this rebuilds it. Notes carry stable ids in their filenames and
link back to the record or extracted entry they came from. Authored analyst
material lives in ``atlas/authored/`` and is rendered into ``_authored/`` with a
visible banner; a generated note may LINK to an authored one, never absorb it.

Evidence labels are rendered exactly as recorded. Nothing here promotes a
static_inference to a direct_reference, or a documented claim to an observed
one.
"""
from __future__ import annotations

import collections
import json
import re
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT.parents[0] / "src"))
from modcheck import yamlio  # noqa: E402
from modcheck.store import Store  # noqa: E402

EXTRACTED = ROOT / "extracted"
# A typed reference: dot-separated segments, never ending in a dot (so a sentence-final
# period is not swallowed). Shared with validate.py.
REF_PATTERN = r"\b(mechanism|question|workflow|event|type|source|extracted|record|request):([A-Za-z0-9_/#$-]+(?:\.[A-Za-z0-9_/#$-]+)*)"
VAULT = ROOT / "vault"
AUTHORED = ROOT / "authored"

GENERATED = ("> [!info] Generated\n> Built by `atlas/extract/vault.py` from the records and "
             "extracted facts it links to. Do not edit; edit the source and regenerate.\n\n")
AUTHORED_BANNER = ("> [!warning] Analyst-authored\n> Interpretation, not extraction. Claims "
                   "cite evidence ids; anything uncited is opinion. Reviewed corrections go "
                   "into the store records or the extractors, never into a generated note.\n\n")


def slug(text: str) -> str:
    return re.sub(r"[^A-Za-z0-9._-]+", "_", text).strip("_")[:120]


def link(target: str, label: str | None = None) -> str:
    return f"[[{target}|{label}]]" if label else f"[[{target}]]"


def write(path: Path, body: str, generated: bool = True) -> None:
    """Write a note: frontmatter first (Obsidian requires it at byte 0), then the banner."""
    path.parent.mkdir(parents=True, exist_ok=True)
    banner = GENERATED if generated else AUTHORED_BANNER
    if body.startswith("---\n") and "\n---\n" in body[4:]:
        head, _, rest = body[4:].partition("\n---\n")
        body = "---\n" + head + "\n---\n\n" + banner + rest.lstrip("\n")
    else:
        body = banner + body
    path.write_text(body)


def fm(**fields) -> str:
    """YAML frontmatter. Every scalar is JSON-encoded, which is valid YAML and never ambiguous."""
    lines = ["---"]
    for k, v in fields.items():
        if isinstance(v, (list, tuple)):
            lines.append(f"{k}:")
            lines += [f"  - {json.dumps(x)}" for x in v]
        else:
            lines.append(f"{k}: {json.dumps(v)}")
    lines.append("---")
    return "\n".join(lines) + "\n\n"


def pkg_of(fqcn: str) -> str:
    """Depth-4 package of a class (`net.minecraft.world.level` for `...level.Level$1`)."""
    return ".".join(fqcn.split(".")[:-1][:4])


# ---------------------------------------------------------------------------
class Atlas:
    def __init__(self) -> None:
        self.corpus = json.loads((EXTRACTED / "corpus.json").read_text())
        self.fabric = json.loads((EXTRACTED / "fabric_api.json").read_text())
        self.registries = json.loads((EXTRACTED / "minecraft_registries.json").read_text())
        self.edges = json.loads((EXTRACTED / "edges.json").read_text())
        mpath = EXTRACTED / "minecraft_members.json"
        self.members = json.loads(mpath.read_text()) if mpath.exists() else None
        self.store = Store()
        self.pack = self.store.pack("minecraft")
        self.workflows = self._load_authored("workflows")
        self.questions = self._load_authored("questions")
        self.systems = self._load_authored("systems")
        self.requests = self._load_authored("requests")
        self.contracts = self._load_authored("contracts")
        self.written: list[Path] = []

    def _load_authored(self, kind: str) -> list[dict]:
        out = []
        for path in sorted((AUTHORED / kind).glob("*.yaml")):
            data = yamlio.load_path(path)
            for entry in (data if isinstance(data, list) else [data]):
                entry["_path"] = str(path.relative_to(ROOT))
                out.append(entry)
        return out

    def put(self, rel: str, body: str, generated: bool = True) -> str:
        path = VAULT / rel
        write(path, body, generated)
        self.written.append(path)
        return rel[:-3] if rel.endswith(".md") else rel

    # -- indexes ------------------------------------------------------------
    def edges_by(self, key: str) -> dict[str, list[dict]]:
        idx: dict[str, list[dict]] = collections.defaultdict(list)
        for e in self.edges["edges"]:
            idx[e.get(key)].append(e)
        return idx

    def vanilla_targets(self) -> dict[str, list[dict]]:
        """vanilla type -> edges whose `to` endpoint lives in that type."""
        idx: dict[str, list[dict]] = collections.defaultdict(list)
        for e in self.edges["edges"]:
            to = e["to"]
            if to["kind"] == "registry":
                continue
            owner = to.get("owner") or (to["id"] if to["kind"] == "type" else None)
            if owner and owner.startswith(("net.minecraft.", "com.mojang.")):
                idx[owner].append(e)
        return idx

    def events(self) -> dict[str, dict]:
        """event id -> {callback, publishers[], module}."""
        ev: dict[str, dict] = {}
        for e in self.edges["edges"]:
            if e["relation"] == "callback_of":
                ev.setdefault(e["to"]["id"], {"callback": None, "publishers": [], "module": e.get("module")})
                ev[e["to"]["id"]]["callback"] = e["from"]["id"]
            elif e["relation"] == "publishes_event":
                ev.setdefault(e["to"]["id"], {"callback": None, "publishers": [], "module": e.get("module")})
                ev[e["to"]["id"]]["publishers"].append(e)
        return ev

    # -- generation ---------------------------------------------------------
    def build(self) -> None:
        for child in VAULT.iterdir() if VAULT.exists() else []:
            if child.name != "_authored":
                shutil.rmtree(child) if child.is_dir() else child.unlink()
        self.scope()
        self.systems_notes()
        self.mechanisms()
        self.interfaces()
        self.interactions()
        self.failures()
        self.workflow_notes()
        self.request_notes()
        self.question_notes()
        self.coverage()
        self.index()

    def scope(self) -> None:
        c = self.corpus
        mc = c["minecraft"]
        v = mc["version_json"]
        rows = ["| group | artifact | version | classes (top/nested) | sha256 |", "|---|---|---|---|---|"]
        for a in sorted(c["artifacts"], key=lambda a: (a["group"], a["name"])):
            rows.append(f"| {a['group']} | {a['name']} | {a['version']} | "
                        f"{a['classes_top_level']}/{a['classes_nested']} | `{a['sha256'][:16]}` |")
        body = fm(type="scope", generated_at=c["generated_at"]) + "# Corpus: what was actually resolved\n\n"
        body += ("This is the concrete input set. Every fact in the atlas that is marked "
                 "`direct_reference` or `static_inference` was read from one of these files, "
                 "identified by hash. Anything not listed here is outside the corpus, and the "
                 f"atlas says so in {link('00-Scope/Branches', 'Branches')}.\n\n")
        body += f"**Source of truth:** {c['source_of_truth']}\n\n"
        body += ("## Minecraft Java Edition 26.3 -- pinned identity\n\n"
                 f"- version id `{v['id']}`, world version {v['world_version']}, protocol {v['protocol_version']}\n"
                 f"- Java {v['java_version']}\n"
                 f"- resource pack format {v['pack_version']['resource_major']}.{v['pack_version']['resource_minor']}, "
                 f"data pack format {v['pack_version']['data_major']}.{v['pack_version']['data_minor']}\n"
                 f"- stable: {v['stable']}\n\n")
        body += f"## Artifacts ({len(c['artifacts'])})\n\n" + "\n".join(rows) + "\n\n"
        body += "## Groups\n\n" + "\n".join(f"- **{g}**: {n}" for g, n in sorted(c["groups"].items())) + "\n"
        self.put("00-Scope/Corpus.md", body)

        # Branches: the parts of "Minecraft modding" this corpus does NOT contain.
        self.put("00-Scope/Branches.md", fm(type="scope") + (Path(AUTHORED / "branches.md").read_text()
                 if (AUTHORED / "branches.md").exists() else "# Branches\n\n(no authored branch inventory yet)\n"), generated=False)

        # Data-driven surface from the jar itself.
        body = fm(type="scope") + "# Data-driven surface shipped in the 26.3 jar\n\n"
        body += ("Everything under `data/minecraft/<type>/` is a data pack the game ships with "
                 "itself. Each type is a modification mechanism that needs no Java: a data pack "
                 "adding or overriding entries here changes the game. Counts are the vanilla "
                 "entries, read from the jar.\n\n| type | vanilla entries |\n|---|---|\n")
        body += "\n".join(f"| `{t}` | {n} |" for t, n in mc["data_entries_by_type"].items())
        body += "\n\n## Assets (resource pack) shipped in the jar\n\n| type | entries |\n|---|---|\n"
        body += "\n".join(f"| `{t}` | {n} |" for t, n in mc["asset_entries_by_type"].items())
        self.put("00-Scope/Data_Driven_Surface.md", body)

        # Sources coverage
        body = fm(type="scope") + "# Source coverage (minecraft pack)\n\n"
        body += "| id | kind | cached | licence |\n|---|---|---|---|\n"
        for sid, s in sorted(self.pack.sources.items()):
            body += f"| `{sid}` | {s.get('kind')} | {'yes' if s.get('evidence_path') else 'no'} | {s.get('license', '?')} |\n"
        self.put("00-Scope/Sources.md", body)

    def systems_notes(self) -> None:
        mc = self.corpus["minecraft"]
        targets = self.vanilla_targets()
        by_pkg: dict[str, list[str]] = collections.defaultdict(list)
        for t in targets:
            by_pkg[pkg_of(t)].append(t)
        authored = {s["id"]: s for s in self.systems}
        body = fm(type="index") + "# Game systems (by package)\n\n"
        body += ("A package is a system boundary the game's own authors drew. The `hooked` column "
                 "counts vanilla types in that package that at least one Fabric API module "
                 "injects into, replaces, wraps, calls or reads -- the modification surface "
                 "Fabric itself uses. A package with zero hooks is not untouchable; it is one "
                 "no shipped Fabric module touches.\n\n| package | classes | hooked types | note |\n|---|---|---|---|\n")
        all_pkgs = dict(mc["classes_by_package_depth4"])
        for pkg in set(by_pkg) | set(authored):
            all_pkgs.setdefault(pkg, mc["classes_by_package_depth3"].get(pkg, "?"))
        for pkg, n in all_pkgs.items():
            hooked = len(by_pkg.get(pkg, []))
            note_link = link(f"20-Systems/{slug(pkg)}", "note") if hooked or pkg in authored else ""
            body += f"| `{pkg}` | {n} | {hooked} | {note_link} |\n"
        self.put("20-Systems/_index.md", body)

        for sysrec in self.systems:
            b = fm(type="system_note", id=sysrec["id"], side=sysrec["side"]) + f"# {sysrec['title']}\n\n"
            b += f"Package `{sysrec['id']}` -- generated view: {link('20-Systems/' + slug(sysrec['id']), 'hooked types')}\n\n"
            b += f"**Responsibility.** {sysrec['responsibility'].strip()}\n\n**Side.** {sysrec['side']}\n\n"
            b += f"**Threads.** {self._render_ref(sysrec['threads'].strip())}\n\n"
            if sysrec.get("persistence"):
                b += f"**Persistence.** {self._render_ref(sysrec['persistence'].strip())}\n\n"
            for key, title in (("extension_points", "Extension points"), ("interactions", "Interactions to expect"),
                               ("evidence", "Evidence"), ("questions", "Open questions")):
                if sysrec.get(key):
                    b += f"## {title}\n\n" + "\n".join(f"- {self._render_ref(i)}" for i in sysrec[key]) + "\n\n"
            self.put(f"_authored/systems/{slug(sysrec['id'])}.md", b, generated=False)

        for pkg in sorted(set(by_pkg) | set(authored)):
            types = by_pkg.get(pkg, [])
            b = fm(type="system", package=pkg) + f"# {pkg}\n\n"
            if pkg in authored:
                b += f"Analyst note: {link('_authored/systems/' + slug(pkg), authored[pkg]['title'])}\n\n"
            b += f"{all_pkgs.get(pkg, '?')} classes in the jar. Hooked types: {len(types)}\n\n"
            for t in sorted(types):
                rel = collections.Counter(e["relation"] for e in targets[t])
                mods = sorted({e.get("module") for e in targets[t] if e.get("module")})
                b += f"- {link('40-Interfaces/' + slug(t), t.split('.')[-1])} -- " + ", ".join(
                    f"{k}:{v}" for k, v in sorted(rel.items())) + f" -- by {', '.join(mods)}\n"
            self.put(f"20-Systems/{slug(pkg)}.md", b)

    def mechanisms(self) -> None:
        # Fabric API modules
        body = fm(type="index") + "# Modification mechanisms\n\n"
        body += ("Three families, deliberately kept apart because they compose differently:\n\n"
                 "1. **Data packs / resource packs** -- no code. See "
                 f"{link('00-Scope/Data_Driven_Surface', 'the data-driven surface')}.\n"
                 f"2. **Registries** -- code registers new content. See {link('30-Mechanisms/Registries')}.\n"
                 "3. **Fabric API modules** -- events, hooks and helpers, each implemented by "
                 "mixins into vanilla. Listed below with what each actually hooks.\n\n"
                 "4. **Direct mixins by a mod** -- the same mechanism Fabric API uses, applied by "
                 f"a mod itself. See {link('30-Mechanisms/Mixin')}.\n\n")
        body += "| module | lifecycle | env | mixin classes | injections | events published | api classes |\n|---|---|---|---|---|---|---|\n"
        pubs = self.edges_by("module")
        for m in sorted(self.fabric["modules"], key=lambda m: m["id"]):
            lifecycle = ((m["declared"].get("custom") or {}).get("fabric-api:module-lifecycle", "?"))
            env = m["declared"].get("environment") or "*"
            es = pubs.get(m["id"], [])
            inj = sum(1 for e in es if e["relation"] in ("injects_into", "replaces", "wraps"))
            pub = len({e["to"]["id"] for e in es if e["relation"] == "publishes_event"})
            body += (f"| {link('30-Mechanisms/' + slug(m['id']), m['id'])} | {lifecycle} | {env} | "
                     f"{m['counts']['mixin_classes']} | {inj} | {pub} | {m['counts']['api_classes']} |\n")
        self.put("30-Mechanisms/_index.md", body)

        events = self.events()
        for m in self.fabric["modules"]:
            es = pubs.get(m["id"], [])
            b = fm(type="mechanism", module=m["id"], version=m["version"], sha256=m["sha256"],
                   lifecycle=(m["declared"].get("custom") or {}).get("fabric-api:module-lifecycle", "?"))
            b += f"# {m['id']}\n\n"
            d = m["declared"]
            b += f"**Version** `{m['version']}` -- **artifact sha256** `{m['sha256']}`\n\n"
            b += "## Declared (fabric.mod.json)\n\n"
            b += f"- environment: `{d.get('environment') or '*'}`\n"
            b += f"- depends: `{json.dumps(d.get('depends'))}`\n"
            if d.get("breaks"):
                b += f"- breaks: `{json.dumps(d.get('breaks'))}`\n"
            b += f"- entrypoints: `{json.dumps(d.get('entrypoints'))}`\n"
            b += f"- mixin configs: `{json.dumps(d.get('mixins'))}`\n"
            if d.get("accessWidener"):
                b += f"- access widener: `{d['accessWidener']}`\n"
            b += "\n## Events this module publishes\n\n"
            module_events = sorted({e["to"]["id"] for e in es if e["relation"] == "publishes_event"} |
                                   {e["to"]["id"] for e in es if e["relation"] == "callback_of"})
            if module_events:
                for ev in module_events:
                    b += f"- {link('50-Interactions/events/' + slug(ev), ev.split('.')[-2] + '.' + ev.split('.')[-1])}\n"
            else:
                b += "- none found by extraction\n"
            b += "\n## Vanilla types this module modifies (mixins)\n\n"
            b += "| vanilla type | method | how | environment | mixin |\n|---|---|---|---|---|\n"
            for e in sorted((e for e in es if e["relation"] in ("injects_into", "replaces", "wraps")),
                            key=lambda e: (e["to"]["owner"], e["to"]["id"])):
                b += (f"| {link('40-Interfaces/' + slug(e['to']['owner']), e['to']['owner'].split('.')[-1])} | "
                      f"`{e['to']['id']}` | {e['relation']} `{e.get('operation', '')}` | "
                      f"{e['applies_to'].get('environment')} | `{e['from'].get('owner', e['from']['id']).split('.')[-1]}.{e['from']['id'] if e['from'].get('owner') else ''}` |\n")
            b += "\n## API surface\n\n"
            for api in sorted(m["api"], key=lambda a: a["fqcn"]):
                b += f"- {link('40-Interfaces/' + slug(api['fqcn']), api['fqcn'].split('.')[-1])} ({api['kind']}, {len(api['members'])} members)\n"
            b += ("\n## What this establishes, and does not\n\n"
                  "- Injection targets and API signatures are `direct_reference`: read from the jar.\n"
                  "- Event publication is `static_inference`: a bytecode pattern, labelled as such.\n"
                  "- Nothing here is `observed`. No game ran.\n")
            self.put(f"30-Mechanisms/{slug(m['id'])}.md", b)

        # Registries
        r = self.registries
        b = fm(type="mechanism", artifact=r["artifact"], sha256=r["sha256"]) + "# Registries\n\n"
        b += (f"Read from `BuiltInRegistries` and `Registries` in the 26.3 jar (`{r['sha256'][:16]}`). "
              "Each built-in registry is a target for `Registry.register`; each key with a "
              "datapack type is also addable from data.\n\n"
              f"## Built-in registries ({len(r['built_in_registries'])})\n\n| registry | element type |\n|---|---|\n")
        for reg in r["built_in_registries"]:
            b += f"| `{reg['field']}` | `{reg['type']}` |\n"
        b += f"\n## Registry keys ({len(r['registry_keys'])})\n\n"
        b += ", ".join(f"`{k['field']}`" for k in r["registry_keys"]) + "\n"
        self.put("30-Mechanisms/Registries.md", b)

        # Mixin as a mechanism (authored explanation + extracted counts)
        counts = self.edges["counts"]
        b = fm(type="mechanism") + "# Mixin (direct bytecode modification)\n\n"
        b += (f"The corpus contains {counts.get('injects_into', 0)} `injects_into`, "
              f"{counts.get('wraps', 0)} `wraps` and {counts.get('replaces', 0)} `replaces` edges from "
              "Fabric API's own mixins. A mod may use the same mechanism. The composition rule is "
              "the one recorded in the pack manifest: additive injections coexist; `@Overwrite` and "
              "cancelling injections on the same method do not.\n\n"
              f"Runtime: `sponge-mixin` -- see {link('00-Scope/Corpus')}.\n\n"
              f"Analyst note: {link('_authored/mechanisms/mixin', 'composition and ordering')}\n")
        self.put("30-Mechanisms/Mixin.md", b)

    def interfaces(self) -> None:
        targets = self.vanilla_targets()
        # Vanilla hooked types
        for t, es in targets.items():
            b = fm(type="interface", fqcn=t, side="vanilla") + f"# {t}\n\n"
            pkg = pkg_of(t)
            b += f"System: {link('20-Systems/' + slug(pkg), pkg)}\n\n"
            b += "## How Fabric API modules touch this type\n\n| relation | member | operation | environment | by | evidence |\n|---|---|---|---|---|---|\n"
            for e in sorted(es, key=lambda e: (e["relation"], e["to"]["id"])):
                b += (f"| {e['relation']} | `{e['to']['id']}{e['to'].get('descriptor', '')[:60]}` | "
                      f"`{e.get('operation', '')[:80]}` | {e['applies_to'].get('environment')} | "
                      f"{link('30-Mechanisms/' + slug(e.get('module', '')), e.get('module', ''))} | {e['evidence_class']} |\n")
            blk = (self.members or {}).get("types", {}).get(t)
            if blk:
                b += (f"\n## Declared members ({len(blk['members'])}, all visibilities)\n\n"
                      f"From `{self.members['artifact']}` `{self.members['sha256'][:16]}` via `javap -p`. "
                      "Inherited members are not listed here.\n\n```java\n" + blk["header"] + " {\n"
                      + "\n".join("    " + m + ";" for m in blk["members"]) + "\n}\n```\n")
            elif self.members and t in self.members.get("types_missing", []):
                b += ("\n## Members\n\nNot in the merged 26.3 jar: this type belongs to a library "
                      "Minecraft depends on (DataFixerUpper, Brigadier or similar) that is outside the "
                      f"corpus. See {link('00-Scope/Branches', 'branches')}; no artifact, no members.\n")
            else:
                b += ("\n## Members\n\nNot extracted -- run `atlas/extract/vanilla_members.py`; see "
                      f"{link('80-Unresolved/_index', 'unresolved')} (incomplete_extraction).\n")
            self.put(f"40-Interfaces/{slug(t)}.md", b)
        # Fabric API classes
        for m in self.fabric["modules"]:
            for api in m["api"] + m["api_nested"]:
                b = fm(type="interface", fqcn=api["fqcn"], module=m["id"], sha256=m["sha256"]) + f"# {api['fqcn']}\n\n"
                b += f"Module: {link('30-Mechanisms/' + slug(m['id']), m['id'])} -- kind: {api['kind']}\n\n```java\n"
                b += "\n".join(api["members"]) + "\n```\n"
                self.put(f"40-Interfaces/{slug(api['fqcn'])}.md", b)
        for api in self.fabric["loader"]["api"]:
            b = fm(type="interface", fqcn=api["fqcn"], module="fabric-loader") + f"# {api['fqcn']}\n\n"
            b += f"fabric-loader {self.fabric['loader']['version']} -- kind: {api['kind']}\n\n```java\n"
            b += "\n".join(api["members"]) + "\n```\n"
            self.put(f"40-Interfaces/{slug(api['fqcn'])}.md", b)

    def interactions(self) -> None:
        events = self.events()
        b = fm(type="index") + "# Interaction contracts\n\n"
        b += ("An edge is a typed relationship with exact endpoints and an evidence class. "
              f"Totals: {json.dumps(self.edges['counts'])}. Evidence: {json.dumps(self.edges['evidence_classes'])}.\n\n"
              f"- {link('50-Interactions/events/_index', 'Events')} -- what publishes each, and its callback\n"
              f"- {link('50-Interactions/contested_methods', 'Contested vanilla methods')} -- methods more than one module modifies\n"
              f"- {link('50-Interactions/overwrites', 'Overwrites')} -- methods replaced outright\n"
              f"- {link('50-Interactions/store_records', 'Curated interaction records')}\n"
              f"- Analyst contracts: {link('_authored/contracts/_index', 'index')}\n")
        self.put("50-Interactions/_index.md", b)

        contracts_by_id = {c["id"]: c for c in self.contracts}
        self.contract_notes(contracts_by_id)
        eb = fm(type="index") + "# Events\n\n| event | callback | publishers | module | contract |\n|---|---|---|---|---|\n"
        for ev, info in sorted(events.items()):
            eb += (f"| {link('50-Interactions/events/' + slug(ev), ev.split('.')[-2] + '.' + ev.split('.')[-1])} | "
                   f"`{(info['callback'] or '?').split('.')[-1]}` | {len(info['publishers'])} | "
                   f"{link('30-Mechanisms/' + slug(info.get('module') or ''), info.get('module') or '?')} | "
                   f"{link('_authored/contracts/' + slug(ev), 'yes') if f'event:{ev}' in contracts_by_id else 'no'} |\n")
        self.put("50-Interactions/events/_index.md", eb)
        for ev, info in events.items():
            b = fm(type="event", event=ev, callback=info["callback"] or "") + f"# {ev}\n\n"
            b += f"Callback interface: `{info['callback']}`\n\nModule: {link('30-Mechanisms/' + slug(info.get('module') or ''), info.get('module') or '?')}\n\n"
            b += "## Published from\n\n"
            if info["publishers"]:
                b += "| site | vanilla injection | environment | evidence |\n|---|---|---|---|\n"
                for p in info["publishers"]:
                    # find the injection edge for the same mixin method, if any
                    inj = [e for e in self.edges["edges"] if e["relation"] in ("injects_into", "wraps", "replaces")
                           and e["from"].get("owner") == p["from"]["owner"] and e["from"]["id"] == p["from"]["id"]]
                    where = "; ".join(f"`{e['to']['owner'].split('.')[-1]}.{e['to']['id']}` {e.get('operation','')}" for e in inj[:3]) \
                        or ("(handler is not itself an injector method: fired from a helper or impl class)"
                            if p["from"]["owner"].startswith("net.fabricmc.fabric.mixin") else "(impl code, not a mixin)")
                    b += f"| `{p['from']['owner'].split('.')[-1]}.{p['from']['id']}` | {where} | {p['applies_to'].get('environment')} | {p['evidence_class']} |\n"
            else:
                b += "- no publisher found by extraction (may be fired from generated or non-module code)\n"
            b += "\n## Contract\n\nWhat a subscriber may assume is NOT established by extraction. "
            if f"event:{ev}" in contracts_by_id:
                b += f"An analyst-stated contract exists: {link('_authored/contracts/' + slug(ev), 'read it')}.\n"
            else:
                b += (f"No analyst has stated one ({link('_authored/contracts/_index', 'contracts')}); "
                      "the callback signature above is all that is known.\n")
            self.put(f"50-Interactions/events/{slug(ev)}.md", b)

        # contested methods: same vanilla (owner, method) modified by >1 module
        contested: dict[tuple, list[dict]] = collections.defaultdict(list)
        for e in self.edges["edges"]:
            if e["relation"] in ("injects_into", "wraps", "replaces"):
                contested[(e["to"]["owner"], e["to"]["id"])].append(e)
        b = fm(type="index") + "# Contested vanilla methods\n\n"
        b += ("Methods that more than one Fabric API module modifies. Each is a place where "
              "injection ORDER matters and where a third mod's mixin joins an existing crowd. "
              "Multiple injections coexisting is the normal case; the list exists because it is "
              "where a cancelling injection or an @Overwrite would break others.\n\n"
              "| vanilla method | modules | relations |\n|---|---|---|\n")
        for (owner, meth), es in sorted(contested.items()):
            mods = sorted({e.get("module") for e in es})
            if len(mods) > 1:
                b += (f"| {link('40-Interfaces/' + slug(owner), owner.split('.')[-1])}.`{meth}` | "
                      f"{', '.join(mods)} | {', '.join(sorted({e['relation'] for e in es}))} |\n")
        self.put("50-Interactions/contested_methods.md", b)

        b = fm(type="index") + "# Overwrites\n\n"
        b += "`@Overwrite` replaces a vanilla method body. Two overwrites of one method cannot both apply.\n\n| vanilla method | module | mixin |\n|---|---|---|\n"
        for e in self.edges["edges"]:
            if e["relation"] == "replaces":
                b += f"| {link('40-Interfaces/' + slug(e['to']['owner']), e['to']['owner'].split('.')[-1])}.`{e['to']['id']}` | {e.get('module')} | `{e['from'].get('owner', e['from']['id']).split('.')[-1]}` |\n"
        self.put("50-Interactions/overwrites.md", b)

        b = fm(type="index") + "# Curated interaction records (store)\n\n"
        recs = self.pack.records("interaction")
        b += "None recorded for minecraft yet.\n" if not recs else ""
        for r in recs:
            b += f"- `{r.id}` -- {r.get('title')} ({r.get('outcome', {}).get('kind')})\n"
        self.put("50-Interactions/store_records.md", b)

    def contract_notes(self, contracts_by_id: dict[str, dict]) -> None:
        b = fm(type="index") + "# Analyst-stated contracts\n\n"
        b += ("Each note separates what is *asserted* (with the basis for it) from what is "
              "*not established*. A contract is an analyst's reading of declared signatures, "
              "bytecode patterns and cached documentation; it is not a runtime guarantee.\n\n"
              "| subject | kind | asserted | not established |\n|---|---|---|---|\n")
        for cid, c in sorted(contracts_by_id.items()):
            ident = cid.split(":", 1)[1]
            b += f"| {link('_authored/contracts/' + slug(ident), ident.split('.')[-2] + '.' + ident.split('.')[-1] if '.' in ident else ident)} | {c['kind']} | {len(c['asserted'])} | {len(c['not_established'])} |\n"
        self.put("_authored/contracts/_index.md", b, generated=False)
        for cid, c in contracts_by_id.items():
            ident = cid.split(":", 1)[1]
            b = fm(type="contract", subject=cid, kind=c["kind"]) + f"# Contract: {ident}\n\n"
            b += f"Subject: {self._render_ref(cid)}\n\n## Asserted\n\n| claim | basis | evidence |\n|---|---|---|\n"
            for a in c["asserted"]:
                b += f"| {self._render_ref(a['claim'])} | `{a['basis']}` | {'; '.join(self._render_ref(e) for e in a['evidence'])} |\n"
            b += "\n## Not established\n\n" + "\n".join(f"- {self._render_ref(n)}" for n in c["not_established"]) + "\n\n"
            b += "## Evidence\n\n" + "\n".join(f"- {self._render_ref(e)}" for e in c["evidence"]) + "\n"
            if c.get("questions"):
                b += "\n## Open questions\n\n" + "\n".join(f"- {self._render_ref(q)}" for q in c["questions"]) + "\n"
            self.put(f"_authored/contracts/{slug(ident)}.md", b, generated=False)

    def failures(self) -> None:
        b = fm(type="index") + "# Known failures and resolutions (store records)\n\n"
        for r in self.pack.records("failure"):
            b += f"## {r.id}\n\n**{r.get('title')}**\n\n- symptom: {r.get('symptom')}\n- mechanism: {r.get('mechanism')}\n"
            det = r.get("detector") or {}
            b += f"- detectable: {det.get('detectable')} via `{det.get('check_id')}`\n"
            for res in r.get("resolutions") or []:
                b += f"- resolution: {res}\n"
            b += "\n"
        b += "## Resolutions\n\n"
        for r in self.pack.records("resolution"):
            b += f"- `{r.id}` -- {r.get('title')} (method: {r.get('method')})\n"
        self.put("60-Failures/_index.md", b)

    def workflow_notes(self) -> None:
        b = fm(type="index") + "# Modder workflows and request families\n\n"
        b += ("Authored. Each family traces intent -> tools today -> decisions -> information -> "
              "automation -> remaining work -> ModCheck's contribution, and names the evidence "
              "each 'today' claim rests on.\n\n| area | family | inventoried | inspected | contract-mapped | analysed | in ModCheck |\n|---|---|---|---|---|---|---|\n")
        for w in sorted(self.workflows, key=lambda w: (w["area"], w["id"])):
            s = w["status"]
            tick = lambda v: "yes" if v else "no"  # noqa: E731
            b += (f"| {w['area']} | {link('10-Workflows/' + slug(w['id']), w['title'])} | {tick(s['inventoried'])} | "
                  f"{tick(s['mechanically_inspected'])} | {tick(s['contract_mapped'])} | "
                  f"{tick(s['interaction_analysed'])} | {tick(s['implemented_in_modcheck'])} |\n")
        self.put("10-Workflows/_index.md", b)
        for w in self.workflows:
            b = fm(type="workflow", id=w["id"], area=w["area"]) + f"# {w['title']}\n\n"
            b += f"**Intent.** {w['intent'].strip()}\n\n"
            for key, title in (("preserve", "Must be preserved"), ("mechanisms", "Mechanisms that can serve it"),
                               ("tools_today", "Tools and artifacts used today"),
                               ("decisions", "Decisions the creator must make"),
                               ("information_needed", "Information those decisions need"),
                               ("automation_existing", "Existing automation"),
                               ("remaining_manual", "Remaining manual or unsupported work"),
                               ("modcheck_contribution", "ModCheck's contribution"),
                               ("interactions_to_check", "Interactions to check"),
                               ("evidence", "Evidence"), ("questions", "Open questions")):
                items = w.get(key) or []
                if items:
                    b += f"## {title}\n\n" + "\n".join(f"- {self._render_ref(i)}" for i in items) + "\n\n"
            s = w["status"]
            b += ("## Status\n\n" + "\n".join(f"- {k}: {v}" for k, v in s.items()) + "\n")
            self.put(f"10-Workflows/{slug(w['id'])}.md", b, generated=False)

    def _render_ref(self, text: str) -> str:
        """Turn `extracted:...`, `source:...`, `record:...`, `question:...`, `mechanism:...` into links."""
        def repl(m):
            kind, ident = m.group(1), m.group(2)
            if kind == "mechanism":
                return link("30-Mechanisms/" + slug(ident), ident)
            if kind == "question":
                return link("80-Unresolved/" + slug(ident), ident)
            if kind == "workflow":
                return link("10-Workflows/" + slug(ident), ident)
            if kind == "event":
                return link("50-Interactions/events/" + slug(ident), ident.split(".")[-1])
            if kind == "type":
                return link("40-Interfaces/" + slug(ident), ident.split(".")[-1])
            if kind == "source":
                return f"{link('00-Scope/Sources', ident)}"
            if kind == "extracted":
                return f"`extracted/{ident}`"
            if kind == "record":
                return f"`{ident}`"
            if kind == "request":
                return link("70-Requests/" + slug(ident), ident)
            return m.group(0)
        return re.sub(REF_PATTERN, repl, text)

    def request_notes(self) -> None:
        b = fm(type="index") + "# Request-specific impact analyses\n\n"
        for r in self.requests:
            b += f"- {link('70-Requests/' + slug(r['id']), r['title'])} -- {r.get('family')}\n"
        self.put("70-Requests/_index.md", b)
        for r in self.requests:
            b = fm(type="request", id=r["id"], family=r.get("family")) + f"# {r['title']}\n\n"
            for key, title in (("request", "Request"), ("approved_behaviour", "Approved behaviour and constraints"),
                               ("preserve", "Preservation obligations"), ("affected_systems", "Affected systems"),
                               ("candidates", "Implementation candidates"), ("dependencies", "Data / control / state dependencies"),
                               ("interactions", "Interactions with the selected environment"),
                               ("alternatives", "Alternatives and tradeoffs"), ("work", "Implementation work"),
                               ("verification", "Verification obligations"), ("unresolved", "Unresolved"),
                               ("evidence", "Evidence")):
                items = r.get(key)
                if not items:
                    continue
                b += f"## {title}\n\n"
                b += (f"{self._render_ref(items.strip())}\n\n" if isinstance(items, str)
                      else "\n".join(f"- {self._render_ref(i)}" for i in items) + "\n\n")
            self.put(f"70-Requests/{slug(r['id'])}.md", b, generated=False)

    def question_notes(self) -> None:
        b = fm(type="index") + "# Unresolved questions and engineering requirements\n\n"
        b += "| id | kind | status | affects |\n|---|---|---|---|\n"
        for q in sorted(self.questions, key=lambda q: (q["status"], q["kind"], q["id"])):
            b += f"| {link('80-Unresolved/' + slug(q['id']), q['id'])} | {q['kind']} | {q['status']} | {', '.join(q['affects'][:3])} |\n"
        self.put("80-Unresolved/_index.md", b)
        for q in self.questions:
            b = fm(type="question", id=q["id"], kind=q["kind"], status=q["status"]) + f"# {q['id']}\n\n"
            b += f"**Question.** {q['question'].strip()}\n\n**Kind.** `{q['kind']}` -- **Status.** {q['status']}\n\n"
            b += f"**Why it matters.** {q['why_it_matters'].strip()}\n\n"
            b += "**Affects.** " + ", ".join(self._render_ref(a) for a in q["affects"]) + "\n\n"
            b += "**Evidence already available.**\n" + "".join(f"- {self._render_ref(e)}\n" for e in q["evidence_available"]) + "\n"
            b += f"**Best remaining source.** {self._render_ref(q['best_remaining_source'])}\n\n"
            b += f"**Procedure.** {q['procedure'].strip()}\n\n**Done when.** {q['completion_condition'].strip()}\n\n"
            b += "**Conclusions affected while open.**\n" + "".join(f"- {c}\n" for c in q["conclusions_affected_while_open"])
            if q.get("resolved_by"):
                b += f"\n**Resolved by.** {self._render_ref(str(q['resolved_by']).strip())}\n"
            self.put(f"80-Unresolved/{slug(q['id'])}.md", b, generated=False)

    def coverage(self) -> None:
        mc = self.corpus["minecraft"]
        targets = self.vanilla_targets()
        pkgs = mc["classes_by_package_depth4"]
        hooked_pkgs = {pkg_of(t) for t in targets}
        total_classes = sum(pkgs.values())
        modules = self.fabric["modules"]
        api_with_members = sum(len(m["api"]) for m in modules)
        b = fm(type="coverage", generated_at=self.edges["generated_at"]) + "# Coverage, with denominators\n\n"
        b += ("Seven separate measures. None of them is 'supported'. A capability record, a linked "
              "note, or a passing schema establishes nothing about behaviour.\n\n")
        b += "## 1. Surface inventory\n\n"
        b += f"- artifacts classified: {len(self.corpus['artifacts'])} / {len(self.corpus['artifacts'])} (0 unclassified)\n"
        b += f"- Minecraft packages (depth 4) inventoried: {len(pkgs)} / {len(pkgs)}, {total_classes} top-level classes\n"
        b += f"- data pack entry types inventoried: {len(mc['data_entries_by_type'])} (from the jar)\n"
        b += f"- Fabric API modules inventoried: {len(modules)} / {self.corpus['groups'].get('fabric_api_module')}\n\n"
        b += "## 2. Extraction\n\n"
        n_declared = sum(len(c.get(k) or []) for m in modules for c in m["mixin_configs"].values()
                         if isinstance(c, dict) for k in ("mixins", "client", "server"))
        n_extracted = sum(len(m["mixins"]) for m in modules)
        n_pkg = sum(m["counts"]["mixin_classes"] for m in modules)
        b += (f"- mixin classes declared in mixin configs: {n_declared}; with @Mixin targets extracted: {n_extracted} "
              f"(classes under mixin packages in total, including nested and helper classes: {n_pkg})\n")
        b += f"- Fabric API classes with public surface extracted: {api_with_members} / {sum(m['counts']['api_classes'] for m in modules)}\n"
        b += f"- vanilla types that are hook targets: {len(targets)}\n"
        b += f"- vanilla packages with at least one hook: {len(hooked_pkgs)} / {len(pkgs)}\n"
        if self.members:
            et = self.members["edge_targets"]
            n_t = et["resolved_on_type"] + len(et["resolved_by_inheritance"]) + len(et["unresolved"])
            b += (f"- hooked types that live outside the merged jar (libraries): {len(self.members['types_missing'])} -- no members, by construction\n")
            b += (f"- vanilla classes whose members were extracted: {self.members['types_extracted'] + 2} / {total_classes} "
                  f"(the {self.members['types_extracted']} hook targets plus BuiltInRegistries, Registries)\n"
                  f"- edge targets cross-checked against those members: {et['resolved_on_type']} declared on the type, "
                  f"{len(et['resolved_by_inheritance'])} declared on a superclass, {len(et['unresolved'])} unresolved "
                  f"/ {n_t} (unresolved = interface-declared, `java.lang.Object`, wildcard `<clinit>*`, or "
                  "Fabric interface-injected members; see unresolved)\n\n")
        else:
            b += f"- vanilla classes whose members were extracted: 2 / {total_classes} (BuiltInRegistries, Registries)\n\n"
        b += "## 3. Workflow information\n\n"
        areas = collections.Counter(w["area"] for w in self.workflows)
        b += f"- workflow families authored: {len(self.workflows)} across {len(areas)} / 8 areas: {dict(areas)}\n"
        b += f"- families with interaction analysis: {sum(1 for w in self.workflows if w['status']['interaction_analysed'])} / {len(self.workflows)}\n\n"
        b += "## 4. Interaction contracts\n\n"
        b += f"- typed edges: {len(self.edges['edges'])} -- by relation {json.dumps(self.edges['counts'])}\n"
        b += f"- by evidence class: {json.dumps(self.edges['evidence_classes'])}\n"
        b += f"- events with an identified publisher: {sum(1 for v in self.events().values() if v['publishers'])} / {len(self.events())}\n"
        b += f"- events with an analyst-stated subscriber contract: {sum(1 for c in self.contracts if c['kind'] == 'event')} / {len(self.events())}\n"
        b += f"- curated interaction records in the store: {len(self.pack.records('interaction'))}\n\n"
        b += "## 5. Executable analysis (ModCheck)\n\n"
        b += f"- capability records: {len(self.pack.records('capability'))}; failures with a detector: {sum(1 for r in self.pack.records('failure') if (r.get('detector') or {}).get('detectable') == 'yes')} / {len(self.pack.records('failure'))}\n"
        b += "- static mixin-collision analysis over arbitrary mod jars: NOT implemented (the jvm inspector lists mixin classes; it does not read their targets)\n\n"
        b += "## 6. Runtime evidence\n\n- observed edges: 0. No game has run.\n\n"
        b += "## 7. Creation / maintenance automation\n\n"
        b += "- deterministic generators: 6 mechanisms (Fabric only)\n- agent execution: boundary exists, unavailable here\n\n"
        # per-system state, each column from a different source of evidence
        contract_pkgs: collections.Counter = collections.Counter()
        handler_sites = {(e["from"].get("owner"), e["from"]["id"]): e["to"]["owner"]
                         for e in self.edges["edges"] if e["relation"] in ("injects_into", "wraps", "replaces")}
        for c in self.contracts:
            if not c["id"].startswith("event:"):
                continue
            ev = c["id"][6:]
            for p in self.events().get(ev, {}).get("publishers", []):
                owner = handler_sites.get((p["from"]["owner"], p["from"]["id"]))
                if owner:
                    contract_pkgs[pkg_of(owner)] += 1
        system_notes = {s["id"] for s in self.systems}
        req_pkgs: dict[str, list[str]] = collections.defaultdict(list)
        for r in self.requests:
            for a in r["affected_systems"]:
                for pkg in pkgs:
                    if a.startswith(pkg + " ") or a.startswith(pkg + "."):
                        req_pkgs[pkg].append(r["id"] + ("" if r["status"]["implemented"] == "none" else " (implemented)"))
        members = (self.members or {}).get("types", {})
        b += ("## Per-system state\n\nColumns are independent: *inspected* = members of its hooked types extracted; "
              "*contract-mapped* = analyst contracts whose publication site is in the package; *analysed* = an analyst "
              "system note exists; *in ModCheck* = a request analysis names it, marked if that request is implemented; "
              "*validated* = the validation scope of that implementation.\n\n"
              "| package | inventoried | inspected | contract-mapped | analysed | in ModCheck | validated |\n|---|---|---|---|---|---|---|\n")
        for pkg in sorted(set(pkgs) | system_notes):
            hooked = pkg in hooked_pkgs
            n_types = sum(1 for t in targets if pkg_of(t) == pkg)
            n_mem = sum(1 for t in targets if pkg_of(t) == pkg and t in members)
            inspected = f"members of {n_mem}/{n_types} hooked types" if hooked else "no"
            cm = f"{contract_pkgs[pkg]} contracts" if contract_pkgs.get(pkg) else "no"
            an = "system note" if pkg in system_notes else ("edges only" if hooked else "no")
            reqs = req_pkgs.get(pkg, [])
            inm = "; ".join(reqs) if reqs else "no"
            val = "JUnit with fakes, gradle build; no game run" if any("(implemented)" in r for r in reqs) else "none"
            b += f"| `{pkg}` | yes | {inspected} | {cm} | {an} | {inm} | {val} |\n"
        self.put("90-Coverage/_index.md", b)

    def index(self) -> None:
        b = fm(type="index") + "# Minecraft Engineering Atlas\n\n"
        b += ("Java Edition 26.3, Fabric loader 0.19.5, Fabric API 0.161.0+26.3 -- the corpus this "
              "project resolved and built against. Other editions, loaders and surfaces are "
              f"enumerated as branches, not silently folded in.\n\n"
              f"- {link('00-Scope/Corpus')} · {link('00-Scope/Branches')} · {link('00-Scope/Data_Driven_Surface')} · {link('00-Scope/Sources')}\n"
              f"- {link('10-Workflows/_index', 'Workflows and request families')}\n"
              f"- {link('20-Systems/_index', 'Game systems')}\n"
              f"- {link('30-Mechanisms/_index', 'Modification mechanisms')}\n"
              f"- {link('40-Interfaces/_index', 'Exact interfaces')}\n"
              f"- {link('50-Interactions/_index', 'Interaction contracts')}\n"
              f"- {link('60-Failures/_index', 'Known failures')}\n"
              f"- {link('70-Requests/_index', 'Request impact analyses')}\n"
              f"- {link('80-Unresolved/_index', 'Unresolved questions')}\n"
              f"- {link('90-Coverage/_index', 'Coverage')}\n\n"
              "## Reading the labels\n\n"
              "`direct_reference` was read from a jar. `static_inference` is a bytecode pattern. "
              "`declared` is a manifest. `documented` is prose. `observed` would be a running "
              "game, and nothing is.\n")
        self.put("_index.md", b)
        # interfaces index
        files = sorted(p.stem for p in (VAULT / "40-Interfaces").glob("*.md") if p.stem != "_index")
        b = fm(type="index") + f"# Exact interfaces ({len(files)})\n\n"
        vanilla = [f for f in files if f.startswith("net.minecraft")]
        fabric = [f for f in files if f.startswith("net.fabricmc")]
        b += f"## Vanilla types hooked by Fabric API ({len(vanilla)})\n\n" + "\n".join(f"- {link('40-Interfaces/' + f, f)}" for f in vanilla)
        b += f"\n\n## Fabric API and loader types ({len(fabric)})\n\n" + "\n".join(f"- {link('40-Interfaces/' + f, f)}" for f in fabric) + "\n"
        self.put("40-Interfaces/_index.md", b)
        # authored passthrough
        for path in AUTHORED.rglob("*.md"):
            rel = path.relative_to(AUTHORED)
            self.put(f"_authored/{rel}", fm(type="authored", source=str(path.relative_to(ROOT))) + path.read_text(), generated=False)


def main() -> int:
    atlas = Atlas()
    atlas.build()
    print(f"vault: {len(atlas.written)} notes under {VAULT}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

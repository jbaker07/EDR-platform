# Minecraft Engineering Atlas

An Obsidian-compatible Markdown vault describing Minecraft's modification surface
and interactions, built from the exact artifacts this project resolved (hashed),
from the ModCheck store records, and from analyst notes that are kept visibly
separate from both.

Obsidian is not installed here and is not required: every note is plain Markdown
with `[[wikilinks]]`, YAML frontmatter and callout banners. Open `atlas/vault/`
as a vault, or read it as files.

## Layout

| path | what it is | who writes it |
|---|---|---|
| `extracted/*.json` | facts read from the resolved jars, each with the artifact sha256 and the javap method that produced it | the extractors; never edited by hand |
| `authored/**` | analyst material: workflow families, unresolved questions, system notes, request analyses, subscriber contracts, the branch inventory, the mixin note, the backlog | people; validated against `schema/` |
| `schema/*.json` | JSON Schemas for edges, workflows, questions, systems, requests, contracts | people |
| `vault/**` | the generated vault; `_authored/**` inside it is the rendered analyst material | `extract/vault.py`; deleted and rebuilt every run |
| `extract/*.py` | corpus classification, registry, hook, member, edge extraction; vault generation; validation | people |

Two banners appear at the top of every note, after the frontmatter:

- `> [!info] Generated` -- built from records and extracted facts; edit the source, not the note.
- `> [!warning] Analyst-authored` -- interpretation; every claim cites an evidence id or is opinion.

Evidence classes on edges and contract claims: `direct_reference` (read from a
jar), `static_inference` (a bytecode pattern), `declared` (a manifest or a
signature), `documented` (cached prose), `observed` (a running game -- currently
zero, and the validator refuses an `observed` edge without runtime provenance).

## Reproduce

All commands run from `modcheck/` with the project virtualenv. The extractors need
the JDK 25 toolchain under `modcheck/toolchains/` (gitignored; provisioned by
`creator/toolchain.py`) and the Gradle caches that a build of the reference mod
leaves behind.

```sh
# 1. list the resolved jars (the corpus is whatever Loom resolved for 26.3 plus
#    every net.fabricmc artifact in the Gradle cache)
{ find /root/.gradle/caches/modules-2/files-2.1/net.fabricmc* -name "*.jar" ! -name "*-sources.jar";
  ls /root/.gradle/caches/fabric-loom/26.3/*.jar;
  ls /root/.gradle/caches/fabric-loom/minecraftMaven/net/minecraft/*/26.3/*.jar; } > /tmp/corpus_jars.txt

# 2. extract (order matters: edges need fabric_api + registries; members need edges)
.venv/bin/python atlas/extract/corpus.py /tmp/corpus_jars.txt   # -> extracted/corpus.json
.venv/bin/python atlas/extract/registries.py                    # -> extracted/minecraft_registries.json
.venv/bin/python atlas/extract/run_hooks.py                     # -> extracted/fabric_api.json (about 13 minutes)
.venv/bin/python atlas/extract/edges.py                         # -> extracted/edges.json
.venv/bin/python atlas/extract/vanilla_members.py               # -> extracted/minecraft_members.json (about 25 s)

# 3. generate and validate
.venv/bin/python atlas/extract/vault.py                         # rebuilds vault/
.venv/bin/python atlas/extract/validate.py                      # exit 1 on any error; --json for a report

# 4. tests
.venv/bin/python -m pytest tests/test_atlas.py -q
```

`validate.py` checks: every edge against `schema/edge.schema.json` with a
provenance sha256 that is in the corpus; every authored record against its
schema with unique ids; every typed reference (`mechanism:`, `event:`, `type:`,
`source:`, `record:`, `extracted:`, `question:`, `workflow:`, `request:`)
resolving to something that exists; every `[[wikilink]]` in the vault resolving
to a note (and heading); exactly one banner per note; no text asserting runtime
certainty without a negation; no numeric confidence anywhere.

## Correcting the atlas

- A wrong extracted fact: fix the extractor or the corpus pin, re-run, commit the
  regenerated JSON. Never edit `extracted/*.json`.
- A wrong store fact: correct the record under `packs/minecraft/` (it is the
  authoritative database; the atlas only renders it).
- A wrong analyst claim: edit the file under `authored/`, rebuild, validate.
- A generated note that reads wrong: the bug is in `extract/vault.py`.

## What the atlas is not

It is not a second database. Facts live in `packs/minecraft/` (curated) and
`extracted/` (mechanical); the vault links to them by id. It is not a runtime
record: nothing in it has been observed in a running game, and the coverage note
says so with denominators.

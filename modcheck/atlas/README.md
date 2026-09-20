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
| `extract/*.py` | class-file reader, corpus and resolved-environment recording, registry / hook / surface / edge extraction with exact resolution, transformation-evidence runner, vault generation, validation | people |
| `harness/mixin_transform/` | the controlled Mixin transformation scenarios (Java) and their runner | people |
| `../tests/fixtures/mixin_inputs/` | independently compiled mixin inputs the extractor tests read | people |

Two banners appear at the top of every note, after the frontmatter:

- `> [!info] Generated` -- built from records and extracted facts; edit the source, not the note.
- `> [!warning] Analyst-authored` -- interpretation; every claim cites an evidence id or is opinion.

Evidence classes on edges and contract claims: `direct_reference` (read from a
jar), `static_inference` (a bytecode pattern), `declared` (a manifest or a
signature), `documented` (cached prose), `observed` (a running game -- currently
zero, and the validator refuses an `observed` edge without runtime provenance).

## Reproduce

All commands run from `modcheck/` with the project virtualenv. Extraction reads
class files directly (`atlas/extract/classfile.py`); no javap. The JDK 25
toolchain under `modcheck/toolchains/` (gitignored) is needed only to compile
the fixtures and the harness, and to extract JDK classes for the resolver.

```sh
# 0. the resolved environment: what the reference build actually resolves against
#    (an init script that prints configurations; the listing is committed as
#    extracted/resolved_classpaths.tsv)
cat > /tmp/cp.gradle <<'G'
allprojects { tasks.register("printResolvedClasspaths") { doLast {
  ["compileClasspath", "runtimeClasspath", "testRuntimeClasspath"].each { n ->
    def c = project.configurations.findByName(n); if (c != null) { c.files.each { println "${n}\t${it}" } } } } } }
G
( cd reference/rainlantern && JAVA_HOME=$PWD/../../toolchains/jdk-25.0.4.1+1 \
  ../../toolchains/gradle-9.5.1/bin/gradle --offline -q --console=plain --init-script /tmp/cp.gradle \
  printResolvedClasspaths > ../../atlas/extracted/resolved_classpaths.tsv )

# 1. the corpus: every jar in the Gradle caches for net.fabricmc*, the Loom
#    Minecraft jars, plus every entry of the resolved classpaths (hashed, classified)
{ find /root/.gradle/caches/modules-2/files-2.1/net.fabricmc* -name "*.jar" ! -name "*-sources.jar";
  ls /root/.gradle/caches/fabric-loom/26.3/*.jar /root/.gradle/caches/fabric-loom/minecraftMaven/net/minecraft/*/26.3/*.jar;
  cut -f2 atlas/extracted/resolved_classpaths.tsv; } | sort -u > atlas/extracted/corpus_jars.txt
.venv/bin/python atlas/extract/corpus.py atlas/extracted/corpus_jars.txt        # -> extracted/corpus.json
.venv/bin/python atlas/extract/resolved_env.py atlas/extracted/resolved_classpaths.tsv  # -> extracted/resolved_environment.json (+ processed-vs-cache jar diff)

# 2. extract (order matters)
.venv/bin/python atlas/extract/registries.py         # -> extracted/minecraft_registries.json
.venv/bin/python atlas/extract/run_hooks.py          # -> extracted/fabric_api.json (about 1 s)
.venv/bin/python atlas/extract/vanilla_surface.py    # -> extracted/minecraft_surface.json.gz (every class of the processed jar)
toolchains/jdk-25.0.4.1+1/bin/jimage extract --include "regex:/java\.base/.*\.class" \
  --dir .cache/jdk_classes toolchains/jdk-25.0.4.1+1/lib/modules              # JDK types for the resolver (gitignored)
.venv/bin/python atlas/extract/edges.py              # -> extracted/edges.json (exact resolution; about 5 s)

# 3. transformation evidence (compiles and runs the harness; needs the JDK)
.venv/bin/python atlas/extract/mixin_transformation.py   # -> extracted/mixin_transformation_tests.json

# 4. generate and validate
.venv/bin/python atlas/extract/vault.py              # rebuilds vault/
.venv/bin/python atlas/extract/validate.py           # exit 1 on any error; --json for a report

# 5. tests (the compiled fixtures are committed; recompile with tests/fixtures/mixin_inputs/compile.sh)
.venv/bin/python -m pytest tests/test_atlas.py tests/test_atlas_extract.py -q
```

`validate.py` checks: every edge against `schema/edge.schema.json` with a
provenance sha256 that is in the corpus; every authored record against its
schema with unique ids; every request analysis bound to a canonical record whose
hash still matches; every typed reference (`mechanism:`, `event:`, `type:`,
`source:`, `record:`, `extracted:`, `question:`, `workflow:`, `request:`,
`scenario:`) resolving to something that exists; every `[[wikilink]]` in the
vault resolving to a note (and heading); exactly one banner per note; no text
asserting runtime certainty without a negation; no numeric confidence anywhere.

## Evidence classes

`direct_reference` (read from a class file), `static_inference` (a bytecode
pattern), `declared` (a manifest or signature), `documented` (cached prose),
`executed_transformation` (the pinned Mixin transformer run over controlled
classes in a plain JVM: `atlas/harness/mixin_transform`), `observed` (a running
game; currently zero, and the validator refuses an `observed` edge without
runtime provenance). Transformation evidence is never Minecraft runtime
evidence and is labelled apart from it.

## Resolution states

Every vanilla endpoint of an edge carries how it resolved against the processed
compile jar (plus Minecraft's libraries and the JDK): `exact` (owner, name and
descriptor), `inherited_exact` (declared on a supertype reached by a cycle-safe
walk over superclasses and interfaces), `name_only` (no descriptor given, one
member of that name), `ambiguous` (several overloads, candidates listed),
`unresolved`, `owner_missing`, `selector_unsupported` (wildcard, quantified,
regex or expression selectors). Constructors resolve only on the owner;
`<clinit>` iff a static initialiser exists.

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

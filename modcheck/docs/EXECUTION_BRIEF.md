# ModCheck engineering execution brief

Audience: engineers implementing ModCheck. Every claim here is either checkable
against this repository or carries its source. Where a fact is not established,
it appears in section E with the smallest procedure that would settle it, not as
an assumption.

Status vocabulary used throughout, and enforced by `modcheck validate`:

| status | meaning |
|---|---|
| `supported` | implemented AND named by at least one passing test node id |
| `partial` | implemented for a stated subset; limits recorded |
| `documented_only` | we hold sourced references; no executable support |
| `unsupported` | not implemented |
| `blocked` | cannot be implemented here; blocker recorded |

---

## A. Existing foundation

Measured from the repository at the time of writing: 186 tests passing, 2
skipped (opt-in real builds), 20/20 evaluation cases, 0 validation errors across
10 packs, ~5,000 lines of implementation.

### A.1 Implemented AND exercised against real upstream data

These are the load-bearing parts. Each has been run against something we did not
author.

| Component | Real data it has been run against |
|---|---|
| `inspect/jvm.py` | Fabric API 0.161.0+26.3 from `maven.fabricmc.net`, including its 44 jar-in-jar modules |
| `inspect/bethesda.py` | cross-validated field-by-field against `esplugin`, the parser LOOT itself uses |
| `integrations/loot.py` | the real Skyrim SE masterlist (3,161 plugin entries, 2,703 exact + 458 regex) |
| `integrations/smapi.py` | SMAPI's own metadata file (188 entries, 178 carrying status overrides) |
| `integrations/rimworld.py` | the RimSort Community Rules Database (631 entries) |
| `integrations/esplugin.py` | the `esplugin` 6.1.4 crate, built and invoked as a subprocess |
| `creator/scaffold.py` + `creator/build.py` | a real Gradle 9.5.1 / JDK 25 build producing a Minecraft 26.3 mod jar |
| `acquire.py` | 242 source records; 225 re-fetched and hash-compared |

Two of these earned their place by finding defects rather than confirming
assumptions:

- `esplugin` rejected our Bethesda fixtures under full parse. They carried
  trailing padding, making them header-valid but not valid plugins. Our
  header-only reader could not have caught that.
- Re-fetching exposed that raw-byte hashing reported permanently-drifting
  sources. The BG3 wiki differs between two fetches one second apart, inside an
  injected `<script>` nonce. Fixed with a content hash; binaries deliberately
  get no such tolerance.

### A.2 Implemented but tested ONLY against fixtures we wrote

This is the honest weak spot. These inspectors parse formats correctly as we
understand them, and nothing independent has confirmed that understanding.

| Component | Fixture basis | Risk |
|---|---|---|
| `inspect/sims4.py` | hand-built DBPF v2.1 index; `.pyc` magic table | no real `.package` has been parsed |
| `inspect/bg3.py` | hand-built LSPK v18 header | v16 layout differs and is unhandled; `.pak` file list never read |
| `inspect/cyberpunk.py` | synthesised archive layouts | `.archive` contents never decoded |
| `inspect/projectzomboid.py` | synthesised `mod.info` + media tree | no real Workshop mod parsed |
| `inspect/rimworld.py` | synthesised `About.xml` | no real mod parsed |
| `inspect/smapi.py` | synthesised manifest + `content.json` | no real content pack parsed |

`inspect/bethesda.py` was in this category until `esplugin` cross-validation
moved it into A.1. **That is the template for fixing the rest** (task T2).

### A.3 Planned or absent

- `runtime_testing`: `blocked` for all ten. We do not have the games.
- `project_setup`: only Fabric. Forge, NeoForge and Quilt have none.
- `source_edit`: three Fabric generators only.
- No `interaction` records authored (0 of 30+30+30 recipe/failure/example
  records). Interaction knowledge currently comes entirely from integrated
  upstream databases — which is the correct reuse posture, and is stated rather
  than hidden.
- No caching layer keyed by analyzer version or rule revision.
- No execution sandbox.

### A.4 Retain / replace

**Retain unchanged**: the schema contract, `acquire`/`store`/`validate`, the
evidence-class model, the three-valued condition evaluator, the per-capability
status model with test-backing enforcement, the evaluation harness with paired
controls.

**Retain but extend**: the inspectors (need real-artifact validation), the
creator path (needs a second loader), the staleness propagation (marks but does
not schedule revalidation).

**Replace nothing.** Nothing built so far has been superseded.

---

## B. Shared core — only what is genuinely required

| Component | Why it must be shared | Why not per-game |
|---|---|---|
| Artifact identity (sha256/CRC) | the join key between creator release and player configuration | identity semantics do not vary by game |
| Source + provenance records | one acquisition and drift pipeline | ten pipelines would drift apart |
| Evidence classes | every report separates declared / extracted / derived / observed / unresolved | a per-game notion of evidence is meaningless |
| `Installation` + `files_known_complete` | "unknown is not false" must hold everywhere | the failure mode is identical across games |
| Findings + coverage | one report shape for creator and player | divergence here breaks continuity |
| Version comparison | shared machinery, per-ecosystem syntax | the *syntaxes* differ and live in adapters |
| Validator | enforces the contract uniformly | per-game rules would be unenforceable |

Game-native semantics stay in adapters: `inspect/*` (formats), `integrations/*`
(upstream databases and tools), `analyze/collisions.py` (composition rules, one
function per ecosystem, with an explicit list of ecosystems deliberately **not**
analysed and why).

**Deliberately not built**: game engine, mod manager, launcher, storefront,
download service, filesystem layer, custom editor, graph database, microservices,
compute cluster, model training. None has a measured requirement.

---

## C. Ten-game integration matrix

`Composition` is the rule that decides outcomes — the thing that makes an
overlap a conflict or not. Getting it wrong is how a tool becomes noise.

| Game | Reused tools / data (state) | Inputs we can read today | Composition rule | Capabilities today | Decisive gap |
|---|---|---|---|---|---|
| **Minecraft** | Fabric Loom (integrated), Fabric meta API, Mojang manifest, Modrinth API (available, unused) | `fabric.mod.json`, `quilt.mod.json`, `mods.toml`, `neoforge.mods.toml`, mixin configs, JiJ modules | mods coexist; deps enforced by id+range at load; JiJ satisfies deps; additive mixin injections compose, `@Overwrite` does not | setup **supported**, build **supported**, inspection **supported**, deps **supported**, edit **partial** | no maintained mod-to-mod interaction database exists |
| **Skyrim SE** | LOOT masterlist CC0 (integrated), esplugin GPL-3.0 (integrated), xEdit / Mutagen / Synthesis (documented) | TES4 header, masters, ESM/ESL flags, CRC; records via esplugin | strict load order; later plugin wins per record | inspection **supported**, interaction **supported**, deps **partial** | UESP 403 → no field-level semantics |
| **Fallout 4** | same as above | same | same | same | same |
| **Fallout: NV** | LOOT masterlist **GPL-3.0** (integrated — *not* CC0, unlike the other two), esplugin | same | same, plus a 255-plugin active limit | inspection **supported**, interaction **supported**, deps **partial** | UESP 403; geckwiki 403 |
| **Stardew Valley** | SMAPI metadata LGPL-3.0 repo (integrated, read as data), Content Patcher (documented) | `manifest.json`, `content.json` Action/Target/When, SMAPI per-version status | `Load` is exclusive per asset; `EditData` merges per entry; Load Priority decides ties | inspection **supported**, interaction **partial** | Load Priority not extracted; no .NET SDK for builds |
| **RimWorld** | RimSort rules (integrated, licence **unknown**), Harmony (documented) | `About.xml` identity + load rules, version folders, Defs/Patches file lists | load order; later mod's Def wins; PatchOperations apply in order | inspection **supported**, interaction **partial** | defName ownership and xpath targets not extracted |
| **Project Zomboid** | PZwiki (primary source; responds 200 to our fetcher), pz-zdoc (documented) | `mod.info`, media-relative Lua paths, script files | Lua loaded by path; later mod wins | inspection **supported**, interaction **partial** | no vanilla `media/lua` inventory; Zedscript block ids unparsed |
| **Cyberpunk 2077** | REDmodding wiki (integrated as source), TweakXL/redscript/WolvenKit (documented) | install-path layers, REDmod `info.json` | install path is the loader contract; collision resolved at install | inspection **supported**, interaction **partial** | `.archive` (RDAR/CR2W) format spec not retrievable |
| **Baldur's Gate 3** | Larian toolkit wiki, bg3se, LSLib (documented) | `info.json`, `meta.lsx`, LSPK v18 header | `modsettings.lsx` order; later module wins | inspection **supported**, interaction **documented_only** | `.pak` file list is LZ4-compressed and unread |
| **The Sims 4** | SimsWiki DBPF spec, S4TK (documented) | DBPF resource index (type/group/instance), `.ts4script` bytecode version | same resource key = same resource overridden; loser undetermined | inspection **supported**, interaction **partial** | which package wins is unsourced; no resource-type table |

Across all ten: **Nexus Mods returns 403** to automated requests, and
**github.com / api.github.com are restricted**, so upstream maintenance state is
recorded `unknown` rather than guessed. 50 gaps are recorded across the ten
packs.

---

## D. Dependency-ordered implementation tasks

Each carries the six-part contract. No task says "understand X".

### T1 — Finding-code registry, and make `detector.check_id` mechanically true

*Depends on: nothing. Do first.*

- **INPUTS**: `src/modcheck/analyze/findings.py`; every `Finding(code=...)` site;
  the 13 distinct `detector.check_id` values in `packs/*/failures/*.yaml`.
- **AUTHORITATIVE BEHAVIOR**: a failure record claiming `detectable: yes|partial`
  asserts ModCheck can find it. That assertion is only true if `check_id` names a
  finding code an analyzer actually emits.
- **PROCESS**: add an explicit `CODES` registry. Add a validator rule: a
  `check_id` must be a registered code. Fix the 6 records whose `check_id` names
  a module internal (`modcheck.inspect.sims4.inspect_package:resource_keys`,
  `…inspect_script:bytecode_python_versions`,
  `modcheck.inspect.cyberpunk.inspect:install_paths`,
  `modcheck.inspect.bg3.inspect:{folder,mod_id,pak_files}`) — three of these now
  have real codes (`collision.sims4_resource_key`,
  `collision.cyberpunk_install_path`); the BG3 three have none and must become
  `detectable: no` with `why_not`.
- **OUTPUT**: every detectable failure links to a code the system emits.
- **VERIFICATION**: a test asserting every `code=` literal in `src/` is in the
  registry, and `modcheck validate` errors on an unregistered `check_id`.
  Control: a deliberately bogus `check_id` must fail validation.
- **LIMITATIONS**: proves the code exists and is emitted; does not prove the
  check fires on the specific failure described.

### T2 — Real-artifact validation for the six fixture-only inspectors

*Depends on: nothing. Highest risk-reduction per unit of work.*

- **INPUTS**: one legally obtainable real artifact per ecosystem. Candidates
  established as reachable: Modrinth API (Minecraft, already used); permissively
  licensed mods published outside Nexus on their own domains or via raw file
  hosts; `s4tk` test data for DBPF; LSLib test data for LSPK.
- **AUTHORITATIVE BEHAVIOR**: the format specification already recorded as a
  source in each pack, plus an independent implementation where one exists
  (S4TK for DBPF, LSLib for LSPK) — the `esplugin` pattern.
- **PROCESS**: per ecosystem, fetch one real artifact into the evidence cache,
  assert our extraction against it, and where a second implementation exists,
  assert the two agree.
- **OUTPUT**: each inspector moves from A.2 to A.1.
- **VERIFICATION**: network-gated tests that skip offline, in the shape of
  `tests/test_real_artifacts.py`. Control: a corrupt artifact must be reported
  unreadable, not silently parsed.
- **LIMITATIONS**: one artifact per ecosystem proves the common case, not the
  long tail of malformed real-world files.

### T3 — Back the `dependency_analysis` claim for Stardew, PZ and RimWorld

*Depends on: T2 (needs real artifacts to be meaningful).*

- **INPUTS**: `analyze/requirements.py`; SMAPI `Dependencies` and
  `ContentPackFor`; PZ `require=`; RimWorld `modDependencies`.
- **AUTHORITATIVE BEHAVIOR**: each loader's own dependency semantics, already
  recorded as sources in the packs.
- **PROCESS**: no new resolver. Add per-game tests exercising the existing one,
  then raise each pack's status from `documented_only` to the truth.
- **OUTPUT**: three packs stop understating what works.
- **VERIFICATION**: per game, a missing-dependency positive and a
  satisfied-dependency control.
- **LIMITATIONS**: resolves declared dependencies only; says nothing about what
  the code actually requires.

### T4 — Content Patcher Load Priority

*Depends on: T2 (a real content pack).*

- **INPUTS**: `content.json` entries; the Content Patcher documentation already
  recorded in `packs/stardewvalley/sources.yaml`.
- **AUTHORITATIVE BEHAVIOR**: Content Patcher's documented conflict resolution.
  Currently `collision.contentpatcher_load` reports two packs loading one asset
  but cannot say which wins, because priority is not extracted.
- **PROCESS**: extract the priority field; order colliding Loads by it; report
  the predicted loser explicitly.
- **OUTPUT**: a conflict finding that names the losing pack.
- **VERIFICATION**: two packs with differing priority → correct loser named.
  Control: equal priority → outcome reported as undetermined, not guessed.
- **LIMITATIONS**: predicts Content Patcher's resolution; does not establish the
  visual or gameplay result.

### T5 — BG3 `.pak` file list

*Depends on: adding `lz4` as an optional dependency.*

- **INPUTS**: LSPK v18 header (already parsed), the compressed file list block.
- **AUTHORITATIVE BEHAVIOR**: LSLib, the maintained open implementation.
- **PROCESS**: decompress the list with the `lz4` package; enumerate entries;
  feed them to the existing collision analyzer.
- **OUTPUT**: BG3 `interaction_analysis` moves from `documented_only` to
  `partial`; a bare `.pak` stops yielding header-only facts.
- **VERIFICATION**: a real `.pak` (T2) whose entry list matches LSLib's. Control:
  with `lz4` absent, coverage must still say the list was not read.
- **LIMITATIONS**: enumerates entries; does not decode stats or scripts.

### T6 — Release-report continuity as a consumable artifact

*Depends on: T1.*

- **INPUTS**: `report.release_report()` output; a player's `analyze` run.
- **AUTHORITATIVE BEHAVIOR**: the artifact sha256 is the join key. This is
  ModCheck's own contract, not an upstream one.
- **PROCESS**: define a versioned release-report file a creator ships; have
  `analyze` ingest one when the installed artifact's hash matches, and surface
  the creator's recorded findings and resolutions alongside the player's own.
- **OUTPUT**: a player does not repeat the creator's diagnostics.
- **VERIFICATION**: end-to-end — build, emit report, analyse the same jar in a
  player configuration, assert creator findings appear. Control: a report whose
  hash does not match the installed artifact must be ignored and said so.
- **LIMITATIONS**: binds report to bytes; does not prove the report's claims.

### T7 — Second loader: NeoForge project setup

*Depends on: nothing; validates that the creator path generalises.*

- **INPUTS**: NeoForge's own MDK/template and documented toolchain versions,
  resolved live as the Fabric generator does.
- **AUTHORITATIVE BEHAVIOR**: upstream template, read at generation time — not a
  version table we would have to maintain.
- **PROCESS**: a second generator behind the existing `GENERATORS` registry.
- **OUTPUT**: `project_setup` and `build_execution` for a second loader.
- **VERIFICATION**: generate and run a real build; inspect the jar back.
  Control: a version with no upstream template must fail with a clear error, not
  emit a guessed project.
- **LIMITATIONS**: proves compile and package only.

### T8 — Analysis cache keyed by identity, context, analyzer and rule revision

*Depends on: T1 (needs a stable code/rule identity).*

- **INPUTS**: artifact sha256, game/toolchain context, analyzer version, source
  `content_sha256`.
- **AUTHORITATIVE BEHAVIOR**: our own invalidation policy — conservative, per
  section 8 of the product definition.
- **PROCESS**: cache keyed on all four; invalidate on any change; reuse the
  existing staleness propagation for rule revisions.
- **OUTPUT**: repeat analysis avoids re-parsing 1MB rule sets (already measured:
  parsing dominated the evaluation suite at 11.7s before an in-process cache cut
  it to 2.1s).
- **VERIFICATION**: changing any key component must miss the cache. Control: an
  unchanged input must hit.
- **LIMITATIONS**: correctness of reuse rests on the key being complete.

### T9 — Isolated execution for builds

*Depends on: T7 (more builds make this matter).*

- **INPUTS**: an untrusted project directory.
- **AUTHORITATIVE BEHAVIOR**: a build script is arbitrary code. Current state:
  explicit `--allow-execute` and a scrubbed environment, but **no sandbox** —
  builds run as the current user.
- **PROCESS**: run builds in a container or equivalent boundary with no
  credentials, no host filesystem beyond the project, and a network policy
  limited to the build's declared repositories.
- **OUTPUT**: `build_execution` becomes safe for third-party projects.
- **VERIFICATION**: a build attempting to read outside the project must fail.
  Control: a legitimate build must still succeed.
- **LIMITATIONS**: bounds filesystem and credential exposure; not a defence
  against everything a malicious build could attempt.

---

## E. Register of genuine unknowns

Each is a fact we do not have, with the smallest procedure that would settle it.
None blocks the tasks above.

| # | Unknown | Why it matters | Smallest decisive procedure |
|---|---|---|---|
| E1 | Which Sims 4 package wins a resource-key collision | `collision.sims4_resource_key` names a collision but refuses to name a loser | Read the load order logic in an open Sims 4 tool (S4TK or Sims 4 Studio); if unstated, it stays unresolved and the finding keeps refusing |
| E2 | The CPython version the Sims 4 game embeds | our inspector reports what was built, not whether the game accepts it | Check the version S4TK or a maintained script-mod template pins, and cite that; a community tutorial recommending 3.7 for *decompiler* reasons is not evidence |
| E3 | Cyberpunk install-path resolution order | decides which of two colliding mods applies | Read the REDmod/archive loader order in the REDmodding wiki's load-order page or CET source |
| E4 | Whether Mutagen/Synthesis support Fallout: New Vegas | decides whether FNV patcher recipes are viable | Inspect the `GameRelease` enum in the Mutagen.Bethesda NuGet package contents |
| E5 | Content Patcher Load Priority semantics | required by T4 | Read the Content Patcher docs already recorded in the Stardew pack |
| E6 | Bethesda record/field semantics | blocks field-level conflict reporting for three games | UESP is 403; use xEdit's own record definitions as the specification instead |
| E7 | `.archive` (RDAR/CR2W) structure | blocks Cyberpunk resource-level analysis | Read WolvenKit's format implementation (GPL-3.0 — reference only, do not vendor) |

---

## F. First implementation batch

Scoped to what is unblocked today and verifiable on completion.

**Batch 1 = T1 + T2 + T3.**

| | |
|---|---|
| **Components to change** | `src/modcheck/analyze/findings.py` (new `CODES` registry), `src/modcheck/validate.py` (check_id rule), 6 failure records across `packs/{sims4,cyberpunk2077,bg3}/failures/`, `tests/test_real_artifacts.py` (extend beyond Minecraft), `packs/{stardewvalley,projectzomboid,rimworld}/pack.yaml` (status correction) |
| **Existing tools to invoke** | Modrinth API (Minecraft artifacts); S4TK and LSLib as independent implementations to agree with; `esplugin` helper already built |
| **Fixtures to use** | keep the synthetic builders as unit fixtures; add one real artifact per ecosystem in the gitignored evidence cache, never committed |
| **Acceptance tests** | (1) every `code=` literal in `src/` is registered; (2) `modcheck validate` errors on an unregistered `check_id`, and passes on the corrected records; (3) per newly-validated ecosystem, a real artifact parses and a corrupt one is reported unreadable; (4) per game in T3, a missing-dependency positive and a satisfied control; (5) `modcheck evaluate` still reports 0 false warnings and 0 error findings on controls |

**Done means**: `modcheck validate` at 0 errors; the full suite green; the
evaluation suite still at 0 false warnings; and the capability matrix changed
only where a test now backs the change — in either direction.

**Explicitly not in batch 1**: sandboxing, the cache, NeoForge, BG3 `.pak`
contents. Each is sequenced above with its contract.

---

## What this brief does not promise

No delivery dates. No universal compatibility. No game is called supported as a
whole, and `runtime_testing` is `blocked` for all ten because we do not have the
games. Every capability claim in this repository names a test, or it is not
called supported.

# ModCheck

A game-specific mod development and compatibility platform, for creators and
players, over one maintained body of evidence.

**Status: early implementation.** Support is tracked per capability per game and
nothing is presented as more finished than it is. Run `modcheck packs` to see
exactly what works.

This lives in `modcheck/` as a self-contained subtree. It does not touch the EDR
platform in the rest of this repository.

## What it does today

```
# What is actually supported, per capability, per game
modcheck packs

# Read a real mod artifact
modcheck inspect ~/mods/SomeMod.jar

# Analyse a configuration
modcheck analyze --game skyrimse --config my-load-order.json

# Creator path: project -> reviewable change -> build -> release report
modcheck scaffold mymod --game minecraft --loader fabric
modcheck apply fabric.add_mixin build_workspaces/mymod --set target_class=net.minecraft.world.item.ItemStack
modcheck build build_workspaces/mymod --allow-execute
modcheck release-report build_workspaces/mymod/build/libs/mymod-1.0.0.jar --game minecraft

# Knowledge base
modcheck search "mixin"
modcheck validate
modcheck sources verify        # re-fetch every source, report drift
modcheck evaluate              # detection AND false-warning measurement
```

## The one rule that shapes everything

**Unknown is not false.**

If ModCheck cannot see whether a file is installed, it says *unresolved*, not
*absent*. Collapsing unknown into false is how a tool confidently reports a
missing requirement that is in fact installed, or a clean bill of health it has
not earned.

This is structural, not aspirational:

- LOOT conditions evaluate in three-valued logic (`tri_and(False, None) is False`,
  `tri_or(True, None) is True`, everything else unknown);
- configurations carry `files_known_complete`, and absence is only evidence when
  it is set;
- a missing dependency is reported as `presence_unresolved` from a partial view
  and `missing` only from a complete one;
- an undecidable version constraint is `constraint_undecided`, never a pass.

Reports say **"passed these checks under these conditions"**, never "compatible".
Every report lists what was *not* checked. There are no confidence percentages,
because we have no calibration to justify one.

## Reuse over reimplementation

The knowledge mostly already exists. ModCheck's job is to connect it to exact
projects and releases, not to rediscover it.

| Upstream | Used for | Licence |
|---|---|---|
| LOOT masterlists (Skyrim SE, Fallout 4, Fallout: New Vegas) | requirements, incompatibilities, load order, CRC-matched dirty edits | CC0-1.0, verified |
| SMAPI metadata (Stardew Valley) | per-version Obsolete / AssumeBroken / AssumeCompatible status | LGPL-3.0 repo; read as data, not redistributed |
| RimSort Community Rules (RimWorld) | load order and incompatibility rules | no licence file — reuse unknown, not redistributed |
| Fabric Loom, Gradle | actually building Minecraft mods | MIT, verified |
| Fabric meta / Mojang manifest / example mod | live toolchain version resolution | CC0-1.0 template, verified |

Their semantics are preserved rather than approximated. The LOOT integration
implements the real metadata grammar — a plugin name is a regex only when it
contains `:\*?|`, `after`/`req`/`inc` names never are, messages substitute from
`subs`, and any entry may be condition-gated.

Nothing is redistributed. Sources are fetched with provenance (url, time, HTTP
status, sha256, size, upstream revision) into a gitignored cache; the provenance
is committed. `modcheck sources verify` re-fetches and reports drift.

## Precise facts, not labels

A label like `Combat.Stamina` is for navigation. It never establishes a conflict.
Compatibility claims carry exact artifact identity (sha256 or CRC), a structured
target (record / field / path / method signature / event), an operation, the
conditions under which they hold, and the ecosystem's actual composition rule.

Concretely: a LOOT dirty-edit record applies to the copy of `Update.esm` whose
**CRC matches**. A same-named plugin does not inherit it. There is a test for
exactly that.

Collisions use each ecosystem's own composition rule. Two Sims 4 packages with
the same resource key collide; two Stardew content packs editing *different
entries* of one asset do not, because that is the supported way they coexist.
Minecraft jars sharing a class path are not reported at all — that is usually a
shaded library and we cannot tell the cases apart.

## Creation is first-class

The creator path produces artifacts, not advice:

1. `scaffold` generates a real project. Every toolchain version is resolved
   live from upstream, including whether the target Minecraft version still
   needs Mojang mappings.
2. `apply` produces a unified diff and writes nothing unless asked.
3. `build` runs the project's own build and records toolchain and artifact
   hashes.
4. `release-report` ties source, build, artifact hash and findings together.

The artifact's sha256 is the join: a player analysing the same bytes gets the
same findings plus whatever their configuration adds. They do not repeat the
creator's diagnostics.

Verified end to end here: a generated Fabric project for **Minecraft 26.3**
builds with Gradle 9.5.1 and JDK 25, an applied change still builds, and our own
inspector reads the resulting jar back.

## Evaluation

Every positive case is paired with a **control** where the right answer is
silence, because a checker that flags every overlap is worthless. `modcheck
evaluate` reports detections, misses, false warnings, and error-level findings
on controls.

It also states what the numbers do not mean: correctly applying a rule upstream
already recorded is not evidence ModCheck would have predicted the failure.
Metrics needing real usage — human correction rate, time to resolution,
edit-to-feedback latency — are listed as not yet measured, not estimated.

## Security

- Fetched docs, repos and mod contents are **untrusted input**: stored and
  hashed, never executed, never treated as instructions.
- Inspection never runs mod code, and refuses oversized archive members.
- `build` executes the project's build script, so it requires `--allow-execute`
  and runs with a scrubbed environment — credential-shaped variables are
  dropped, and passing one in is refused.
- ModCheck never touches a game installation or saves.
- **Builds are not sandboxed.** They run as the current user in the project
  directory. Do not run `modcheck build` on a project you do not trust.

## Layout

```
schemas/       the knowledge contract (JSON Schema)
packs/<game>/  sources, recipes, failures, interactions, resolutions,
               examples, attestations
src/modcheck/
  acquire.py     fetching with provenance; drift verification
  store.py       loading and indexing packs
  validate.py    schema + integrity enforcement
  inspect/       real artifact parsers per format family
  analyze/       configurations, versions, dependencies, collisions
  integrations/  upstream databases and tooling
  creator/       scaffold, apply, build, attestations
  report.py      creator and player reports over the same analyzers
  evaluate.py    the evaluation harness
evaluation/cases/  cases, kept apart from the knowledge base
docs/SPEC.md       product specification and knowledge contract
```

## Enforced, not merely intended

`modcheck validate` fails the knowledge base on:

- evidence citing a source that does not exist;
- a source we could not actually read (403, deleted, Discord-only) backing
  anything other than `unresolved` evidence — it is a **gap**, and must say what
  we therefore do not know;
- a capability marked `supported` with no test node id;
- a recipe claiming `build_tested` with no attestation from a run that happened;
- dangling references, duplicate ids, game mismatches.

## Development

```bash
cd modcheck
python3 -m venv .venv && ./.venv/bin/pip install -e ".[dev]"
./.venv/bin/python -m pytest tests/ -q

# Real builds: needs network, a JDK 25 toolchain and a few minutes
MODCHECK_BUILD_TESTS=1 ./.venv/bin/python -m pytest tests/ -q
```

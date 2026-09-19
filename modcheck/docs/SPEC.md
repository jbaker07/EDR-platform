# ModCheck — product specification and knowledge contract

Status: early implementation. This document describes what ModCheck is, what it
promises, and the rules its data must obey. Where something is not implemented,
it says so.

## 1. What ModCheck is

A game-specific mod development and compatibility platform serving creators and
players from one maintained body of evidence.

The creator workflow:

    idea → applicable approach → existing template or project → reviewable
    changes → build → supported checks and targeted tests → release report →
    ongoing maintenance

The player workflow:

    exact configuration → applicable requirements and findings → clear
    explanation → known resolutions → diagnostic evidence when unresolved

The two meet at the artifact. A creator's release report and a player's
analysis of the same bytes are produced by the same analyzers over the same
knowledge, joined by the artifact's sha256. The player should not have to redo
the creator's diagnostic work.

Creation is a first-class capability. ModCheck generates real projects and real
reviewable diffs, and runs real builds. It is not a manifest checker with
documentation attached.

## 2. What ModCheck promises, and what it refuses to say

It reports **"passed these checks under these conditions"**. It never reports
"compatible" or "safe".

Every report separates:

| class        | meaning                                              |
|--------------|------------------------------------------------------|
| `declared`   | an author or manifest says so                        |
| `extracted`  | read mechanically out of the bytes                   |
| `derived`    | produced by our analysis from the above              |
| `observed`   | seen in a run that actually happened                 |
| `unresolved` | evidence insufficient to decide                      |

Every report states what was **not** checked. A clean result is never a
guarantee, and the report says so in those words.

There are no numeric confidence scores. An invented percentage would imply a
calibration we do not have.

### Things ModCheck must never assume

- that two mods touching one function are incompatible — overlap is not conflict;
- that different files imply no interaction;
- that different fields merge safely;
- that a compatibility patch proves every version of its parents conflict;
- that a README describes the shipped binary;
- that a creator's declaration equals an executed test;
- that matching version strings mean matching artifacts — identity is a hash;
- that a successful launch, or a successful build, means gameplay and saves are safe.

### Unknown is not false

The single most important rule. If ModCheck cannot see whether a file is
installed, the answer is *unresolved*, not *absent*. Collapsing unknown into
false is how a tool reports a missing requirement that is in fact installed, or
a clean bill of health it has not earned. This is enforced structurally: LOOT
conditions evaluate in three-valued logic, configurations carry
`files_known_complete`, and dependency resolution downgrades `missing` to
`presence_unresolved` when the configuration is a partial view.

## 3. Capability model

Support is tracked **per capability, per game**. There is no global "supported"
flag, and no game is presented as supported as a whole.

| capability | meaning |
|---|---|
| `docs_guidance` | documentation and implementation guidance |
| `project_setup` | generating a real project |
| `source_edit` | producing reviewable source/data/config changes |
| `package_inspection` | reading real artifacts |
| `dependency_analysis` | requirement and version resolution |
| `interaction_analysis` | known-interaction analysis |
| `build_execution` | running the project's build |
| `runtime_testing` | executing against the game |
| `release_maintenance` | release reports and staying current |

Statuses: `supported`, `partial`, `documented_only`, `unsupported`, `blocked`.

**`supported` requires at least one test node id that exercises it.** This is
enforced by `modcheck validate`, not by convention. `blocked` requires a stated
blocker.

## 4. The knowledge contract

Records are YAML under `packs/<game>/`, validated against `schemas/*.schema.json`.

| record | holds |
|---|---|
| `source` | one retrieved upstream artifact with provenance |
| `recipe` | a procedure a creator can follow |
| `failure` | a real sourced failure mode and its established cause |
| `interaction` | how specific modifications compose |
| `resolution` | a fix and the scope of its verification |
| `example` | a reusable template, library, analyzer or tool |
| `pack.yaml` | the game's ecosystem, capabilities, toolchain and gaps |

### Precise facts, not labels

A label like `Combat.Stamina` is for navigation. It never establishes a
conflict. Compatibility-relevant records carry:

- exact artifact identity (sha256 or CRC) where available;
- game, loader and toolchain context;
- a structured `target`: record, field, path, method signature, event or
  injection point;
- an `operation`: replace, wrap, append, subscribe, require, provide, …;
- `conditions` under which the statement holds;
- the ecosystem's actual `composition` rule;
- evidence and provenance;
- `analysis_coverage`: what was checked and what was not.

### Enforced integrity rules

`modcheck validate` fails on:

- evidence citing a source that does not exist;
- a source we could not actually read (403, deleted, Discord-only) backing
  anything other than `unresolved` evidence — such a source is a **gap**, and
  must say what we therefore do not know;
- a capability marked `supported` with no test;
- a recipe claiming `build_tested` without an attestation from a run that
  actually happened;
- dangling cross-record references, duplicate ids, game mismatches.

It warns on overbroad version claims, conflicts derived with no stated
conditions, and failures marked detectable that name no implemented check.

### Keeping knowledge current

Every source records url, retrieval time, HTTP status, sha256, size and
whatever revision identity the server gave. `modcheck sources verify`
re-fetches and reports drift. Cached bytes are gitignored — they are
re-fetchable and frequently not ours to redistribute. The provenance, which is
ours, is committed.

Not yet implemented: automatic revalidation of records that depend on a changed
source. Drift is currently reported, not propagated.

## 5. Reuse posture

Our contribution is connecting evidence to exact projects and releases,
development workflows, cross-tool interpretation, version-aware impact
analysis, maintained interaction knowledge, and creator-to-player continuity.

We do not rebuild parsers, compilers or analyzers to own more code. Where a
maintained upstream exists — the LOOT masterlists, Fabric Loom, the loaders'
own metadata APIs — we use it and preserve its semantics rather than
approximating them.

Reuse terms are recorded per source. Public availability implies nothing about
redistribution, commercial use or training rights; `unknown` is an honest value
and is used where a licence was not verified.

## 6. Security posture

- Fetched documentation, repositories and mod contents are **untrusted input**.
  They are stored and hashed, never executed, and never treated as instructions.
- Static inspection and execution are separate. Inspection never runs mod code.
- Running a build executes the project's build script, which is untrusted code.
  It requires explicit authorisation (`--allow-execute`) and runs with a
  scrubbed environment: credential-shaped variables are dropped, and passing one
  in is refused.
- Inspection refuses oversized archive members rather than inflating them.
- ModCheck does not touch a user's game installation or saves.
- Creator source and diagnostics stay local. Nothing is sent to an external
  service.

Not implemented: a real execution sandbox. Builds run as the current user in the
project directory. A git worktree is not a security boundary, and neither is
this. Do not run `modcheck build` on a project you do not trust.

## 7. Architecture

One modular Python package, files as the source of truth, no database.

```
schemas/      the knowledge contract
packs/<game>/ versioned game packs: sources, recipes, failures, interactions,
              resolutions, examples, attestations
src/modcheck/
  acquire.py       fetching with provenance; drift verification
  store.py         loading and indexing packs
  validate.py      schema + integrity enforcement
  inspect/         real artifact parsers, one per format family
  analyze/         configurations, version comparison, dependency resolution
  integrations/    upstream tooling and databases (LOOT today)
  creator/         project setup, reviewable changes, builds, attestations
  report.py        creator and player reports over the same analyzers
  evaluate.py      the evaluation harness
  cli.py           the execution interface
evaluation/cases/  evaluation cases, kept apart from the knowledge base
```

Deliberately not built: a mod manager, a launcher, a marketplace, a
filesystem virtualisation layer, a universal sandbox, an IDE, a graph database,
a model, or ten separate applications.

## 8. Evaluation

Every positive case is paired with a control where the correct answer is
silence. Both count. The harness reports detections, misses, false warnings and
error-level findings on controls.

It also states what the numbers do not mean: correctly applying a rule that
upstream already recorded is not evidence ModCheck would have predicted the
failure beforehand.

Metrics that need real usage — human correction rate, time to a correct
resolution, creator task completion, edit-to-feedback latency — are listed as
not yet measured rather than estimated.

---
type: "workflow"
id: "wf.integration.mixin_coexistence"
area: "integration"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Coexist with other mods' mixins

**Intent.** The mod's bytecode changes and everyone else's must apply together. When they cannot, the creator wants to know before players do.

## Must be preserved

- Fabric API's injections: know, per injector pair, what the transformer does ([[30-Mechanisms/Mixin|Mixin]], executed_transformation).

## Mechanisms that can serve it

- [[30-Mechanisms/Mixin|Mixin]]; the generated shared-target index (potential interactions only); the composition table in _authored/mechanisms/mixin.md.

## Tools and artifacts used today

- Reading the other mod's mixin configs by hand; running the game.

## Decisions the creator must make

- Injector kind and priority; whether to require or soft-fail the injection (require=0 does not avoid the merged-by refusal: [[30-Mechanisms/Transformation_Tests#B2|scenario B2]]).

## Information those decisions need

- Both mods' targets, exactly resolved (atlas/extract/jvm.py + resolve.py do this for Fabric API; not yet wired into inspection: [[80-Unresolved/q.inspector_mixin_targets|q.inspector_mixin_targets]]).
- Applicability: environment of both mixins.
- The composition rule for the injector pair (`extracted/mixin_transformation_tests.json`).

## Existing automation

- The atlas extractor over Fabric API only.

## Remaining manual or unsupported work

- The pairwise check for arbitrary jars, in the order index -> exact resolution -> applicability -> composition rule ([[80-Unresolved/q.mixin_collision_analyser|q.mixin_collision_analyser]]).

## ModCheck's contribution

- The collision analyser is the highest-value unimplemented item in the backlog.

## Interactions to check

- Same-method injections (potential); same-call-site redirects (fail: [[30-Mechanisms/Transformation_Tests#C|scenario C]]); overwrites versus INVOKE-point injections (priority-dependent: [[30-Mechanisms/Transformation_Tests#B|scenario B]], [[30-Mechanisms/Transformation_Tests#K|scenario K]]); cancelling HEAD injections ([[30-Mechanisms/Transformation_Tests#H|scenario H]]).

## Evidence

- `extracted/edges.json#shared_targets`
- `extracted/edges.json#injects_into`
- `extracted/mixin_transformation_tests.json`

## Open questions

- [[80-Unresolved/q.mixin_collision_analyser|q.mixin_collision_analyser]]
- [[80-Unresolved/q.inspector_mixin_targets|q.inspector_mixin_targets]]
- [[80-Unresolved/q.mixin_docs_application_order|q.mixin_docs_application_order]]
- [[80-Unresolved/q.runtime_mixin_application|q.runtime_mixin_application]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: True
- interaction_analysed: True
- implemented_in_modcheck: False
- validated_scope: none

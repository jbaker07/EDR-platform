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

- Fabric API's injections (never Overwrite an injected method).

## Mechanisms that can serve it

- [[30-Mechanisms/Mixin|Mixin]]; the generated contested-method list; the analyst note under _authored/mechanisms.

## Tools and artifacts used today

- Reading the other mod's mixin configs by hand; running the game.

## Decisions the creator must make

- Injector kind and priority; whether to require or soft-fail the injection.

## Information those decisions need

- Both mods' targets (extractable with atlas/extract/jvm.py, not yet wired into inspection: [[80-Unresolved/q.inspector_mixin_targets|q.inspector_mixin_targets]]).
- Application order ([[80-Unresolved/q.mixin_docs_application_order|q.mixin_docs_application_order]]).

## Existing automation

- The atlas extractor over Fabric API only.

## Remaining manual or unsupported work

- The pairwise check for arbitrary jars ([[80-Unresolved/q.mixin_collision_analyser|q.mixin_collision_analyser]]).

## ModCheck's contribution

- The collision analyser is the highest-value unimplemented item in the backlog.

## Interactions to check

- Same-method injections; same-call-site redirects; any Overwrite.

## Evidence

- `extracted/edges.json#injects_into`
- `extracted/edges.json#wraps`
- `extracted/fabric_api.json`

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

---
type: "workflow"
id: "wf.integration.dependencies_conditions"
area: "integration"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Depend on, or adapt to, other mods

**Intent.** The mod needs another mod (hard dependency), works better with one (optional), or must refuse to run with one (break). Players should get a clear message, not a stack trace.

## Must be preserved

- The other mod's behaviour; integration must not patch it.

## Mechanisms that can serve it

- fabric.mod.json depends / recommends / breaks / provides ([[00-Scope/Sources|fabric_mod_json_spec]]; every Fabric API module's own declaration is in `extracted/fabric_api.json`).
- [[40-Interfaces/net.fabricmc.loader.api.FabricLoader|FabricLoader]] isModLoaded for optional integration.
- [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] for data files that load only when a mod is present.
- [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] for capability-style lookups without hard class references.

## Tools and artifacts used today

- The failure record for a missing dependency (`failure/missing_required_dependency_prevents_launch`) and ModCheck's inspector reading depends.

## Decisions the creator must make

- Hard, optional or negative dependency.
- Version range expressions.

## Information those decisions need

- Exact mod ids and versions of the counterpart (from its jar, inspected).

## Existing automation

- Inspector reads dependencies; the failure record has a detector.

## Remaining manual or unsupported work

- Optional-integration code.

## ModCheck's contribution

- Delivered: dependency inspection and the missing-dependency failure; the mixin dimension is separate ([[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]]).

## Evidence

- `failure/missing_required_dependency_prevents_launch`
- [[00-Scope/Sources|fabric_mod_json_spec]]
- `extracted/fabric_api.json`

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: True
- interaction_analysed: True
- implemented_in_modcheck: True
- validated_scope: inspector and detector have unit tests; no game run

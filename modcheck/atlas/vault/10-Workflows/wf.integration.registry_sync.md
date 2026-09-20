---
type: "workflow"
id: "wf.integration.registry_sync"
area: "integration"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Keep registries consistent between server and client

**Intent.** A client with a different mod set than the server must be told so cleanly, and ids must map identically on both sides for the content that is shared.

## Must be preserved

- Vanilla's own id assignment.

## Mechanisms that can serve it

- [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] -- id sync during configuration; [[50-Interactions/events/net.fabricmc.fabric.api.event.registry.DynamicRegistrySetupCallback.EVENT|EVENT]] before dynamic registries load.

## Tools and artifacts used today

- Nothing to write; the module handles it when both sides have Fabric API.

## Decisions the creator must make

- Whether a registry entry is optional for the client (the module supports marking).

## Information those decisions need

- The module's API for optional entries (in its extracted surface).

## Existing automation

- The module itself.

## Remaining manual or unsupported work

- None for the common case.

## ModCheck's contribution

- A check that a mod registering content declares fabric-api (or the sync module) as a dependency.

## Evidence

- `extracted/fabric_api.json#fabric-registry-sync-v0`
- `extracted/edges.json#publishes_event`

## Open questions

- [[80-Unresolved/q.registry_freeze_timing|q.registry_freeze_timing]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none

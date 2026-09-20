---
type: "contract"
subject: "event:net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_LOAD"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_LOAD

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_LOAD|ENTITY_LOAD]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Fired from an @Inject at TAIL of ServerLevel$EntityCallbacks.onTrackingStart, so it fires whenever an entity starts being tracked in a level -- on spawn and on chunk load alike. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |
| Callback onLoad(Entity, ServerLevel); a listener must therefore be idempotent per entity because chunk unload/reload fires it again. | `analyst_inference` | `extracted/edges.json#callback_of` |

## Not established

- Whether the entity's own data (attachments, NBT) is fully loaded before onTrackingStart.

## Evidence

- `extracted/edges.json#publishes_event`

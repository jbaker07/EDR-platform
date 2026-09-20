---
type: "contract"
subject: "event:net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.END_CLIENT_TICK"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.END_CLIENT_TICK

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.END_CLIENT_TICK|END_CLIENT_TICK]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Fired from MinecraftMixin.onEndTick, an @Inject at RETURN of Minecraft.tick; client environment only. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |
| A dedicated server never loads this class; referencing it from a common entrypoint is a class-loading failure on the server. | `declared` | `extracted/fabric_api.json#fabric-lifecycle-events-v1` |

## Not established

- Thread identity of Minecraft.tick beyond "the client's main thread" by convention.

## Evidence

- `extracted/edges.json#publishes_event`

---
type: "contract"
subject: "event:net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTED"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTED

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTED|SERVER_STARTED]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Fired from an @Inject into MinecraftServer.runServer at the INVOKE of buildServerStatus, after initServer has returned, so levels exist. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |
| Callback signature onServerStarted(MinecraftServer); no return value. | `declared` | `extracted/edges.json#callback_of` |

## Not established

- Whether every level has finished its initial chunk load by then.

## Evidence

- `extracted/edges.json#publishes_event`

---
type: "contract"
subject: "event:net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTING"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTING

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTING|SERVER_STARTING]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Fired from an @Inject into MinecraftServer.runServer at the INVOKE of initServer, so it runs before the server initialises levels. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |
| Levels are not yet available in the callback; a listener that needs a ServerLevel must wait for SERVER_STARTED or ServerLevelEvents.LOAD. | `analyst_inference` | `extracted/edges.json#injects_into` |

## Not established

- Exactly which server fields are initialised at that point (runServer's body is not disassembled in the atlas; [[80-Unresolved/q.injection_points_not_resolved|q.injection_points_not_resolved]]).

## Evidence

- `extracted/edges.json#publishes_event`

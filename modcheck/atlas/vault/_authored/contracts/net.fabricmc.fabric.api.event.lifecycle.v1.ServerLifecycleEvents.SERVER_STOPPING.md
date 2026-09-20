---
type: "contract"
subject: "event:net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STOPPING"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STOPPING

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STOPPING|SERVER_STOPPING]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Fired from an @Inject at HEAD of MinecraftServer.stopServer, so levels are still loaded and saveable when the listener runs. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |

## Not established

- Whether stopServer is reached on every shutdown path (a crash may bypass it).

## Evidence

- `extracted/edges.json#publishes_event`

---
type: "contract"
subject: "event:net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_SERVER_TICK"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_SERVER_TICK

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_SERVER_TICK|END_SERVER_TICK]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Callback signature is onEndTick(MinecraftServer); no return value, so a listener cannot cancel or alter anything through the event itself. | `declared` | `extracted/edges.json#callback_of`; `extracted/fabric_api.json#fabric-lifecycle-events-v1` |
| Fired from MinecraftServerMixin.onEndTick, an @Inject at TAIL of MinecraftServer.tickServer, so it runs after every level and player has ticked for that server tick. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |
| Runs on the server thread, because tickServer is called from the server run loop and the injection is inline. | `analyst_inference` | `extracted/minecraft_members.json`; [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] |
| Applies on both the dedicated server and the integrated server (mixin environment is both). | `declared` | `extracted/fabric_api.json#fabric-lifecycle-events-v1` |

## Not established

- That the callback is invoked at all in a running 26.3 game ([[80-Unresolved/q.runtime_event_delivery|q.runtime_event_delivery]]).
- The order in which listeners from different mods run ([[80-Unresolved/q.event_phase_ordering|q.event_phase_ordering]]).
- The tick-time cost a listener can afford ([[80-Unresolved/q.runtime_performance|q.runtime_performance]]).

## Evidence

- `capability/subscribe_event.fabric_server_tick`
- `extracted/edges.json#publishes_event`

## Open questions

- [[80-Unresolved/q.runtime_event_delivery|q.runtime_event_delivery]]
- [[80-Unresolved/q.event_phase_ordering|q.event_phase_ordering]]

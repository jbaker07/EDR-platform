---
type: "contract"
subject: "event:net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_LEVEL_TICK"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_LEVEL_TICK

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_LEVEL_TICK|END_LEVEL_TICK]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Callback signature is onEndTick(ServerLevel); the level is the unit, so the listener is called once per loaded level per tick. | `declared` | `extracted/edges.json#callback_of` |
| Fired from ServerLevelMixin.endLevelTick, an @Inject at TAIL of ServerLevel.tick. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |
| The ServerLevel handed in is the one whose weather, game rules (getGameRules() is declared on ServerLevel, not Level) and saved data a per-level feature should read. | `direct_reference` | `extracted/minecraft_surface.json.gz`; [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]] |
| This is the event ModCheck's server-tick generator subscribes to (used by the discarded lantern scaffold, not by the hand-authored reference lantern, which ticks per block entity); the earlier name END_WORLD_TICK does not exist in this Fabric API and fails to compile. | `direct_reference` | `capability/subscribe_event.fabric_server_tick` |

## Not established

- Whether ServerLevel.tick can be invoked off the server thread for any level in 26.3 (assumed not; unobserved).
- Listener ordering across mods.

## Evidence

- `capability/subscribe_event.fabric_server_tick`
- `extracted/edges.json#injects_into`

## Open questions

- [[80-Unresolved/q.runtime_event_delivery|q.runtime_event_delivery]]

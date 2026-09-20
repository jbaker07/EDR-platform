---
type: "system_note"
id: "net.minecraft.server.level"
side: "server"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# ServerLevel, chunks and entity tracking

Package `net.minecraft.server.level` -- generated view: [[20-Systems/net.minecraft.server.level|hooked types]]

**Responsibility.** The authoritative per-dimension world on the server: ServerLevel.tick, chunk loading and status tasks, entity callbacks (tracking start/end), the ServerEntity pairing that decides which players receive an entity, and ServerPlayer.

**Side.** server

**Threads.** ServerLevel.tick is called from the server thread's tickChildren. Chunk status tasks run through a scheduler whose executor is not yet extracted ([[80-Unresolved/q.worldgen_threading|q.worldgen_threading]]), so listeners published from ChunkStatusTasks (CHUNK_GENERATE) have no stated thread.

**Persistence.** Per-level saved data and chunk storage are accessed through this package; the lantern's SavedData lives at this scope (`capability/persist_state.fabric_saveddata`).

## Extension points

- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_LEVEL_TICK|END_LEVEL_TICK]] is injected at TAIL of ServerLevel.tick and hands the ServerLevel to the listener.
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_LOAD|ENTITY_LOAD]] fires from ServerLevel$EntityCallbacks.onTrackingStart.
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents.START_TRACKING|START_TRACKING]] fires from ServerEntity.addPairing, the point at which a player begins receiving an entity.
- [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]] declares getGameRules(), which Level does not (`capability/add_configuration.fabric_gamerule`).

## Interactions to expect

- Per-level state keyed by the ServerLevel instance must be cleared on ServerLevelEvents.UNLOAD or it leaks across world reloads.

## Evidence

- `extracted/edges.json#publishes_event`
- `extracted/minecraft_members.json`
- `capability/subscribe_event.fabric_server_tick`

## Open questions

- [[80-Unresolved/q.worldgen_threading|q.worldgen_threading]]


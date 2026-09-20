---
type: "system_note"
id: "net.minecraft.world.level"
side: "shared_by_design"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Level, blocks, block entities, chunks and world generation

Package `net.minecraft.world.level` -- generated view: [[20-Systems/net.minecraft.world.level|inventory and hooked types]]

**Responsibility.** The shared world model used by both sides: Level and its subclasses, block and block-state definitions, block entities, chunk data, biomes, dimension types, game rules, storage and the entire world-generation package tree. 2176 top-level classes; 106 of them are hooked by Fabric API, the most of any package.

**Side.** shared_by_design

**Threads.** Level code runs on whichever side owns the instance (server thread for ServerLevel, render/client thread for ClientLevel). World-generation classes run on chunk workers by design, which is why the thread of anything under levelgen is a separate question ([[80-Unresolved/q.worldgen_threading|q.worldgen_threading]]).

**Persistence.** Block entities, chunk data and SavedData are the three persistence shapes in this package; game rules persist with the level.

## Extension points

- [[40-Interfaces/net.minecraft.world.level.Level|Level]] declares isRaining(), isThundering() and isRainingAt(BlockPos): the three readings behind [[80-Unresolved/q.lantern_rain_semantics|q.lantern_rain_semantics]].
- Blocks, block entities and biomes are registered through [[30-Mechanisms/Registries|Registries]] (BLOCK, BLOCK_ENTITY_TYPE) and extended in data through the datapack types listed in `extracted/corpus.json`.
- [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] modifies biome generation settings; [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] hooks dimension attributes ([[50-Interactions/events/net.fabricmc.fabric.api.dimension.v1.DimensionEvents.MODIFY_ATTRIBUTES|MODIFY_ATTRIBUTES]]).
- Block interaction is intercepted before vanilla by [[50-Interactions/events/net.fabricmc.fabric.api.event.player.UseBlockCallback.EVENT|EVENT]] and [[50-Interactions/events/net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.BEFORE|BEFORE]], both injected into ServerPlayerGameMode.

## Interactions to expect

- Two mods adding placed features to one biome at the same generation step can conflict ([[80-Unresolved/q.datapack_load_order|q.datapack_load_order]]).
- Block-state changes made from a tick listener are visible to all players through vanilla's own sync; block-entity data is not, unless the block entity sends an update packet.

## Evidence

- `extracted/corpus.json`
- `extracted/minecraft_surface.json.gz`
- `extracted/edges.json#injects_into`

## Open questions

- [[80-Unresolved/q.datapack_load_order|q.datapack_load_order]]
- [[80-Unresolved/q.worldgen_threading|q.worldgen_threading]]
- [[80-Unresolved/q.lantern_rain_semantics|q.lantern_rain_semantics]]


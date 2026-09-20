---
type: "system"
package: "net.minecraft.server.level"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level

Analyst note: [[_authored/systems/net.minecraft.server.level|ServerLevel, chunks and entity tracking]]

71 classes (42 top-level) across 2 packages in the processed jar; 2 changed by Loom processing; 14 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]] -- calls:1, injects_into:5, reads:2 -- by fabric-data-attachment-api-v1, fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.server.level.ChunkLevel|ChunkLevel]] -- calls:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.server.level.ChunkMap|ChunkMap]] -- calls:2, injects_into:1, reads:1 -- by fabric-data-attachment-api-v1, fabric-lifecycle-events-v1, fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.level.ChunkResult|ChunkResult]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.server.level.ClientInformation|ClientInformation]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.server.level.FullChunkStatus|FullChunkStatus]] -- calls:7, reads:13 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.server.level.GenerationChunkHolder|GenerationChunkHolder]] -- calls:2 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.server.level.ServerChunkCache|ServerChunkCache]] -- calls:1, reads:3 -- by fabric-data-attachment-api-v1, fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.level.ServerEntity|ServerEntity]] -- injects_into:2, reads:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]] -- calls:23, injects_into:3, reads:1 -- by fabric-api-lookup-api-v1, fabric-data-attachment-api-v1, fabric-events-interaction-v0, fabric-game-rule-api-v1, fabric-lifecycle-events-v1, fabric-loot-api-v3, fabric-networking-api-v1, fabric-recipe-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.server.level.ServerLevel_EntityCallbacks|ServerLevel$EntityCallbacks]] -- injects_into:2, reads:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]] -- calls:19, injects_into:7, reads:18, wraps:5 -- by fabric-data-attachment-api-v1, fabric-entity-events-v1, fabric-events-interaction-v0, fabric-menu-api-v1, fabric-networking-api-v1, fabric-particles-v1, fabric-permission-api-v1, fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.server.level.ServerPlayer_RespawnConfig|ServerPlayer$RespawnConfig]] -- calls:1 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]] -- injects_into:5, reads:2 -- by fabric-events-interaction-v0

## Declared inventory

### `net.minecraft.server.level` (37 top-level)

`BlockDestructionProgress`, `ChunkGenerationTask`, [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]], [[40-Interfaces/net.minecraft.server.level.ChunkLevel|ChunkLevel]], `ChunkLoadCounter`, [[40-Interfaces/net.minecraft.server.level.ChunkMap|ChunkMap]], [[40-Interfaces/net.minecraft.server.level.ChunkResult|ChunkResult]], `ChunkTaskDispatcher`, `ChunkTaskPriorityQueue`, `ChunkTracker`, `ChunkTrackingView`, [[40-Interfaces/net.minecraft.server.level.ClientInformation|ClientInformation]], `ColumnPos`, `DemoMode`, `DistanceManager`, [[40-Interfaces/net.minecraft.server.level.FullChunkStatus|FullChunkStatus]], `GeneratingChunkMap`, [[40-Interfaces/net.minecraft.server.level.GenerationChunkHolder|GenerationChunkHolder]], `LoadingChunkTracker`, `ParticleStatus`, `PlayerMap`, `PlayerSpawnFinder`, `SectionTracker`, `ServerBossEvent`, [[40-Interfaces/net.minecraft.server.level.ServerChunkCache|ServerChunkCache]], [[40-Interfaces/net.minecraft.server.level.ServerEntity|ServerEntity]], `ServerEntityGetter`, [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]], [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]], [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]], `SimulationChunkTracker`, `ThreadedLevelLightEngine`, `ThrottlingChunkTaskDispatcher`, `Ticket`, `TicketType`, `WorldGenRegion`, `package-info`

### `net.minecraft.server.level.progress` (5 top-level)

`ChunkLoadStatusView`, `LevelLoadListener`, `LevelLoadProgressTracker`, `LoggingLevelLoadListener`, `package-info`


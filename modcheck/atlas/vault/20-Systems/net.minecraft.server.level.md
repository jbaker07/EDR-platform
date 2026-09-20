---
type: "system"
package: "net.minecraft.server.level"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level

Analyst note: [[_authored/systems/net.minecraft.server.level|ServerLevel, chunks and entity tracking]]

71 classes in the jar. Hooked types: 13

- [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]] -- calls:1, injects_into:5 -- by fabric-data-attachment-api-v1, fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.server.level.ChunkLevel|ChunkLevel]] -- calls:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.server.level.ChunkMap|ChunkMap]] -- calls:1, injects_into:1 -- by fabric-data-attachment-api-v1, fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.server.level.ChunkResult|ChunkResult]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.server.level.FullChunkStatus|FullChunkStatus]] -- calls:5, reads:4 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.server.level.GenerationChunkHolder|GenerationChunkHolder]] -- calls:2 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.server.level.ServerChunkCache|ServerChunkCache]] -- calls:1, reads:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.server.level.ServerEntity|ServerEntity]] -- injects_into:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]] -- calls:18, injects_into:3 -- by fabric-api-lookup-api-v1, fabric-data-attachment-api-v1, fabric-events-interaction-v0, fabric-game-rule-api-v1, fabric-lifecycle-events-v1, fabric-loot-api-v3, fabric-networking-api-v1, fabric-recipe-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.server.level.ServerLevel_EntityCallbacks|ServerLevel$EntityCallbacks]] -- injects_into:2 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]] -- calls:10, injects_into:6, reads:2, wraps:3 -- by fabric-data-attachment-api-v1, fabric-entity-events-v1, fabric-events-interaction-v0, fabric-menu-api-v1, fabric-networking-api-v1, fabric-permission-api-v1, fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.server.level.ServerPlayer_RespawnConfig|ServerPlayer$RespawnConfig]] -- calls:1 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]] -- injects_into:5 -- by fabric-events-interaction-v0

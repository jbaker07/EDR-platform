---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.CHUNK_UNLOAD"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents$Unload"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.CHUNK_UNLOAD

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents$Unload`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ChunkMapMixin.onChunkUnload` @30 | [[40-Interfaces/net.minecraft.server.level.ChunkMap|ChunkMap]].`lambda$scheduleUnload$0` @Inject INVOKE `Lnet/minecraft/server/level/ChunkMap;save(Lnet/minecraft/world/level/chunk/ChunkAccess;)Z` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.

---
type: "event"
event: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientChunkEvents.CHUNK_UNLOAD"
callback: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientChunkEvents$Unload"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientChunkEvents.CHUNK_UNLOAD

Callback interface: `net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientChunkEvents$Unload`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ClientChunkCacheMixin.onChunkUnload` @20 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]].`replaceWithPacketData` @Inject NEW `net/minecraft/world/level/chunk/LevelChunk`; [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]].`drop` @Inject INVOKE `Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;drop(ILnet/minecraft/world/level/chunk/LevelChunk;)V` | unknown | static_inference |
| `ClientChunkCacheMixin.onChunkUnload` @14 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]].`replaceWithPacketData` @Inject NEW `net/minecraft/world/level/chunk/LevelChunk`; [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]].`drop` @Inject INVOKE `Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;drop(ILnet/minecraft/world/level/chunk/LevelChunk;)V` | unknown | static_inference |
| `ClientChunkCacheMixin.onUpdateLoadDistance` @32 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]].`updateViewRadius` @Inject INVOKE `Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;inRange(II)Z` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.

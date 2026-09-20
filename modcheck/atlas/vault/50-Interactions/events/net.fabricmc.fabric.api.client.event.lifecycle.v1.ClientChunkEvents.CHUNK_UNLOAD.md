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

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ClientChunkCacheMixin.onChunkUnload` | `ClientChunkCache.replaceWithPacketData` @Inject at NEW net/minecraft/world/level/chunk/LevelChunk; `ClientChunkCache.drop` @Inject at INVOKE Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;drop(ILnet/minecraft/world/level/chunk/LevelChunk;)V | client | static_inference |
| `ClientChunkCacheMixin.onChunkUnload` | `ClientChunkCache.replaceWithPacketData` @Inject at NEW net/minecraft/world/level/chunk/LevelChunk; `ClientChunkCache.drop` @Inject at INVOKE Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;drop(ILnet/minecraft/world/level/chunk/LevelChunk;)V | client | static_inference |
| `ClientChunkCacheMixin.onUpdateLoadDistance` | `ClientChunkCache.updateViewRadius` @Inject at INVOKE Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;inRange(II)Z | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.

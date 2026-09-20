---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerBlockEntityEvents.BLOCK_ENTITY_LOAD"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerBlockEntityEvents$Load"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerBlockEntityEvents.BLOCK_ENTITY_LOAD

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerBlockEntityEvents$Load`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `LevelChunkMixin.onLoadBlockEntity` @36 | [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` @ModifyExpressionValue INVOKE `Ljava/util/Map;put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | unknown | static_inference |
| `LevelChunkMixin.onLoadBlockEntity` @36 | [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` @ModifyExpressionValue INVOKE `Ljava/util/Map;put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.

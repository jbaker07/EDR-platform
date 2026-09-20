---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerBlockEntityEvents.BLOCK_ENTITY_UNLOAD"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerBlockEntityEvents$Unload"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerBlockEntityEvents.BLOCK_ENTITY_UNLOAD

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerBlockEntityEvents$Unload`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `LifecycleEventsImpl.lambda$onInitialize$3` @84 | (impl code, not a mixin) | unknown | static_inference |
| `LifecycleEventsImpl.lambda$onInitialize$2` @45 | (impl code, not a mixin) | unknown | static_inference |
| `LevelChunkMixin.onRemoveBlockEntity` @31 | [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`getBlockEntity` @Redirect INVOKE `Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`removeBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | unknown | static_inference |
| `LevelChunkMixin.onRemoveBlockEntity` @42 | [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`getBlockEntity` @Redirect INVOKE `Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`removeBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | unknown | static_inference |
| `LevelChunkMixin.onRemoveBlockEntity` @31 | [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`getBlockEntity` @Redirect INVOKE `Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`removeBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | unknown | static_inference |
| `LevelChunkMixin.onRemoveBlockEntity` @27 | [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`getBlockEntity` @Redirect INVOKE `Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`removeBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | unknown | static_inference |
| `LevelChunkMixin.onRemoveBlockEntity` @42 | [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`getBlockEntity` @Redirect INVOKE `Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`removeBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | unknown | static_inference |
| `LevelChunkMixin.onRemoveBlockEntity` @31 | [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`getBlockEntity` @Redirect INVOKE `Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`removeBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.

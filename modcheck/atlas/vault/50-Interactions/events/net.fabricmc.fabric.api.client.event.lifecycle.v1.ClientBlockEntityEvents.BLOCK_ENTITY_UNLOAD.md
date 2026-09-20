---
type: "event"
event: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientBlockEntityEvents.BLOCK_ENTITY_UNLOAD"
callback: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientBlockEntityEvents$Unload"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientBlockEntityEvents.BLOCK_ENTITY_UNLOAD

Callback interface: `net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientBlockEntityEvents$Unload`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ClientLifecycleEventsImpl.lambda$onInitializeClient$2` @45 | (impl code, not a mixin) | unknown | static_inference |
| `ClientPacketListenerMixin.onPlayerRespawn` @155 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleRespawn` @Inject NEW `net/minecraft/client/multiplayer/ClientLevel` | unknown | static_inference |
| `ClientPacketListenerMixin.onGameJoin` @155 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleLogin` @Inject NEW `net/minecraft/client/multiplayer/ClientLevel` | unknown | static_inference |
| `ClientPacketListenerMixin.onClearLevel` @151 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`clearLevel` @Inject HEAD | unknown | static_inference |
| `LevelChunkMixin.onRemoveBlockEntity` @66 | [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`getBlockEntity` @Redirect INVOKE `Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`removeBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | unknown | static_inference |
| `LevelChunkMixin.onRemoveBlockEntity` @80 | [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`getBlockEntity` @Redirect INVOKE `Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`removeBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | unknown | static_inference |
| `LevelChunkMixin.onRemoveBlockEntity` @66 | [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`getBlockEntity` @Redirect INVOKE `Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;`; [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`removeBlockEntity` @Inject INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.

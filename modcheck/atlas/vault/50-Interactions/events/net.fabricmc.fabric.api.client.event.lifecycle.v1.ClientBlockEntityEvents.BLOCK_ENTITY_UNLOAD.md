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

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ClientPacketListenerMixin.onPlayerRespawn` | `ClientPacketListener.handleRespawn` @Inject at NEW net/minecraft/client/multiplayer/ClientLevel | client | static_inference |
| `ClientPacketListenerMixin.onGameJoin` | `ClientPacketListener.handleLogin` @Inject at NEW net/minecraft/client/multiplayer/ClientLevel | client | static_inference |
| `ClientPacketListenerMixin.onClearLevel` | `ClientPacketListener.clearLevel` @Inject at HEAD | client | static_inference |
| `LevelChunkMixin.onRemoveBlockEntity` | `LevelChunk.setBlockEntity` @Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V; `LevelChunk.getBlockEntity(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelChunk$EntityCreationType;)Lnet/minecraft/world/level/block/entity/BlockEntity;` @Redirect at INVOKE Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;; `LevelChunk.removeBlockEntity` @Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V | client | static_inference |
| `LevelChunkMixin.onRemoveBlockEntity` | `LevelChunk.setBlockEntity` @Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V; `LevelChunk.getBlockEntity(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelChunk$EntityCreationType;)Lnet/minecraft/world/level/block/entity/BlockEntity;` @Redirect at INVOKE Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;; `LevelChunk.removeBlockEntity` @Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V | client | static_inference |
| `LevelChunkMixin.onRemoveBlockEntity` | `LevelChunk.setBlockEntity` @Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V; `LevelChunk.getBlockEntity(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelChunk$EntityCreationType;)Lnet/minecraft/world/level/block/entity/BlockEntity;` @Redirect at INVOKE Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;; `LevelChunk.removeBlockEntity` @Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V | client | static_inference |
| `ClientLifecycleEventsImpl.lambda$onInitializeClient$2` | (impl code, not a mixin) | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.

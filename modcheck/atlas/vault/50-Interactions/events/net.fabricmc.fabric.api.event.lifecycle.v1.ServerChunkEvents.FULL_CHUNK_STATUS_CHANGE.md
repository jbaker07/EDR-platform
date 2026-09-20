---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.FULL_CHUNK_STATUS_CHANGE"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents$FullChunkStatusChange"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.FULL_CHUNK_STATUS_CHANGE

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents$FullChunkStatusChange`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ChunkHolderMixin.updateFutures$inaccessibleToFull` @55 | [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]].`updateFutures` @Inject INVOKE `Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V` | unknown | static_inference |
| `ChunkHolderMixin.updateFutures$fullToBlockTicking` @42 | [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]].`updateFutures` @Inject INVOKE `Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V` | unknown | static_inference |
| `ChunkHolderMixin.updateFutures$blockTickingToEntityTicking` @42 | [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]].`updateFutures` @Inject INVOKE `Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V` | unknown | static_inference |
| `ChunkHolderMixin.decreaseLevel` @89 | [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]].`demoteFullChunk` @Inject HEAD | unknown | static_inference |
| `ChunkStatusTasksMixin.onChunkLoad` @137 | [[40-Interfaces/net.minecraft.world.level.chunk.status.ChunkStatusTasks|ChunkStatusTasks]].`lambda$full$0` @Inject TAIL | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.

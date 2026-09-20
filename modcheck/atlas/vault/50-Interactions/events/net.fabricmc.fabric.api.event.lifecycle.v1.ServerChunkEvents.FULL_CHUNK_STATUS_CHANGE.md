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

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ChunkHolderMixin.updateFutures$inaccessibleToFull` | `ChunkHolder.updateFutures` @Inject at INVOKE Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V | both | static_inference |
| `ChunkHolderMixin.updateFutures$fullToBlockTicking` | `ChunkHolder.updateFutures` @Inject at INVOKE Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V | both | static_inference |
| `ChunkHolderMixin.updateFutures$blockTickingToEntityTicking` | `ChunkHolder.updateFutures` @Inject at INVOKE Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V | both | static_inference |
| `ChunkHolderMixin.decreaseLevel` | `ChunkHolder.demoteFullChunk` @Inject at HEAD | both | static_inference |
| `ChunkStatusTasksMixin.onChunkLoad` | `ChunkStatusTasks.lambda$full$0` @Inject at TAIL | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.

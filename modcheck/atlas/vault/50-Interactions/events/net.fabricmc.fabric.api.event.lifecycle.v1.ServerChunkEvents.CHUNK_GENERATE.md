---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.CHUNK_GENERATE"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents$Generate"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.CHUNK_GENERATE

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents$Generate`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ChunkStatusTasksMixin.onChunkLoad` @65 | [[40-Interfaces/net.minecraft.world.level.chunk.status.ChunkStatusTasks|ChunkStatusTasks]].`lambda$full$0` @Inject TAIL | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.CHUNK_GENERATE|read it]].

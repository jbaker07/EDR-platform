---
type: "contract"
subject: "event:net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.CHUNK_GENERATE"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.CHUNK_GENERATE

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.CHUNK_GENERATE|CHUNK_GENERATE]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Fired from ChunkStatusTasksMixin.onChunkLoad, an @Inject at TAIL of ChunkStatusTasks.lambda$full$0 -- inside the chunk status task pipeline, not from ServerLevel.tick. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |
| Callback onChunkGenerate(ServerLevel, LevelChunk). | `declared` | `extracted/edges.json#callback_of` |

## Not established

- The thread this lambda runs on ([[80-Unresolved/q.worldgen_threading|q.worldgen_threading]]). A listener must not assume the server thread.

## Evidence

- `extracted/edges.json#publishes_event`

## Open questions

- [[80-Unresolved/q.worldgen_threading|q.worldgen_threading]]

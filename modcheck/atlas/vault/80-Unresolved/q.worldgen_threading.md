---
type: "question"
id: "q.worldgen_threading"
kind: "unmodeled_behaviour"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.worldgen_threading

**Question.** Which thread executes a feature's place() call and the ServerChunkEvents CHUNK_GENERATE callback -- the server thread, a chunk-generation worker, or either depending on chunk status -- and what state may a listener safely touch from there?

**Kind.** `unmodeled_behaviour` -- **Status.** open

**Why it matters.** CHUNK_GENERATE is published from ChunkStatusTasks.lambda$full$0 (an injection at TAIL of a lambda used in the chunk task pipeline). A listener that touches level state from a worker thread corrupts it; the atlas cannot yet say which thread that lambda runs on.

**Affects.** [[10-Workflows/wf.world.features_biomes|wf.world.features_biomes]], [[70-Requests/request.crystal_caves|request.crystal_caves]], [[10-Workflows/wf.behaviour.lifecycle_events|wf.behaviour.lifecycle_events]]

**Evidence already available.**
- `extracted/edges.json#publishes_event`
- `extracted/minecraft_members.json`

**Best remaining source.** The vanilla chunk task scheduler classes (net.minecraft.server.level and net.minecraft.world.level.chunk.status), extracted and read for the executor each status task is submitted to; then a runtime probe.

**Procedure.** Extract ChunkStatusTasks and its scheduler with vanilla_members.py (extra types), read which executor `full` is scheduled on, record a runs_on_thread edge with static_inference, and confirm with the probe from q.runtime_event_delivery.

**Done when.** The CHUNK_GENERATE contract has a runs_on_thread row with evidence.

**Conclusions affected while open.**
- The crystal caves request cannot state where its post-generation logic may run.

---
type: "question"
id: "q.runtime_performance"
kind: "unavailable_runtime_observation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.runtime_performance

**Question.** What is the tick-time cost of the reference lantern's per-level tick handler, and of a chunk-generation listener, on a 26.3 dedicated server at a stated player count?

**Kind.** `unavailable_runtime_observation` -- **Status.** open

**Why it matters.** The tick contracts warn that slow handlers stall every player; the atlas has no measurement to say what slow means for these handlers.

**Affects.** [[70-Requests/request.rain_lantern|request.rain_lantern]], [[70-Requests/request.crystal_caves|request.crystal_caves]], [[10-Workflows/wf.world.features_biomes|wf.world.features_biomes]]

**Evidence already available.**
- `capability/subscribe_event.fabric_server_tick`

**Best remaining source.** The vanilla profiler (net.minecraft.util.profiling, in corpus) on a headless run.

**Procedure.** Run headless with the profiler enabled around the handler; record ms/tick under a stated load.

**Done when.** The request note carries a measured cost with run provenance.

**Conclusions affected while open.**
- performance_sensitive edges cannot be emitted.

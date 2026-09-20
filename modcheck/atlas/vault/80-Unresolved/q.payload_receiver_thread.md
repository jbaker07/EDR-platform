---
type: "question"
id: "q.payload_receiver_thread"
kind: "unmodeled_behaviour"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.payload_receiver_thread

**Question.** On which thread does a ServerPlayNetworking global receiver execute for a custom payload, and does the API re-schedule to the server thread before calling the handler?

**Kind.** `unmodeled_behaviour` -- **Status.** open

**Why it matters.** A handler that mutates level state from a netty thread is a data race. The reference implementation assumes the handler runs on the server thread; the corpus has the networking module's bytecode but the atlas has not traced the dispatch path.

**Affects.** [[10-Workflows/wf.multiplayer.networking|wf.multiplayer.networking]], [[70-Requests/request.team_counter|request.team_counter]], [[70-Requests/request.rain_lantern|request.rain_lantern]]

**Evidence already available.**
- [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]]
- `capability/sync_state.fabric_custom_payload`
- `extracted/edges.json#calls`

**Best remaining source.** fabric-networking-api-v1 impl classes (ServerPlayNetworkAddon and its handler dispatch), read with javap -c for an execute()/submit call on the server before the handler invocation.

**Procedure.** Trace the receive path in the module's impl classes; record a runs_on_thread edge with static_inference; then confirm at runtime.

**Done when.** The networking workflow states the receiver thread with an edge id.

**Conclusions affected while open.**
- The generated sync_payload code's thread assumptions are unverified.

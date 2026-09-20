---
type: "question"
id: "q.runtime_event_delivery"
kind: "unavailable_runtime_observation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.runtime_event_delivery

**Question.** Does each event fire when its analyst-stated contract says it does, on the thread the contract infers, and once per the unit the callback names (per tick, per level, per player)?

**Kind.** `unavailable_runtime_observation` -- **Status.** open

**Why it matters.** The reference implementation subscribes to END_LEVEL_TICK and to a player join event; its JUnit tests exercise the handlers with fakes, not the game. Whether the game calls them is the single fact the tests cannot establish.

**Affects.** [[10-Workflows/wf.behaviour.lifecycle_events|wf.behaviour.lifecycle_events]], [[70-Requests/request.rain_lantern|request.rain_lantern]], [[70-Requests/request.team_counter|request.team_counter]]

**Evidence already available.**
- `capability/subscribe_event.fabric_server_tick`
- `extracted/edges.json#publishes_event`

**Best remaining source.** The same headless run as q.runtime_mixin_application, with a probe mod that logs each callback with thread name and tick count.

**Procedure.** Build a probe mod from the atlas' event list that registers a logging listener for every server-side event; run headless; ingest the log; write `observed` edges of relation callback_of with thread and count fields.

**Done when.** Each contract note has an "Observed" row or a recorded absence for the pinned versions.

**Conclusions affected while open.**
- All thread and frequency claims in _authored/contracts are inferences.

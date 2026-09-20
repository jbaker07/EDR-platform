---
type: "question"
id: "q.event_publishers_missing"
kind: "incomplete_extraction"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.event_publishers_missing

**Question.** For events that show no publisher in the extraction, is the invoker call made from generated code, from a lambda in a helper class outside the module's mixin and impl packages, or via a pattern the getstatic/invokeinterface scan does not match?

**Kind.** `incomplete_extraction` -- **Status.** open

**Why it matters.** An event with no known publisher cannot be placed in the vanilla call graph, so its contract (thread, side, timing) cannot even be inferred.

**Affects.** [[10-Workflows/wf.behaviour.lifecycle_events|wf.behaviour.lifecycle_events]]

**Evidence already available.**
- `extracted/edges.json#publishes_event`
- `extracted/edges.json#callback_of`

**Best remaining source.** The module jars themselves, scanned for `invoker()` calls on any class rather than only the mixin and impl packages.

**Procedure.** Widen atlas/extract/run_hooks.py to run class_refs over every class in each module jar and record event_fires on all of them; compare the publisher count per event before and after.

**Done when.** Every callback_of event has at least one publishes_event edge, or a recorded "fired only by mods" status.

**Conclusions affected while open.**
- 50-Interactions/events notes that say "no publisher found by extraction" are silent about timing and side.

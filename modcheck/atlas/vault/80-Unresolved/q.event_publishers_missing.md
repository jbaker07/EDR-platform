---
type: "question"
id: "q.event_publishers_missing"
kind: "incomplete_extraction"
status: "in_progress"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.event_publishers_missing

**Question.** Two events have no publisher found after scanning every class of every module: LevelRenderEvents.AFTER_BLOCK_OUTLINE_EXTRACTION and LevelRenderEvents.END_EXTRACTION. Are they deprecated aliases whose invoker is reached through another field, or fired only by mods?

**Kind.** `incomplete_extraction` -- **Status.** in_progress

**Why it matters.** An event with no known publisher cannot be placed in the vanilla call graph, so its contract (thread, side, timing) cannot even be inferred.

**Affects.** [[10-Workflows/wf.behaviour.lifecycle_events|wf.behaviour.lifecycle_events]]

**Evidence already available.**
- `extracted/edges.json#publishes_event`
- `extracted/edges.json#callback_of`

**Best remaining source.** The module jars themselves, scanned for `invoker()` calls on any class rather than only the mixin and impl packages.

**Procedure.** Widen atlas/extract/run_hooks.py to run class_refs over every class in each module jar and record event_fires on all of them; compare the publisher count per event before and after.

**Done when.** Both events have a publisher edge or a recorded 'declared, never fired by Fabric API' status.

**Conclusions affected while open.**
- The two event notes are silent about timing and side.

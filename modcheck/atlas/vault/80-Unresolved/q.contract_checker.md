---
type: "question"
id: "q.contract_checker"
kind: "unimplemented_automation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.contract_checker

**Question.** Can generated or user code that registers an event listener be checked against the event's analyst-stated contract (side, thread, cancellation return value) before a build?

**Kind.** `unimplemented_automation` -- **Status.** open

**Why it matters.** Contracts that nobody checks are documentation. A listener that touches client classes from a server event, or ignores a cancellation return, is detectable statically.

**Affects.** [[10-Workflows/wf.behaviour.lifecycle_events|wf.behaviour.lifecycle_events]], [[10-Workflows/wf.multiplayer.side_separation|wf.multiplayer.side_separation]]

**Evidence already available.**
- `extracted/edges.json#callback_of`

**Best remaining source.** The contracts under _authored/contracts and the generated sources of the creator pipeline.

**Procedure.** Add a planner check that reads the contract for each subscribed event and flags side mismatches using the module environment on the edge.

**Done when.** The creator pipeline emits a warning for a server-side listener that references a client-only class.

**Conclusions affected while open.**
- Side-separation mistakes are found at runtime, and no runtime is available.

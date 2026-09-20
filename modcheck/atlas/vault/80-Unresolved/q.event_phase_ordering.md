---
type: "question"
id: "q.event_phase_ordering"
kind: "incomplete_extraction"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.event_phase_ordering

**Question.** Which Fabric events define phases (Event.addPhaseOrdering / Identifier phases) and which Fabric API code registers listeners into non-default phases, so that the ordering of two subscribers to the same event can be stated?

**Kind.** `incomplete_extraction` -- **Status.** open

**Why it matters.** Subscriber order within an event is the interaction most mods depend on without knowing it. The Event type's phase API is in the corpus but no edge records who uses it.

**Affects.** [[10-Workflows/wf.behaviour.lifecycle_events|wf.behaviour.lifecycle_events]], [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]]

**Evidence already available.**
- [[40-Interfaces/net.fabricmc.fabric.api.event.Event|Event]]
- `extracted/fabric_api.json#fabric-api-base`

**Best remaining source.** javap -c over every module class for invocations of Event.addPhaseOrdering and Event.register with a phase argument.

**Procedure.** Add an `orders_before` extractor to atlas/extract/edges.py that scans for addPhaseOrdering(Identifier, Identifier) call sites and register(Identifier, listener) call sites; emit orders_before edges with static_inference evidence.

**Done when.** edges.json contains an orders_before count and each event note lists its phases.

**Conclusions affected while open.**
- The contracts under _authored/contracts cannot state any ordering between subscribers.

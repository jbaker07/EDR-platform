---
type: "question"
id: "q.injection_points_not_resolved"
kind: "incomplete_extraction"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.injection_points_not_resolved

**Question.** For each injects_into and wraps edge, where exactly inside the target method does the injection apply (bytecode offset, slice, ordinal), and what does a Redirect or ModifyArg change about the call it wraps?

**Kind.** `incomplete_extraction` -- **Status.** open

**Why it matters.** Two injections into one method only collide if they touch the same point. The atlas records the @At kind and target descriptor as a string, which is enough to say "same method", not "same point".

**Affects.** [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]], [[10-Workflows/wf.behaviour.vanilla_modification|wf.behaviour.vanilla_modification]], [[70-Requests/request.villager_fear|request.villager_fear]]

**Evidence already available.**
- `extracted/edges.json#injects_into`
- `extracted/edges.json#wraps`
- `extracted/fabric_api.json`

**Best remaining source.** The vanilla method bodies (javap -c on the hooked types) matched against each @At target descriptor; the sponge-mixin injection-point classes in the corpus.

**Procedure.** For each edge, disassemble the target method with javap -c, locate the instruction(s) matching the @At target (INVOKE descriptor, NEW type, RETURN, HEAD, TAIL), record the ordinal count, and attach the instruction index to the edge as `operation.offset`. Redirect and ModifyArg then get a `wraps.call` endpoint naming the wrapped callee.

**Done when.** Every injects_into/wraps edge has an instruction-level locator or a recorded reason it cannot have one.

**Conclusions affected while open.**
- contested_methods lists methods, not points; two entries there may not actually interfere.
- No claim about a Redirect's effect on other injections can be made.

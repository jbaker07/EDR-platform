---
type: "question"
id: "q.injection_points_not_resolved"
kind: "incomplete_extraction"
status: "in_progress"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.injection_points_not_resolved

**Question.** For each injects_into and wraps edge, every @At now resolves to an exact member (332 exact, 49 inherited_exact, 4 MixinExtras expression points unresolvable by construction); what remains is the instruction OFFSET inside the target method where each point applies, so that two injections into the same method can be told apart as same-point or different-point.

**Kind.** `incomplete_extraction` -- **Status.** in_progress

**Why it matters.** Two injections into one method only collide if they touch the same point. The atlas records the @At kind and target descriptor as a string, which is enough to say "same method", not "same point".

**Affects.** [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]], [[10-Workflows/wf.behaviour.vanilla_modification|wf.behaviour.vanilla_modification]], [[70-Requests/request.villager_fear|request.villager_fear]]

**Evidence already available.**
- `extracted/edges.json#injects_into`
- `extracted/edges.json#wraps`
- `extracted/minecraft_surface.json.gz`

**Best remaining source.** The vanilla method bodies (javap -c on the hooked types) matched against each @At target descriptor; the sponge-mixin injection-point classes in the corpus.

**Procedure.** Decode the target method's Code attribute with atlas/extract/classfile.py, locate the instruction(s) matching each point (INVOKE/FIELD/NEW by resolved member, HEAD/TAIL/RETURN by position, ordinal applied), and attach the offset list to the edge's `points`.

**Done when.** Every point on every injects_into/wraps edge carries offsets, or a recorded reason (expression point).

**Conclusions affected while open.**
- shared_targets in `extracted/edges.json` is a same-METHOD index; same-POINT overlap is not yet computed.

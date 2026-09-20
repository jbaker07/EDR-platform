---
type: "question"
id: "q.selector_ambiguity"
kind: "incomplete_extraction"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.selector_ambiguity

**Question.** Six injection selectors resolve to several overloads (ambiguous) and five are wildcard, quantified or regex selectors; Mixin picks among overloads by matching the handler's parameter types. Which exact member does each of the eleven apply to?

**Kind.** `incomplete_extraction` -- **Status.** open

**Why it matters.** A shared-target entry whose selector is ambiguous cannot be compared for same-point overlap.

**Affects.** [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]]

**Evidence already available.**
- `extracted/edges.json#injects_into`
- `extracted/fabric_api.json`

**Best remaining source.** Mixin's own selector matching (MemberInfo / TargetSelector in the pinned sponge-mixin jar), replayed statically or through the transformation harness.

**Procedure.** Implement handler-signature matching for name-only selectors (parameter types minus CallbackInfo must match the candidate's parameters); for quantified selectors expand against the target's member list and record every match; regex likewise.

**Done when.** edges.json reports 0 ambiguous and every quantified selector expanded to its matches.

**Conclusions affected while open.**
- Six injects_into/wraps edges name a method by name only, with candidates listed.

---
type: "question"
id: "q.contested_method_policy"
kind: "conflicting_requirements"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.contested_method_policy

**Question.** When a creator's mixin must target a vanilla method that Fabric API already injects into (a contested method), which of these conflicting requirements wins: "modify vanilla directly" versus "preserve every Fabric API event fired from that method"?

**Kind.** `conflicting_requirements` -- **Status.** open

**Why it matters.** A cancelling injection at HEAD of ServerPlayerGameMode.useItemOn, for example, would suppress UseBlockCallback for every other mod. The atlas can list the contested methods; it cannot decide the policy.

**Affects.** [[10-Workflows/wf.behaviour.vanilla_modification|wf.behaviour.vanilla_modification]], [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]], [[70-Requests/request.villager_fear|request.villager_fear]]

**Evidence already available.**
- `extracted/edges.json#injects_into`
- [[30-Mechanisms/Mixin|Mixin]]

**Best remaining source.** A project policy decided by the founder, recorded as a store record.

**Procedure.** Propose the policy "prefer the Fabric event; if a mixin is unavoidable, never cancel at HEAD of a contested method and never Overwrite it", get it approved, then encode it as a planner constraint.

**Done when.** A policy record exists and the planner enforces it.

**Conclusions affected while open.**
- The planner may select a mixin recipe for a contested method without warning.

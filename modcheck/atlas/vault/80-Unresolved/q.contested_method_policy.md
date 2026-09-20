---
type: "question"
id: "q.contested_method_policy"
kind: "conflicting_requirements"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.contested_method_policy

**Question.** When a creator's mixin must target a vanilla method that Fabric API already injects into, which policy applies given what the transformer actually does: HEAD/TAIL/RETURN injections survive an @Overwrite of the method ([[30-Mechanisms/Transformation_Tests#A|scenario A]], [[30-Mechanisms/Transformation_Tests#L|scenario L]], [[30-Mechanisms/Transformation_Tests#N|scenario N]]), INVOKE-point injections and Redirects into an overwritten method are refused unless the injecting mixin's priority is higher ([[30-Mechanisms/Transformation_Tests#B|scenario B]], [[30-Mechanisms/Transformation_Tests#J|scenario J]], [[30-Mechanisms/Transformation_Tests#K|scenario K]]), a second Redirect of the same call fails ([[30-Mechanisms/Transformation_Tests#C|scenario C]]), a cancelling HEAD suppresses later TAIL/RETURN listeners ([[30-Mechanisms/Transformation_Tests#H|scenario H]])?

**Kind.** `conflicting_requirements` -- **Status.** open

**Why it matters.** A cancelling injection at HEAD of ServerPlayerGameMode.useItemOn, for example, would suppress UseBlockCallback for every other mod. The atlas can list the contested methods; it cannot decide the policy.

**Affects.** [[10-Workflows/wf.behaviour.vanilla_modification|wf.behaviour.vanilla_modification]], [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]], [[70-Requests/request.villager_fear|request.villager_fear]]

**Evidence already available.**
- `extracted/mixin_transformation_tests.json`
- `extracted/edges.json#shared_targets`
- [[30-Mechanisms/Mixin|Mixin]]

**Best remaining source.** A project policy decided by the founder, recorded as a store record.

**Procedure.** Propose: prefer the Fabric event; if a mixin is unavoidable, no @Overwrite of a method with a shared target entry, no cancel at HEAD of a shared target, no Redirect where another Redirect or WrapOperation exists (WrapOperation composes over a Redirect: [[30-Mechanisms/Transformation_Tests#D|scenario D]]). Get it approved; encode it as a planner constraint that reads shared_targets and the composition table.

**Done when.** A policy record exists and the planner enforces it.

**Conclusions affected while open.**
- The planner may select a mixin recipe for a shared target without warning.

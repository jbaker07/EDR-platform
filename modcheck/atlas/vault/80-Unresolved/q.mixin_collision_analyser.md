---
type: "question"
id: "q.mixin_collision_analyser"
kind: "unimplemented_automation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.mixin_collision_analyser

**Question.** Given N mod jars, which pairs contain mixins that target the same vanilla method, and which of those pairs contain a Redirect, cancellable Inject at the same point, or Overwrite?

**Kind.** `unimplemented_automation` -- **Status.** open

**Why it matters.** This is the compatibility check creators ask for by name. The atlas has the extractor (atlas/extract/jvm.py mixin_facts) and the vocabulary; ModCheck's inspector and planner do not call it.

**Affects.** [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]], [[10-Workflows/wf.engineering.testing|wf.engineering.testing]]

**Evidence already available.**
- `extracted/edges.json#injects_into`
- [[30-Mechanisms/Mixin|Mixin]]

**Best remaining source.** The existing extractor, applied to arbitrary jars.

**Procedure.** Add `modcheck inspect --mixins` that runs mixin_facts over each jar's declared mixin classes, joins on (owner, method), and reports pairs with severity by injector kind; add a failure record for the Overwrite case with a detector id.

**Done when.** The command exists, has tests over the reference lantern jar plus one Fabric API module, and the coverage note counts it.

**Conclusions affected while open.**
- 90-Coverage item 5 stays "NOT implemented".

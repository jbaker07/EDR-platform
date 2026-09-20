---
type: "question"
id: "q.mixin_collision_analyser"
kind: "unimplemented_automation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.mixin_collision_analyser

**Question.** Given N mod jars, which pairs share an injection target, and -- only after exact resolution of both selectors, environment applicability of both mixins, and the transformer's composition rule for the pair of injectors -- which of those pairs actually conflict?

**Kind.** `unimplemented_automation` -- **Status.** open

**Why it matters.** This is the compatibility check creators ask for by name. The atlas has the extractor (atlas/extract/jvm.py mixin_facts) and the vocabulary; ModCheck's inspector and planner do not call it.

**Affects.** [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]], [[10-Workflows/wf.engineering.testing|wf.engineering.testing]]

**Evidence already available.**
- `extracted/edges.json#injects_into`
- [[30-Mechanisms/Mixin|Mixin]]

**Best remaining source.** The existing extractor, applied to arbitrary jars.

**Procedure.** In this order: (1) shared-target index from mixin_facts over each jar, the same index edges.json already builds for Fabric API (`extracted/edges.json#shared_targets`); (2) exact resolution of every selector and point (atlas/extract/resolve.py); (3) applicability: both mixins active in the same environment; (4) the composition rule from `extracted/mixin_transformation_tests.json` for the pair of effects and priorities. Report potential interactions at (1) and conflicts only at (4). Never turn an overlap into a conflict.

**Done when.** `modcheck inspect --mixins` reports the shared-target index for arbitrary jars, and marks a conflict only when all four steps hold; tested over the reference lantern jar plus one Fabric API module.

**Conclusions affected while open.**
- 90-Coverage item 5 stays "NOT implemented".

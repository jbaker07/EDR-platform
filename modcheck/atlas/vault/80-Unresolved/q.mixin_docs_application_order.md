---
type: "question"
id: "q.mixin_docs_application_order"
kind: "missing_documentation"
status: "in_progress"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.mixin_docs_application_order

**Question.** The transformation tests established, for this pinned Mixin, that lower-priority mixins are applied first and that equal-priority mixins are applied in configuration order for that run ([[30-Mechanisms/Transformation_Tests#E|scenario E]], [[30-Mechanisms/Transformation_Tests#G|scenario G]]). What does the Mixin documentation state as the rule for equal priorities across mods, and does fabric-loader's config ordering match it?

**Kind.** `missing_documentation` -- **Status.** in_progress

**Why it matters.** The priority default was read from the jar; the ordering rule was not, and no claim about two mods' mixin order can be made without it.

**Affects.** [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]]

**Evidence already available.**
- `extracted/mixin_transformation_tests.json`
- `extracted/corpus.json`
- [[30-Mechanisms/Mixin|Mixin]]

**Best remaining source.** The SpongePowered Mixin wiki and the sponge-mixin source, cached as sources.

**Procedure.** Retrieve, record terms, cite from _authored/mechanisms/mixin.md and the Mixin contract.

**Done when.** The mixin note's ordering paragraph cites a documented source.

**Conclusions affected while open.**
- Equal-priority ordering between two mods is observed for one configuration order, not documented as a rule.

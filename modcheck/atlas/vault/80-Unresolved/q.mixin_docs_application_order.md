---
type: "question"
id: "q.mixin_docs_application_order"
kind: "missing_documentation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.mixin_docs_application_order

**Question.** What does the Mixin documentation state about application order between mixins of equal priority from different mods, and about the interaction of @Redirect with @Inject at the same call site?

**Kind.** `missing_documentation` -- **Status.** open

**Why it matters.** The priority default was read from the jar; the ordering rule was not, and no claim about two mods' mixin order can be made without it.

**Affects.** [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]]

**Evidence already available.**
- `extracted/corpus.json`
- [[30-Mechanisms/Mixin|Mixin]]

**Best remaining source.** The SpongePowered Mixin wiki and the sponge-mixin source, cached as sources.

**Procedure.** Retrieve, record terms, cite from _authored/mechanisms/mixin.md and the Mixin contract.

**Done when.** The mixin note's ordering paragraph cites a documented source.

**Conclusions affected while open.**
- contested_methods cannot say which mod's injection runs first.

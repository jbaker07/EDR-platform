---
type: "question"
id: "q.pack_format_old_packs"
kind: "conflicting_requirements"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.pack_format_old_packs

**Question.** Must a mod's bundled resources target pack format 97.1 exactly (rejecting older formats) or declare a supported range, and how does that interact with players' own older resource packs layered above the mod's?

**Kind.** `conflicting_requirements` -- **Status.** open

**Why it matters.** Strict targeting keeps the mod correct; permissive ranges keep players' packs working. The corpus gives the number, not the policy.

**Affects.** [[10-Workflows/wf.presentation.models_animation|wf.presentation.models_animation]], [[10-Workflows/wf.engineering.version_migration|wf.engineering.version_migration]]

**Evidence already available.**
- `extracted/corpus.json`

**Best remaining source.** The 26.3 release notes (q.mojang_changelog_26_3) and the pack.mcmeta schema in the jar.

**Procedure.** Extract pack.mcmeta from the jar's own resources; retrieve the notes; decide per project.

**Done when.** The migration workflow states the supported-formats policy with evidence.

**Conclusions affected while open.**
- Generated pack.mcmeta content in scaffolds is a guess at the range field.

---
type: "question"
id: "q.mojang_changelog_26_3"
kind: "missing_documentation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.mojang_changelog_26_3

**Question.** What changed in resource pack format 97.1 and data pack format 121.0 relative to the previous formats, and what does protocol 777 change for custom payloads?

**Kind.** `missing_documentation` -- **Status.** open

**Why it matters.** The pack formats are read from version.json, but the atlas cannot tell a creator what to change in an older pack without the change notes.

**Affects.** [[10-Workflows/wf.presentation.models_animation|wf.presentation.models_animation]], [[10-Workflows/wf.content.data_driven_content|wf.content.data_driven_content]], [[10-Workflows/wf.engineering.version_migration|wf.engineering.version_migration]]

**Evidence already available.**
- `extracted/corpus.json`
- [[00-Scope/Sources|mojang_version_manifest]]

**Best remaining source.** The official 26.3 release notes (minecraft.net / feedback.minecraft.net), cached as a source.

**Procedure.** Retrieve the release notes, record terms, cite from wf.engineering.version_migration.

**Done when.** The migration workflow cites a cached change log for the pack format bumps.

**Conclusions affected while open.**
- Pack-format migration steps are stated only as "the number changed".

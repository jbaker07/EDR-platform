---
type: "question"
id: "q.external_asset_tools"
kind: "missing_documentation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.external_asset_tools

**Question.** Which external tools (Blockbench for models and animations, image editors for textures, audio tools for sounds) produce artefacts in the formats the 26.3 jar expects, and what are their export options for those formats?

**Kind.** `missing_documentation` -- **Status.** open

**Why it matters.** The presentation workflows depend on tools outside the corpus; without their documentation the atlas can only name the target file formats.

**Affects.** [[10-Workflows/wf.presentation.models_animation|wf.presentation.models_animation]], [[10-Workflows/wf.presentation.particles_sounds|wf.presentation.particles_sounds]], [[70-Requests/request.lantern_moth|request.lantern_moth]]

**Evidence already available.**
- `extracted/corpus.json`

**Best remaining source.** The tools' own documentation, cached as sources with terms.

**Procedure.** Retrieve with `modcheck sources add`, cite from the workflows' tools_today.

**Done when.** wf.presentation.models_animation cites a cached tool source for each artefact type.

**Conclusions affected while open.**
- The models_animation workflow's "tools today" for authoring is unsupported by cached evidence.

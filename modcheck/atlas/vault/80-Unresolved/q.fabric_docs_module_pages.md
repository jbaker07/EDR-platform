---
type: "question"
id: "q.fabric_docs_module_pages"
kind: "missing_documentation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.fabric_docs_module_pages

**Question.** Which docs.fabricmc.net pages (networking, events, rendering, data generation, mixins) describe the workflows this atlas maps, and what do they say? Only the documentation home page is cached.

**Kind.** `missing_documentation` -- **Status.** open

**Why it matters.** "Tools today" and "existing automation" in each workflow note are drawn from artifacts and from the reference implementation; the official guidance for each step is not in the evidence set.

**Affects.** [[10-Workflows/wf.multiplayer.networking|wf.multiplayer.networking]], [[10-Workflows/wf.presentation.hud_screens|wf.presentation.hud_screens]], [[10-Workflows/wf.content.data_driven_content|wf.content.data_driven_content]], [[10-Workflows/wf.engineering.project_setup|wf.engineering.project_setup]]

**Evidence already available.**
- [[00-Scope/Sources|fabric_docs_home]]
- [[00-Scope/Sources|fabric_mod_json_spec]]

**Best remaining source.** docs.fabricmc.net subpages, retrieved and hashed with `modcheck sources add`.

**Procedure.** Add each relevant page as a source with reuse terms, then cite it from the workflow's tools_today with a `source:` reference.

**Done when.** Every workflow family cites at least one cached documentation source for its tools_today.

**Conclusions affected while open.**
- Workflow tools_today lists are artifact-derived and may omit officially recommended steps.

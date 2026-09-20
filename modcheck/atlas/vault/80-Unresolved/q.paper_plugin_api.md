---
type: "question"
id: "q.paper_plugin_api"
kind: "missing_artifact_access"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.paper_plugin_api

**Question.** Can the Paper server jar and paper-api be resolved so that the plugin surface (Bukkit events, plugin.yml, no client component) can be inventoried as its own branch?

**Kind.** `missing_artifact_access` -- **Status.** open

**Why it matters.** Server operators often ask for "a plugin" when they mean a server-side mod, and the two are not interchangeable. The atlas cannot currently show the difference with evidence.

**Affects.** [[10-Workflows/wf.multiplayer.side_separation|wf.multiplayer.side_separation]], [[10-Workflows/wf.behaviour.lifecycle_events|wf.behaviour.lifecycle_events]]

**Evidence already available.**

**Best remaining source.** The PaperMC Maven repository (paper-api) and the Paper download API.

**Procedure.** Retrieve paper-api with `modcheck sources add`, record terms, add a `paper_plugin` group to corpus.py, extract its event classes with jvm.py.

**Done when.** A 00-Scope/Branches row for Paper reads "in corpus".

**Conclusions affected while open.**
- Requests phrased as "plugin" are answered with Fabric server-side mods, and the atlas says so.

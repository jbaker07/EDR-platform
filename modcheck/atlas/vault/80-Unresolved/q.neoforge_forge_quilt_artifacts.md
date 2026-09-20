---
type: "question"
id: "q.neoforge_forge_quilt_artifacts"
kind: "missing_artifact_access"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.neoforge_forge_quilt_artifacts

**Question.** Can the NeoForge, Forge and Quilt loader artifacts for a 26.x version be resolved and hashed, so that their event buses, mod descriptors and mixin/coremod mechanisms can be extracted with the same tools?

**Kind.** `missing_artifact_access` -- **Status.** open

**Why it matters.** The pack manifest names four loaders; the corpus contains one. Every "loader: fabric" field in the atlas is a scope statement, and the other three have only a getting-started page.

**Affects.** [[10-Workflows/wf.integration.dependencies_conditions|wf.integration.dependencies_conditions]], [[10-Workflows/wf.engineering.project_setup|wf.engineering.project_setup]]

**Evidence already available.**
- [[00-Scope/Sources|neoforge_docs_getting_started]]
- `extracted/corpus.json`

**Best remaining source.** The NeoForge Maven (maven.neoforged.net) and Quilt Maven, resolved through their Gradle plugins in scratch projects.

**Procedure.** Same as q.older_minecraft_versions but per loader; extend corpus.py groups; run jvm.py over the loader jars; add the loader's event type to the edge vocabulary if it is not Fabric's Event.

**Done when.** corpus.json lists at least one non-Fabric loader with extracted surfaces.

**Conclusions affected while open.**
- No capability, edge or contract in the atlas applies to NeoForge, Forge or Quilt.

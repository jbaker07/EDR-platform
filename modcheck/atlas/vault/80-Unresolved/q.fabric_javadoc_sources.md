---
type: "question"
id: "q.fabric_javadoc_sources"
kind: "missing_documentation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.fabric_javadoc_sources

**Question.** What do the Fabric API javadocs say about each event's thread, side, timing and cancellation semantics? The -sources jars were not resolved (0 in the Gradle cache), so no documented contract exists in the corpus.

**Kind.** `missing_documentation` -- **Status.** open

**Why it matters.** Every analyst-stated contract under _authored/contracts is inferred from injection sites. The javadoc is the module author's own statement and would upgrade many claims from analyst_inference to documented.

**Affects.** [[10-Workflows/wf.behaviour.lifecycle_events|wf.behaviour.lifecycle_events]], [[10-Workflows/wf.multiplayer.networking|wf.multiplayer.networking]], [[10-Workflows/wf.behaviour.entity_ai|wf.behaviour.entity_ai]]

**Evidence already available.**
- `extracted/fabric_api.json`
- `extracted/edges.json#publishes_event`

**Best remaining source.** maven.fabricmc.net -sources.jar for each module at the pinned version, or the FabricMC/fabric repository at the tag matching 0.161.0+26.3.

**Procedure.** Resolve sources through Loom (`genSources` or the sources classifier), hash them, write a javadoc extractor that attaches the doc comment of each Event field to the event note as `documented` evidence with the sources sha256.

**Done when.** Each event note has a "Documented" section or a recorded absence.

**Conclusions affected while open.**
- No contract in the atlas is `documented`; all are declared, static_inference or analyst_inference.

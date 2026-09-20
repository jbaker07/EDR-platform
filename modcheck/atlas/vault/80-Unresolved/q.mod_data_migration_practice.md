---
type: "question"
id: "q.mod_data_migration_practice"
kind: "missing_documentation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.mod_data_migration_practice

**Question.** How do Fabric mods migrate their own persisted data across versions in practice: does Fabric expose a data-fixer hook, or do mods version their codecs and migrate on read?

**Kind.** `missing_documentation` -- **Status.** open

**Why it matters.** The corpus shows vanilla's DataFixer package (444 classes in net.minecraft.util.datafix) and two Fabric hooks into it, but no API for mods to add fixers. Whether that is a gap or a deliberate practice determines the migration design in every state workflow.

**Affects.** [[10-Workflows/wf.state.migration|wf.state.migration]], [[70-Requests/request.port_1_21_mod|request.port_1_21_mod]], [[70-Requests/request.team_counter|request.team_counter]]

**Evidence already available.**
- `extracted/corpus.json`
- `extracted/edges.json`
- [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]]

**Best remaining source.** Fabric documentation on persistent data and the data attachment API javadoc.

**Procedure.** Retrieve; if no hook exists, record the codec-versioning pattern as the documented practice.

**Done when.** wf.state.migration cites a documented migration practice.

**Conclusions affected while open.**
- Migration designs in 70-Requests use codec versioning by analyst choice, not by documented practice.

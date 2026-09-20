---
type: "question"
id: "q.inspector_mixin_targets"
kind: "unimplemented_automation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.inspector_mixin_targets

**Question.** Should the jvm inspector's mod facts include each mixin class's targets and injection points, so that inspection of a downloaded mod yields edges directly?

**Kind.** `unimplemented_automation` -- **Status.** open

**Why it matters.** The inspector lists mixin_classes only. The atlas extractor reads targets from the same bytes. Two code paths for one fact is the second database the atlas must not become.

**Affects.** [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]], [[10-Workflows/wf.integration.dependencies_conditions|wf.integration.dependencies_conditions]]

**Evidence already available.**
- `extracted/fabric_api.json`

**Best remaining source.** src/modcheck/inspect/jvm.py and atlas/extract/jvm.py, to be merged.

**Procedure.** Move mixin_facts into the inspector package; have the atlas extractor import it; add a `mixin_targets` fact key.

**Done when.** One implementation, used by both, with the inspector's tests extended.

**Conclusions affected while open.**
- Downloaded mods are inspected without their targets.

---
type: "question"
id: "q.runtime_mixin_application"
kind: "unavailable_runtime_observation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.runtime_mixin_application

**Question.** When the 26.3 client or dedicated server starts with Fabric loader 0.19.5 and a given set of mods, which mixins actually apply, in what order, and which fail with a target-not-found or conflict error?

**Kind.** `unavailable_runtime_observation` -- **Status.** open

**Why it matters.** Every injects_into edge names a target; none establishes the injection applied. Fabric loader logs mixin application and failures at startup, which is the only ground truth for "these mods coexist".

**Affects.** [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]], [[10-Workflows/wf.engineering.testing|wf.engineering.testing]], [[70-Requests/request.rain_lantern|request.rain_lantern]]

**Evidence already available.**
- `extracted/edges.json#injects_into`
- `capability/build_and_test.fabric_project_setup_current_version`

**Best remaining source.** A headless dedicated-server run of 26.3 with the mod set under test, with the Mixin debug flags enabled, capturing the log.

**Procedure.** Provision a dedicated server run in a sandbox (no user install touched), start it with `-Dmixin.debug.export=true` and the mods, capture the log, and ingest it with `modcheck runtime observe` so each edge can be promoted to `observed` with the run id as provenance.

**Done when.** edges.json evidence_classes contains a nonzero `observed` count with run provenance.

**Conclusions affected while open.**
- Nothing in the atlas is observed. No compatibility verdict is game-tested.

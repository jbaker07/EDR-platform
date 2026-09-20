---
type: "question"
id: "q.lantern_runtime_gate"
kind: "unavailable_runtime_observation"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.lantern_runtime_gate

**Question.** For rain_charged_lantern_v2: does Minecraft call LanternBlockEntity.loadAdditional/saveAdditional and round-trip the chunk; is the ticker installed for the block entity type; does isRainingAt(pos.above()) hold in every biome and weather state; is the payload encoded, sent, received and decoded; does the HUD render at every GUI scale; does hitResult name the block the player means?

**Kind.** `unavailable_runtime_observation` -- **Status.** open

**Why it matters.** These are the only claims the 40 JUnit tests cannot reach; each is a way the feature could be broken while every test passes.

**Affects.** [[70-Requests/request.rain_lantern|request.rain_lantern]], [[10-Workflows/wf.content.block_entity|wf.content.block_entity]]

**Evidence already available.**
- `capability/build_and_test.fabric_project_setup_current_version`
- `extracted/minecraft_surface.json.gz`

**Best remaining source.** A headless dedicated-server run with fabric-gametest-api-v1 (on the resolved classpath: `extracted/resolved_environment.json`) covering the load/save round trip and ticker installation; a client run for the HUD.

**Procedure.** Write gametests for the four server-side claims; run headless in a sandbox; ingest the log with modcheck runtime observe; the HUD claims need a client run.

**Done when.** Each of the six claims has an observed row with run provenance in the request note.

**Conclusions affected while open.**
- docs/RAIN_LANTERN_STATUS.md 'observed in game' stays no for every requirement.

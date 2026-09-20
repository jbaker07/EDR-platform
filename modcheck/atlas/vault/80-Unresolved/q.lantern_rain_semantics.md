---
type: "question"
id: "q.lantern_rain_semantics"
kind: "ambiguous_creator_intent"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.lantern_rain_semantics

**Question.** For the rain lantern, does "raining" mean the level's weather flag (Level.isRaining()), rain at the lantern's position (Level.isRainingAt(BlockPos), which accounts for biome and sky exposure), or precipitation of any kind including snow?

**Kind.** `ambiguous_creator_intent` -- **Status.** open

**Why it matters.** Both methods exist on the hooked Level type. A lantern under a roof, or in a desert, or in snow, behaves differently under each reading, and the difference is visible to players.

**Affects.** [[70-Requests/request.rain_lantern|request.rain_lantern]]

**Evidence already available.**
- [[40-Interfaces/net.minecraft.world.level.Level|Level]]
- `extracted/minecraft_members.json`

**Best remaining source.** The creator.

**Procedure.** Ask, with the three readings and their visible consequences listed; record the answer in the request note's approved_behaviour.

**Done when.** approved_behaviour names one method and the expected behaviour under a roof, in a dry biome, and in snow.

**Conclusions affected while open.**
- The reference implementation's choice is a default, not an approved behaviour.

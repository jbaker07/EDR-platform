---
type: "question"
id: "q.datapack_load_order"
kind: "unmodeled_behaviour"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.datapack_load_order

**Question.** In what order are data packs (built-in, mod-provided via fabric-resource-loader, world) applied in 26.3, and how are placed-feature ordering conflicts within one generation step detected and reported?

**Kind.** `unmodeled_behaviour` -- **Status.** open

**Why it matters.** Two mods adding features to the same biome at the same step can produce a feature-order cycle that aborts world load; which pack wins for a same-named JSON decides what a creator sees.

**Affects.** [[10-Workflows/wf.world.features_biomes|wf.world.features_biomes]], [[10-Workflows/wf.content.data_driven_content|wf.content.data_driven_content]], [[70-Requests/request.crystal_caves|request.crystal_caves]]

**Evidence already available.**
- [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]]
- [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]]
- `extracted/corpus.json`

**Best remaining source.** The resource-loader module's pack ordering code and vanilla's feature-order validation in the world-generation packages, extracted.

**Procedure.** Extract the relevant classes as extra types; record orders_before edges; confirm with a headless world load.

**Done when.** The world workflow states pack precedence and the cycle failure with evidence.

**Conclusions affected while open.**
- The crystal caves request cannot promise coexistence with another biome-modifying mod.

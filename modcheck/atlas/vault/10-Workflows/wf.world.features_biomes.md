---
type: "workflow"
id: "wf.world.features_biomes"
area: "world"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Add or change world-generation features and biomes

**Intent.** The creator wants something new in generated terrain -- an ore, a plant, a cave decoration, a whole biome -- appearing where they intend and not elsewhere, in new chunks, without breaking worlds that already exist.

## Must be preserved

- Already-generated chunks are never regenerated; the change affects new chunks only.
- Other mods' features in the same biomes.

## Mechanisms that can serve it

- Data packs: configured_feature, placed_feature, biome and worldgen entry types shipped in the jar (`extracted/corpus.json`).
- [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] -- adds features and spawns to existing biomes by selector, without replacing the biome JSON.
- [[30-Mechanisms/Registries|Registries]] -- FEATURE for a new feature type in code.
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.CHUNK_GENERATE|CHUNK_GENERATE]] -- post-generation hook whose thread is not established.

## Tools and artifacts used today

- JSON definitions (hand-written or data-generated) plus a BiomeModifications call in the main entrypoint.
- A new Feature subclass only when no vanilla feature type fits (its base classes are unhooked; [[80-Unresolved/q.unhooked_vanilla_members|q.unhooked_vanilla_members]]).

## Decisions the creator must make

- Data-only (vanilla feature type with new configuration) or a coded feature.
- Biome selection: by tag, by key list, or all overworld.
- Generation step and placement modifiers, which decide frequency and height.

## Information those decisions need

- The 26.3 configured/placed feature JSON schema (only jar examples in corpus).
- Feature ordering rules within a step and how conflicts surface ([[80-Unresolved/q.datapack_load_order|q.datapack_load_order]]).
- The thread of any post-generation listener ([[80-Unresolved/q.worldgen_threading|q.worldgen_threading]]).

## Existing automation

- None in ModCheck.

## Remaining manual or unsupported work

- All of it; the biome API's surface is inspected but no generator exists.

## ModCheck's contribution

- A capability record for BiomeModifications-based feature addition with the JSON pair, compiled and validated against the jar's own placed_feature examples.

## Interactions to check

- Same-step feature ordering across mods.
- Two mods replacing the same biome JSON.

## Evidence

- `extracted/fabric_api.json#fabric-biome-api-v1`
- `extracted/corpus.json`
- `extracted/edges.json#publishes_event`

## Open questions

- [[80-Unresolved/q.datapack_load_order|q.datapack_load_order]]
- [[80-Unresolved/q.worldgen_threading|q.worldgen_threading]]
- [[80-Unresolved/q.unhooked_vanilla_members|q.unhooked_vanilla_members]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: True
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none

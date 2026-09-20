---
type: "workflow"
id: "wf.world.structures"
area: "world"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Add structures

**Intent.** A building or ruin generated into terrain from templates, with loot, spawns and a map marker, appearing at the intended rarity.

## Must be preserved

- Existing structure spacing for other mods and vanilla.

## Mechanisms that can serve it

- Data packs: structure, structure_set, template_pool and the structure NBT templates (`extracted/corpus.json` entry types).
- [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] for biome-tag membership; [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] for chest loot.

## Tools and artifacts used today

- Structure blocks in-game to author templates (needs the game), then JSON.

## Decisions the creator must make

- Single template versus jigsaw pools.
- Spacing and salt values in the structure set.

## Information those decisions need

- Template NBT authoring requires a running game -- not available here ([[80-Unresolved/q.runtime_event_delivery|q.runtime_event_delivery]]).

## Existing automation

- None.

## Remaining manual or unsupported work

- All.

## ModCheck's contribution

- Not yet planned.

## Evidence

- `extracted/corpus.json`

## Status

- inventoried: True
- mechanically_inspected: False
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none

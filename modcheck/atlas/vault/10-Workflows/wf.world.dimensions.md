---
type: "workflow"
id: "wf.world.dimensions"
area: "world"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Add a dimension

**Intent.** A separate world with its own generator, sky and rules, reachable from the overworld, that saves and loads like the vanilla dimensions.

## Must be preserved

- The three vanilla dimensions and any other mod's dimensions.

## Mechanisms that can serve it

- Data packs: dimension and dimension_type entries (`extracted/corpus.json`).
- [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] -- [[50-Interactions/events/net.fabricmc.fabric.api.dimension.v1.DimensionEvents.MODIFY_ATTRIBUTES|MODIFY_ATTRIBUTES]] and teleport helpers.
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLevelEvents.LOAD|LOAD]] -- per-level setup point.

## Tools and artifacts used today

- JSON dimension type plus a chunk generator reference (vanilla noise generator with custom settings, or a coded generator).

## Decisions the creator must make

- Vanilla noise generator with custom settings (data only) versus a custom ChunkGenerator (code, unhooked base class).
- Portal or command travel.

## Information those decisions need

- The 26.3 dimension_type JSON fields.
- Whether a dimension added by data pack after world creation is loaded for existing worlds.

## Existing automation

- None.

## Remaining manual or unsupported work

- All.

## ModCheck's contribution

- Not yet planned; listed for completeness of the surface.

## Evidence

- `extracted/fabric_api.json#fabric-dimensions-v1`
- `extracted/corpus.json`

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none

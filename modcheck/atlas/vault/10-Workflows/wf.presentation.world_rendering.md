---
type: "workflow"
id: "wf.presentation.world_rendering"
area: "presentation"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Custom world rendering

**Intent.** Draw something in the world that is not a block or entity: outlines, overlays, effects tied to the level render passes.

## Must be preserved

- Frame time; these hooks run every frame.

## Mechanisms that can serve it

- [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] -- level render pass events (for example [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.AFTER_TRANSLUCENT_TERRAIN|AFTER_TRANSLUCENT_TERRAIN]]), 14 pass events in `extracted/edges.json#callback_of`.
- [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] / [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] for custom block model quads.

## Tools and artifacts used today

- A listener on the pass event using the render state handed in.

## Decisions the creator must make

- Which pass; whether extraction (state capture) or submission (draw) is the right phase in the 26.x split renderer.

## Information those decisions need

- The extraction/render split semantics in 26.3 (LevelExtractionEvents versus LevelRenderEvents both exist; no documentation cached: [[80-Unresolved/q.fabric_docs_module_pages|q.fabric_docs_module_pages]]).

## Existing automation

- None.

## Remaining manual or unsupported work

- All.

## ModCheck's contribution

- Not yet planned.

## Evidence

- `extracted/edges.json#callback_of`
- `extracted/fabric_api.json#fabric-rendering-v1`

## Open questions

- [[80-Unresolved/q.fabric_docs_module_pages|q.fabric_docs_module_pages]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none

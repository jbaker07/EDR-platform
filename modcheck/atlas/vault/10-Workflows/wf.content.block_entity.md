---
type: "workflow"
id: "wf.content.block_entity"
area: "content"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Add a block with state (block entity)

**Intent.** The creator's block must remember something -- an inventory, a timer, an owner -- and often show it. The state must survive save/load and reach the client when it matters for rendering.

## Must be preserved

- Existing block entities of other mods and vanilla.
- Data shape stability once shipped ([[10-Workflows/wf.state.migration|wf.state.migration]]).

## Mechanisms that can serve it

- [[30-Mechanisms/Registries|Registries]] -- BLOCK_ENTITY_TYPE.
- [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] -- an alternative to a block entity when only extra data, not ticking, is needed (`capability/persist_state.fabric_attachment`).
- [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] -- block entity renderer registration on the client.

## Tools and artifacts used today

- A BlockEntity subclass with save/load via the 26.3 value I/O API (its signatures were corrected during the lantern build: no codec() on BlockEntityType in 26.3).

## Decisions the creator must make

- Block entity (ticking, rendered) versus attachment (data only) versus SavedData at level scope (`capability/persist_state.fabric_saveddata`).
- What subset of state the client needs, sent as the block entity's update tag.

## Information those decisions need

- The block entity save/load method signatures in 26.3 (BlockEntity is in the world.level package; its members are extracted only if hooked -- check `extracted/minecraft_members.json`).
- Whether the reference lantern's approach (SavedData at level scope) or a per-block entity fits the request.

## Existing automation

- ModCheck generates SavedData and attachment code (`capability/persist_state.fabric_saveddata`, `capability/persist_state.fabric_attachment`); no block-entity generator.

## Remaining manual or unsupported work

- Block entity class, renderer, update packet.

## ModCheck's contribution

- A block-entity capability with the update-tag path and a compile check; a planner rule choosing between the three persistence shapes from the request's stated scope.

## Interactions to check

- Another mod's mixin into BlockEntity.saveAdditional or the block entity type registry.

## Evidence

- `capability/persist_state.fabric_saveddata`
- `capability/persist_state.fabric_attachment`
- `extracted/minecraft_registries.json`

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: False
- interaction_analysed: True
- implemented_in_modcheck: True
- validated_scope: SavedData and attachment generators compile against the pinned corpus and are exercised by the reference lantern's JUnit tests; the block-entity path itself is not implemented

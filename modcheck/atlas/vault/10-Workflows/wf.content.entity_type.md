---
type: "workflow"
id: "wf.content.entity_type"
area: "content"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Add an entity type

**Intent.** A creator wants a new creature or object that exists in the world as an entity: it spawns, has attributes, moves, is saved with the chunk, and is rendered on the client with its own model.

## Must be preserved

- Vanilla spawn rules for every other entity.
- Save compatibility: an entity type id, once shipped, is in players' worlds.

## Mechanisms that can serve it

- [[30-Mechanisms/Registries|Registries]] -- ENTITY_TYPE.
- [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] -- default attributes through [[50-Interactions/events/net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry.MODIFY|MODIFY]].
- [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] -- entity renderer and model layer registration on the client ([[10-Workflows/wf.presentation.models_animation|wf.presentation.models_animation]]).
- [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] -- spawn entries added to biomes.

## Tools and artifacts used today

- The same Loom project; an entity class extending a vanilla base in [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]] or [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] (members extracted).
- A model and texture from an external tool ([[80-Unresolved/q.external_asset_tools|q.external_asset_tools]]).

## Decisions the creator must make

- Base class (Mob with goals, or a brain-driven base) -- decides how behaviour is added ([[10-Workflows/wf.behaviour.entity_ai|wf.behaviour.entity_ai]]).
- Spawn: natural in biomes, spawn egg only, or command only.
- Client rendering: reuse a vanilla model or ship a new one.

## Information those decisions need

- EntityType.Builder's 26.3 signature (not on a hooked type; [[80-Unresolved/q.unhooked_vanilla_members|q.unhooked_vanilla_members]]).
- Which attributes a LivingEntity requires to construct without crashing.

## Existing automation

- None in ModCheck; the object-builder and rendering modules are inspected but no generator exists.

## Remaining manual or unsupported work

- Everything beyond project setup.

## ModCheck's contribution

- A capability record for entity registration plus attributes, generated with a compile check; a companion client-side renderer capability.

## Interactions to check

- Spawn weight competition in a biome with other mods' entities.
- Entity load events firing for the new type in every mod that subscribes to ENTITY_LOAD.

## Evidence

- `extracted/minecraft_registries.json`
- `extracted/minecraft_members.json`
- `extracted/edges.json#callback_of`

## Open questions

- [[80-Unresolved/q.unhooked_vanilla_members|q.unhooked_vanilla_members]]
- [[80-Unresolved/q.villager_ai_architecture|q.villager_ai_architecture]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none

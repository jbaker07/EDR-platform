---
type: "request"
id: "request.lantern_moth"
canonical: "exercise.lantern_moth"
kind: "analyst_exercise"
family: "workflow:wf.presentation.models_animation"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Lantern moth -- an animated flying entity that circles lit lanterns

**Canonical request.** `exercise.lantern_moth` (analyst_exercise)

> [!note] Analyst exercise
> No creator wrote this request. Nothing in it is approved intent.

## Request

Analyst-authored exercise request: a small moth entity with flapping wings that spawns near lit lanterns at night and circles them; it has no drops and takes no damage from players; the animation must look smooth in multiplayer.

## Approved behaviour (the request's own words or acceptance criteria)

- Provisional: spawns near lit lantern blocks after dark; despawns at dawn; wing flap animated on the client; passive.

## Preservation obligations

- Vanilla ambient mobs and their spawn caps.

## Affected systems

- net.minecraft.world.entity -- the entity type and its movement ([[10-Workflows/wf.content.entity_type|wf.content.entity_type]]).
- net.minecraft.client.renderer -- renderer, model layer and animation (net.minecraft.client.animation has 29 classes; unhooked).
- net.minecraft.world.level -- spawn placement near lanterns.

## Implementation candidates

- Entity with a coded model and vanilla keyframe animation (AnimationDefinition-style), registered through [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] renderer/model-layer registration; movement as a flying goal on a Mob subclass (goal-driven, so [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]] goalSelector applies).
- Procedural wing animation in the renderer from the entity's age (no animation API dependency; simplest; less designer-friendly).
- Particle-based moth (no entity): rejected -- cannot circle a target or be interacted with.

## Data / control / state dependencies

- Data: model geometry and texture (external tool; [[80-Unresolved/q.external_asset_tools|q.external_asset_tools]]); spawn rules.
- Control: entity registration and attributes ([[50-Interactions/events/net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry.MODIFY|MODIFY]] or the builder); client-only renderer registration from the client entrypoint.
- State: entity position is synced by vanilla; the animation state derives from age and does not need sync.

## Interactions with the selected environment

- Spawn caps shared with other ambient mobs; a mod raising or lowering caps changes moth frequency.
- Entity load events of every subscribed mod fire for the moth (ENTITY_LOAD contract).
- The renderer must not be referenced from the entity class (dedicated server).

## Alternatives and tradeoffs

- Reuse a vanilla flying base (bat-like) to inherit movement; fewer new classes, less control.

## Implementation work

- Entity class and registration; renderer, model layer, animation definition; spawn placement; unit tests for spawn predicate and animation phase with fakes.
- Extraction of the animation package as extra types first.

## Verification obligations

- Compile of registrations and renderer against the pinned corpus (possible now).
- Animation smoothness and spawn behaviour: need a client run; none observed.

## Unresolved

- [[80-Unresolved/q.unhooked_vanilla_members|q.unhooked_vanilla_members]]
- [[80-Unresolved/q.external_asset_tools|q.external_asset_tools]]
- [[80-Unresolved/q.runtime_event_delivery|q.runtime_event_delivery]]

## Evidence

- `extracted/corpus.json`
- `extracted/fabric_api.json#fabric-rendering-v1`
- `extracted/minecraft_surface.json.gz`

## Status

- analysed: True
- implemented: none
- validated_scope: none

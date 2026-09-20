---
type: "workflow"
id: "wf.behaviour.entity_ai"
area: "behaviour"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Change or add entity behaviour (AI)

**Intent.** Make a creature act differently: flee from something, seek something, attack or ignore a target, follow a schedule. The creator describes the behaviour in game terms and expects it to hold in multiplayer.

## Must be preserved

- The creature's other behaviours; a new goal must not starve vanilla goals unless asked.
- Server authority: AI runs on the server and the client only sees the result.

## Mechanisms that can serve it

- [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]] declares the protected GoalSelector; a goal is added through an accessor mixin or an access widener ([[30-Mechanisms/fabric-transitive-access-wideners-v1|fabric-transitive-access-wideners-v1]] ships vanilla widenings; whether goalSelector is among them is not extracted).
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_LOAD|ENTITY_LOAD]] -- the point to add a goal to a vanilla mob instance, idempotently.
- [[30-Mechanisms/Mixin|Mixin]] into the mob's registerGoals or the brain-building method for brain-driven mobs.
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.ALLOW_DAMAGE|ALLOW_DAMAGE]] and related for reaction-style behaviour without AI changes.

## Tools and artifacts used today

- A Goal subclass; registration at ENTITY_LOAD or via mixin.

## Decisions the creator must make

- GoalSelector-based mob or Brain-based mob ([[80-Unresolved/q.villager_ai_architecture|q.villager_ai_architecture]]) -- the two need different code.
- Event-based (composable, limited to instance-level) versus mixin (any change, contested-method risk).
- Goal priority relative to vanilla goals.

## Information those decisions need

- The target mob's AI architecture and the members of its class (unhooked types; [[80-Unresolved/q.unhooked_vanilla_members|q.unhooked_vanilla_members]]).
- Whether an access widener already opens goalSelector.
- Thread and re-entrancy of ENTITY_LOAD (analyst contract exists).

## Existing automation

- The mixin recipe record (`recipe/fabric_extend_behaviour_with_mixin`) scaffolds a mixin; no AI-specific generator.

## Remaining manual or unsupported work

- Goal logic; choosing between the two AI systems.

## ModCheck's contribution

- Extraction of the AI packages as extra types so candidates are signature-checked; a capability for "add a goal to a vanilla mob at ENTITY_LOAD" with the idempotence check built in.

## Interactions to check

- Other mods adding goals to the same mob at the same priority.
- Mixins into the same registerGoals method (contested).

## Evidence

- `extracted/minecraft_surface.json.gz`
- `extracted/edges.json#publishes_event`
- `recipe/fabric_extend_behaviour_with_mixin`

## Open questions

- [[80-Unresolved/q.villager_ai_architecture|q.villager_ai_architecture]]
- [[80-Unresolved/q.unhooked_vanilla_members|q.unhooked_vanilla_members]]
- [[80-Unresolved/q.contested_method_policy|q.contested_method_policy]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: True
- interaction_analysed: True
- implemented_in_modcheck: False
- validated_scope: none

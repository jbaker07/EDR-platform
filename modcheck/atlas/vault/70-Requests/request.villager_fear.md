---
type: "request"
id: "request.villager_fear"
family: "workflow:wf.behaviour.entity_ai"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Villagers flee from a player holding a cursed totem

## Request

Analyst-authored exercise request: villagers should run away from any player holding a new "cursed totem" item, the way they flee zombies; other villager behaviour is unchanged; it must work on servers.

## Approved behaviour and constraints

- Provisional: flee radius similar to the vanilla zombie avoidance; server-side only; no change to trading.

## Preservation obligations

- All other villager behaviours and schedules.
- Vanilla avoidance of hostile mobs.

## Affected systems

- net.minecraft.world.entity -- villagers are brain-driven; [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] declares getBrain(), and the villager and behaviour classes are unhooked ([[80-Unresolved/q.villager_ai_architecture|q.villager_ai_architecture]]).
- net.minecraft.world.item -- the totem item ([[10-Workflows/wf.content.block_item|wf.content.block_item]], item half).

## Implementation candidates

- Brain behaviour injection: a mixin into the villager's brain-building code adding an avoid-entity behaviour with a predicate on the player's held item. Correct architecture for villagers; the target method and its signature are unverified until the villager classes are extracted.
- Sensor extension: extend the villager's nearest-hostile sensing so a totem-holder counts as hostile, reusing vanilla's existing flee behaviour. Fewer moving parts; needs the sensor classes extracted and is a mixin into a method Fabric API does not touch (not contested, per `extracted/edges.json#injects_into`).
- GoalSelector approach via [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_LOAD|ENTITY_LOAD]] on [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]] goalSelector: rejected for villagers -- brain-driven mobs do not run goals; listed because it is the right answer for goal-driven mobs.

## Data / control / state dependencies

- Data: the totem item id; the flee parameters.
- Control: the mixin config lists the villager mixin (server side, but the class exists on both, so environment both).
- State: none persisted; the behaviour reads the held item each evaluation.

## Interactions with the selected environment

- Other mods modifying villager brains (a common category): two mixins into the same brain-building method are same-method injections; additive Injects coexist, a Redirect or Overwrite there does not ([[30-Mechanisms/Mixin|Mixin]]).
- Mods that change what villagers consider hostile interact with the sensor approach directly.
- The item must exist on the client for rendering; the AI code must not.

## Alternatives and tradeoffs

- Attach a status effect to the player instead of checking held items; then vanilla's own fear of the effect could be reused if one exists -- not verified.

## Implementation work

- Extract villager, brain, sensor and behaviour classes as extra types; confirm the injection target; write the mixin from `recipe/fabric_extend_behaviour_with_mixin`; add the item; unit-test the predicate with fakes.

## Verification obligations

- Target existence by extraction (possible now, after the extra-type run).
- Compile (possible now).
- Flee behaviour, radius, no regression in trading: need a game; none observed.

## Unresolved

- [[80-Unresolved/q.villager_ai_architecture|q.villager_ai_architecture]]
- [[80-Unresolved/q.unhooked_vanilla_members|q.unhooked_vanilla_members]]
- [[80-Unresolved/q.contested_method_policy|q.contested_method_policy]]
- [[80-Unresolved/q.injection_points_not_resolved|q.injection_points_not_resolved]]

## Evidence

- `extracted/minecraft_members.json`
- `extracted/edges.json#injects_into`
- `recipe/fabric_extend_behaviour_with_mixin`


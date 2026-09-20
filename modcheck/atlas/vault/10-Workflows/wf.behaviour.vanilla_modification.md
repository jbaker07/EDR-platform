---
type: "workflow"
id: "wf.behaviour.vanilla_modification"
area: "behaviour"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Change vanilla behaviour directly

**Intent.** Something vanilla does should happen differently: a different drop, a different damage rule, a different interaction result. The creator wants the smallest change that achieves it and keeps other mods working.

## Must be preserved

- Every Fabric API event fired from the modified method (removed by @Overwrite).
- Other mods' injections into the same method.

## Mechanisms that can serve it

- Fabric events first: the interaction and entity events ([[50-Interactions/events/net.fabricmc.fabric.api.event.player.UseBlockCallback.EVENT|EVENT]], [[50-Interactions/events/net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.BEFORE|BEFORE]], [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.ALLOW_DEATH|ALLOW_DEATH]] and the rest of `extracted/edges.json#callback_of`).
- [[30-Mechanisms/Mixin|Mixin]] when no event covers the point (`capability/modify_behaviour.fabric_extend_behaviour_with_mixin`).

## Tools and artifacts used today

- A mixin class listed in the mod's mixin config; javap on the target to confirm the method exists.

## Decisions the creator must make

- Event or mixin.
- For a mixin: injector kind (Inject adds, Redirect replaces one call, Overwrite replaces the body) -- see the mixin note under [[30-Mechanisms/Mixin|Mixin]].
- Whether to cancel, and where (a HEAD cancel on a contested method suppresses other mods).

## Information those decisions need

- The exact target method signature in 26.3 (`extracted/minecraft_members.json` for hooked types).
- Whether the target is contested (generated list under 50-Interactions).
- Application order relative to other mods ([[80-Unresolved/q.mixin_docs_application_order|q.mixin_docs_application_order]]).

## Existing automation

- Mixin scaffold generator (`capability/modify_behaviour.fabric_extend_behaviour_with_mixin`); target existence is not checked by the planner.

## Remaining manual or unsupported work

- Choosing the injection point; the collision check.

## ModCheck's contribution

- Target-existence check from minecraft_members.json; contested-method warning; the policy from [[80-Unresolved/q.contested_method_policy|q.contested_method_policy]] once decided.

## Interactions to check

- Any other mod's mixin into the same method ([[80-Unresolved/q.mixin_collision_analyser|q.mixin_collision_analyser]]).

## Evidence

- `capability/modify_behaviour.fabric_extend_behaviour_with_mixin`
- `extracted/edges.json#injects_into`
- `extracted/minecraft_members.json`

## Open questions

- [[80-Unresolved/q.contested_method_policy|q.contested_method_policy]]
- [[80-Unresolved/q.mixin_collision_analyser|q.mixin_collision_analyser]]
- [[80-Unresolved/q.injection_points_not_resolved|q.injection_points_not_resolved]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: True
- interaction_analysed: True
- implemented_in_modcheck: True
- validated_scope: the mixin scaffold compiles against the pinned corpus; no target-existence or collision check runs

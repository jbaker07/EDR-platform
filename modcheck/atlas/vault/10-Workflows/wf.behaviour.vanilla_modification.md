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

- Every Fabric API injection into the modified method and the events fired from it. What an @Overwrite does to them depends on the injection point and priority, established by transformation tests ([[30-Mechanisms/Mixin|Mixin]]).
- Other mods' injections into the same method.

## Mechanisms that can serve it

- Fabric events first: the interaction and entity events ([[50-Interactions/events/net.fabricmc.fabric.api.event.player.UseBlockCallback.EVENT|EVENT]], [[50-Interactions/events/net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.BEFORE|BEFORE]], [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.ALLOW_DEATH|ALLOW_DEATH]] and the rest of `extracted/edges.json#callback_of`).
- [[30-Mechanisms/Mixin|Mixin]] when no event covers the point (`capability/modify_behaviour.fabric_extend_behaviour_with_mixin`).

## Tools and artifacts used today

- A mixin class listed in the mod's mixin config; javap on the target to confirm the method exists.

## Decisions the creator must make

- Event or mixin.
- For a mixin: injector kind. @Inject adds; @Redirect replaces one call site and cannot share it with another @Redirect ([[30-Mechanisms/Transformation_Tests#C|scenario C]]) but composes under a @WrapOperation ([[30-Mechanisms/Transformation_Tests#D|scenario D]]); @Overwrite replaces the body and keeps other mods' HEAD/TAIL/RETURN injections while refusing their INVOKE-point injections unless theirs outrank it ([[30-Mechanisms/Transformation_Tests#A|scenario A]], [[30-Mechanisms/Transformation_Tests#B|scenario B]], [[30-Mechanisms/Transformation_Tests#K|scenario K]]).
- Whether to cancel, and where: a cancelling HEAD suppresses every later injection in the method, other mods' included ([[30-Mechanisms/Transformation_Tests#H|scenario H]]).

## Information those decisions need

- The exact target method in the processed compile jar (`extracted/minecraft_surface.json.gz`; note it differs from Mojang's jar in 363 classes: `extracted/resolved_environment.json`).
- Whether the target is a shared target (`extracted/edges.json#shared_targets`) and with which injector effects.
- Application order relative to other mods: lower priority first, configuration order at equal priority in the harness ([[30-Mechanisms/Transformation_Tests#E|scenario E]], [[30-Mechanisms/Transformation_Tests#G|scenario G]]); the documented rule is [[80-Unresolved/q.mixin_docs_application_order|q.mixin_docs_application_order]].

## Existing automation

- Mixin scaffold generator (`capability/modify_behaviour.fabric_extend_behaviour_with_mixin`); target existence is not checked by the planner.

## Remaining manual or unsupported work

- Choosing the injection point; the collision check.

## ModCheck's contribution

- Target-existence check from the surface; shared-target warning; the policy from [[80-Unresolved/q.contested_method_policy|q.contested_method_policy]] once decided, encoded with the composition table.

## Interactions to check

- Any other mod's mixin into the same method ([[80-Unresolved/q.mixin_collision_analyser|q.mixin_collision_analyser]]).

## Evidence

- `capability/modify_behaviour.fabric_extend_behaviour_with_mixin`
- `extracted/edges.json#injects_into`
- `extracted/mixin_transformation_tests.json`
- `extracted/minecraft_surface.json.gz`

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

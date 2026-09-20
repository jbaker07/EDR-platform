---
type: "mechanism"
module: "fabric-content-registries-v0"
version: "15.0.4+74ed1ea55d"
sha256: "e83273ce3a8d06e08c00f31bdc38497d653f678af3693a2fc3ba094537fa8879"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-content-registries-v0

**Version** `15.0.4+74ed1ea55d` -- **artifact sha256** `e83273ce3a8d06e08c00f31bdc38497d653f678af3693a2fc3ba094537fa8879`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-lifecycle-events-v1": "*", "fabric-resource-loader-v1": "*", "fabric-item-api-v1": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-content-registries-v0.mixins.json", {"config": "fabric-content-registries-v0.client.mixins.json", "environment": "client"}]`
- mixin classes: 20 found by annotation, 20 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.gui.Hud|Hud]].`extractAirBubbles` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/entity/player/Player;III)V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/world/entity/player/Player;isEyeInFluid(Lnet/minecraft/tags/TagKey;)Z` (inherited_exact) | client | 1000 (default) | `HudMixin.popTheBubbleForCustomFluids` |
| [[40-Interfaces/net.minecraft.client.player.LocalPlayer|LocalPlayer]].`aiStep` | `()V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/client/player/LocalPlayer;isInWater()Z` (inherited_exact) | client | 1000 (default) | `LocalPlayerMixin.handleCustomDownSwimmableFluids` |
| [[40-Interfaces/net.minecraft.client.player.LocalPlayer|LocalPlayer]].`isSprintingPossible` | `(Z)Z` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/client/player/LocalPlayer;isInShallowWater()Z` (inherited_exact) | client | 1000 (default) | `LocalPlayerMixin.preventSprintingInFluid` |
| [[40-Interfaces/net.minecraft.client.player.LocalPlayer|LocalPlayer]].`shouldStopSwimSprinting` | `()Z` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/client/player/LocalPlayer;isInWater()Z` (inherited_exact) | client | 1000 (default) | `LocalPlayerMixin.handleCustomSwimming` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`<init>` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/world/entity/EntityFluidInteraction;<init>(Ljava/util/Set;)V` (exact) | both | 1000 (default) | `EntityMixin.addCustomTags` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`canSpawnSprintParticle` | `()Z` | name_only | @ModifyReturnValue | RETURN | both | 1000 (default) | `EntityMixin.preventParticlesInFluids` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`isInLiquid` | `()Z` | name_only | @ModifyReturnValue | RETURN | both | 1000 (default) | `EntityMixin.checkForCustomFluids` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`isVisuallyCrawling` | `()Z` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/world/entity/Entity;isInWater()Z` (exact) | both | 1000 (default) | `EntityMixin.checkCustomFluids` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`updateFluidInteraction` | `()Z` | name_only | @ModifyReturnValue | RETURN | both | 1000 (default) | `EntityMixin.handleCustomFluidInteractionUpdates` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`updateSwimming` | `()V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/world/entity/Entity;isInWater()Z` (exact) | both | 1000 (default) | `EntityMixin.checkIfInSwimmableFluid` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`updateSwimming` | `()V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/world/entity/Entity;isUnderWater()Z` (exact) | both | 1000 (default) | `EntityMixin.checkIfUnderSwimmableFluid` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`updateSwimming` | `()V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/level/material/FluidState;is(Lnet/minecraft/tags/TagKey;)Z` (inherited_exact) | both | 1000 (default) | `EntityMixin.checkIfStandingInSwimmableFluid` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`aiStep` | `()V` | name_only | @ModifyExpressionValue | MIXINEXTRAS:EXPRESSION (selector_unsupported) | both | 1000 (default) | `LivingEntityMixin.tryOtherFluidsForFluidJumping` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`aiStep` | `()V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/world/entity/LivingEntity;isInLava()Z` (inherited_exact) | both | 1000 (default) | `LivingEntityMixin.jumpInCustomFluid` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`aiStep` | `()V` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/world/entity/LivingEntity;isInShallowFluid(Lnet/minecraft/tags/TagKey;)Z` (exact) | both | 1000 (default) | `LivingEntityMixin.swapFluidForShallowCheck` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`aiStep` | `()V` | name_only | @ModifyArg | MIXINEXTRAS:EXPRESSION (selector_unsupported) | both | 1000 (default) | `LivingEntityMixin.swapFluidTag` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`baseTick` | `()V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/world/entity/LivingEntity;isEyeInFluid(Lnet/minecraft/tags/TagKey;)Z` (inherited_exact) | both | 1000 (default) | `LivingEntityMixin.customFluidDrowning` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`checkFallDamage` | `(DZLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/world/entity/LivingEntity;isInWater()Z` (inherited_exact) | both | 1000 (default) | `LivingEntityMixin.fixDoubleUpdateForCustomFluids` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`travelFlying` | `(Lnet/minecraft/world/phys/Vec3;FFF)V` | exact | @Inject | HEAD | both | 1000 (default) | `LivingEntityMixin.travelFlyingInCustomFluid` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`travelInFluid` | `(Lnet/minecraft/world/phys/Vec3;)V` | name_only | @WrapWithCondition | INVOKE `Lnet/minecraft/world/entity/LivingEntity;travelInLava(Lnet/minecraft/world/phys/Vec3;DZD)V` (exact) | both | 1000 (default) | `LivingEntityMixin.travelInCustomFluid` |
| [[40-Interfaces/net.minecraft.world.entity.ai.behavior.GiveGiftToHero|GiveGiftToHero]].`<clinit>` | `()V` | exact | @Inject | TAIL | both | 1000 (default) | `GiveGiftToHeroMixin.makeMutable` |
| [[40-Interfaces/net.minecraft.world.entity.ai.behavior.Swim|Swim]].`checkExtraStartConditions` | `?` | ambiguous | @ModifyReturnValue | RETURN | both | 1000 (default) | `SwimMixin.floatInCustomFluids` |
| [[40-Interfaces/net.minecraft.world.entity.ai.goal.FloatGoal|FloatGoal]].`canUse` | `()Z` | name_only | @ModifyReturnValue | RETURN | both | 1000 (default) | `FloatGoalMixin.floatInCustomFluids` |
| [[40-Interfaces/net.minecraft.world.entity.npc.villager.Villager|Villager]].`wantsToPickUp` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;)Z` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/item/ItemStack;is(Lnet/minecraft/tags/TagKey;)Z` (inherited_exact) | both | 1000 (default) | `VillagerMixin.useGatherableItemsSet` |
| [[40-Interfaces/net.minecraft.world.entity.vehicle.boat.AbstractBoat|AbstractBoat]].`checkInWater` | `()Z` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/level/material/FluidState;is(Lnet/minecraft/tags/TagKey;)Z` (inherited_exact) | both | 1000 (default) | `AbstractBoatMixin.customFluidSupport` |
| [[40-Interfaces/net.minecraft.world.entity.vehicle.boat.AbstractBoat|AbstractBoat]].`isUnderwater` | `()Lnet/minecraft/world/entity/vehicle/boat/AbstractBoat$Status;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/level/material/FluidState;is(Lnet/minecraft/tags/TagKey;)Z` (inherited_exact) | both | 1000 (default) | `AbstractBoatMixin.customFluidSupport` |
| [[40-Interfaces/net.minecraft.world.item.HoneycombItem|HoneycombItem]].`lambda$static$0` | `?` | ambiguous | @Inject | RETURN | both | 1000 (default) | `HoneycombItemMixin.createWaxables` |
| [[40-Interfaces/net.minecraft.world.level.block.FireBlock|FireBlock]].`<init>` | `(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `FireBlockMixin.afterConstruct` |
| [[40-Interfaces/net.minecraft.world.level.block.FireBlock|FireBlock]].`getBurnOdds` | `(Lnet/minecraft/world/level/block/state/BlockState;)I` | name_only | @Inject | HEAD | both | 1000 (default) | `FireBlockMixin.getFabricSpreadChance` |
| [[40-Interfaces/net.minecraft.world.level.block.FireBlock|FireBlock]].`getIgniteOdds` | `(Lnet/minecraft/world/level/block/state/BlockState;)I` | exact | @Inject | HEAD | both | 1000 (default) | `FireBlockMixin.getFabricBurnChance` |
| [[40-Interfaces/net.minecraft.world.level.block.WeatheringCopper|WeatheringCopper]].`lambda$static$0` | `()Lcom/google/common/collect/BiMap;` | name_only | @Inject | RETURN | both | 1000 (default) | `WeatheringCopperMixin.createOxidationLevelIncreasesMap` |
| [[40-Interfaces/net.minecraft.world.level.pathfinder.PathfindingContext|PathfindingContext]].`getPathTypeFromState` | `(III)Lnet/minecraft/world/level/pathfinder/PathType;` | name_only | @Inject | INVOKE_ASSIGN `Lnet/minecraft/core/BlockPos$MutableBlockPos;set(III)Lnet/minecraft/core/BlockPos$MutableBlockPos;` (exact) | both | 1000 (default) | `PathfindingContextMixin.onGetNodeType` |
| [[40-Interfaces/net.minecraft.world.level.pathfinder.WalkNodeEvaluator|WalkNodeEvaluator]].`getPathTypeFromState` | `(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/pathfinder/PathType;` | name_only | @Inject | INVOKE `Lnet/minecraft/world/level/block/state/BlockState;getBlock()Lnet/minecraft/world/level/block/Block;` (inherited_exact) | both | 999 | `WalkNodeEvaluatorMixin.getCommonNodeType` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.registry.FlammableBlockRegistry|FlammableBlockRegistry]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.LandPathTypeRegistry|LandPathTypeRegistry]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.OxidizableBlocksRegistry|OxidizableBlocksRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.VibrationFrequencyRegistry|VibrationFrequencyRegistry]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.VillagerInteractionRegistries|VillagerInteractionRegistries]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.fluid.EntityFluidExtension|EntityFluidExtension]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.fluid.EntityFluidInteractionRegistry|EntityFluidInteractionRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.fluid.FluidBehavior|FluidBehavior]] (interface, 13 members)
- [[40-Interfaces/net.fabricmc.fabric.api.util.Block2ObjectMap|Block2ObjectMap]] (interface, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.util.Item2ObjectMap|Item2ObjectMap]] (interface, 7 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.

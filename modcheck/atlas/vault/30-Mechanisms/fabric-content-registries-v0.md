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

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]] | `<init>` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/world/entity/EntityFluidInteraction;<init>(Ljava/util/Set;)V` | both | `EntityMixin.addCustomTags` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `aiStep` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/world/entity/LivingEntity;isInShallowFluid(Lnet/minecraft/tags/TagKey;)Z` | both | `LivingEntityMixin.swapFluidForShallowCheck` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `aiStep` | injects_into `@ModifyArg at MIXINEXTRAS:EXPRESSION` | both | `LivingEntityMixin.swapFluidTag` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `travelFlying(Lnet/minecraft/world/phys/Vec3;FFF)V` | injects_into `@Inject at HEAD` | both | `LivingEntityMixin.travelFlyingInCustomFluid` |
| [[40-Interfaces/net.minecraft.world.entity.ai.behavior.GiveGiftToHero|GiveGiftToHero]] | `<clinit>` | injects_into `@Inject at TAIL` | both | `GiveGiftToHeroMixin.makeMutable` |
| [[40-Interfaces/net.minecraft.world.item.HoneycombItem|HoneycombItem]] | `lambda$static$0` | injects_into `@Inject at RETURN` | both | `HoneycombItemMixin.createWaxables` |
| [[40-Interfaces/net.minecraft.world.level.block.FireBlock|FireBlock]] | `<init>` | injects_into `@Inject at RETURN` | both | `FireBlockMixin.afterConstruct` |
| [[40-Interfaces/net.minecraft.world.level.block.FireBlock|FireBlock]] | `getBurnOdds` | injects_into `@Inject at HEAD` | both | `FireBlockMixin.getFabricSpreadChance` |
| [[40-Interfaces/net.minecraft.world.level.block.FireBlock|FireBlock]] | `getIgniteOdds(Lnet/minecraft/world/level/block/state/BlockState;)I` | injects_into `@Inject at HEAD` | both | `FireBlockMixin.getFabricBurnChance` |
| [[40-Interfaces/net.minecraft.world.level.block.WeatheringCopper|WeatheringCopper]] | `lambda$static$0` | injects_into `@Inject at RETURN` | both | `WeatheringCopperMixin.createOxidationLevelIncreasesMap` |
| [[40-Interfaces/net.minecraft.world.level.pathfinder.PathfindingContext|PathfindingContext]] | `getPathTypeFromState` | injects_into `@Inject at INVOKE_ASSIGN Lnet/minecraft/core/BlockPos$MutableBlockPos;set(III)Lnet/minecraft/core/BlockPos$MutableBlockPos;` | both | `PathfindingContextMixin.onGetNodeType` |
| [[40-Interfaces/net.minecraft.world.level.pathfinder.WalkNodeEvaluator|WalkNodeEvaluator]] | `getPathTypeFromState` | injects_into `@Inject at INVOKE Lnet/minecraft/world/level/block/state/BlockState;getBlock()Lnet/minecraft/world/level/block/Block;` | both | `WalkNodeEvaluatorMixin.getCommonNodeType` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.registry.FlammableBlockRegistry|FlammableBlockRegistry]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.LandPathTypeRegistry|LandPathTypeRegistry]] (class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.OxidizableBlocksRegistry|OxidizableBlocksRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.VibrationFrequencyRegistry|VibrationFrequencyRegistry]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.VillagerInteractionRegistries|VillagerInteractionRegistries]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.fluid.EntityFluidExtension|EntityFluidExtension]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.fluid.EntityFluidInteractionRegistry|EntityFluidInteractionRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.registry.fluid.FluidBehavior|FluidBehavior]] (interface, 14 members)
- [[40-Interfaces/net.fabricmc.fabric.api.util.Block2ObjectMap|Block2ObjectMap]] (interface, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.util.Item2ObjectMap|Item2ObjectMap]] (interface, 7 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.

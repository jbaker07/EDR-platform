---
type: "mechanism"
module: "fabric-block-api-v1"
version: "3.1.0+0ae6f9115d"
sha256: "12df8ce066403f03483ef89d13600072b56c5be290fb12260035fdaf5604761f"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-block-api-v1

**Version** `3.1.0+0ae6f9115d` -- **artifact sha256** `12df8ce066403f03483ef89d13600072b56c5be290fb12260035fdaf5604761f`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `{}`
- mixin configs: `["fabric-block-api-v1.mixins.json"]`
- access widener: `fabric-block-api-v1.classtweaker`
- mixin classes: 10 found by annotation, 10 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.block.v1.FluidFlowEvents.ALLOW|FluidFlowEvents.ALLOW]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`trapdoorUsableAsLadder` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z` | name_only | @Inject | INVOKE_ASSIGN `Lnet/minecraft/world/level/Level;getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;` (exact) | both | 1000 (default) | `LivingEntityMixin.allowTaggedBlocksForTrapdoorClimbing` |
| [[40-Interfaces/net.minecraft.world.inventory.EnchantmentMenu|EnchantmentMenu]].`lambda$slotsChanged$0` | `(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/level/block/EnchantingTableBlock;isValidBookShelf(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;)Z` (exact) | both | 1000 (default) | `EnchantmentMenuMixin.addEnchantingPower` |
| [[40-Interfaces/net.minecraft.world.inventory.EnchantmentMenu|EnchantmentMenu]].`lambda$slotsChanged$0` | `(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/world/item/enchantment/EnchantmentHelper;getEnchantmentCost(Lnet/minecraft/util/RandomSource;IILnet/minecraft/world/item/ItemStack;)I` (exact) | both | 1000 (default) | `EnchantmentMenuMixin.replaceBookcasesWithPower` |
| [[40-Interfaces/net.minecraft.world.level.block.LiquidBlock|LiquidBlock]].`shouldSpreadLiquid` | `(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z` | name_only | @Inject | HEAD | both | 1000 (default) | `LiquidBlockMixin.shouldSpreadLiquid` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunkSection|LevelChunkSection]].`setBlockState` | `(IIILnet/minecraft/world/level/block/state/BlockState;Z)Lnet/minecraft/world/level/block/state/BlockState;` | exact | @Redirect | INVOKE `Lnet/minecraft/world/level/block/state/BlockState;isAir()Z` (inherited_exact) | both | 1000 (default) | `LevelChunkSectionMixin.modifyAirCheck` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunkSection_1BlockCounter|LevelChunkSection$1BlockCounter]].`accept` | `(Lnet/minecraft/world/level/block/state/BlockState;I)V` | exact | @Redirect | INVOKE `Lnet/minecraft/world/level/block/state/BlockState;isAir()Z` (inherited_exact) | both | 1000 (default) | `ChunkSectionBlockStateCounterMixin.modifyAirCheck` |
| [[40-Interfaces/net.minecraft.world.level.material.FlowingFluid|FlowingFluid]].`spreadTo` | `(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/material/FluidState;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `FlowingFluidMixin.shouldSpreadLiquid` |
| [[40-Interfaces/net.minecraft.world.level.material.LavaFluid|LavaFluid]].`spreadTo` | `(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/material/FluidState;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `LavaFluidMixin.shouldSpreadLiquid` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.block.v1.BlockFunctionalityTags|BlockFunctionalityTags]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.block.v1.FabricBlock|FabricBlock]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.block.v1.FabricBlockState|FabricBlockState]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.block.v1.FluidFlowEvents|FluidFlowEvents]] (class, 1 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.

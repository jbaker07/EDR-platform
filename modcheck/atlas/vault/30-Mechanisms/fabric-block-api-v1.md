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

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.block.v1.FluidFlowEvents.ALLOW|FluidFlowEvents.ALLOW]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `trapdoorUsableAsLadder` | injects_into `@Inject at INVOKE_ASSIGN Lnet/minecraft/world/level/Level;getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;` | both | `LivingEntityMixin.allowTaggedBlocksForTrapdoorClimbing` |
| [[40-Interfaces/net.minecraft.world.inventory.EnchantmentMenu|EnchantmentMenu]] | `lambda$slotsChanged$0` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/world/item/enchantment/EnchantmentHelper;getEnchantmentCost(Lnet/minecraft/util/RandomSource;IILnet/minecraft/world/item/ItemStack;)I` | both | `EnchantmentMenuMixin.replaceBookcasesWithPower` |
| [[40-Interfaces/net.minecraft.world.level.block.LiquidBlock|LiquidBlock]] | `shouldSpreadLiquid` | injects_into `@Inject at HEAD` | both | `LiquidBlockMixin.shouldSpreadLiquid` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunkSection|LevelChunkSection]] | `setBlockState(IIILnet/minecraft/world/level/block/state/BlockState;Z)Lnet/minecraft/world/level/block/state/BlockState;` | wraps `@Redirect at INVOKE Lnet/minecraft/world/level/block/state/BlockState;isAir()Z` | both | `LevelChunkSectionMixin.modifyAirCheck` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunkSection_1BlockCounter|LevelChunkSection$1BlockCounter]] | `accept(Lnet/minecraft/world/level/block/state/BlockState;I)V` | wraps `@Redirect at INVOKE Lnet/minecraft/world/level/block/state/BlockState;isAir()Z` | both | `ChunkSectionBlockStateCounterMixin.modifyAirCheck` |
| [[40-Interfaces/net.minecraft.world.level.material.FlowingFluid|FlowingFluid]] | `spreadTo` | injects_into `@Inject at HEAD` | both | `FlowingFluidMixin.shouldSpreadLiquid` |
| [[40-Interfaces/net.minecraft.world.level.material.LavaFluid|LavaFluid]] | `spreadTo` | injects_into `@Inject at HEAD` | both | `LavaFluidMixin.shouldSpreadLiquid` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.block.v1.BlockFunctionalityTags|BlockFunctionalityTags]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.block.v1.FabricBlock|FabricBlock]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.block.v1.FabricBlockState|FabricBlockState]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.block.v1.FluidFlowEvents|FluidFlowEvents]] (class, 2 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.

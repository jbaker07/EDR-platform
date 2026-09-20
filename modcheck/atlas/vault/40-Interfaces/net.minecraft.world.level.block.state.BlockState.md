---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.BlockState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.BlockState

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase`; implements `net/fabricmc/fabric/api/block/v1/FabricBlockState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@53 in `BlockApiCacheImpl.find` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@58 in `BlockApiLookupImpl.find` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@8 in `FabricBlockState.getAppearance` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@6 in `FabricBlockState.getProvidedEnchantmentPower` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@13 in `LivingEntityMixin.allowTaggedBlocksForTrapdoorClimbing` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@22 in `LandPathTypeRegistry.getPathType` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@5 in `FireBlockMixin.getFabricBurnChance` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@5 in `FireBlockMixin.getFabricSpreadChance` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@16 in `LivingEntityMixin.modifyBedForOccupiedState` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@16 in `LivingEntityMixin.modifyWakeUpPosition` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@42 in `InteractionEventsRouter.lambda$onInitialize$0` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@53 in `InteractionEventsRouter.lambda$onInitialize$0` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@4 in `BlockColorRegistryImpl.getFactory` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@12 in `FluidVariantAttributeHandler.getName` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@4 in `ItemStorage.lambda$static$2` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@89 in `ItemStorage.lambda$static$2` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@4 in `CauldronStorage.getCurrentContent` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | inherited_exact | invokevirtual@262 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getLightEmission` | `()I` | inherited_exact | invokevirtual@1 in `AoLuminanceFix.fixed` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightEmission` | `()I` | inherited_exact | invokevirtual@70 in `AltModelBlockRendererImpl.tesselateBlock` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightEmission` | `()I` | inherited_exact | invokevirtual@12 in `FluidVariantAttributeHandler.getLightEmission` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOffset` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@4 in `AltModelBlockRendererImpl.tesselateBlock` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getProvidedEnchantmentPower` | `(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)` | inherited_exact | invokevirtual@56 in `EnchantmentMenuMixin.addEnchantingPower` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getSeed` | `(Lnet/minecraft/core/BlockPos;)J` | inherited_exact | invokevirtual@7 in `LevelExtractorMixin.hasMaterialFlagProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getSeed` | `(Lnet/minecraft/core/BlockPos;)J` | inherited_exact | invokevirtual@12 in `SubmitNodeCollectionMixin.hasMaterialFlagProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getShadeBrightness` | `(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)` | inherited_exact | invokevirtual@3 in `AoLuminanceFix.vanilla` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getShadeBrightness` | `(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)` | inherited_exact | invokevirtual@10 in `AoLuminanceFix.fixed` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lan` | inherited_exact | invokevirtual@27 in `LivingEntityMixin.allowTaggedBlocksForTrapdoorClimbing` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lan` | inherited_exact | invokevirtual@34 in `LivingEntityMixin.allowTaggedBlocksForTrapdoorClimbing` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lan` | inherited_exact | invokevirtual@30 in `FireBlockMixin.getFabricBurnChance` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lan` | inherited_exact | invokevirtual@30 in `FireBlockMixin.getFabricSpreadChance` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lan` | inherited_exact | invokevirtual@11 in `LivingEntityMixin.setOccupiedState` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lan` | inherited_exact | invokevirtual@4 in `DetectorRailBlockMixin.getCustomComparatorOutput` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lan` | inherited_exact | invokevirtual@26 in `CauldronFluidContent.currentLevel` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lan` | inherited_exact | invokevirtual@26 in `ComposterWrapper$BottomStorage.hasBoneMeal` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lan` | inherited_exact | invokevirtual@41 in `ComposterWrapper$TopStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lan` | inherited_exact | invokevirtual@170 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lan` | inherited_exact | invokevirtual@43 in `ContainerSlotWrapper.updateSnapshots` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lan` | inherited_exact | invokevirtual@18 in `DropperBlockMixin.hookDispense` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `hasBlockEntity` | `()Z` | inherited_exact | invokevirtual@30 in `BlockApiLookupImpl.find` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `hasBlockEntity` | `()Z` | inherited_exact | invokevirtual@75 in `ServerPlayerGameModeMixin.startBlockBreak` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `hasProperty` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Z` | inherited_exact | invokevirtual@20 in `FireBlockMixin.getFabricBurnChance` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `hasProperty` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Z` | inherited_exact | invokevirtual@20 in `FireBlockMixin.getFabricSpreadChance` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `hasProperty` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Z` | inherited_exact | invokevirtual@55 in `LivingEntityMixin.setOccupiedState` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `hasProperty` | `(Lnet/minecraft/world/level/block/state/properties/Property;)Z` | inherited_exact | invokevirtual@2 in `ServerPlayerMixin.redirectSleepDirection` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `initCache` | `()V` | inherited_exact | invokevirtual@33 in `BlocksMixin.lambda$initShapeCache$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `is` | `(Ljava/lang/Object;)Z` | inherited_exact | invokevirtual@4 in `ChunkSectionBlockStateCounterMixin.modifyAirCheck` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/lang/Object;)Z` | inherited_exact | invokevirtual@14 in `ChunkSectionBlockStateCounterMixin.modifyAirCheck` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/lang/Object;)Z` | inherited_exact | invokevirtual@24 in `ChunkSectionBlockStateCounterMixin.modifyAirCheck` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/lang/Object;)Z` | inherited_exact | invokevirtual@4 in `LevelChunkSectionMixin.modifyAirCheck` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/lang/Object;)Z` | inherited_exact | invokevirtual@14 in `LevelChunkSectionMixin.modifyAirCheck` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/lang/Object;)Z` | inherited_exact | invokevirtual@24 in `LevelChunkSectionMixin.modifyAirCheck` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/lang/Object;)Z` | inherited_exact | invokevirtual@19 in `ModelLoadingEventDispatcher$BlockStateResolverContext.setModel` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `is` | `(Lnet/minecraft/tags/TagKey;)Z` | inherited_exact | invokevirtual@5 in `LivingEntityMixin.allowTaggedBlocksForTrapdoorClimbing` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `isAir` | `()Z` | inherited_exact | invokevirtual@4 in `BreezeMixin.modifyBlockStateParticleOption` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `isCollisionShapeFullBlock` | `(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)` | inherited_exact | invokevirtual@39 in `AoCalculator.calcFastVanilla` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isCollisionShapeFullBlock` | `(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)` | inherited_exact | invokevirtual@57 in `FlatLighter.light` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isLightPermeable` | `()Z` | inherited_exact | invokevirtual@142 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isLightPermeable` | `()Z` | inherited_exact | invokevirtual@229 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isLightPermeable` | `()Z` | inherited_exact | invokevirtual@316 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isLightPermeable` | `()Z` | inherited_exact | invokevirtual@403 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isLightPermeable` | `()Z` | inherited_exact | invokevirtual@500 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isLightPermeable` | `()Z` | inherited_exact | invokevirtual@597 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isLightPermeable` | `()Z` | inherited_exact | invokevirtual@694 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isLightPermeable` | `()Z` | inherited_exact | invokevirtual@791 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isLightPermeable` | `()Z` | inherited_exact | invokevirtual@845 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isLightPermeable` | `()Z` | inherited_exact | invokevirtual@870 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isSolidRender` | `()Z` | inherited_exact | invokevirtual@822 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang` | inherited_exact | invokevirtual@73 in `LivingEntityMixin.setOccupiedState` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `setValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang` | inherited_exact | invokevirtual@31 in `CauldronStorage.updateLevel` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang` | inherited_exact | invokevirtual@24 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setValue` | `(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang` | inherited_exact | invokevirtual@198 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `toString` | `()Ljava/lang/String;` | inherited_exact | invokevirtual@185 in `AoCalculator.compute` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (3 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final FULL_CODEC : Lcom/mojang/serialization/Codec;
private static final CONSTANT_OR_DISPATCH_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public <init>(Lnet/minecraft/world/level/block/Block;[Lnet/minecraft/world/level/block/state/properties/Property;[Ljava/lang/Comparable;)V
protected asState()Lnet/minecraft/world/level/block/state/BlockState;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/block/state/BlockState;)Lcom/mojang/datafixers/util/Either;
private static synthetic lambda$static$0(Lcom/mojang/datafixers/util/Either;)Lnet/minecraft/world/level/block/state/BlockState;
private static synthetic lambda$static$1(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;
static <clinit>()V
```

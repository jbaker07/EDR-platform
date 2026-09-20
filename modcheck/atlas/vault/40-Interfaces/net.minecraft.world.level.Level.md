---
type: "interface"
fqcn: "net.minecraft.world.level.Level"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.Level

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/level/LevelAccessor`, `java/lang/AutoCloseable`, `net/fabricmc/fabric/api/attachment/v1/AttachmentTarget`, `net/fabricmc/fabric/api/attachment/v1/GlobalAttachmentsProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/storage/WritableLevelData;Lnet/minecraft/r` | exact | invokespecial@14 in `ServerLevelMixin.<init>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/world/level/storage/WritableLevelData;Lnet/minecraft/r` | exact | invokespecial@14 in `ClientLevelMixin.<init>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `addFreshEntity` | `(Lnet/minecraft/world/entity/Entity;)Z` | inherited_exact | invokevirtual@109 in `PlayerInventoryStorageImpl$DroppedStacks.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `dimension` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@103 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `dimensionTypeRegistration` | `()Lnet/minecraft/core/Holder;` | exact | invokevirtual@5 in `DebugMessages.forGlobalPos` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `environmentAttributes` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeSystem;` | exact | invokevirtual@5 in `FluidVariantAttributes$3.getViscosity` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `gameEvent` | `(Lnet/minecraft/core/Holder;Lnet/minecraft/core/BlockPos;Lnet/minecraf` | inherited_exact | invokevirtual@60 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `gameEvent` | `(Lnet/minecraft/core/Holder;Lnet/minecraft/core/BlockPos;Lnet/minecraf` | inherited_exact | invokevirtual@237 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlockEntity` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity` | exact | invokevirtual@38 in `BlockApiLookupImpl.find` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getBlockEntity` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity` | exact | invokevirtual@5 in `AttachmentTargetInfo$BlockEntityTarget.getTarget` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getBlockEntity` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity` | exact | invokevirtual@72 in `ContainerSlotWrapper.updateSnapshots` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | exact | invokevirtual@25 in `BlockApiLookupImpl.find` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | exact | invokevirtual@47 in `EnchantmentMenuMixin.addEnchantingPower` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | exact | invokevirtual@25 in `LivingEntityMixin.onIsSleepingInBed` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | exact | invokevirtual@2 in `LivingEntityMixin.setOccupiedState` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | exact | invokevirtual@2 in `InteractionEventsRouter.lambda$onInitialize$0` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | exact | invokevirtual@14 in `CauldronStorage.createSnapshot` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | exact | invokevirtual@8 in `ComposterWrapper$LevelLocation.getBlockState` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getChunk` | `(IILnet/minecraft/world/level/chunk/status/ChunkStatus;Z)Lnet/minecraf` | exact | invokevirtual@19 in `AttachmentTargetInfo$ChunkTarget.getTarget` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getChunkSource` | `()Lnet/minecraft/world/level/chunk/ChunkSource;` | inherited_exact | invokevirtual@11 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getEntity` | `(I)Lnet/minecraft/world/entity/Entity;` | exact | invokevirtual@5 in `AttachmentTargetInfo$EntityTarget.getTarget` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getFluidState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/material/Flu` | exact | invokevirtual@11 in `LiquidBlockMixin.shouldSpreadLiquid` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getServer` | `()Lnet/minecraft/server/MinecraftServer;` | exact | invokevirtual@34 in `EntityPermissionContext.<init>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getServer` | `()Lnet/minecraft/server/MinecraftServer;` | exact | invokevirtual@44 in `EntityPermissionContext.<init>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `globalAttachments` | `()Lnet/fabricmc/fabric/api/attachment/v1/GlobalAttachments;` | inherited_exact | invokevirtual@1 in `AttachmentTargetInfo$GlobalTarget.getTarget` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `isClientSide` | `()Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| calls | `isClientSide` | `()Z` | exact | invokevirtual@11 in `BlockEntityMixin.fabric_shouldTryToSync` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `isClientSide` | `()Z` | exact | invokevirtual@4 in `EntityMixin.fabric_syncChange` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `isClientSide` | `()Z` | exact | invokevirtual@4 in `EntityMixin.fabric_shouldTryToSync` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `isClientSide` | `()Z` | exact | invokevirtual@4 in `LevelChunkMixin.fabric_shouldTryToSync` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `isClientSide` | `()Z` | exact | invokevirtual@4 in `LivingEntityMixin.isClient` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `isClientSide` | `()Z` | exact | invokevirtual@27 in `LivingEntityMixin.injectElytraTick` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `isClientSide` | `()Z` | exact | invokevirtual@18 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `isClientSide` | `()Z` | exact | invokevirtual@21 in `PlayerInventoryStorageImpl.drop` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `levelEvent` | `(ILnet/minecraft/core/BlockPos;I)V` | inherited_exact | invokevirtual@297 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `playSound` | `(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/sounds/SoundEven` | exact | invokevirtual@269 in `FluidStorageUtil.moveWithSound` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `playSound` | `(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/BlockPos;Lnet/` | exact | invokevirtual@86 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess;` | exact | invokevirtual@4 in `BlockEntityMixin.fabric_getRegistryAccess` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess;` | exact | invokevirtual@4 in `EntityMixin.fabric_getRegistryAccess` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess;` | exact | invokevirtual@4 in `LevelChunkMixin.fabric_getRegistryAccess` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `scheduleTick` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;I` | inherited_exact | invokevirtual@267 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setBlock` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/B` | inherited_exact | invokevirtual@56 in `CauldronStorage.updateLevel` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setBlock` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/B` | inherited_exact | invokevirtual@96 in `CauldronStorage.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setBlock` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/B` | inherited_exact | invokevirtual@16 in `CauldronStorage.readSnapshot` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setBlock` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/B` | inherited_exact | invokevirtual@31 in `CauldronStorage.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setBlockAndUpdate` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/B` | inherited_exact | invokevirtual@79 in `LivingEntityMixin.setOccupiedState` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `setBlockAndUpdate` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/B` | inherited_exact | invokevirtual@50 in `CauldronStorage.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setBlockAndUpdate` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/B` | inherited_exact | invokevirtual@9 in `ComposterWrapper$LevelLocation.setBlockState` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (36 fields, 144 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final RESOURCE_KEY_CODEC : Lcom/mojang/serialization/Codec;
public static final OVERWORLD : Lnet/minecraft/resources/ResourceKey;
public static final NETHER : Lnet/minecraft/resources/ResourceKey;
public static final END : Lnet/minecraft/resources/ResourceKey;
public static final MAX_LEVEL_SIZE : I
public static final ACROSS_THE_WHOLE_WORLD : I
public static final LONG_PARTICLE_CLIP_RANGE : I
public static final SHORT_PARTICLE_CLIP_RANGE : I
public static final MAX_BRIGHTNESS : I
public static final MAX_ENTITY_SPAWN_Y : I
public static final MIN_ENTITY_SPAWN_Y : I
private static final DEFAULT_EXPLOSION_BLOCK_PARTICLES : Lnet/minecraft/util/random/WeightedList;
protected final blockEntityTickers : Ljava/util/List;
protected final neighborUpdater : Lnet/minecraft/world/level/redstone/CollectingNeighborUpdater;
private final pendingBlockEntityTickers : Ljava/util/List;
private tickingBlockEntities : Z
private final thread : Ljava/lang/Thread;
private final isDebug : Z
private skyDarken : I
protected randValue : I
protected final addend : I
protected oRainLevel : F
protected rainLevel : F
protected oThunderLevel : F
protected thunderLevel : F
protected final random : Lnet/minecraft/util/RandomSource;
private final soundSeedGenerator : Lnet/minecraft/util/RandomSource;
private final dimensionTypeRegistration : Lnet/minecraft/core/Holder;
protected final levelData : Lnet/minecraft/world/level/storage/WritableLevelData;
private final isClientSide : Z
private final biomeManager : Lnet/minecraft/world/level/biome/BiomeManager;
private final dimension : Lnet/minecraft/resources/ResourceKey;
private final registryAccess : Lnet/minecraft/core/RegistryAccess;
private final damageSources : Lnet/minecraft/world/damagesource/DamageSources;
private final palettedContainerFactory : Lnet/minecraft/world/level/chunk/PalettedContainerFactory;
private subTickCount : J
protected <init>(Lnet/minecraft/world/level/storage/WritableLevelData;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/RegistryAccess;Lnet/minecraft/core/Holder;ZZJI)V
public getNextEntityId()I
public isClientSide()Z
public getServer()Lnet/minecraft/server/MinecraftServer;
public isInWorldBounds(Lnet/minecraft/core/BlockPos;)Z
public isInValidBounds(Lnet/minecraft/core/BlockPos;)Z
public static isInSpawnableBounds(Lnet/minecraft/core/BlockPos;)Z
private static isInWorldBoundsHorizontal(Lnet/minecraft/core/BlockPos;)Z
private static isInValidBoundsHorizontal(Lnet/minecraft/core/BlockPos;)Z
private static isOutsideSpawnableHeight(I)Z
public getChunkAt(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/chunk/LevelChunk;
public getChunk(II)Lnet/minecraft/world/level/chunk/LevelChunk;
public getChunk(IILnet/minecraft/world/level/chunk/status/ChunkStatus;Z)Lnet/minecraft/world/level/chunk/ChunkAccess;
public setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;II)Z
public updatePOIOnBlockStateChange(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;)V
public removeBlock(Lnet/minecraft/core/BlockPos;Z)Z
public destroyBlock(Lnet/minecraft/core/BlockPos;ZLnet/minecraft/world/entity/Entity;I)Z
public addDestroyBlockEffect(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public abstract sendBlockUpdated(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;I)V
public setBlocksDirty(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;)V
public updateNeighborsAt(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/redstone/Orientation;)V
public updateNeighborsAtExceptFromFacing(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/redstone/Orientation;)V
public neighborChanged(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/redstone/Orientation;)V
public neighborChanged(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/redstone/Orientation;Z)V
public neighborShapeChanged(Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;II)V
public getHeight(Lnet/minecraft/world/level/levelgen/Heightmap$Types;II)I
public getLightEngine()Lnet/minecraft/world/level/lighting/LevelLightEngine;
public getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public getFluidState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/material/FluidState;
public isBrightOutside()Z
public isDarkOutside()Z
public playSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/BlockPos;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FF)V
public abstract playSeededSound(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V
public playSeededSound(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FFJ)V
public abstract playSeededSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V
public playSound(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;)V
public playSound(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FF)V
public playSound(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FF)V
public playSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FF)V
public playSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FF)V
public playLocalSound(Lnet/minecraft/core/BlockPos;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FFZ)V
public playLocalSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FF)V
public playLocalSound(DDDLnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FFZ)V
public playPlayerSound(Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FF)V
public addParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V
public addParticle(Lnet/minecraft/core/particles/ParticleOptions;ZZDDDDDD)V
public addAlwaysVisibleParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V
public addAlwaysVisibleParticle(Lnet/minecraft/core/particles/ParticleOptions;ZDDDDDD)V
public addBlockEntityTicker(Lnet/minecraft/world/level/block/entity/TickingBlockEntity;)V
public tickBlockEntities()V
public guardEntityTick(Ljava/util/function/Consumer;Lnet/minecraft/world/entity/Entity;)V
public shouldTickDeath(Lnet/minecraft/world/entity/Entity;)Z
public shouldTickBlocksAt(J)Z
public shouldTickBlocksAt(Lnet/minecraft/core/BlockPos;)Z
public explode(Lnet/minecraft/world/entity/Entity;DDDFLnet/minecraft/world/level/Level$ExplosionInteraction;)V
public explode(Lnet/minecraft/world/entity/Entity;DDDFZLnet/minecraft/world/level/Level$ExplosionInteraction;)V
public explode(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lnet/minecraft/world/level/ExplosionDamageCalculator;Lnet/minecraft/world/phys/Vec3;FZLnet/minecraft/world/level/Level$ExplosionInteraction;)V
public explode(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lnet/minecraft/world/level/ExplosionDamageCalculator;DDDFZLnet/minecraft/world/level/Level$ExplosionInteraction;)V
public abstract explode(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lnet/minecraft/world/level/ExplosionDamageCalculator;DDDFZLnet/minecraft/world/level/Level$ExplosionInteraction;Lnet/minecraft/core/particles/ParticleOptions;Lnet/minecraft/core/particles/ParticleOptions;Lnet/minecraft/util/random/WeightedList;Lnet/minecraft/core/Holder;)V
public abstract gatherChunkSourceStats()Ljava/lang/String;
public getBlockEntity(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity/BlockEntity;
public setBlockEntity(Lnet/minecraft/world/level/block/entity/BlockEntity;)V
public removeBlockEntity(Lnet/minecraft/core/BlockPos;)V
public isLoaded(Lnet/minecraft/core/BlockPos;)Z
public loadedAndEntityCanStandOnFace(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Direction;)Z
public loadedAndEntityCanStandOn(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;)Z
public updateSkyBrightness()V
public setSpawnSettings(Z)V
public abstract setRespawnData(Lnet/minecraft/world/level/storage/LevelData$RespawnData;)V
public abstract getRespawnData()Lnet/minecraft/world/level/storage/LevelData$RespawnData;
public getWorldBorderAdjustedRespawnData(Lnet/minecraft/world/level/storage/LevelData$RespawnData;)Lnet/minecraft/world/level/storage/LevelData$RespawnData;
public close()V
public getChunkForCollisions(II)Lnet/minecraft/world/level/BlockGetter;
public getEntities(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;
public getEntities(Lnet/minecraft/world/level/entity/EntityTypeTest;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;
public getEntities(Lnet/minecraft/world/level/entity/EntityTypeTest;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;Ljava/util/List;)V
public getEntities(Lnet/minecraft/world/level/entity/EntityTypeTest;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;Ljava/util/List;I)V
public hasEntities(Lnet/minecraft/world/level/entity/EntityTypeTest;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Z
public getPushableEntities(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;
public abstract getEntity(I)Lnet/minecraft/world/entity/Entity;
public getEntity(Ljava/util/UUID;)Lnet/minecraft/world/entity/Entity;
public getEntityInAnyDimension(Ljava/util/UUID;)Lnet/minecraft/world/entity/Entity;
public getPlayerInAnyDimension(Ljava/util/UUID;)Lnet/minecraft/world/entity/player/Player;
public abstract dragonParts()Ljava/util/Collection;
public blockEntityChanged(Lnet/minecraft/core/BlockPos;)V
public onBlockEntityAdded(Lnet/minecraft/world/level/block/entity/BlockEntity;)V
public getOverworldClockTime()J
public getDefaultClockTime()J
private getClockTimeTicks(Ljava/util/Optional;)J
public mayInteract(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/BlockPos;)Z
public broadcastEntityEvent(Lnet/minecraft/world/entity/Entity;B)V
public broadcastDamageEvent(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;)V
public blockEvent(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;II)V
public getLevelData()Lnet/minecraft/world/level/storage/LevelData;
public abstract tickRateManager()Lnet/minecraft/world/TickRateManager;
public getRelativeTickSpeed()F
public getThunderLevel(F)F
public setThunderLevel(F)V
public getRainLevel(F)F
public setRainLevel(F)V
public canHaveWeather()Z
public isThundering()Z
public isRaining()Z
public isRainingAt(Lnet/minecraft/core/BlockPos;)Z
public precipitationAt(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/biome/Biome$Precipitation;
public abstract getMapData(Lnet/minecraft/world/level/saveddata/maps/MapId;)Lnet/minecraft/world/level/saveddata/maps/MapItemSavedData;
public globalLevelEvent(ILnet/minecraft/core/BlockPos;I)V
public fillReportDetails(Lnet/minecraft/CrashReport;)Lnet/minecraft/CrashReportCategory;
public abstract destroyBlockProgress(ILnet/minecraft/core/BlockPos;I)V
public createFireworks(DDDDDDLjava/util/List;Z)V
public abstract getScoreboard()Lnet/minecraft/world/scores/Scoreboard;
public updateNeighbourForOutputSignal(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;)V
public getSkyDarken()I
public setSkyFlashTime(I)V
public sendPacketToServer(Lnet/minecraft/network/protocol/Packet;)V
public dimensionType()Lnet/minecraft/world/level/dimension/DimensionType;
public dimensionTypeRegistration()Lnet/minecraft/core/Holder;
public dimension()Lnet/minecraft/resources/ResourceKey;
public getRandom()Lnet/minecraft/util/RandomSource;
public isStateAtPosition(Lnet/minecraft/core/BlockPos;Ljava/util/function/Predicate;)Z
public isFluidAtPosition(Lnet/minecraft/core/BlockPos;Ljava/util/function/Predicate;)Z
public abstract recipeAccess()Lnet/minecraft/world/item/crafting/RecipeAccess;
public getBlockRandomPos(IIII)Lnet/minecraft/core/BlockPos;
public noSave()Z
public getBiomeManager()Lnet/minecraft/world/level/biome/BiomeManager;
public final isDebug()Z
protected abstract getEntities()Lnet/minecraft/world/level/entity/LevelEntityGetter;
public nextSubTickCount()J
public registryAccess()Lnet/minecraft/core/RegistryAccess;
public damageSources()Lnet/minecraft/world/damagesource/DamageSources;
public abstract clockManager()Lnet/minecraft/world/clock/ClockManager;
public abstract environmentAttributes()Lnet/minecraft/world/attribute/EnvironmentAttributeSystem;
public getClientLeafTintColor(Lnet/minecraft/core/BlockPos;)I
public palettedContainerFactory()Lnet/minecraft/world/level/chunk/PalettedContainerFactory;
public synthetic environmentAttributes()Lnet/minecraft/world/attribute/EnvironmentAttributeReader;
public synthetic getChunk(II)Lnet/minecraft/world/level/chunk/ChunkAccess;
private synthetic lambda$fillReportDetails$2()Ljava/lang/String;
private synthetic lambda$fillReportDetails$1()Ljava/lang/String;
private synthetic lambda$fillReportDetails$0()Ljava/lang/String;
private synthetic lambda$getClockTimeTicks$0(Lnet/minecraft/core/Holder;)Ljava/lang/Long;
private static synthetic lambda$hasEntities$0(Ljava/util/function/Predicate;Lorg/apache/commons/lang3/mutable/MutableBoolean;Lnet/minecraft/world/level/entity/EntityTypeTest;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/util/Continuation;
private static synthetic lambda$getEntities$1(Ljava/util/function/Predicate;Ljava/util/List;ILnet/minecraft/world/level/entity/EntityTypeTest;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/util/Continuation;
private static synthetic lambda$getEntities$0(Lnet/minecraft/world/entity/Entity;Ljava/util/function/Predicate;Ljava/util/List;Lnet/minecraft/world/entity/Entity;)V
static <clinit>()V
```

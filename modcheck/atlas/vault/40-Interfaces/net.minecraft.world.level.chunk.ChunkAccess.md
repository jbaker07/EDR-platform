---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.ChunkAccess"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.ChunkAccess

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/level/chunk/LightChunk`, `net/minecraft/world/level/chunk/StructureAccess`, `net/minecraft/world/level/biome/BiomeResolver`, `net/fabricmc/fabric/api/attachment/v1/AttachmentTarget`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getPersistedStatus` | `()Lnet/minecraft/world/level/chunk/status/ChunkStatus;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| calls | `getPos` | `()Lnet/minecraft/world/level/ChunkPos;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| calls | `getPos` | `()Lnet/minecraft/world/level/ChunkPos;` | exact | invokevirtual@25 in `LevelChunkMixin.fabric_syncChange` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `markUnsaved` | `()V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| reads | `chunkPos` | `Lnet/minecraft/world/level/ChunkPos;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |

## Declared members (19 fields, 73 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NO_FILLED_SECTION : I
private static final LOGGER : Lorg/slf4j/Logger;
private static final EMPTY_REFERENCE_SET : Lit/unimi/dsi/fastutil/longs/LongSet;
protected final postProcessing : [Lit/unimi/dsi/fastutil/shorts/ShortList;
private unsaved : Z
private isLightCorrect : Z
protected final chunkPos : Lnet/minecraft/world/level/ChunkPos;
private inhabitedTime : J
private carverBiomeSettings : Lnet/minecraft/world/level/biome/BiomeGenerationSettings;
protected final upgradeData : Lnet/minecraft/world/level/chunk/UpgradeData;
protected final blendingData : Lnet/minecraft/world/level/levelgen/blending/BlendingData;
protected final heightmaps : Ljava/util/Map;
protected skyLightSources : Lnet/minecraft/world/level/lighting/ChunkSkyLightSources;
private final structureStarts : Ljava/util/Map;
private final structureReferences : Ljava/util/Map;
protected final pendingBlockEntities : Ljava/util/Map;
protected final blockEntities : Ljava/util/Map;
protected final levelHeightAccessor : Lnet/minecraft/world/level/LevelHeightAccessor;
protected final sections : [Lnet/minecraft/world/level/chunk/LevelChunkSection;
public <init>(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/world/level/chunk/UpgradeData;Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/chunk/PalettedContainerFactory;J[Lnet/minecraft/world/level/chunk/LevelChunkSection;Lnet/minecraft/world/level/levelgen/blending/BlendingData;)V
private static replaceMissingSections(Lnet/minecraft/world/level/chunk/PalettedContainerFactory;[Lnet/minecraft/world/level/chunk/LevelChunkSection;)V
public getListenerRegistry(I)Lnet/minecraft/world/level/gameevent/GameEventListenerRegistry;
public setBlockState(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;
public abstract setBlockState(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Lnet/minecraft/world/level/block/state/BlockState;
public abstract setBlockEntity(Lnet/minecraft/world/level/block/entity/BlockEntity;)V
public abstract addEntity(Lnet/minecraft/world/entity/Entity;)V
public getHighestFilledSectionIndex()I
public getHighestSectionPosition()I
public getBlockEntitiesPos()Ljava/util/Set;
public getSections()[Lnet/minecraft/world/level/chunk/LevelChunkSection;
public getSection(I)Lnet/minecraft/world/level/chunk/LevelChunkSection;
public getHeightmaps()Ljava/util/Collection;
public setHeightmap(Lnet/minecraft/world/level/levelgen/Heightmap$Types;[J)V
public getOrCreateHeightmapUnprimed(Lnet/minecraft/world/level/levelgen/Heightmap$Types;)Lnet/minecraft/world/level/levelgen/Heightmap;
public hasPrimedHeightmap(Lnet/minecraft/world/level/levelgen/Heightmap$Types;)Z
public getHeight(Lnet/minecraft/world/level/levelgen/Heightmap$Types;II)I
public getPos()Lnet/minecraft/world/level/ChunkPos;
public getStartForStructure(Lnet/minecraft/world/level/levelgen/structure/Structure;)Lnet/minecraft/world/level/levelgen/structure/StructureStart;
public setStartForStructure(Lnet/minecraft/world/level/levelgen/structure/Structure;Lnet/minecraft/world/level/levelgen/structure/StructureStart;)V
public getAllStarts()Ljava/util/Map;
public setAllStarts(Ljava/util/Map;)V
public getReferencesForStructure(Lnet/minecraft/world/level/levelgen/structure/Structure;)Lit/unimi/dsi/fastutil/longs/LongSet;
public addReferenceForStructure(Lnet/minecraft/world/level/levelgen/structure/Structure;J)V
public getAllReferences()Ljava/util/Map;
public setAllReferences(Ljava/util/Map;)V
public isYSpaceEmpty(II)Z
public markUnsaved()V
public tryMarkSaved()Z
public isUnsaved()Z
public abstract getPersistedStatus()Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public getHighestGeneratedStatus()Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public abstract removeBlockEntity(Lnet/minecraft/core/BlockPos;)V
public markPosForPostProcessing(Lnet/minecraft/core/BlockPos;)V
public getPostProcessing()[Lit/unimi/dsi/fastutil/shorts/ShortList;
public addPackedPostProcess(Lit/unimi/dsi/fastutil/shorts/ShortList;I)V
public setBlockEntityNbt(Lnet/minecraft/nbt/CompoundTag;)V
public getBlockEntityNbt(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/nbt/CompoundTag;
public abstract getBlockEntityNbtForSaving(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/nbt/CompoundTag;
public final findBlockLightSources(Ljava/util/function/BiConsumer;)V
public findBlocks(Ljava/util/function/Predicate;Ljava/util/function/BiConsumer;)V
public abstract getBlockTicks()Lnet/minecraft/world/ticks/TickContainerAccess;
public abstract getFluidTicks()Lnet/minecraft/world/ticks/TickContainerAccess;
public collectBiomesInPalette(Ljava/util/Set;)V
public canBeSerialized()Z
public abstract getTicksForSerialization(J)Lnet/minecraft/world/level/chunk/ChunkAccess$PackedTicks;
public getUpgradeData()Lnet/minecraft/world/level/chunk/UpgradeData;
public isOldNoiseGeneration()Z
public getBlendingData()Lnet/minecraft/world/level/levelgen/blending/BlendingData;
public getInhabitedTime()J
public incrementInhabitedTime()V
public setInhabitedTime(J)V
public static getOrCreateOffsetList([Lit/unimi/dsi/fastutil/shorts/ShortList;I)Lit/unimi/dsi/fastutil/shorts/ShortList;
public isLightCorrect()Z
public setLightCorrect(Z)V
public getMinY()I
public getHeight()I
public carverBiome(Ljava/util/function/Supplier;)Lnet/minecraft/world/level/biome/BiomeGenerationSettings;
public getNoiseBiome(III)Lnet/minecraft/core/Holder;
public fillBiomesFromNoise(Lnet/minecraft/world/level/biome/BiomeResolver;)V
public getBelowZeroRetrogen()Lnet/minecraft/world/level/levelgen/BelowZeroRetrogen;
public isUpgrading()Z
public getHeightAccessorForGeneration()Lnet/minecraft/world/level/LevelHeightAccessor;
public initializeLightSources()V
public getSkyLightSources()Lnet/minecraft/world/level/lighting/ChunkSkyLightSources;
public static problemPath(Lnet/minecraft/world/level/ChunkPos;)Lnet/minecraft/util/ProblemReporter$PathElement;
public problemPath()Lnet/minecraft/util/ProblemReporter$PathElement;
private synthetic lambda$getNoiseBiome$0(III)Ljava/lang/String;
private static synthetic lambda$findBlocks$0(Ljava/util/function/BiConsumer;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/util/Continuation;
private static synthetic lambda$findBlockLightSources$0(Lnet/minecraft/world/level/block/state/BlockState;)Z
private static synthetic lambda$addReferenceForStructure$0(Lnet/minecraft/world/level/levelgen/structure/Structure;)Lit/unimi/dsi/fastutil/longs/LongSet;
private synthetic lambda$getOrCreateHeightmapUnprimed$0(Lnet/minecraft/world/level/levelgen/Heightmap$Types;)Lnet/minecraft/world/level/levelgen/Heightmap;
static <clinit>()V
```

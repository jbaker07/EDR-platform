---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.ImposterProtoChunk"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.ImposterProtoChunk

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/chunk/ProtoChunk`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `wrapped` | `Lnet/minecraft/world/level/chunk/LevelChunk;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |

## Declared members (2 fields, 45 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final wrapped : Lnet/minecraft/world/level/chunk/LevelChunk;
private final allowWrites : Z
public <init>(Lnet/minecraft/world/level/chunk/LevelChunk;Z)V
public getBlockEntity(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity/BlockEntity;
public getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public getFluidState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/material/FluidState;
public getSection(I)Lnet/minecraft/world/level/chunk/LevelChunkSection;
public setBlockState(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Lnet/minecraft/world/level/block/state/BlockState;
public setBlockEntity(Lnet/minecraft/world/level/block/entity/BlockEntity;)V
public addEntity(Lnet/minecraft/world/entity/Entity;)V
public setPersistedStatus(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)V
public getSections()[Lnet/minecraft/world/level/chunk/LevelChunkSection;
public setHeightmap(Lnet/minecraft/world/level/levelgen/Heightmap$Types;[J)V
private fixType(Lnet/minecraft/world/level/levelgen/Heightmap$Types;)Lnet/minecraft/world/level/levelgen/Heightmap$Types;
public getOrCreateHeightmapUnprimed(Lnet/minecraft/world/level/levelgen/Heightmap$Types;)Lnet/minecraft/world/level/levelgen/Heightmap;
public getHeight(Lnet/minecraft/world/level/levelgen/Heightmap$Types;II)I
public getNoiseBiome(III)Lnet/minecraft/core/Holder;
public getPos()Lnet/minecraft/world/level/ChunkPos;
public getStartForStructure(Lnet/minecraft/world/level/levelgen/structure/Structure;)Lnet/minecraft/world/level/levelgen/structure/StructureStart;
public setStartForStructure(Lnet/minecraft/world/level/levelgen/structure/Structure;Lnet/minecraft/world/level/levelgen/structure/StructureStart;)V
public getAllStarts()Ljava/util/Map;
public setAllStarts(Ljava/util/Map;)V
public getReferencesForStructure(Lnet/minecraft/world/level/levelgen/structure/Structure;)Lit/unimi/dsi/fastutil/longs/LongSet;
public addReferenceForStructure(Lnet/minecraft/world/level/levelgen/structure/Structure;J)V
public getAllReferences()Ljava/util/Map;
public setAllReferences(Ljava/util/Map;)V
public markUnsaved()V
public canBeSerialized()Z
public tryMarkSaved()Z
public isUnsaved()Z
public getPersistedStatus()Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public removeBlockEntity(Lnet/minecraft/core/BlockPos;)V
public markPosForPostProcessing(Lnet/minecraft/core/BlockPos;)V
public setBlockEntityNbt(Lnet/minecraft/nbt/CompoundTag;)V
public getBlockEntityNbt(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/nbt/CompoundTag;
public getBlockEntityNbtForSaving(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/nbt/CompoundTag;
public findBlocks(Ljava/util/function/Predicate;Ljava/util/function/BiConsumer;)V
public getBlockTicks()Lnet/minecraft/world/ticks/TickContainerAccess;
public getFluidTicks()Lnet/minecraft/world/ticks/TickContainerAccess;
public getTicksForSerialization(J)Lnet/minecraft/world/level/chunk/ChunkAccess$PackedTicks;
public getBlendingData()Lnet/minecraft/world/level/levelgen/blending/BlendingData;
public getWrapped()Lnet/minecraft/world/level/chunk/LevelChunk;
public isLightCorrect()Z
public setLightCorrect(Z)V
public fillBiomesFromNoise(Lnet/minecraft/world/level/biome/BiomeResolver;)V
public initializeLightSources()V
public getSkyLightSources()Lnet/minecraft/world/level/lighting/ChunkSkyLightSources;
```

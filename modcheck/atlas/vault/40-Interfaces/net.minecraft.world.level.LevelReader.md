---
type: "interface"
fqcn: "net.minecraft.world.level.LevelReader"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.LevelReader

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/level/BlockAndLightGetter`, `net/minecraft/world/level/CollisionGetter`, `net/minecraft/world/level/SignalGetter`, `net/minecraft/world/level/biome/BiomeResolver`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBiome` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/Holder;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | declared |

## Declared members (0 fields, 45 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getChunk(IILnet/minecraft/world/level/chunk/status/ChunkStatus;Z)Lnet/minecraft/world/level/chunk/ChunkAccess;
public abstract hasChunk(II)Z
public abstract getHeight(Lnet/minecraft/world/level/levelgen/Heightmap$Types;II)I
public getHeight(Lnet/minecraft/world/level/levelgen/Heightmap$Types;Lnet/minecraft/core/BlockPos;)I
public abstract getSkyDarken()I
public abstract getBiomeManager()Lnet/minecraft/world/level/biome/BiomeManager;
public getBiome(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/Holder;
public getBlockStatesIfLoaded(Lnet/minecraft/world/phys/AABB;)Ljava/util/stream/Stream;
public getNoiseBiome(III)Lnet/minecraft/core/Holder;
public abstract getUncachedNoiseBiome(III)Lnet/minecraft/core/Holder;
public abstract isClientSide()Z
public abstract getSeaLevel()I
public abstract dimensionType()Lnet/minecraft/world/level/dimension/DimensionType;
public getMinY()I
public getHeight()I
public getHeightmapPos(Lnet/minecraft/world/level/levelgen/Heightmap$Types;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
public isEmptyBlock(Lnet/minecraft/core/BlockPos;)Z
public canSeeSkyFromBelowWater(Lnet/minecraft/core/BlockPos;)Z
public getPathfindingCostFromLightLevels(Lnet/minecraft/core/BlockPos;)F
public getLightLevelDependentMagicValue(Lnet/minecraft/core/BlockPos;)F
public getChunk(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/chunk/ChunkAccess;
public getChunk(II)Lnet/minecraft/world/level/chunk/ChunkAccess;
public getChunk(IILnet/minecraft/world/level/chunk/status/ChunkStatus;)Lnet/minecraft/world/level/chunk/ChunkAccess;
public getChunkForCollisions(II)Lnet/minecraft/world/level/BlockGetter;
public isWaterAt(Lnet/minecraft/core/BlockPos;)Z
public containsAnyLiquid(Lnet/minecraft/world/phys/AABB;)Z
public getMaxLocalRawBrightness(Lnet/minecraft/core/BlockPos;)I
public getMaxLocalRawBrightness(Lnet/minecraft/core/BlockPos;I)I
public getEffectiveSkyBrightness(Lnet/minecraft/core/BlockPos;)I
public findBlocksIn(Lnet/minecraft/world/phys/AABB;)Lnet/minecraft/world/level/blockscan/BlockMatcher;
public findBlocksIn(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/blockscan/BlockMatcher;
public findBlocksIn(Ljava/lang/Iterable;)Lnet/minecraft/world/level/blockscan/OrderedBlockMatcher;
public findBlocksInBoxByManhattanDistance(Lnet/minecraft/core/BlockPos;I)Lnet/minecraft/world/level/blockscan/OrderedBlockMatcher;
public findBlocksInBoxByManhattanDistance(Lnet/minecraft/core/BlockPos;II)Lnet/minecraft/world/level/blockscan/OrderedBlockMatcher;
public findBlocksInManhattan(Lnet/minecraft/core/BlockPos;I)Lnet/minecraft/world/level/blockscan/OrderedBlockMatcher;
public hasChunkAt(II)Z
public hasChunkAt(Lnet/minecraft/core/BlockPos;)Z
public hasChunksAt(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;)Z
public hasChunksAt(IIIIII)Z
public hasChunksAt(IIII)Z
public abstract registryAccess()Lnet/minecraft/core/RegistryAccess;
public abstract enabledFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
public holderLookup(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderLookup;
public abstract environmentAttributes()Lnet/minecraft/world/attribute/EnvironmentAttributeReader;
private static synthetic lambda$containsAnyLiquid$0(Lnet/minecraft/world/level/block/state/BlockState;)Z
```

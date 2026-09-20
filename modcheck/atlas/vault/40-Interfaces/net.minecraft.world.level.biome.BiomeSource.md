---
type: "interface"
fqcn: "net.minecraft.world.level.biome.BiomeSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.BiomeSource

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `possibleBiomes` | `()Ljava/util/Set;` | exact | invokevirtual@7 in `BiomeModificationImpl.lambda$finalizeWorldGen$2` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `possibleBiomes` | `()Ljava/util/Set;` | exact | invokevirtual@35 in `BiomeSelectionContextImpl.canGenerateIn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| wraps | `possibleBiomes` | `()Ljava/util/Set;` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (2 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
private final possibleBiomes : Ljava/util/function/Supplier;
protected <init>()V
protected abstract codec()Lcom/mojang/serialization/MapCodec;
protected abstract collectPossibleBiomes()Ljava/util/stream/Stream;
public possibleBiomes()Ljava/util/Set;
public findBiomeHorizontal(IIIILjava/util/function/Predicate;Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/level/levelgen/RandomState;)Lcom/mojang/datafixers/util/Pair;
public findClosestBiome3d(Lnet/minecraft/core/BlockPos;IIILjava/util/function/Predicate;Lnet/minecraft/world/level/levelgen/RandomState;Lnet/minecraft/world/level/LevelReader;)Lcom/mojang/datafixers/util/Pair;
public findBiomeHorizontal(IIIIILjava/util/function/Predicate;Lnet/minecraft/util/RandomSource;ZLnet/minecraft/world/level/levelgen/RandomState;)Lcom/mojang/datafixers/util/Pair;
public final createUncachedResolver(Lnet/minecraft/world/level/levelgen/RandomState;)Lnet/minecraft/world/level/biome/BiomeResolver;
public final createCachingResolver(Lnet/minecraft/world/level/levelgen/RandomState;)Lnet/minecraft/world/level/biome/BiomeResolver;
public abstract createResolver(Lnet/minecraft/world/level/biome/Climate$Sampler;)Lnet/minecraft/world/level/biome/BiomeResolver;
public createResolverForChunk(Lnet/minecraft/world/level/biome/Climate$Sampler;IIIIII)Lnet/minecraft/world/level/biome/BiomeResolver;
public addDebugInfo(Ljava/util/List;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/biome/Climate$Sampler;)V
private synthetic lambda$new$0()Ljava/util/Set;
static <clinit>()V
```

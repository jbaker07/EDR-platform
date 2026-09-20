---
type: "interface"
fqcn: "net.minecraft.world.level.biome.BiomeSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.BiomeSource

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `possibleBiomes()Ljava/util/Set;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `possibleBiomes()Ljava/util/Set;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| wraps | `possibleBiomes` | `@Redirect at INVOKE Ljava/util/function/Supplier;get()Ljava/lang/Object;` | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.world.level.biome.BiomeSource {
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.biome.BiomeSource> CODEC;
    private final java.util.function.Supplier<java.util.Set<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>>> possibleBiomes;
    protected net.minecraft.world.level.biome.BiomeSource();
    protected abstract com.mojang.serialization.MapCodec<? extends net.minecraft.world.level.biome.BiomeSource> codec();
    protected abstract java.util.stream.Stream<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>> collectPossibleBiomes();
    public java.util.Set<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>> possibleBiomes();
    public com.mojang.datafixers.util.Pair<net.minecraft.core.BlockPos, net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>> findBiomeHorizontal(int, int, int, int, java.util.function.Predicate<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>>, net.minecraft.util.RandomSource, net.minecraft.world.level.levelgen.RandomState);
    public com.mojang.datafixers.util.Pair<net.minecraft.core.BlockPos, net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>> findClosestBiome3d(net.minecraft.core.BlockPos, int, int, int, java.util.function.Predicate<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>>, net.minecraft.world.level.levelgen.RandomState, net.minecraft.world.level.LevelReader);
    public com.mojang.datafixers.util.Pair<net.minecraft.core.BlockPos, net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>> findBiomeHorizontal(int, int, int, int, int, java.util.function.Predicate<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>>, net.minecraft.util.RandomSource, boolean, net.minecraft.world.level.levelgen.RandomState);
    public final net.minecraft.world.level.biome.BiomeResolver createUncachedResolver(net.minecraft.world.level.levelgen.RandomState);
    public final net.minecraft.world.level.biome.BiomeResolver createCachingResolver(net.minecraft.world.level.levelgen.RandomState);
    public abstract net.minecraft.world.level.biome.BiomeResolver createResolver(net.minecraft.world.level.biome.Climate$Sampler);
    public net.minecraft.world.level.biome.BiomeResolver createResolverForChunk(net.minecraft.world.level.biome.Climate$Sampler, int, int, int, int, int, int);
    public void addDebugInfo(java.util.List<java.lang.String>, net.minecraft.core.BlockPos, net.minecraft.world.level.biome.Climate$Sampler);
    private java.util.Set lambda$new$0();
    static {};
}
```

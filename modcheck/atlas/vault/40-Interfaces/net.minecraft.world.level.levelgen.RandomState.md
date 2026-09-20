---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.RandomState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.RandomState

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `createClimateSampler` | `(Lnet/minecraft/world/level/levelgen/densityfunction/SamplerContext;)L` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `seed` | `J` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | declared |

## Declared members (12 fields, 17 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final MAX_BUFFER_POOLS : I
private static final MAX_BUFFER_AGE_TICKS : I
private final seed : J
private final random : Lnet/minecraft/world/level/levelgen/PositionalRandomFactory;
private final noises : Lnet/minecraft/core/HolderGetter;
private final router : Lnet/minecraft/world/level/levelgen/NoiseRouter;
private final materialSystem : Lnet/minecraft/world/level/levelgen/material/MaterialSystem;
private final noiseInstances : Ljava/util/Map;
private final positionalRandoms : Ljava/util/Map;
private final densityFunctionCompiler : Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunctionCompiler;
private final densityBufferPoolLock : Ljava/util/concurrent/locks/ReentrantLock;
private final densityBufferPools : Ljava/util/List;
public static create(Lnet/minecraft/core/HolderGetter;JLnet/minecraft/world/level/levelgen/NoiseGeneratorSettings;)Lnet/minecraft/world/level/levelgen/RandomState;
public static create(Lnet/minecraft/core/HolderGetter;JZLnet/minecraft/world/level/block/state/BlockState;ILnet/minecraft/world/level/levelgen/NoiseRouter;)Lnet/minecraft/world/level/levelgen/RandomState;
private <init>(Lnet/minecraft/core/HolderGetter;JZLnet/minecraft/world/level/block/state/BlockState;ILnet/minecraft/world/level/levelgen/NoiseRouter;)V
public samplersWithContext(Lnet/minecraft/world/level/levelgen/densityfunction/SamplerContext;)Lnet/minecraft/world/level/levelgen/densityfunction/DensitySamplerSet;
public createClimateSampler(Lnet/minecraft/world/level/levelgen/densityfunction/SamplerContext;)Lnet/minecraft/world/level/biome/Climate$Sampler;
public getOrCreateNoise(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/level/levelgen/synth/Noise;
public getOrCreateRandomFactory(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/world/level/levelgen/PositionalRandomFactory;
public surfaceSystem()Lnet/minecraft/world/level/levelgen/material/MaterialSystem;
public getSampler(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;)Lnet/minecraft/world/level/levelgen/densityfunction/DensitySampler;
public sampleBlockValueUncached(Lnet/minecraft/world/level/levelgen/densityfunction/DensityFunction;III)F
public acquireDensityBufferPool()Lnet/minecraft/world/level/levelgen/densityfunction/DensityBufferPool;
public releaseDensityBufferPool(Lnet/minecraft/world/level/levelgen/densityfunction/DensityBufferPool;)V
public garbageCollect()V
public seed()J
private static synthetic lambda$garbageCollect$0(Lnet/minecraft/world/level/levelgen/densityfunction/DensityBufferPool;)Z
private synthetic lambda$getOrCreateRandomFactory$0(Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/world/level/levelgen/PositionalRandomFactory;
private synthetic lambda$getOrCreateNoise$0(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/level/levelgen/synth/Noise;
```

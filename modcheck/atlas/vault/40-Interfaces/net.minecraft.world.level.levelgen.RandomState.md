---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.RandomState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.RandomState

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `createClimateSampler` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (29, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.levelgen.RandomState {
    private static final int MAX_BUFFER_POOLS;
    private static final int MAX_BUFFER_AGE_TICKS;
    private final long seed;
    private final net.minecraft.world.level.levelgen.PositionalRandomFactory random;
    private final net.minecraft.core.HolderGetter<net.minecraft.world.level.levelgen.synth.NormalNoise> noises;
    private final net.minecraft.world.level.levelgen.NoiseRouter router;
    private final net.minecraft.world.level.levelgen.material.MaterialSystem materialSystem;
    private final java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.level.levelgen.synth.NormalNoise>, net.minecraft.world.level.levelgen.synth.Noise> noiseInstances;
    private final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.world.level.levelgen.PositionalRandomFactory> positionalRandoms;
    private final net.minecraft.world.level.levelgen.densityfunction.DensityFunctionCompiler densityFunctionCompiler;
    private final java.util.concurrent.locks.ReentrantLock densityBufferPoolLock;
    private final java.util.List<net.minecraft.world.level.levelgen.densityfunction.DensityBufferPool> densityBufferPools;
    public static net.minecraft.world.level.levelgen.RandomState create(net.minecraft.core.HolderGetter<net.minecraft.world.level.levelgen.synth.NormalNoise>, long, net.minecraft.world.level.levelgen.NoiseGeneratorSettings);
    public static net.minecraft.world.level.levelgen.RandomState create(net.minecraft.core.HolderGetter<net.minecraft.world.level.levelgen.synth.NormalNoise>, long, boolean, net.minecraft.world.level.block.state.BlockState, int, net.minecraft.world.level.levelgen.NoiseRouter);
    private net.minecraft.world.level.levelgen.RandomState(net.minecraft.core.HolderGetter<net.minecraft.world.level.levelgen.synth.NormalNoise>, long, boolean, net.minecraft.world.level.block.state.BlockState, int, net.minecraft.world.level.levelgen.NoiseRouter);
    public net.minecraft.world.level.levelgen.densityfunction.DensitySamplerSet samplersWithContext(net.minecraft.world.level.levelgen.densityfunction.SamplerContext);
    public net.minecraft.world.level.biome.Climate$Sampler createClimateSampler(net.minecraft.world.level.levelgen.densityfunction.SamplerContext);
    public net.minecraft.world.level.levelgen.synth.Noise getOrCreateNoise(net.minecraft.resources.ResourceKey<net.minecraft.world.level.levelgen.synth.NormalNoise>);
    public net.minecraft.world.level.levelgen.PositionalRandomFactory getOrCreateRandomFactory(net.minecraft.resources.Identifier);
    public net.minecraft.world.level.levelgen.material.MaterialSystem surfaceSystem();
    public net.minecraft.world.level.levelgen.densityfunction.DensitySampler getSampler(net.minecraft.world.level.levelgen.densityfunction.DensityFunction);
    public float sampleBlockValueUncached(net.minecraft.world.level.levelgen.densityfunction.DensityFunction, int, int, int);
    public net.minecraft.world.level.levelgen.densityfunction.DensityBufferPool acquireDensityBufferPool();
    public void releaseDensityBufferPool(net.minecraft.world.level.levelgen.densityfunction.DensityBufferPool);
    public void garbageCollect();
    public long seed();
    private static boolean lambda$garbageCollect$0(net.minecraft.world.level.levelgen.densityfunction.DensityBufferPool);
    private net.minecraft.world.level.levelgen.PositionalRandomFactory lambda$getOrCreateRandomFactory$0(net.minecraft.resources.Identifier, net.minecraft.resources.Identifier);
    private net.minecraft.world.level.levelgen.synth.Noise lambda$getOrCreateNoise$0(net.minecraft.resources.ResourceKey, net.minecraft.resources.ResourceKey);
}
```

---
type: "interface"
fqcn: "net.minecraft.util.RandomSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.RandomSource

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `createThreadLocalInstance(J)Lnet/minecraft/util/RandomSource;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `createThreadLocalInstance(J)Lnet/minecraft/util/RandomSource;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `createThreadLocalInstance(J)Lnet/minecraft/util/RandomSource;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `nextLong()J` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `nextLong()J` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `setSeed(J)V` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `setSeed(J)V` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `setSeed(J)V` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `setSeed(J)V` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (21, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.util.RandomSource {
    public static final double GAUSSIAN_SPREAD_FACTOR;
    public static net.minecraft.util.RandomSource create();
    public static net.minecraft.util.RandomSource createThreadSafe();
    public static net.minecraft.util.RandomSource create(long);
    public static net.minecraft.util.RandomSource createThreadLocalInstance();
    public static net.minecraft.util.RandomSource createThreadLocalInstance(long);
    public abstract net.minecraft.util.RandomSource fork();
    public abstract net.minecraft.world.level.levelgen.PositionalRandomFactory forkPositional();
    public abstract void setSeed(long);
    public abstract int nextInt();
    public abstract int nextInt(int);
    public default int nextIntBetweenInclusive(int, int);
    public abstract long nextLong();
    public abstract boolean nextBoolean();
    public abstract float nextFloat();
    public abstract double nextDouble();
    public abstract double nextGaussian();
    public default double triangle(double, double);
    public default float triangle(float, float);
    public default void consumeCount(int);
    public default int nextInt(int, int);
}
```

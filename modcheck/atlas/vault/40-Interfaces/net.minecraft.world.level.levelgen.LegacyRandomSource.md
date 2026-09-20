---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.LegacyRandomSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.LegacyRandomSource

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(J)V` | `` | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.levelgen.LegacyRandomSource implements net.minecraft.world.level.levelgen.BitRandomSource {
    private static final int MODULUS_BITS;
    private static final long MODULUS_MASK;
    private static final long MULTIPLIER;
    private static final long INCREMENT;
    private final java.util.concurrent.atomic.AtomicLong seed;
    private final net.minecraft.world.level.levelgen.MarsagliaPolarGaussian gaussianSource;
    public net.minecraft.world.level.levelgen.LegacyRandomSource(long);
    public net.minecraft.util.RandomSource fork();
    public net.minecraft.world.level.levelgen.PositionalRandomFactory forkPositional();
    public void setSeed(long);
    public int next(int);
    public double nextGaussian();
}
```

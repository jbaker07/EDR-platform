---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.WorldgenRandom"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.WorldgenRandom

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/util/RandomSource;)V` | `` | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.levelgen.WorldgenRandom extends net.minecraft.world.level.levelgen.LegacyRandomSource {
    private final net.minecraft.util.RandomSource randomSource;
    private int count;
    public net.minecraft.world.level.levelgen.WorldgenRandom(net.minecraft.util.RandomSource);
    public int getCount();
    public net.minecraft.util.RandomSource fork();
    public net.minecraft.world.level.levelgen.PositionalRandomFactory forkPositional();
    public int next(int);
    public synchronized void setSeed(long);
    public long setDecorationSeed(long, int, int);
    public void setFeatureSeed(long, int, int);
    public void setLargeFeatureSeed(long, int, int);
    public void setLargeFeatureWithSalt(long, int, int, int);
    public static net.minecraft.util.RandomSource seedSlimeChunk(int, int, long, long);
}
```

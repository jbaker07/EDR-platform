---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.synth.PerlinNoise"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.synth.PerlinNoise

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/util/RandomSource;)V` | `` | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `get(DDD)F` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.levelgen.synth.PerlinNoise extends net.minecraft.world.level.levelgen.synth.GradientNoise {
    public static final net.minecraft.util.Interval RANGE;
    public static final double STANDARD_DEVIATION;
    public net.minecraft.world.level.levelgen.synth.PerlinNoise(net.minecraft.util.RandomSource);
    public net.minecraft.util.Interval range();
    public float get(double, double);
    public float get(double, double, double);
    public float noiseWithDerivative(double, double, double, float[]);
    protected float sampleAndLerp(int, int, int, float, float, float, float);
    public void addToVolume(net.minecraft.world.level.levelgen.densityfunction.DensityBuffer, net.minecraft.world.level.levelgen.densityfunction.DensityVolume, double, double, float);
    private float sampleWithDerivative(int, int, int, float, float, float, float[]);
    public void parityConfigString(java.lang.StringBuilder);
    static {};
}
```

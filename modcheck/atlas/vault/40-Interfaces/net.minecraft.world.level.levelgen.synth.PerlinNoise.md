---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.synth.PerlinNoise"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.synth.PerlinNoise

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/levelgen/synth/GradientNoise`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/util/RandomSource;)V` | exact | invokespecial@50 in `ClimateSamplerMixin.fabric_getEndBiomesSampler` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `get` | `(DDD)F` | exact | invokevirtual@6 in `WeightedPicker.pickFromNoise` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (2 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final RANGE : Lnet/minecraft/util/Interval;
public static final STANDARD_DEVIATION : D
public <init>(Lnet/minecraft/util/RandomSource;)V
public range()Lnet/minecraft/util/Interval;
public get(DD)F
public get(DDD)F
public noiseWithDerivative(DDD[F)F
protected sampleAndLerp(IIIFFFF)F
public addToVolume(Lnet/minecraft/world/level/levelgen/densityfunction/DensityBuffer;Lnet/minecraft/world/level/levelgen/densityfunction/DensityVolume;DDF)V
private sampleWithDerivative(IIIFFF[F)F
public parityConfigString(Ljava/lang/StringBuilder;)V
static <clinit>()V
```

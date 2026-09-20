---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.WorldgenRandom"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.WorldgenRandom

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/levelgen/LegacyRandomSource`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/util/RandomSource;)V` | exact | invokespecial@47 in `ClimateSamplerMixin.fabric_getEndBiomesSampler` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (2 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final randomSource : Lnet/minecraft/util/RandomSource;
private count : I
public <init>(Lnet/minecraft/util/RandomSource;)V
public getCount()I
public fork()Lnet/minecraft/util/RandomSource;
public forkPositional()Lnet/minecraft/world/level/levelgen/PositionalRandomFactory;
public next(I)I
public setSeed(J)V
public setDecorationSeed(JII)J
public setFeatureSeed(JII)V
public setLargeFeatureSeed(JII)V
public setLargeFeatureWithSalt(JIII)V
public static seedSlimeChunk(IIJJ)Lnet/minecraft/util/RandomSource;
```

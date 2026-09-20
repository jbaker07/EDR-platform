---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.LegacyRandomSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.LegacyRandomSource

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/level/levelgen/BitRandomSource`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(J)V` | exact | invokespecial@44 in `ClimateSamplerMixin.fabric_getEndBiomesSampler` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (6 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final MODULUS_BITS : I
private static final MODULUS_MASK : J
private static final MULTIPLIER : J
private static final INCREMENT : J
private final seed : Ljava/util/concurrent/atomic/AtomicLong;
private final gaussianSource : Lnet/minecraft/world/level/levelgen/MarsagliaPolarGaussian;
public <init>(J)V
public fork()Lnet/minecraft/util/RandomSource;
public forkPositional()Lnet/minecraft/world/level/levelgen/PositionalRandomFactory;
public setSeed(J)V
public next(I)I
public nextGaussian()D
```

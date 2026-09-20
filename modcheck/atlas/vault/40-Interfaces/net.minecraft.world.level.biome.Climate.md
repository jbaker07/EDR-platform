---
type: "interface"
fqcn: "net.minecraft.world.level.biome.Climate"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.Climate

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `parameters` | `(FFFFFFF)Lnet/minecraft/world/level/biome/Climate$ParameterPoint;` | exact | invokestatic@32 in `NetherBiomes.addNetherBiome` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (3 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final DEBUG_SLOW_BIOME_SEARCH : Z
private static final QUANTIZATION_FACTOR : F
protected static final PARAMETER_COUNT : I
public <init>()V
public static target(FFFFFF)Lnet/minecraft/world/level/biome/Climate$TargetPoint;
public static parameters(FFFFFFF)Lnet/minecraft/world/level/biome/Climate$ParameterPoint;
public static parameters(Lnet/minecraft/world/level/biome/Climate$Parameter;Lnet/minecraft/world/level/biome/Climate$Parameter;Lnet/minecraft/world/level/biome/Climate$Parameter;Lnet/minecraft/world/level/biome/Climate$Parameter;Lnet/minecraft/world/level/biome/Climate$Parameter;Lnet/minecraft/world/level/biome/Climate$Parameter;F)Lnet/minecraft/world/level/biome/Climate$ParameterPoint;
public static quantizeCoord(F)J
public static unquantizeCoord(J)F
```
